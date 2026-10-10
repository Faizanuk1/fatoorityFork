//! Owned UBL signing structures and deterministic source edits.
use super::{
    constants::*,
    edit::{self, Edit},
    lexical,
    nav::{self, Element},
};
use base64ct::{Base64, Encoding};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;

const EXTENSION_URI: &str = "urn:oasis:names:specification:ubl:dsig:enveloped:xades";
const SIGNATURE_ID: &str = "urn:oasis:names:specification:ubl:signature:Invoice";

pub(crate) struct SigningFragments {
    pub extension: String,
    pub qr: String,
    pub signature: String,
}

fn singleton<'a, 'input>(
    nodes: impl Iterator<Item = Node<'a, 'input>>,
) -> Result<Option<Node<'a, 'input>>, String> {
    let mut nodes = nodes;
    let first = nodes.next();
    if nodes.next().is_some() {
        return Err("ambiguous repeated signing structure".into());
    }
    Ok(first)
}
fn children<'a, 'input>(
    node: Node<'a, 'input>,
    ns: &'static str,
    name: &'static str,
) -> impl Iterator<Item = Node<'a, 'input>> {
    node.children().filter(move |node| nav::is(*node, ns, name))
}

struct Owned<'a, 'input> {
    extensions: Option<Node<'a, 'input>>,
    extension: Option<Node<'a, 'input>>,
    qr: Option<Node<'a, 'input>>,
    signature: Option<Node<'a, 'input>>,
}
fn owned<'a, 'input>(doc: &'a Document<'input>) -> Result<Owned<'a, 'input>, String> {
    let root = doc.root_element();
    if !nav::is(root, INVOICE_NS, "Invoice") {
        return Err("expected a UBL Invoice root".into());
    }
    let extensions = singleton(children(root, EXT_NS, "UBLExtensions"))?;
    let extension = if let Some(wrapper) = extensions {
        singleton(children(wrapper, EXT_NS, "UBLExtension").filter(|node| {
            children(*node, EXT_NS, "ExtensionURI").any(|uri| nav::content(uri) == EXTENSION_URI)
        }))?
    } else {
        None
    };
    let qr = singleton(
        children(root, CAC_NS, "AdditionalDocumentReference")
            .filter(|node| children(*node, CBC_NS, "ID").any(|id| nav::content(id) == "QR")),
    )?;
    let signature = singleton(children(root, CAC_NS, "Signature"))?;
    if let Some(node) = signature {
        let id = singleton(children(node, CBC_NS, "ID"))?.ok_or("missing owned signature ID")?;
        let method = singleton(children(node, CBC_NS, "SignatureMethod"))?
            .ok_or("missing owned signature method")?;
        if nav::content(id) != SIGNATURE_ID || nav::content(method) != EXTENSION_URI {
            return Err("unsupported externally owned UBL signature".into());
        }
    }
    if let Some(node) = qr
        && (children(node, CBC_NS, "ID").count() != 1
            || children(node, CAC_NS, "Attachment").count() != 1)
    {
        return Err("unsupported QR reference structure".into());
    }
    let signatures: Vec<_> = doc
        .descendants()
        .filter(|node| nav::is(*node, DS_NS, "Signature"))
        .collect();
    if signatures.len() > usize::from(extension.is_some())
        || signatures
            .iter()
            .any(|node| !node.ancestors().any(|parent| Some(parent) == extension))
    {
        return Err("unsupported externally owned XML signature".into());
    }
    if let Some(extension) = extension {
        let signature = signatures
            .first()
            .ok_or("owned signature extension is incomplete")?;
        for (ns, local, expected) in [
            (DS_NS, "KeyInfo", 1),
            (DS_NS, "X509Data", 1),
            (DS_NS, "DigestValue", 3),
            (DS_NS, "DigestMethod", 3),
            (DS_NS, "SignedInfo", 1),
            (DS_NS, "SignatureValue", 1),
            (DS_NS, "Object", 1),
            (DS_NS, "Reference", 2),
            (SAC_NS, "SignatureInformation", 1),
            (XADES_NS, "QualifyingProperties", 1),
            (XADES_NS, "SignedProperties", 1),
            (XADES_NS, "SignedSignatureProperties", 1),
        ] {
            if extension
                .descendants()
                .filter(|node| nav::is(*node, ns, local))
                .count()
                != expected
            {
                return Err("incomplete or ambiguous owned signing structure".into());
            }
        }
        let info = signature
            .parent_element()
            .ok_or("missing SignatureInformation")?;
        let container = info
            .parent_element()
            .ok_or("missing UBLDocumentSignatures")?;
        let content = container
            .parent_element()
            .ok_or("missing ExtensionContent")?;
        if !nav::is(container, SIG_NS, "UBLDocumentSignatures")
            || !nav::is(content, EXT_NS, "ExtensionContent")
            || content.parent_element() != Some(extension)
            || children(extension, EXT_NS, "ExtensionContent").count() != 1
            || children(content, SIG_NS, "UBLDocumentSignatures").count() != 1
            || !nav::is(info, SAC_NS, "SignatureInformation")
            || children(info, CBC_NS, "ID")
                .filter(|id| nav::content(*id) == "urn:oasis:names:specification:ubl:signature:1")
                .count()
                != 1
            || children(info, SBC_NS, "ReferencedSignatureID")
                .filter(|id| nav::content(*id) == SIGNATURE_ID)
                .count()
                != 1
            || signature.attribute("Id") != Some("signature")
            || children(extension, EXT_NS, "ExtensionURI").count() != 1
        {
            return Err("unsupported owned signature extension".into());
        }
    }
    Ok(Owned {
        extensions,
        extension,
        qr,
        signature,
    })
}

