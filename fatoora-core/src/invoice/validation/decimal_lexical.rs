//! Exact xs:decimal lexical validation, independent of numeric storage capacity.

pub(super) fn valid(value: &str) -> bool {
    let value = value.trim_matches([' ', '\t', '\r', '\n']);
    unsigned(value.strip_prefix(['+', '-']).unwrap_or(value))
}

pub(super) fn unsigned(value: &str) -> bool {
    let mut point = false;
    let mut digit = false;
    for byte in value.bytes() {
        if byte.is_ascii_digit() {
            digit = true;
        } else if byte == b'.' && !point {
            point = true;
        } else {
            return false;
        }
    }
    digit
}
