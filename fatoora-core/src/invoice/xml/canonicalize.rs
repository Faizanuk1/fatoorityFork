//! Digest preimages isolated from application navigation and cryptography.
use super::constants::{CAC_NS, CBC_NS, DS_NS, EXT_NS, XADES_NS};
use super::{
    edit::{self, Edit},
    lexical, nav,
};
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};
use roxmltree::Node;
use std::collections::BTreeMap;
const XML: &str = "http://www.w3.org/XML/1998/namespace";
const OUTPUT_LIMIT: usize = super::constants::MAX_XML_RETAINED_BYTES;

fn excluded(node: Node<'_, '_>) -> bool {
    nav::is(node, EXT_NS, "UBLExtensions")
        || nav::is(node, CAC_NS, "Signature")
        || (nav::is(node, CAC_NS, "AdditionalDocumentReference")
            && node
                .children()
                .any(|id| nav::is(id, CBC_NS, "ID") && nav::content(id) == "QR"))
}

pub(crate) fn prepare_invoice_digest_input(xml: &str) -> Result<Vec<u8>, String> {
    let filtered = {
        let doc = lexical::document(xml).map_err(|e| e.to_string())?;
        let edits = doc
            .descendants()
            .filter(Node::is_element)
            .filter(|node| excluded(*node) && !node.ancestors().skip(1).any(excluded))
            .map(|node| Edit {
                range: node.range(),
                replacement: String::new(),
            })
            .collect();
        edit::apply(xml, edits)?
    };
    // C14N rejects relative namespace URIs, including unused declarations.
    // Check the filtered tree: excluded declarations must not affect the digest.
    let filtered_doc = lexical::document(&filtered).map_err(|e| e.to_string())?;
    let mut checked = std::collections::HashSet::new();
    for node in filtered_doc.descendants().filter(Node::is_element) {
        for namespace in node.namespaces() {
            let uri = namespace.uri();
            if uri.is_empty() || !checked.insert(uri) {
                continue;
            }
            let absolute = uri.split_once(':').is_some_and(|(scheme, _)| {
                scheme
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphabetic)
                    && scheme.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.')
                    })
            });
            if !absolute {
                return Err("canonicalization rejects relative namespace URIs".into());
            }
        }
    }
    xml_sec::c14n::canonicalize_xml_with_backend(
        filtered.as_bytes(),
        &xml_sec::c14n::C14nAlgorithm::new(xml_sec::c14n::C14nMode::Inclusive1_1, false),
        xml_sec::XmlBackend::Roxmltree,
    )
    .map_err(|e| format!("invoice canonicalization: {e}"))
}

pub(crate) fn properties_preimage(xml: &str) -> Result<String, String> {
    let doc = lexical::document(xml).map_err(|e| e.to_string())?;
    let mut selected = doc.descendants().filter(|node| {
        if !nav::is(*node, XADES_NS, "SignedProperties") {
            return false;
        }
        node.ancestors()
            .skip(1)
            .take(3)
            .zip([
                (XADES_NS, "QualifyingProperties"),
                (DS_NS, "Object"),
                (DS_NS, "Signature"),
            ])
            .filter(|(node, (ns, local))| nav::is(*node, ns, local))
            .count()
            == 3
    });
    let node = selected
        .next()
        .ok_or("missing signature SignedProperties subtree")?;
    if selected.next().is_some() {
        return Err("multiple signature SignedProperties subtrees".into());
    }
    let mut inherited = initial_scope();
    if let Some(parent) = node.parent_element() {
        for ns in parent.namespaces() {
            inherited.insert(
                ns.name().unwrap_or_default().to_owned(),
                ns.uri().to_owned(),
            );
        }
    }
    standalone(&xml[node.range()], inherited)
}

fn normalize_eols(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}

type Scope = BTreeMap<String, String>;
type Changes = Vec<(String, Option<String>)>;
fn restore(scope: &mut Scope, changes: Changes) {
    for (prefix, old) in changes.into_iter().rev() {
        if let Some(uri) = old {
            scope.insert(prefix, uri);
        } else {
            scope.remove(&prefix);
        }
    }
}
fn initial_scope() -> Scope {
    [(String::new(), String::new()), ("xml".into(), XML.into())].into()
}

