use super::*;
const INVOICE: &str =
    include_str!("../../../tests/fixtures/invoices/sample-simplified-invoice.xml");
const CBC: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";

fn field(name: &str, attributes: &str, value: &str) -> String {
    format!("<cbc:{name} xmlns:cbc=\"{CBC}\" {attributes}>{value}</cbc:{name}>")
}

#[test]
fn decimal_families_preserve_lexical_and_content_constraints() {
    for (name, attrs) in [
        ("PayableAmount", "currencyID=\"SAR\""),
        ("GrossWeightMeasure", "unitCode=\"KGM\""),
        ("Percent", ""),
        ("InvoicedQuantity", "unitCode=\"PCE\""),
    ] {
        for value in [
            "10000000000000000000000000000000000000000",
            "0.00000000000000000000000000000000000000001",
            "+.5",
            "-0",
            "1.",
            " \t12.3\r\n",
            "1<![CDATA[.2]]>3",
            "&#49;.2",
        ] {
            assert!(
                validate(&field(name, attrs, value)).is_ok(),
                "{name}: {value}"
            );
        }
        for value in [
            "",
            " ",
            "+",
            ".",
            "1e40",
            "NaN",
            "INF",
            "1 2",
            "--1",
            "1.2.3",
            "١٢",
            "\u{a0}1",
            "<child>1</child>",
        ] {
            assert!(
                matches!(
                    validate(&field(name, attrs, value)),
                    Err(XmlValidationError::SchemaValidation { .. })
                ),
                "{name}: {value}"
            );
        }
    }
    assert!(validate(&field("PayableAmount", "", "1")).is_err());
    assert!(
        validate(&field(
            "PayableAmount",
            "currencyID=\"SAR\" unexpected=\"x\"",
            "1"
        ))
        .is_err()
    );
}

#[test]
fn effective_decimal_type_is_checked_under_wildcards() {
    let object = |value: &str| {
        format!(
            "<ds:Object xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:cct=\"{CCT}\"><probe xmlns=\"urn:probe\" xsi:type=\"cct:QuantityType\">{value}</probe></ds:Object>"
        )
    };
    assert!(validate(&object("10000000000000000000000000000000000000000")).is_ok());
    assert!(validate(&object("abc")).is_err());
    let string_override = format!(
        "<ds:XPath xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:cct=\"{CCT}\" xsi:type=\"cct:QuantityType\">123</ds:XPath>"
    );
    let string_override = format!(
        "<ds:Transform xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" Algorithm=\"urn:test\">{string_override}</ds:Transform>"
    );
    assert!(validate(&string_override).is_err());
    let invalid_override = field(
        "InvoicedQuantity",
        "xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xsi:type=\"xs:string\"",
        "abc",
    );
    assert!(validate(&invalid_override).is_err());
}

#[test]
fn cached_schema_preserves_public_errors_and_fresh_run_state() {
    validate(INVOICE).unwrap();
    assert!(matches!(
        validate("<not-an-invoice/>"),
        Err(XmlValidationError::SchemaValidation { .. })
    ));
    assert!(matches!(
        validate("<Invoice"),
        Err(XmlValidationError::XmlParse { .. })
    ));
    validate(INVOICE).unwrap();
    for case in [
        "division-large-divisor",
        "division-large-divisor-no-zeros",
        "division-tiny-price",
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "tests/fixtures/business-rules/ksa-arithmetic/cases/{case}/input.xml"
        ));
        validate(&std::fs::read_to_string(path).unwrap()).unwrap();
    }
}

#[test]
fn schema_loader_only_reads_embedded_resources() {
    assert!(!BundledLoader.can_load("https://example.org/schema.xsd"));
    assert!(!BundledLoader.can_load("/etc/passwd"));
    assert!(BundledLoader.load("/fatoora-ubl/../etc/passwd").is_err());
    assert!(
        BundledLoader
            .load("/fatoora-ubl/maindoc/UBL-Invoice-2.1.xsd")
            .is_ok()
    );
}

