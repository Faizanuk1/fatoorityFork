# XSD adapter compatibility probe

The production adapter uses pinned `xsd-schema` 0.2.2 with an in-memory workaround
for the four bundled CCTS decimal families. This follows the XSD adapter boundary
agreed in [issue #4](https://github.com/mqqz/fatoora-rs/issues/4).
Bundled schema files and frozen SDK fixtures remain unchanged.

From the repository root, with libxml2 and Clang development dependencies:

```sh
cargo run --locked --manifest-path scripts/xsd-compat/Cargo.toml > /tmp/fatoora-xsd-compat.json
cargo test --locked --manifest-path scripts/xsd-compat/Cargo.toml
cargo clippy --locked --manifest-path scripts/xsd-compat/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path scripts/xsd-compat/Cargo.toml --check
```

This independent workspace compares the public production validator, raw upstream
validation against the original schemas, and libxml. It verifies all fixture
manifest hashes before validation. Captured SDK XSD verdicts are checked separately
from libxml. Minimal datatype examples use raw upstream validation, since the
workaround is restricted to the bundled decimal families.

The raw probe uses `drive_quick_xml`, whose pinned implementation calls
`end_validation()` internally. Its unit regression checks unresolved IDREFs,
forward references, and rejection of empty/truncated documents; the Rust CI
workflow runs this test. The production adapter uses `drive_quick_xml_with`
and therefore finalizes explicitly.

The command deliberately exits **1** for any difference or raw upstream minimal
failure, including the documented differences below. Read `sdk_mismatches` and the
individual results before interpreting the exit status as a migration failure.

## Validated results (2026-10-10)

| Check | Result |
| --- | --- |
| Frozen artifact checksums | 3,209 verified |
| Tracked XML fixtures plus targeted mutations | 595 + 12 = 607 |
| Production versus captured SDK XSD verdicts | 448/448 agree |
| Raw upstream versus captured SDK XSD verdicts | 445/448 agree |
| Production versus libxml | 606/607 agree |
| Raw upstream minimal datatype cases | 13/14 expected outcomes |

The only production/libxml difference is `ksa-dates/issue-time-whitespace`:
production and the frozen SDK accept it; libxml rejects it.

The three upstream overflow failures are the generated arithmetic fixtures
`division-large-divisor`, `division-large-divisor-no-zeros`, and
`division-tiny-price`. Their quantity is 10^40. The production adapter accepts
all three, matching their captured SDK XSD results. These are locally generated
edge cases with frozen SDK evidence, not examples distributed by the SDK.

The remaining minimal upstream failure is a direct `xs:decimal` value of 10^40.
It is outside the scoped workaround. The adapter does not promise unlimited
decimals for arbitrary extension schemas.

`workaround-evidence.json` records current summary counts, differing cases,
resolved overflow cases, and schema/input inventory hashes. `evidence.json`
preserves the earlier pre-fix PR #20 baseline (441/448 SDK agreement and eight
libxml differences); it is historical evidence, not the current result.

Production regressions in `fatoora-core/src/invoice/validation/schema_tests.rs`
cover all four decimal families, lexical errors, huge values, tiny fractions,
CDATA/entities, attributes, children, wildcard extensions, type overrides, and
the existing digit-string serial-number constraint. A schema-contract test makes future decimal
facets, defaults, or identity constraints require review of the workaround.