struct Output(String);
impl Output {
    fn push(&mut self, value: &str) -> Result<(), String> {
        if value.len() > OUTPUT_LIMIT.saturating_sub(self.0.len()) {
            return Err("SignedProperties serialization exceeds byte limit".into());
        }
        self.0.push_str(value);
        Ok(())
    }
    fn escaped(&mut self, value: &str, attribute: bool) -> Result<(), String> {
        for c in value.chars() {
            self.push(match c {
                '&' => "&amp;",
                '<' => "&lt;",
                '>' => "&gt;",
                '"' if attribute => "&quot;",
                _ => {
                    let mut bytes = [0; 4];
                    self.push(c.encode_utf8(&mut bytes))?;
                    continue;
                }
            })?;
        }
        Ok(())
    }
    fn namespace(
        &mut self,
        prefix: &str,
        uri: &str,
        scope: &mut Scope,
        changes: &mut Changes,
    ) -> Result<(), String> {
        if scope.get(prefix).is_some_and(|current| current == uri) {
            return Ok(());
        }
        self.push(" xmlns")?;
        if !prefix.is_empty() {
            self.push(":")?;
            self.push(prefix)?;
        }
        self.push("=\"")?;
        // The pinned DOM4J XMLWriter writes namespace URIs directly; normal
        // attributes and text use its separate escaping rules.
        self.push(uri)?;
        self.push("\"")?;
        changes.push((prefix.into(), scope.insert(prefix.into(), uri.into())));
        Ok(())
    }
}

fn utf8(bytes: &[u8]) -> Result<&str, String> {
    std::str::from_utf8(bytes).map_err(|e| e.to_string())
}

fn namespace_prefix(name: &str) -> Option<&str> {
    if name == "xmlns" {
        Some("")
    } else {
        name.strip_prefix("xmlns:")
    }
}

fn start(
    event: &BytesStart<'_>,
    _reader: &Reader<&[u8]>,
    source: &mut Scope,
    emitted: &mut Scope,
    output: &mut Output,
    source_changes: &mut Changes,
    emitted_changes: &mut Changes,
) -> Result<bool, String> {
    let qname = event.name();
    let name = utf8(qname.as_ref())?;
    let attributes: Vec<_> = event
        .attributes()
        .map(|a| {
            let a = a.map_err(|e| e.to_string())?;
            Ok((
                utf8(a.key.as_ref())?.to_owned(),
                quick_xml::escape::unescape(
                    &normalize_eols(utf8(a.value.as_ref())?).replace(['\n', '\t'], " "),
                )
                .map_err(|e| e.to_string())?
                .into_owned(),
            ))
        })
        .collect::<Result<_, String>>()?;
    for (key, value) in &attributes {
        if let Some(prefix) = namespace_prefix(key) {
            source_changes.push((prefix.into(), source.insert(prefix.into(), value.clone())));
        }
    }
    output.push("<")?;
    output.push(name)?;
    let prefix = name.split_once(':').map_or("", |(prefix, _)| prefix);
    let uri = source.get(prefix).ok_or("undeclared element namespace")?;
    output.namespace(prefix, uri, emitted, emitted_changes)?;
    let mut declarations = false;
    for (key, value) in &attributes {
        if let Some(prefix) = namespace_prefix(key) {
            declarations = true;
            output.namespace(prefix, value, emitted, emitted_changes)?;
        }
    }
    for (key, value) in &attributes {
        if namespace_prefix(key).is_some() {
            continue;
        }
        if let Some((prefix, _)) = key.split_once(':') {
            let uri = source.get(prefix).ok_or("undeclared attribute namespace")?;
            output.namespace(prefix, uri, emitted, emitted_changes)?;
        }
        output.push(" ")?;
        output.push(key)?;
        output.push("=\"")?;
        output.escaped(value, true)?;
        output.push("\"")?;
    }
    Ok(declarations)
}

