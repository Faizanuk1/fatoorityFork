//! Test-only selectors and mutations, independent of production XML helpers.
use roxmltree::{Document, Node};

fn namespace(prefix: &str) -> &str {
    match prefix {
        "ubl" => "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2",
        "cbc" => "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2",
        "cac" => "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
        "ext" => "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2",
        "ds" => "http://www.w3.org/2000/09/xmldsig#",
        "xades" => "http://uri.etsi.org/01903/v1.3.2#",
        _ => panic!("unsupported test namespace: {prefix}"),
    }
}
fn named(node: Node<'_, '_>, name: &str) -> bool {
    if !node.is_element() {
        return false;
    }
    if name == "*" {
        return true;
    }
    let (prefix, local) = name.split_once(':').expect("qualified test name");
    node.tag_name().namespace() == Some(namespace(prefix)) && node.tag_name().name() == local
}
pub fn content(node: Node<'_, '_>) -> String {
    node.descendants()
        .filter(Node::is_text)
        .filter_map(|node| node.text())
        .collect()
}

/// Deliberately supports only the fixed paths used by regression tests.
pub fn select<'a, 'input>(doc: &'a Document<'input>, path: &str) -> Vec<Node<'a, 'input>> {
    if let Some(group) = path.strip_prefix('(') {
        let (inner, index) = group.rsplit_once(")[").expect("fixed indexed path");
        let index = index.trim_end_matches(']').parse::<usize>().unwrap();
        return select(doc, inner)
            .into_iter()
            .nth(index - 1)
            .into_iter()
            .collect();
    }
    if path.contains("local-name()") {
        if path.contains("AdditionalDocumentReference") && path.contains("='QR'") {
            return doc
                .descendants()
                .filter(|node| named(*node, "cac:AdditionalDocumentReference"))
                .filter(|node| {
                    node.children()
                        .any(|child| named(child, "cbc:ID") && content(child) == "QR")
                })
                .flat_map(|node| node.descendants())
                .filter(|node| named(*node, "cbc:EmbeddedDocumentBinaryObject"))
                .collect();
        }
        let local = path
            .split("local-name()='")
            .nth(1)
            .unwrap()
            .split('\'')
            .next()
            .unwrap();
        return doc
            .descendants()
            .filter(|node| node.is_element() && node.tag_name().name() == local)
            .collect();
    }
    let mut nodes = vec![doc.root()];
    let mut rest = path;
    while !rest.is_empty() {
        let descend = rest.starts_with("//");
        rest = rest
            .strip_prefix(if descend { "//" } else { "/" })
            .unwrap_or_else(|| panic!("unsupported fixed test path {path}: {rest}"));
        let end = rest.find('/').unwrap_or(rest.len());
        let step = &rest[..end];
        rest = &rest[end..];
        if step.starts_with('@') {
            break;
        }
        let (name, predicate) = step
            .split_once('[')
            .map_or((step, None), |(name, predicate)| {
                (name, Some(predicate.trim_end_matches(']')))
            });
        let mut next = Vec::new();
        for parent in nodes {
            let candidates: Vec<_> = if descend {
                parent.descendants().skip(1).collect()
            } else {
                parent.children().collect()
            };
            let mut matches: Vec<_> = candidates
                .into_iter()
                .filter(|node| named(*node, name))
                .collect();
            if let Some(predicate) = predicate {
                if let Ok(index) = predicate.parse::<usize>() {
                    matches = matches.into_iter().nth(index - 1).into_iter().collect();
                } else {
                    let (left, right) =
                        predicate.split_once('=').expect("fixed equality predicate");
                    let value = right.trim_matches('\'');
                    matches.retain(|node| {
                        if let Some(attr) = left.strip_prefix('@') {
                            node.attribute(attr) == Some(value)
                        } else {
                            node.children()
                                .any(|child| named(child, left) && content(child) == value)
                        }
                    });
                }
            }
            next.extend(matches);
        }
        nodes = next;
    }
    nodes
}

pub fn text(doc: &Document<'_>, path: &str) -> Result<String, String> {
    let nodes = select(doc, path);
    if nodes.len() != 1 {
        return Err(format!("expected one {path}, found {}", nodes.len()));
    }
    if let Some((_, attr)) = path.rsplit_once("/@") {
        nodes[0]
            .attribute(attr)
            .map(str::to_owned)
            .ok_or_else(|| format!("missing {attr}"))
    } else {
        Ok(content(nodes[0]))
    }
}
pub fn replace_text(xml: &str, path: &str, value: &str) -> String {
    let doc = Document::parse(xml).unwrap();
    let nodes = select(&doc, path);
    assert_eq!(nodes.len(), 1, "mutation must select one field: {path}");
    let node = nodes[0];
    let range = if let Some((_, attr)) = path.rsplit_once("/@") {
        node.attributes()
            .find(|attribute| attribute.name() == attr)
            .unwrap()
            .range_value()
    } else {
        let range = node.range();
        let raw = &xml[range.clone()];
        range.start + raw.find('>').unwrap() + 1..range.start + raw.rfind("</").unwrap()
    };
    let mut output = xml.to_owned();
    output.replace_range(range, &quick_xml::escape::escape(value));
    output
}
