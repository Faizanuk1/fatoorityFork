//! Source edits refer to one immutable, already parsed UTF-8 snapshot.
use std::ops::Range;

pub(crate) struct Edit {
    pub range: Range<usize>,
    pub replacement: String,
}

pub(crate) fn apply(source: &str, mut edits: Vec<Edit>) -> Result<String, String> {
    edits.sort_by_key(|edit| (edit.range.start, edit.range.end));
    let mut end = 0;
    let mut previous_start = None;
    let mut output_bytes = source.len();
    for edit in &edits {
        let range = &edit.range;
        if range.start > range.end
            || range.end > source.len()
            || !source.is_char_boundary(range.start)
            || !source.is_char_boundary(range.end)
        {
            return Err("XML edit is outside valid UTF-8 source boundaries".into());
        }
        if range.start < end || previous_start == Some(range.start) {
            return Err("XML edits overlap or share an uncoalesced anchor".into());
        }
        end = range.end;
        previous_start = Some(range.start);
        output_bytes = output_bytes
            .checked_sub(range.len())
            .and_then(|size| size.checked_add(edit.replacement.len()))
            .ok_or("XML edit output exceeds byte limit")?;
    }
    if output_bytes > 8 * 1024 * 1024 {
        return Err("XML edit output exceeds input byte limit".into());
    }
    let mut output = source.to_owned();
    for edit in edits.into_iter().rev() {
        output.replace_range(edit.range, &edit.replacement);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edits_preserve_unowned_trivia_and_use_original_offsets() {
        let source = "أ<!--keep--><x/>\r\n<?keep yes?><y/>";
        let x = source.find("<x/>").unwrap();
        let y = source.find("<y/>").unwrap();
        let output = apply(
            source,
            vec![
                Edit {
                    range: y..y + 4,
                    replacement: "<last/>".into(),
                },
                Edit {
                    range: x..x + 4,
                    replacement: "<first/>".into(),
                },
            ],
        )
        .unwrap();
        assert_eq!(output, "أ<!--keep--><first/>\r\n<?keep yes?><last/>");
    }
    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn invalid_overlapping_and_same_offset_edits_are_rejected() {
        for ranges in [
            vec![1..2],
            vec![0..100],
            vec![Range { start: 3, end: 2 }],
            vec![0..3, 2..4],
            vec![2..2, 2..2],
        ] {
            assert!(
                apply(
                    "أxy",
                    ranges
                        .into_iter()
                        .map(|range| Edit {
                            range,
                            replacement: String::new()
                        })
                        .collect()
                )
                .is_err()
            );
        }
    }
}
