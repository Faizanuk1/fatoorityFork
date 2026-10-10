//! Optional maintenance oracle. Never linked into the library or normal tests.
use libxml::{parser::Parser, tree::c14n, xpath::Context};

fn main() {
    let path = std::env::args().nth(1).expect("input XML path");
    let xml = std::fs::read(path).expect("read input");
    let doc = Parser::default().parse_string(xml).expect("parse input");
    let ctx = Context::new(&doc).unwrap();
    for (prefix, namespace) in [
        (
            "ext",
            "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2",
        ),
        (
            "cac",
            "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
        ),
        (
            "cbc",
            "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2",
        ),
    ] {
        ctx.register_namespace(prefix, namespace).unwrap();
    }
    // Union gives document order; descendants are freed before their ancestors.
    let nodes = ctx.evaluate("//ext:UBLExtensions | //cac:Signature | //cac:AdditionalDocumentReference[cbc:ID='QR']")
        .unwrap().get_nodes_as_vec();
    for mut node in nodes.into_iter().rev() {
        node.unlink();
    }
    let canonical = doc
        .canonicalize(
            c14n::CanonicalizationOptions {
                mode: c14n::CanonicalizationMode::Canonical1_1,
                inclusive_ns_prefixes: vec![],
                with_comments: false,
            },
            None,
        )
        .expect("C14N 1.1");
    print!("{canonical}");
}
