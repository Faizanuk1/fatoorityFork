//! In-memory UBL validation with a narrowly scoped decimal-capacity adapter.
use super::{XmlValidationError, decimal_lexical};
use crate::Diagnostic;
use std::{cell::OnceCell, convert::Infallible};
use xsd_schema::schema::model::DerivationSet;
use xsd_schema::validation::{
    ElementStartView, EndElementInfo, ErrorOnlySink, SchemaInfo, SchemaValidator, SchemaValidity,
    TextKind, TypeSource, ValidationError, ValidationEventHandler, ValidationFlags,
    drive_quick_xml_with,
};
use xsd_schema::{SchemaError, SchemaLoader, SchemaResult, SchemaSet, SchemaSetBuilder, TypeKey};

const CCT: &str = "urn:un:unece:uncefact:data:specification:CoreComponentTypeSchemaModule:2";
const DECIMAL_TYPES: [&str; 4] = ["AmountType", "MeasureType", "NumericType", "QuantityType"];

macro_rules! resource {
    ($path:literal) => {
        (
            $path,
            include_bytes!(concat!("../../../assets/schemas/UBL2.1/xsd/", $path))
                as &'static [u8],
        )
    };
}

const RESOURCES: &[(&str, &[u8])] = &[
    resource!("maindoc/UBL-Invoice-2.1.xsd"),
    resource!("common/CCTS_CCT_SchemaModule-2.1.xsd"),
    resource!("common/UBL-CommonAggregateComponents-2.1.xsd"),
    resource!("common/UBL-CommonBasicComponents-2.1.xsd"),
    resource!("common/UBL-CommonExtensionComponents-2.1.xsd"),
    resource!("common/UBL-CommonSignatureComponents-2.1.xsd"),
    resource!("common/UBL-CoreComponentParameters-2.1.xsd"),
    resource!("common/UBL-ExtensionContentDataType-2.1.xsd"),
    resource!("common/UBL-QualifiedDataTypes-2.1.xsd"),
    resource!("common/UBL-SignatureAggregateComponents-2.1.xsd"),
    resource!("common/UBL-SignatureBasicComponents-2.1.xsd"),
    resource!("common/UBL-UnqualifiedDataTypes-2.1.xsd"),
    resource!("common/UBL-XAdESv132-2.1.xsd"),
    resource!("common/UBL-XAdESv141-2.1.xsd"),
    resource!("common/UBL-xmldsig-core-schema-2.1.xsd"),
];

#[derive(Debug)]
struct BundledLoader;

impl SchemaLoader for BundledLoader {
    fn can_load(&self, location: &str) -> bool {
        resource_at(location).is_some()
    }

    fn load(&self, location: &str) -> SchemaResult<String> {
        let (name, bytes) = resource_at(location).ok_or_else(|| {
            SchemaError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "schema is not a bundled resource",
            ))
        })?;
        let text = std::str::from_utf8(bytes).expect("embedded schemas are UTF-8");
        if name == "common/CCTS_CCT_SchemaModule-2.1.xsd" {
            // Only these four unbounded families are adapted. Original resource
            // bytes remain unchanged; numeric syntax is checked after validation.
            assert_eq!(
                text.matches("base=\"xsd:decimal\"").count(),
                DECIMAL_TYPES.len()
            );
            Ok(text.replace("base=\"xsd:decimal\"", "base=\"xsd:string\""))
        } else {
            Ok(text.to_owned())
        }
    }
}

fn resource_at(location: &str) -> Option<(&'static str, &'static [u8])> {
    let relative = std::path::Path::new(location)
        .strip_prefix("/fatoora-ubl")
        .ok()?;
    RESOURCES
        .iter()
        .copied()
        .find(|(name, _)| std::path::Path::new(name) == relative)
}

struct Adapter {
    schema: SchemaSet,
    decimals: Vec<TypeKey>,
}