pub(crate) fn signing_time(xml: &str, certificate: &[u8]) -> Result<String, String> {
    let doc = lexical::document(xml).map_err(|e| e.to_string())?;
    let owned = owned(&doc)?;
    let invoice = Element::invoice(&doc);
    if owned.extension.is_some() {
        let signature = nav::signature(invoice)
            .0
            .ok_or("incomplete owned signature")?;
        let certificates: Vec<_> = signature
            .descendants()
            .filter(|node| nav::is(*node, DS_NS, "X509Certificate"))
            .collect();
        if certificates.len() != 1 {
            return Err("expected one owned signing certificate".into());
        }
        let compact: String = nav::content(certificates[0])
            .chars()
            .filter(|c| !matches!(c, ' ' | '\t' | '\n' | '\r'))
            .collect();
        if compact.is_empty() {
            if signature
                .descendants()
                .filter(|node| {
                    nav::is(*node, DS_NS, "SignatureValue")
                        || nav::is(*node, DS_NS, "DigestValue")
                        || nav::is(*node, DS_NS, "X509IssuerName")
                        || nav::is(*node, DS_NS, "X509SerialNumber")
                        || nav::is(*node, XADES_NS, "SigningTime")
                })
                .any(|node| !nav::content(node).trim().is_empty())
            {
                return Err("populated signature has no signing certificate".into());
            }
        } else if signature
            .descendants()
            .filter(|node| {
                nav::is(*node, DS_NS, "SignatureValue") || nav::is(*node, DS_NS, "DigestValue")
            })
            .any(|node| nav::content(node).trim().is_empty())
        {
            return Err("populated certificate has incomplete signature material".into());
        } else if Base64::decode_vec(&compact).map_err(|_| "malformed owned certificate")?
            != certificate
        {
            return Err("existing signature uses a different certificate".into());
        }
    }
    let properties = nav::signed_properties(invoice);
    let times: Vec<_> = properties
        .child(XADES_NS, "SignedSignatureProperties")
        .children(XADES_NS, "SigningTime")
        .collect();
    if times.len() > 1 {
        return Err("multiple owned signing times".into());
    }
    if let Some(value) = times
        .first()
        .and_then(|node| node.content())
        .filter(|value| !value.trim().is_empty())
    {
        let parsed = chrono::NaiveDateTime::parse_from_str(value.trim(), "%Y-%m-%dT%H:%M:%S")
            .map_err(|_| "Invalid signing time")?;
        return Ok(parsed.format("%Y-%m-%dT%H:%M:%S").to_string());
    }
    let date = invoice
        .basic("IssueDate")
        .content()
        .ok_or("missing issue date")?;
    let time = invoice
        .basic("IssueTime")
        .content()
        .ok_or("missing issue time")?;
    let date = chrono::NaiveDate::parse_from_str(date.trim(), "%Y-%m-%d")
        .map_err(|_| "invalid issue date")?;
    let time = chrono::NaiveTime::parse_from_str(time.trim(), "%H:%M:%S")
        .map_err(|_| "invalid issue time")?;
    Ok(date.and_time(time).format("%Y-%m-%dT%H:%M:%S").to_string())
}