fn standalone(raw: &str, inherited: Scope) -> Result<String, String> {
    let mut reader = Reader::from_str(raw);
    let mut source = inherited;
    let mut emitted = initial_scope();
    let mut stack = vec![];
    let mut output = Output(String::new());
    loop {
        match reader.read_event().map_err(|e| e.to_string())? {
            Event::Start(event) => {
                let mut source_changes = vec![];
                let mut emitted_changes = vec![];
                let declarations = start(
                    &event,
                    &reader,
                    &mut source,
                    &mut emitted,
                    &mut output,
                    &mut source_changes,
                    &mut emitted_changes,
                )?;
                output.push(">")?;
                stack.push((
                    source_changes,
                    emitted_changes,
                    declarations,
                    output.0.len(),
                ));
            }
            Event::Empty(event) => {
                let mut source_changes = vec![];
                let mut emitted_changes = vec![];
                let declarations = start(
                    &event,
                    &reader,
                    &mut source,
                    &mut emitted,
                    &mut output,
                    &mut source_changes,
                    &mut emitted_changes,
                )?;
                restore(&mut source, source_changes);
                restore(&mut emitted, emitted_changes);
                if declarations {
                    // DOM4J counts explicitly declared namespaces as content.
                    output.push("></")?;
                    output.push(utf8(event.name().as_ref())?)?;
                    output.push(">")?;
                } else {
                    output.push("/>")?;
                }
            }
            Event::End(event) => {
                let (source_changes, emitted_changes, declarations, empty_at) =
                    stack.pop().ok_or("unbalanced SignedProperties")?;
                if output.0.len() == empty_at && !declarations {
                    // A DOM serializes an empty element with a self-closing tag.
                    output.0.truncate(empty_at - 1);
                    output.push("/>")?;
                } else {
                    output.push("</")?;
                    output.push(utf8(event.name().as_ref())?)?;
                    output.push(">")?;
                }
                restore(&mut source, source_changes);
                restore(&mut emitted, emitted_changes);
            }
            Event::Text(event) => {
                output.escaped(&event.xml10_content().map_err(|e| e.to_string())?, false)?
            }
            Event::GeneralRef(event) => {
                let name = event.decode().map_err(|e| e.to_string())?;
                let encoded = format!("&{name};");
                let value = quick_xml::escape::unescape(&encoded).map_err(|e| e.to_string())?;
                output.escaped(&value, false)?;
            }
            Event::CData(event) => {
                output.push("<![CDATA[")?;
                output.push(&normalize_eols(utf8(event.as_ref())?))?;
                output.push("]]>")?;
            }
            Event::Comment(event) => {
                output.push("<!--")?;
                output.push(&normalize_eols(utf8(event.as_ref())?))?;
                output.push("-->")?;
            }
            Event::PI(event) => {
                output.push("<?")?;
                output.push(&normalize_eols(utf8(event.as_ref())?))?;
                output.push("?>")?;
            }
            Event::Eof => break,
            _ => return Err("unexpected SignedProperties document event".into()),
        }
    }
    Ok(output.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preimage_preserves_dom_lexical_rules_and_source_prefixes() {
        let input = format!(
            "<r xmlns:p='{XADES_NS}' xmlns:d='{DS_NS}'><d:Signature><d:Object><p:QualifyingProperties><p:SignedProperties Id='foo' note='a\r\nb&#x9;c'><p:Empty></p:Empty><p:Ns xmlns:q='urn:q'></p:Ns><p:Text>أ&amp;\r\n<![CDATA[x\r\ny]]><!--c\r\nd--><?pi x\r\ny?></p:Text></p:SignedProperties></p:QualifyingProperties></d:Object></d:Signature></r>"
        );
        let expected = format!(
            "<p:SignedProperties xmlns:p=\"{XADES_NS}\" Id=\"foo\" note=\"a b\tc\"><p:Empty/><p:Ns xmlns:q=\"urn:q\"></p:Ns><p:Text>أ&amp;\n<![CDATA[x\ny]]><!--c\nd--><?pi x\ny?></p:Text></p:SignedProperties>"
        );
        assert_eq!(properties_preimage(&input).unwrap(), expected);
    }
    #[test]
    fn range_exclusions_equal_frozen_reference_dom_bytes() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/xml-migration");
        for case in [
            "namespace-axis",
            "inherited-xml",
            "processing-instructions",
            "nested-exclusions",
            "foreign-and-lexical",
        ] {
            let input = std::fs::read_to_string(root.join(case).join("input.xml")).unwrap();
            let expected = std::fs::read(root.join(case).join("canonical.xml")).unwrap();
            assert_eq!(
                prepare_invoice_digest_input(&input).unwrap(),
                expected,
                "{case}"
            );
        }
    }
}
