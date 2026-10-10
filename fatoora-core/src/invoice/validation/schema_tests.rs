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

#[test]
fn streaming_adapter_rejects_parse_errors_and_missing_roots() {
    let adapter = Adapter::compile().unwrap();
    assert!(matches!(
        adapter.validate("<Invoice"),
        Err(XmlValidationError::XmlParse { .. })
    ));
    let Err(XmlValidationError::SchemaValidation { errors }) = adapter.validate("") else {
        panic!("an empty stream must not count as a valid document");
    };
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message(), "document has no valid UBL root");
    adapter.validate(INVOICE).unwrap();
}

#[test]
fn completion_failure_overrides_valid_root_without_losing_diagnostics() {
    let completion = ValidationError {
        constraint: "validation-resource-limit",
        message: "content model completion exceeded its execution limit".into(),
        location: None,
        element_path: Some("/Invoice".into()),
    };
    let Err(XmlValidationError::SchemaValidation { errors }) = validation_result(
        Some(SchemaValidity::Valid),
        Vec::new(),
        Err(completion),
        vec![Diagnostic::new("invalid decimal value")],
    ) else {
        panic!("a valid root cannot override a failed completion check");
    };
    assert_eq!(errors.len(), 2);
    assert!(errors[0].message().contains("validation-resource-limit"));
    assert!(errors[0].message().contains("/Invoice"));
    assert_eq!(errors[0].severity(), Some(crate::DiagnosticSeverity::Error));
    assert_eq!(errors[1].message(), "invalid decimal value");
}

#[test]
fn cached_compilation_failure_is_reported_and_isolated_to_its_thread() {
    std::thread::spawn(|| {
        let failure = SchemaSetBuilder::with_loader(Box::new(BundledLoader))
            .add("", "/fatoora-ubl/missing.xsd")
            .and_then(|builder| builder.compile())
            .err()
            .expect("an unbundled schema must not compile")
            .to_string();
        ADAPTER.with(|cache| assert!(cache.set(Err(failure.clone())).is_ok()));
        for _ in 0..2 {
            let error = validate(INVOICE).unwrap_err();
            assert_eq!(error.kind(), crate::ErrorKind::Parse);
            let XmlValidationError::SchemaParse { errors } = error else {
                panic!("schema setup errors must preserve their public classification");
            };
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message(), failure);
        }
    })
    .join()
    .unwrap();
    validate(INVOICE).unwrap();
}

#[test]
fn bundled_schema_paths_ignore_host_drive_and_separator_spelling() {
    for (name, expected) in RESOURCES {
        let logical = format!("/fatoora-ubl/{name}");
        for location in [
            logical.clone(),
            format!("C:{logical}"),
            format!("Z:{logical}"),
            format!("D:{logical}").replace('/', "\\"),
        ] {
            assert!(BundledLoader.can_load(&location), "{location}");
            assert_eq!(resource_at(&location).unwrap().1, *expected);
            assert!(BundledLoader.load(&location).is_ok());
        }
    }
    for location in [
        "/other/maindoc/UBL-Invoice-2.1.xsd",
        r"C:\other\maindoc\UBL-Invoice-2.1.xsd",
        "C:fatoora-ubl/maindoc/UBL-Invoice-2.1.xsd",
        "file:///fatoora-ubl/maindoc/UBL-Invoice-2.1.xsd",
        "https://example.com/ubl/maindoc/UBL-Invoice-2.1.xsd",
        "//server/fatoora-ubl/maindoc/UBL-Invoice-2.1.xsd",
        "/fatoora-ubl/../maindoc/UBL-Invoice-2.1.xsd",
        "/fatoora-ubl/common/unknown.xsd",
    ] {
        assert!(!BundledLoader.can_load(location), "{location}");
        assert!(BundledLoader.load(location).is_err(), "{location}");
    }
}