/// Render text with SDK preimage escaping, preserving quotes and apostrophes.
pub(crate) fn text(value: &str) -> String {
    let mut writer = quick_xml::Writer::new(Vec::new());
    writer
        .write_event(quick_xml::events::Event::Text(
            quick_xml::events::BytesText::from_escaped(
                quick_xml::escape::minimal_escape(value).replace('>', "&gt;"),
            ),
        ))
        .expect("Vec writer");
    String::from_utf8(writer.into_inner()).expect("UTF-8 XML text")
}

pub(crate) fn render(
    properties: &str,
    hash: &str,
    signature: &str,
    properties_hash: &str,
    certificate: &str,
    qr: &str,
) -> Result<SigningFragments, String> {
    render_using(
        Templates {
            extension: UBL_EXTENSIONS_TEMPLATE,
            qr: QR_REFERENCE_TEMPLATE,
            signature: CAC_SIGNATURE_TEMPLATE,
        },
        properties,
        hash,
        signature,
        properties_hash,
        certificate,
        qr,
    )
}

struct Templates<'a> {
    extension: &'a str,
    qr: &'a str,
    signature: &'a str,
}

fn replace_exactly(
    source: &str,
    needle: &str,
    replacement: &str,
    expected: usize,
) -> Result<String, String> {
    if source.matches(needle).count() != expected {
        return Err("signing template placeholder is missing or repeated".into());
    }
    Ok(source.replace(needle, replacement))
}

fn replace_once(source: &str, needle: &str, replacement: &str) -> Result<String, String> {
    replace_exactly(source, needle, replacement, 1)
}

