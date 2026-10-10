# XML and Schemas

Details about XML handling, templates, and bundled schemas.

## Bundled Schema

???+ note "UBL 2.1"
    - UBL 2.1 XSD is bundled at:
      fatoora-core/assets/schemas/UBL2.1/xsd/maindoc/UBL-Invoice-2.1.xsd
    - Validation always loads this bundled schema.

## Decimal compatibility

XSD validation uses `xsd-schema` 0.2.2 with a per-thread compiled schema cache
and a fresh validation runtime for every document. Imports resolve only to the
15 embedded resources; validation does not fetch instance schema locations.

The backend's fixed-size decimal representation cannot hold every valid UBL
value. The adapter changes the four CCTS decimal bases (`AmountType`,
`MeasureType`, `NumericType`, and `QuantityType`) to strings **in memory** and
checks their decimal syntax separately, without numeric conversion. This
covers 196 bundled element declarations, including amounts, quantities,
measurements, percentages, rates, and numeric values. Bundled XSD files remain
unchanged. Attributes, content models, and type overrides are still validated.
Tests guard the assumption that these types have no numeric facets or identity
constraints that would require numeric comparison.

This is limited to the bundled decimal families. An arbitrary extension using
`xs:decimal` directly still has the upstream capacity limit. Business-rule
arithmetic retains its separate exact-decimal representation and resource limits;
passing XSD does not guarantee that an invoice passes those rules.

## Signing Templates

???+ note "Embedded templates"
    - fatoora-core/assets/templates/ubl_extensions.xml
    - fatoora-core/assets/templates/cac_signature.xml
    - fatoora-core/assets/templates/qr_reference.xml

## Canonicalization

!!! note "Canonicalization"
    - Signing canonicalizes XML using C14N 1.1 before hashing and signature generation.
