# Native XML migration

Issue [#4](https://github.com/mqqz/fatoora-rs/issues/4) tracks completion.
Each stage is reviewed in its own PR. Work proceeds on `remove-libxml-v2`;
subsequent stages use the preceding reviewed checkpoint as their PR base.

## Architecture

- `quick-xml` generates invoices and complete owned signing fragments.
- `roxmltree` interprets all application XML using expanded names and semantic
  UBL helpers. Its `positions` feature supplies original byte ranges.
- `xml-sec = "=0.1.20"`, with default features disabled and only `std`, `c14n`,
  and `xml-backend-roxmltree`, performs C14N 1.1 without comments.
- `xsd-schema = "=0.2.2"` remains the XSD backend behind the existing adapter.

Keep XML operations private in `invoice/xml/{constants,nav,lexical,edit,
fragments,canonicalize,parse}.rs`. `sign.rs` owns orchestration and cryptography.
Dependency types stay out of public/domain APIs. Preserve public error categories.

Lexical processing handles declaration version/encoding, DTD/entity rejection,
resource limits, original prefixes and selected SignedProperties serialization.
Semantic selection belongs exclusively to the tree. Preserve the current 8 MiB
input, 100,000 element, depth 128 and 64 MiB retained/output limits. Direct text
coalesces CDATA; element, comment and PI boundaries retain existing semantics.

## Stages

1. Freeze compatibility evidence and correct existing signing semantics in
   separate commits. Protect typed fixtures against checkout conversion. Preserve
   the 38 canonical and 15 SignedProperties SDK cases and capture separate
   corrected reference-DOM cases for namespace/C14N equivalence.
2. Introduce bounded lexical parsing and namespace-aware UBL navigation. Migrate
   invoice import, QR extraction, business-rule views and integrity readers;
   remove dynamically indexed XPath traversal.
3. Replace invoice canonicalization and SignedProperties preimages. Require
   source-range exclusion followed by C14N to equal reference DOM subtree deletion
   byte-for-byte, including namespace axes, redeclarations/default resets,
   inherited `xml:*`, nested exclusions and processing instructions.
4. Replace signing mutation with complete fragments and one validated edit batch.
   Enforce ownership, certificate policy, byte preservation and full postconditions.
5. Remove libxml from ordinary dependencies/tests and packaging. Verify actual
   native artifacts, newest SDK and ZATCA Simulation submissions before closing #4.

## Signing contract

Capture valid existing owned signing time; reject malformed nonempty values;
fall back to invoice issue date/time for absent or empty template values. Select
only the supported UBL signature extension. Exclusions match namespace URIs and
exact `QR` text, and delete complete subtrees without promoting descendants.

Unsigned input and recognized blank templates are supported. Re-sign only
clearly owned structures with the same decoded certificate DER. Reject a different
certificate (including renewal using the same key), malformed/missing populated
certificate material, duplicate/ambiguous structures and external signatures.
Verify the supplied key matches the certificate. Re-signing does not authenticate
an earlier signature. With the same key/certificate and preserved signing time,
require `sign(sign(input)) == sign(input)` byte-for-byte.

Pipeline:

1. Parse original input and enforce ownership/input rules.
2. Capture signing time and other inputs.
3. Exclude ranges, C14N 1.1 and SHA-256 the original invoice.
4. Sign the digest and construct certificate/properties digests and QR payload.
5. Render complete owned fragments.
6. Calculate all edits against the immutable input; validate and apply once.
7. Parse final output.
8. Recompute the final invoice digest.
9. Verify all signing postconditions before returning output.

Anchor at the next required successor element start, otherwise the last permitted
predecessor end, otherwise the container closing tag. Preserve adjacent whitespace,
comments and processing instructions. Reject ordering that requires unrelated
edits. Coalesce insertions at one offset in UBL order (QR before `cac:Signature`).
A parent replacement owns its children. Validate bounds, UTF-8 boundaries and
non-overlap; apply descending and discard every borrowed node/range afterward.
Handle empty containers explicitly and keep namespace declarations fragment-local.

Postconditions read the final XML: digest equality; certificate DER and signature
bytes equal intended values; certificate verifies invoice signature; independently
recomputed SignedProperties digest; correct IDs, targets, references, transforms,
algorithms, issuer/serial, certificate digest and signing time; exact intended QR
payload and decoded fields. Reuse lower integrity checks for every invoice type.
The public report skips standard invoice signature checks and uses the SDK
PayableAmount QR policy; the signing check must use the generator's TaxInclusiveAmount
policy. Signing does not invoke full XSD/business validation or establish chain trust.

## Verification and completion

Tests must independently derive expected results without production signing,
exclusion or preimage helpers. Cover exact source slices with entities, CDATA,
namespace aliases, CRLF, Unicode and empty elements; cardinality/mixed namespaces;
trivia anchors, overlaps and untouched bytes; certificate mismatch/partial input;
repeated signing; and corruption of excluded signature material with an unchanged
invoice digest. Keep fuzz checks bounded and on PRs, without a scheduled campaign.

Retain decimal shim tests for overflow, ordinary decimals, invalid lexical forms,
non-decimal and unrelated derived types, attributes/content and `xsi:type`.
Preserve the captured 448/448 SDK XSD agreement. The four CCTS decimal bases are a
compatibility shim with a schema contract guard; arbitrary extension decimals do
not gain relaxed validation.

Remove native install steps/notices only when unused by remaining dependencies.
Inspect both dependency graphs and built binaries: Linux `readelf`/`ldd`, macOS
`otool -L`, Windows PE imports and unpacked Python wheels, including their library
closure. Run native CLI, FFI and Python smoke checks without libxml installed.
The optional `scripts/xml-reference` maintenance oracle stays outside normal builds.

Capture the newest official SDK version/checksum in a separate corpus; preserve
frozen evidence. Require all six standard/simplified invoice/credit/debit Simulation
compliance cases, simplified reporting and standard clearance with consistent
identity/chain context. Keep credentials out of tracked files; retain sanitized
evidence tied to the commit. Validation rejections fail the gate. Missing credentials,
authentication failures or transport failures leave it incomplete. Native removal,
SDK acceptance and Simulation acceptance are separate recorded gates. WASM follows
in a later project; this migration makes no WASM verification claim.