fn render_using(
    templates: Templates<'_>,
    properties: &str,
    hash: &str,
    signature: &str,
    properties_hash: &str,
    certificate: &str,
    qr: &str,
) -> Result<SigningFragments, String> {
    let doc = lexical::document(templates.extension).map_err(|e| e.to_string())?;
    let mut counts = BTreeMap::new();
    let mut edits = Vec::new();
    for node in doc.descendants().filter(Node::is_element) {
        let value = if nav::is(node, DS_NS, "SignatureValue") {
            Some(signature)
        } else if nav::is(node, DS_NS, "X509Certificate") {
            Some(certificate)
        } else if nav::is(node, DS_NS, "DigestValue") {
            match node
                .parent_element()
                .and_then(|parent| parent.attribute("Id"))
            {
                Some("invoiceSignedData") => {
                    if node.parent_element().and_then(|n| n.attribute("URI")) != Some("") {
                        return Err("unexpected invoice reference in signing template".into());
                    }
                    Some(hash)
                }
                _ if node
                    .parent_element()
                    .is_some_and(|parent| nav::is(parent, DS_NS, "Reference")) =>
                {
                    if node.parent_element().and_then(|n| n.attribute("URI"))
                        != Some("#xadesSignedProperties")
                    {
                        return Err("unexpected properties reference in signing template".into());
                    }
                    Some(properties_hash)
                }
                _ => None,
            }
        } else {
            None
        };
        if let Some(value) = value {
            if !nav::content(node).trim().is_empty() {
                return Err("signing template scalar placeholder is populated".into());
            }
            let slot = match node.tag_name().name() {
                "DigestValue"
                    if node.parent_element().and_then(|n| n.attribute("Id"))
                        == Some("invoiceSignedData") =>
                {
                    "invoice digest"
                }
                "DigestValue" => "properties digest",
                other => other,
            };
            *counts.entry(slot).or_insert(0usize) += 1;
            let local = node.tag_name().name();
            edits.push(Edit {
                range: node.range(),
                replacement: format!("<ds:{local}>{}</ds:{local}>", text(value)),
            });
        } else if nav::is(node, XADES_NS, "SignedProperties") {
            // The standalone preimage supplies declarations; the embedded XML
            // inherits them so DOM4J retains its empty-element representation.
            *counts.entry("properties").or_insert(0usize) += 1;
            let embedded = replace_once(properties, &format!(" xmlns:xades=\"{XADES_NS}\""), "")?;
            let embedded = replace_exactly(&embedded, &format!(" xmlns:ds=\"{DS_NS}\""), "", 4)?;
            edits.push(Edit {
                range: node.range(),
                replacement: embedded,
            });
        }
    }
    if [
        "SignatureValue",
        "X509Certificate",
        "invoice digest",
        "properties digest",
        "properties",
    ]
    .iter()
    .any(|slot| counts.get(slot) != Some(&1))
    {
        return Err("signing template slots are missing or repeated".into());
    }
    let wrapper = edit::apply(templates.extension, edits)?;
    let doc = lexical::document(&wrapper).map_err(|e| e.to_string())?;
    if !nav::is(doc.root_element(), EXT_NS, "UBLExtensions") {
        return Err("unexpected signing template root".into());
    }
    let node = singleton(doc.root_element().children().filter(Node::is_element))?
        .filter(|node| nav::is(*node, EXT_NS, "UBLExtension"))
        .ok_or("missing generated extension")?;
    let raw = &wrapper[node.range()];
    let declarations = format!(
        " xmlns:ext=\"{EXT_NS}\" xmlns:cbc=\"{CBC_NS}\" xmlns:sig=\"{SIG_NS}\" xmlns:sac=\"{SAC_NS}\" xmlns:sbc=\"{SBC_NS}\" xmlns:ds=\"{DS_NS}\" xmlns:xades=\"{XADES_NS}\""
    );
    let extension = replace_once(
        raw,
        "<ext:UBLExtension>",
        &format!("<ext:UBLExtension{declarations}>"),
    )?;
    let qr = replace_once(
        templates.qr.trim_end(),
        "<cbc:EmbeddedDocumentBinaryObject mimeCode=\"text/plain\"></cbc:EmbeddedDocumentBinaryObject>",
        &format!(
            "<cbc:EmbeddedDocumentBinaryObject mimeCode=\"text/plain\">{}</cbc:EmbeddedDocumentBinaryObject>",
            text(qr)
        ),
    )?;
    let fragments = SigningFragments {
        extension,
        qr,
        signature: templates.signature.trim_end().to_owned(),
    };
    let generated = format!(
        "<Invoice xmlns=\"{INVOICE_NS}\"><ext:UBLExtensions xmlns:ext=\"{EXT_NS}\">{}</ext:UBLExtensions>{}{} </Invoice>",
        fragments.extension, fragments.qr, fragments.signature
    );
    let doc = lexical::document(&generated).map_err(|e| e.to_string())?;
    let selected = owned(&doc)?;
    if selected.extension.is_none() || selected.qr.is_none() || selected.signature.is_none() {
        return Err("generated signing template structures are missing".into());
    }
    Ok(fragments)
}

