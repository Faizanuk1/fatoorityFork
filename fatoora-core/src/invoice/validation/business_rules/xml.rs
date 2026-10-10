//! Immutable view of the original XML, without an invoice model round trip.
use super::{FailureKind, Limits};
use roxmltree::Node;
use std::collections::HashMap;

pub(super) const UBL: &str = "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2";
pub(super) const CREDIT_NOTE: &str = "urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2";
pub(super) const CAC: &str =
    "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";
pub(super) const CBC: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";
pub(super) type NodeId = usize;
pub(super) type Name = (&'static str, &'static str);
pub(super) const fn a(name: &'static str) -> Name {
    (CAC, name)
}
pub(super) const fn b(name: &'static str) -> Name {
    (CBC, name)
}

#[derive(Debug)]
pub(super) struct Element {
    pub namespace: String,
    pub name: String,
    pub text: String,
    /// XDM child::text() values, excluding descendant element text.
    pub direct_text: Vec<String>,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub location: String,
    attributes: HashMap<(String, String), String>,
}

#[derive(Debug)]
pub(super) struct XmlView {
    nodes: Vec<Element>,
}

impl XmlView {
    pub fn parse(input: &str, limits: &Limits) -> Result<Self, FailureKind> {
        preflight(input, limits)?;
        // Preflight bounds elements and source bytes. roxmltree's node limit
        // also counts text, comments, PIs, and the document root.
        let document = roxmltree::Document::parse(input)
            .map_err(|e| FailureKind::InvalidXml(e.to_string()))?;
        let root = document.root_element();
        let mut view = Self { nodes: Vec::new() };
        view.copy_element(root, None, "", 1, &mut 0, limits)?;
        Ok(view)
    }

    fn copy_element(
        &mut self,
        node: Node<'_, '_>,
        parent: Option<NodeId>,
        prefix: &str,
        ordinal: usize,
        retained: &mut usize,
        limits: &Limits,
    ) -> Result<NodeId, FailureKind> {
        let name = node.tag_name().name().to_owned();
        let namespace = node.tag_name().namespace().unwrap_or_default().to_owned();
        let location = format!(
            "{prefix}/*[local-name()={} and namespace-uri()={}][{ordinal}]",
            xpath_literal(&name),
            xpath_literal(&namespace)
        );
        let text = crate::invoice::xml::nav::content(node);
        // XML CDATA boundaries do not create XDM text-node boundaries. Other
        // child node kinds do, and zero-length text nodes are omitted.
        let mut direct_text = Vec::new();
        let mut text_run = String::new();
        for child in node.children() {
            if child.is_text() {
                text_run.push_str(child.text().unwrap_or_default());
            } else if !text_run.is_empty() {
                direct_text.push(std::mem::take(&mut text_run));
            }
        }
        if !text_run.is_empty() {
            direct_text.push(text_run);
        }
        let attributes: HashMap<_, _> = node
            .attributes()
            .map(|attr| {
                (
                    (
                        attr.namespace().unwrap_or_default().to_owned(),
                        attr.name().to_owned(),
                    ),
                    attr.value().to_owned(),
                )
            })
            .collect();
        let size = name.len()
            + namespace.len()
            + location.len()
            + text.len()
            + direct_text.iter().map(String::len).sum::<usize>()
            + attributes
                .iter()
                .map(|((ns, name), value)| ns.len() + name.len() + value.len())
                .sum::<usize>();
        *retained = retained
            .checked_add(size)
            .ok_or(FailureKind::Limit("XML retained bytes"))?;
        if *retained > limits.retained_bytes {
            return Err(FailureKind::Limit("XML retained bytes"));
        }
        let id = self.nodes.len();
        self.nodes.push(Element {
            namespace,
            name,
            text,
            direct_text,
            parent,
            children: Vec::new(),
            location: location.clone(),
            attributes,
        });
        let mut counts = HashMap::new();
        for child in node.children().filter(Node::is_element) {
            let key = (child.tag_name().namespace(), child.tag_name().name());
            let ordinal = counts.entry(key).or_insert(0);
            *ordinal += 1;
            let child =
                self.copy_element(child, Some(id), &location, *ordinal, retained, limits)?;
            self.nodes[id].children.push(child);
        }
        Ok(id)
    }

    pub fn node(&self, id: NodeId) -> &Element {
        &self.nodes[id]
    }
    pub fn elements(&self) -> impl Iterator<Item = (NodeId, &Element)> {
        self.nodes.iter().enumerate()
    }
    pub fn all(&self, namespace: &str, name: &str) -> Vec<NodeId> {
        (0..self.nodes.len())
            .filter(|&id| self.is(id, namespace, name))
            .collect()
    }
    pub fn is(&self, id: NodeId, namespace: &str, name: &str) -> bool {
        self.node(id).namespace == namespace && self.node(id).name == name
    }
    pub fn is_document_root(&self, id: NodeId) -> bool {
        id == 0 && (self.is(id, UBL, "Invoice") || self.is(id, CREDIT_NOTE, "CreditNote"))
    }
    pub fn children(&self, id: NodeId, namespace: &str, name: &str) -> Vec<NodeId> {
        self.node(id)
            .children
            .iter()
            .copied()
            .filter(|&id| self.is(id, namespace, name))
            .collect()
    }
    pub fn path(&self, id: NodeId, steps: &[Name]) -> Vec<NodeId> {
        let mut nodes = vec![id];
        for &(namespace, name) in steps {
            nodes = nodes
                .into_iter()
                .flat_map(|id| self.children(id, namespace, name))
                .collect();
        }
        nodes
    }
    pub fn all_path(&self, steps: &[Name]) -> Vec<NodeId> {
        let Some((&(namespace, name), rest)) = steps.split_first() else {
            return Vec::new();
        };
        let mut nodes: Vec<_> = self
            .all(namespace, name)
            .into_iter()
            .flat_map(|id| self.path(id, rest))
            .collect();
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }
    pub fn attribute(&self, id: NodeId, namespace: &str, name: &str) -> Option<&str> {
        self.node(id)
            .attributes
            .get(&(namespace.to_owned(), name.to_owned()))
            .map(String::as_str)
    }
    pub fn singleton_text(&self, nodes: &[NodeId]) -> Result<Option<&str>, FailureKind> {
        match nodes {
            [] => Ok(None),
            [id] => Ok(Some(&self.node(*id).text)),
            _ => Err(FailureKind::Cardinality),
        }
    }
}

fn preflight(input: &str, limits: &Limits) -> Result<(), FailureKind> {
    use crate::invoice::xml::lexical::{self, XmlError, XmlLimits};
    lexical::preflight(
        input,
        &XmlLimits {
            xml_bytes: limits.xml_bytes,
            nodes: limits.nodes,
            depth: limits.depth,
        },
    )
    .map_err(|error| match error {
        XmlError::InvalidXml(message) => FailureKind::InvalidXml(message),
        XmlError::UnsupportedXml(message) => FailureKind::UnsupportedXml(message),
        XmlError::Limit(limit) => FailureKind::Limit(limit),
    })
}

pub(super) fn is_xml_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}
pub(super) fn normalize_space(value: &str) -> String {
    value
        .split(is_xml_space)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn xpath_literal(value: &str) -> String {
    if !value.contains('\'') {
        format!("'{value}'")
    } else if !value.contains('"') {
        format!("\"{value}\"")
    } else {
        format!(
            "concat({})",
            value
                .split('\'')
                .map(|part| format!("'{part}'"))
                .collect::<Vec<_>>()
                .join(",\"'\",")
        )
    }
}