impl Adapter {
    fn compile() -> Result<Self, String> {
        let schema = SchemaSetBuilder::with_loader(Box::new(BundledLoader))
            .add("", "/fatoora-ubl/maindoc/UBL-Invoice-2.1.xsd")
            .and_then(|builder| builder.compile())
            .map_err(|error| error.to_string())?
            .into_schema_set();
        let ns = schema.name_table.get(CCT).ok_or("missing CCTS namespace")?;
        let decimals = DECIMAL_TYPES
            .iter()
            .map(|name| {
                let name = schema
                    .name_table
                    .get(name)
                    .ok_or("missing CCTS type name")?;
                schema
                    .lookup_type(Some(ns), name)
                    .ok_or("missing CCTS type")
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { schema, decimals })
    }

    fn is_decimal(&self, ty: TypeKey) -> bool {
        self.decimals.iter().any(|base| {
            self.schema
                .is_type_derived_from(ty, *base, DerivationSet::empty())
        })
    }

    fn validate(&self, xml: &str) -> Result<(), XmlValidationError> {
        let validator = SchemaValidator::new(
            &self.schema,
            ValidationFlags::default() | ValidationFlags::PROCESS_IDENTITY_CONSTRAINTS,
        );
        let mut errors = Vec::new();
        let mut handler = DecimalGuard {
            adapter: self,
            frames: Vec::new(),
            errors: Vec::new(),
        };
        let mut runtime = validator.start_run(ErrorOnlySink {
            errors: &mut errors,
        });
        let outcome =
            drive_quick_xml_with(xml.as_bytes(), &mut runtime, &self.schema, &mut handler)
                .map_err(|error| XmlValidationError::XmlParse {
                    message: format!("{error:?}"),
                })?;
        let completion = runtime.end_validation();
        drop(runtime);
        validation_result(outcome.root_validity, errors, completion, handler.errors)
    }
}

fn validation_result(
    root_validity: Option<SchemaValidity>,
    errors: Vec<ValidationError>,
    completion: Result<(), ValidationError>,
    decimal_errors: Vec<Diagnostic>,
) -> Result<(), XmlValidationError> {
    let mut diagnostics: Vec<_> = errors.into_iter().map(Diagnostic::from_schema).collect();
    if let Err(error) = completion {
        diagnostics.push(Diagnostic::from_schema(error));
    }
    diagnostics.extend(decimal_errors);
    if diagnostics.is_empty() && root_validity == Some(SchemaValidity::Valid) {
        Ok(())
    } else {
        if diagnostics.is_empty() {
            diagnostics.push(Diagnostic::new("document has no valid UBL root"));
        }
        Err(XmlValidationError::SchemaValidation {
            errors: diagnostics,
        })
    }
}

// NameTable uses interior mutability: cache compiled schemas per thread, while
// every document gets a fresh runtime and guard (IDs and errors cannot leak).
thread_local! {
    static ADAPTER: OnceCell<Result<Adapter, String>> = const { OnceCell::new() };
}

pub(super) fn validate(xml: &str) -> Result<(), XmlValidationError> {
    roxmltree::Document::parse(xml).map_err(|error| XmlValidationError::XmlParse {
        message: error.to_string(),
    })?;
    ADAPTER.with(|cache| match cache.get_or_init(Adapter::compile) {
        Ok(adapter) => adapter.validate(xml),
        Err(message) => Err(XmlValidationError::SchemaParse {
            errors: vec![Diagnostic::new(message.clone())],
        }),
    })
}

struct Frame {
    name: String,
    decimal: bool,
    text: String,
}

struct DecimalGuard<'a> {
    adapter: &'a Adapter,
    frames: Vec<Frame>,
    errors: Vec<Diagnostic>,
}

impl ValidationEventHandler for DecimalGuard<'_> {
    type Error = Infallible;

    fn after_element(
        &mut self,
        view: ElementStartView<'_>,
        info: &SchemaInfo,
    ) -> Result<(), Infallible> {
        self.frames.push(Frame {
            name: format!("{{{}}}{}", view.namespace_uri, view.local_name),
            decimal: false,
            text: String::new(),
        });
        let decimal = info
            .schema_type
            .is_some_and(|ty| self.adapter.is_decimal(ty));
        self.frames.last_mut().expect("element frame").decimal = decimal && !info.is_nil;
        if decimal && info.type_source == Some(TypeSource::XsiType) {
            let schema = &self.adapter.schema;
            if let Some(declared) = info
                .element_decl
                .and_then(|key| schema.arenas.get_element(key))
                .and_then(|decl| decl.resolved_type)
            {
                // Replacing the decimal base must not introduce legal overrides
                // for elements originally declared as strings.
                if !self.adapter.is_decimal(declared)
                    && !schema.is_any_type(declared)
                    && declared != TypeKey::Simple(schema.builtin_types().any_simple_type)
                {
                    self.errors.push(Diagnostic::new(
                        "xsi:type is not derived from the original declared type",
                    ));
                }
            }
        }
        Ok(())
    }

    fn on_text(&mut self, _: TextKind, text: &str) -> Result<(), Infallible> {
        if let Some(frame) = self.frames.last_mut().filter(|frame| frame.decimal) {
            frame.text.push_str(text);
        }
        Ok(())
    }

    fn after_end_element(&mut self, _: &EndElementInfo, _: usize) -> Result<(), Infallible> {
        let frame = self.frames.pop().expect("element frame");
        if frame.decimal && !decimal_lexical::valid(&frame.text) {
            self.errors.push(Diagnostic::new(format!(
                "{}: invalid xs:decimal lexical value",
                frame.name
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "schema_tests.rs"]
mod tests;