fn rank(node: Node<'_, '_>) -> Option<usize> {
    let ns = node.tag_name().namespace()?;
    if ns == EXT_NS && node.tag_name().name() == "UBLExtensions" {
        return Some(0);
    }
    let names: &[&str] = if ns == CBC_NS {
        &[
            "UBLVersionID",
            "CustomizationID",
            "ProfileID",
            "ProfileExecutionID",
            "ID",
            "CopyIndicator",
            "UUID",
            "IssueDate",
            "IssueTime",
            "DueDate",
            "InvoiceTypeCode",
            "Note",
            "TaxPointDate",
            "DocumentCurrencyCode",
            "TaxCurrencyCode",
            "PricingCurrencyCode",
            "PaymentCurrencyCode",
            "PaymentAlternativeCurrencyCode",
            "AccountingCostCode",
            "AccountingCost",
            "LineCountNumeric",
            "BuyerReference",
        ]
    } else if ns == CAC_NS {
        &[
            "InvoicePeriod",
            "OrderReference",
            "BillingReference",
            "DespatchDocumentReference",
            "ReceiptDocumentReference",
            "StatementDocumentReference",
            "OriginatorDocumentReference",
            "ContractDocumentReference",
            "AdditionalDocumentReference",
            "ProjectReference",
            "Signature",
            "AccountingSupplierParty",
            "AccountingCustomerParty",
            "PayeeParty",
            "BuyerCustomerParty",
            "SellerSupplierParty",
            "TaxRepresentativeParty",
            "Delivery",
            "DeliveryTerms",
            "PaymentMeans",
            "PaymentTerms",
            "PrepaidPayment",
            "AllowanceCharge",
            "TaxExchangeRate",
            "PricingExchangeRate",
            "PaymentExchangeRate",
            "PaymentAlternativeExchangeRate",
            "TaxTotal",
            "WithholdingTaxTotal",
            "LegalMonetaryTotal",
            "InvoiceLine",
        ]
    } else {
        return None;
    };
    names
        .iter()
        .position(|name| *name == node.tag_name().name())
        .map(|index| index + if ns == CBC_NS { 1 } else { 100 })
}
fn close_offset(source: &str, node: Node<'_, '_>) -> Result<usize, String> {
    let range = node.range();
    source[range.clone()]
        .rfind("</")
        .map(|offset| range.start + offset)
        .ok_or_else(|| "cannot insert into a self-closing container".into())
}
fn anchor(source: &str, root: Node<'_, '_>, target: usize) -> Result<usize, String> {
    let mut last_rank = 0;
    let mut predecessor = None;
    let mut successor = None;
    for node in root.children().filter(Node::is_element) {
        if let Some(order) = rank(node) {
            if order < last_rank {
                return Err("existing UBL elements are out of order".into());
            }
            last_rank = order;
            if order <= target {
                predecessor = Some(node.range().end);
            } else if successor.is_none() {
                successor = Some(node.range().start);
            }
        }
    }
    successor
        .or(predecessor)
        .map(Ok)
        .unwrap_or_else(|| close_offset(source, root))
}

pub(crate) fn apply_signature_fragments(
    xml: &str,
    fragments: &SigningFragments,
) -> Result<String, String> {
    let doc = lexical::document(xml).map_err(|e| e.to_string())?;
    let owned = owned(&doc)?;
    let root = doc.root_element();
    // Validate ordering even when every owned structure is being replaced.
    anchor(xml, root, 0)?;
    let mut edits = Vec::new();
    let mut insertions: BTreeMap<usize, String> = BTreeMap::new();
    if let Some(extension) = owned.extension {
        edits.push(Edit {
            range: extension.range(),
            replacement: fragments.extension.clone(),
        });
    } else if let Some(wrapper) = owned.extensions {
        if xml[wrapper.range()].trim_end().ends_with("/>") {
            let raw = &xml[wrapper.range()];
            let name = raw[1..]
                .split(|c: char| c.is_ascii_whitespace() || matches!(c, '/' | '>'))
                .next()
                .ok_or("missing wrapper QName")?;
            edits.push(Edit {
                range: wrapper.range(),
                replacement: format!("{}>{}</{name}>", &raw[..raw.len() - 2], fragments.extension),
            });
        } else {
            insertions
                .entry(close_offset(xml, wrapper)?)
                .or_default()
                .push_str(&fragments.extension);
        }
    } else {
        insertions
            .entry(anchor(xml, root, 0)?)
            .or_default()
            .push_str(&format!(
                "<ext:UBLExtensions xmlns:ext=\"{EXT_NS}\">{}</ext:UBLExtensions>",
                fragments.extension
            ));
    }
    for (existing, fragment, order) in [
        (owned.qr, &fragments.qr, 108),
        (owned.signature, &fragments.signature, 110),
    ] {
        if let Some(node) = existing {
            edits.push(Edit {
                range: node.range(),
                replacement: fragment.clone(),
            });
        } else {
            insertions
                .entry(anchor(xml, root, order)?)
                .or_default()
                .push_str(fragment);
        }
    }
    for (at, prefix) in insertions {
        if let Some(edit) = edits.iter_mut().find(|edit| edit.range.start == at) {
            edit.replacement.insert_str(0, &prefix);
        } else {
            edits.push(Edit {
                range: at..at,
                replacement: prefix,
            });
        }
    }
    edit::apply(xml, edits)
}

#[cfg(test)]
mod render_tests {
    use super::*;
    const PROPERTIES: &str = include_str!(
        "../../../tests/fixtures/sdk-parity/cases/simplified-credit/signed-properties.xml"
    );

    fn using(
        extension: &str,
        qr: &str,
        signature: &str,
        properties: &str,
    ) -> Result<SigningFragments, String> {
        render_using(
            Templates {
                extension,
                qr,
                signature,
            },
            properties,
            "invoice hash",
            "signature value",
            "properties hash",
            "cert<&>",
            "qr<&>",
        )
    }