// A schema update must revisit this adapter if decimals gain value facets,
// defaults, or identity constraints: treating them as strings would change
// numeric equality/order semantics even though lexical validation still passes.
#[test]
fn bundled_decimal_contract_has_no_numeric_facets_or_identity_constraints() {
    use std::collections::{HashMap, HashSet};
    const XSD: &str = "http://www.w3.org/2001/XMLSchema";
    let documents: Vec<_> = RESOURCES
        .iter()
        .map(|(_, bytes)| {
            roxmltree::Document::parse_with_options(
                std::str::from_utf8(bytes).unwrap(),
                roxmltree::ParsingOptions {
                    allow_dtd: true,
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect();
    let qname = |node: roxmltree::Node<'_, '_>, name: &str| {
        let (prefix, local) = name
            .split_once(':')
            .map_or((None, name), |(p, n)| (Some(p), n));
        (
            node.lookup_namespace_uri(prefix).unwrap_or("").to_owned(),
            local.to_owned(),
        )
    };
    let mut types = HashMap::new();
    for doc in &documents {
        let root = doc.root_element();
        for node in root
            .children()
            .filter(|n| n.has_tag_name((XSD, "complexType")) || n.has_tag_name((XSD, "simpleType")))
        {
            types.insert(
                (
                    root.attribute("targetNamespace").unwrap().to_owned(),
                    node.attribute("name").unwrap().to_owned(),
                ),
                node,
            );
        }
        assert!(
            !root
                .descendants()
                .any(|n| n.tag_name().namespace() == Some(XSD)
                    && matches!(n.tag_name().name(), "key" | "keyref" | "unique"))
        );
    }
    let mut decimals = HashSet::from([(XSD.to_owned(), "decimal".to_owned())]);
    loop {
        let before = decimals.len();
        for (name, node) in &types {
            if node.descendants().any(|n| {
                n.attribute("base")
                    .is_some_and(|base| decimals.contains(&qname(n, base)))
            }) {
                decimals.insert(name.clone());
            }
        }
        if decimals.len() == before {
            break;
        }
    }
    let mut roots = Vec::new();
    for (name, node) in &types {
        if !decimals.contains(name) {
            continue;
        }
        for child in node
            .descendants()
            .filter(|n| n.tag_name().namespace() == Some(XSD))
        {
            assert!(
                !matches!(
                    child.tag_name().name(),
                    "minInclusive"
                        | "maxInclusive"
                        | "minExclusive"
                        | "maxExclusive"
                        | "totalDigits"
                        | "fractionDigits"
                        | "pattern"
                        | "enumeration"
                        | "whiteSpace"
                ),
                "facet added to {name:?}"
            );
            if child
                .attribute("base")
                .is_some_and(|base| qname(child, base) == (XSD.to_owned(), "decimal".to_owned()))
            {
                roots.push(name.clone());
            }
        }
    }
    roots.sort();
    let mut expected: Vec<_> = DECIMAL_TYPES
        .iter()
        .map(|name| (CCT.to_owned(), (*name).to_owned()))
        .collect();
    expected.sort();
    assert_eq!(roots, expected);
    let mut fields = 0;
    for doc in &documents {
        for node in doc
            .descendants()
            .filter(|n| n.has_tag_name((XSD, "element")))
        {
            if node
                .attribute("type")
                .is_some_and(|ty| decimals.contains(&qname(node, ty)))
            {
                fields += 1;
                for attr in ["default", "fixed", "nillable"] {
                    assert!(node.attribute(attr).is_none());
                }
            }
        }
    }
    assert_eq!(
        fields, 196,
        "review decimal coverage when bundled schemas change"
    );
    assert_eq!(decimals.len() - 1, 207);
}

#[test]
fn decimal_descendant_of_unknown_lax_extension_is_checked() {
    let xml = format!(
        "<ds:Object xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"><wrapper xmlns=\"urn:probe\">{}</wrapper></ds:Object>",
        field("InvoicedQuantity", "", "abc")
    );
    assert!(validate(&xml).is_err());
}

#[test]
fn certificate_serial_numbers_keep_existing_digit_string_constraint() {
    let serial = |value: &str| {
        format!(
            "<ds:X509Data xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"><ds:X509IssuerSerial><ds:X509IssuerName>issuer</ds:X509IssuerName><ds:X509SerialNumber>{value}</ds:X509SerialNumber></ds:X509IssuerSerial></ds:X509Data>"
        )
    };
    assert!(validate(&serial(&"9".repeat(100))).is_ok());
    assert!(validate(&serial("1.5")).is_err());
}