#[test]
fn decimal_overrides_preserve_declared_type_and_attribute_constraints() {
    let xsi = "xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"";
    let huge = "9".repeat(100);
    let same_type = format!("{xsi} currencyID=\"SAR\" xsi:type=\"cbc:PayableAmountType\"");
    validate(&field("PayableAmount", &same_type, &huge)).unwrap();
    for (name, attributes) in [
        (
            "PayableAmount",
            format!(
                "{xsi} xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" currencyID=\"SAR\" xsi:type=\"xs:string\""
            ),
        ),
        (
            "PayableAmount",
            format!("{xsi} xmlns:cct=\"{CCT}\" currencyID=\"SAR\" xsi:type=\"cct:QuantityType\""),
        ),
        (
            "ID",
            format!("{xsi} xmlns:cct=\"{CCT}\" xsi:type=\"cct:QuantityType\""),
        ),
        ("PayableAmount", "cbc:currencyID=\"SAR\"".into()),
    ] {
        assert!(
            matches!(
                validate(&field(name, &attributes, &huge)),
                Err(XmlValidationError::SchemaValidation { .. })
            ),
            "{name}: {attributes}"
        );
    }
}

#[test]
fn decimal_shim_does_not_widen_unrelated_numeric_restrictions() {
    let object = |ty: &str, value: &str| {
        format!(
            "<ds:Object xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\"><probe xmlns=\"urn:probe\" xsi:type=\"xs:{ty}\">{value}</probe></ds:Object>"
        )
    };
    for (ty, valid, invalid) in [
        ("unsignedByte", "255", "256"),
        ("unsignedByte", "0", "-1"),
        ("positiveInteger", "1", "0"),
        ("integer", "123", "1.5"),
        ("decimal", "1.5", "1e2"),
    ] {
        validate(&object(ty, valid)).unwrap();
        assert!(
            matches!(
                validate(&object(ty, invalid)),
                Err(XmlValidationError::SchemaValidation { .. })
            ),
            "{ty}: {invalid}"
        );
    }
}

#[test]
fn decimal_shim_preserves_invoice_order_cardinality_and_namespaces() {
    validate(INVOICE).unwrap();
    let document = roxmltree::Document::parse(INVOICE).unwrap();
    let range = |name: &str| {
        document
            .root_element()
            .children()
            .find(|node| node.has_tag_name((CBC, name)))
            .unwrap()
            .range()
    };
    let issue_date = range("IssueDate");
    let id = range("ID");
    let uuid = range("UUID");
    let mut reordered = INVOICE.to_owned();
    reordered.replace_range(uuid.clone(), &INVOICE[id.clone()]);
    reordered.replace_range(id.clone(), &INVOICE[uuid]);
    let replace = |range: std::ops::Range<usize>, replacement: &str| {
        let mut xml = INVOICE.to_owned();
        xml.replace_range(range, replacement);
        xml
    };
    for xml in [
        replace(issue_date.clone(), ""),
        replace(issue_date.clone(), &INVOICE[issue_date.clone()].repeat(2)),
        replace(
            id,
            "<wrong:ID xmlns:wrong=\"urn:wrong\">SME00010</wrong:ID>",
        ),
        reordered,
        INVOICE.replace(
            "currencyID=\"SAR\">231.15</cbc:PayableAmount>",
            ">231.15</cbc:PayableAmount>",
        ),
    ] {
        assert_ne!(xml, INVOICE);
        assert!(matches!(
            validate(&xml),
            Err(XmlValidationError::SchemaValidation { .. })
        ));
    }
    // Prefix spelling may change while expanded names stay the same.
    validate(
        &INVOICE
            .replace("xmlns:cbc=", "xmlns:basic=")
            .replace("cbc:", "basic:"),
    )
    .unwrap();
}

#[test]
fn decimal_shim_preserves_extension_wildcard_boundaries() {
    const EXT: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2";
    let content = |child: &str| {
        format!("<ext:ExtensionContent xmlns:ext=\"{EXT}\">{child}</ext:ExtensionContent>")
    };
    validate(&content("<probe xmlns=\"urn:probe\"/>")).unwrap();
    validate(&content(&field(
        "PayableAmount",
        "currencyID=\"SAR\"",
        &"9".repeat(100),
    )))
    .unwrap();
    for child in [
        "".to_owned(),
        "<probe xmlns=\"urn:probe\"/><second xmlns=\"urn:probe\"/>".into(),
        "<ext:Unknown/>".into(),
        field("PayableAmount", "currencyID=\"SAR\"", "1e100"),
        field("PayableAmount", "", "123"),
    ] {
        assert!(
            matches!(
                validate(&content(&child)),
                Err(XmlValidationError::SchemaValidation { .. })
            ),
            "{child}"
        );
    }
}