    #[test]
    fn rendered_fragments_bind_intended_values_and_namespace_scopes() {
        let fragments = using(
            UBL_EXTENSIONS_TEMPLATE,
            QR_REFERENCE_TEMPLATE,
            CAC_SIGNATURE_TEMPLATE,
            PROPERTIES,
        )
        .unwrap();
        let doc = Document::parse(&fragments.extension).unwrap();
        let select = |ns, local| {
            doc.descendants()
                .filter(move |n| nav::is(*n, ns, local))
                .collect::<Vec<_>>()
        };
        let refs = select(DS_NS, "Reference");
        assert_eq!(refs.len(), 2);
        for (uri, digest) in [
            ("", "invoice hash"),
            ("#xadesSignedProperties", "properties hash"),
        ] {
            let reference = refs
                .iter()
                .find(|n| n.attribute("URI") == Some(uri))
                .unwrap();
            assert_eq!(
                nav::content(children(*reference, DS_NS, "DigestValue").next().unwrap()),
                digest
            );
        }
        assert_eq!(select(DS_NS, "X509Certificate").len(), 1);
        assert_eq!(nav::content(select(DS_NS, "X509Certificate")[0]), "cert<&>");
        assert_eq!(select(XADES_NS, "SignedProperties").len(), 1);
        assert_eq!(fragments.extension.matches("xmlns:ds=").count(), 1);
        assert_eq!(fragments.extension.matches("xmlns:xades=").count(), 1);
        let qr = Document::parse(&fragments.qr).unwrap();
        let value = qr
            .descendants()
            .find(|n| nav::is(*n, CBC_NS, "EmbeddedDocumentBinaryObject"))
            .unwrap();
        assert_eq!(nav::content(value), "qr<&>");
    }

    #[test]
    fn changed_templates_fail_instead_of_silently_skipping_substitutions() {
        for extension in [
            UBL_EXTENSIONS_TEMPLATE.replace("<ds:SignatureValue></ds:SignatureValue>", ""),
            UBL_EXTENSIONS_TEMPLATE.replace(
                "<ds:SignatureValue></ds:SignatureValue>",
                "<ds:SignatureValue/><ds:SignatureValue/>",
            ),
            UBL_EXTENSIONS_TEMPLATE.replace(
                "<ds:SignatureValue></ds:SignatureValue>",
                "<ds:SignatureValue>stale</ds:SignatureValue>",
            ),
            UBL_EXTENSIONS_TEMPLATE.replace("<ext:UBLExtension>", "<ext:UBLExtension >"),
            UBL_EXTENSIONS_TEMPLATE.replace("URI=\"#xadesSignedProperties\"", "URI=\"#external\""),
        ] {
            assert!(
                using(
                    &extension,
                    QR_REFERENCE_TEMPLATE,
                    CAC_SIGNATURE_TEMPLATE,
                    PROPERTIES
                )
                .is_err()
            );
        }
        for qr in [
            QR_REFERENCE_TEMPLATE.replace("mimeCode=\"text/plain\"", "mimeCode='text/plain'"),
            QR_REFERENCE_TEMPLATE.repeat(2),
            QR_REFERENCE_TEMPLATE.replace(">QR<", ">other<"),
        ] {
            assert!(
                using(
                    UBL_EXTENSIONS_TEMPLATE,
                    &qr,
                    CAC_SIGNATURE_TEMPLATE,
                    PROPERTIES
                )
                .is_err()
            );
        }
        let properties = PROPERTIES.replace(&format!(" xmlns:ds=\"{DS_NS}\""), "");
        assert!(
            using(
                UBL_EXTENSIONS_TEMPLATE,
                QR_REFERENCE_TEMPLATE,
                CAC_SIGNATURE_TEMPLATE,
                &properties
            )
            .is_err()
        );
        let signature = CAC_SIGNATURE_TEMPLATE.replace(SIGNATURE_ID, "external");
        assert!(
            using(
                UBL_EXTENSIONS_TEMPLATE,
                QR_REFERENCE_TEMPLATE,
                &signature,
                PROPERTIES
            )
            .is_err()
        );
    }
}
