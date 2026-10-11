# Invoice Model

C signatures use the [C API](../development/c-api.md). See [C and C++ bindings](bindings/c.md) for ownership and build instructions.
The declarations below come from the generated headers. C++ exposes the same types
in namespace `fatoora` through `fatoora/Type.hpp` headers.

Core data types for building and inspecting invoices. See [Decimal numbers and rounding](numbers.md) for numeric inputs, calculation rules and binding representations.

## InvoiceBuilder

Rust configuration methods consume and return the builder. Chain calls or assign
back to the builder for conditional configuration. `line_item` appends; other
configuration methods replace their corresponding values. `build(self)` moves
owned fields into the invoice and consumes the builder on success or error.

C and Python keep mutable builder handles and their existing setter names.
Setters validate their arguments before moving the builder; rejected arguments
leave it available for correction. Building consumes the handle on success or
validation failure.

```rust
let invoice = InvoiceBuilder::new(invoice_type)
    .id("INV-1")
    // Configure the remaining required fields.
    .build()?;
```

### `new`

???+ note "Create builder"
    Create a builder for a specific invoice type/subtype.

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::new(invoice_type: InvoiceType) -> InvoiceBuilder
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.new(invoice_type: InvoiceTypeKind, invoice_subtype: InvoiceSubType, original_invoice_id: Optional[str] = None, original_invoice_uuid: Optional[str] = None, original_invoice_issue_date: Optional[str] = None, original_invoice_reason: Optional[str] = None) -> InvoiceBuilder
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_new(const FatooraInvoiceOptions *options,
            FatooraInvoiceBuilder **out,
            FatooraError **error);
        ```

### `id`

???+ note "Set invoice ID"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::id(id: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_id(invoice_id: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_id(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `uuid`

???+ note "Set invoice UUID"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::uuid(uuid: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_uuid(uuid: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_uuid(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `issue_datetime`

???+ note "Set issue datetime"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::issue_datetime(value: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_issue_datetime(issue_datetime: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_issue_datetime(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `currency`

???+ note "Set currency"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::currency(code: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_currency(currency_code: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_currency(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `previous_invoice_hash`

???+ note "Set previous invoice hash"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::previous_invoice_hash(hash: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_previous_invoice_hash(hash: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_previous_invoice_hash(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `invoice_counter`

???+ note "Set invoice counter"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::invoice_counter(counter: u64) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_invoice_counter(counter: int) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_invoice_counter(FatooraInvoiceBuilder *self,
            uint64_t value,
            FatooraError **error);
        ```

### `payment_means_code`

???+ note "Set payment means code"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::payment_means_code(code: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_payment_means_code(code: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_payment_means_code(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `vat_category`

???+ note "Set invoice VAT category"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::vat_category(category: VatCategory) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_vat_category(category: VatCategory) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_vat_category(FatooraInvoiceBuilder *self,
            int32_t value,
            FatooraError **error);
        ```

### `seller`

???+ note "Set seller party"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::seller(seller: Seller) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_seller(...) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_seller(FatooraInvoiceBuilder *self,
            const FatooraPartyOptions *options,
            FatooraError **error);
        ```

### `buyer`

???+ note "Set buyer party"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::buyer(buyer: Buyer) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_buyer(...) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_buyer(FatooraInvoiceBuilder *self,
            const FatooraPartyOptions *options,
            FatooraError **error);
        ```

### `note`

???+ note "Set invoice note"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::note(note: InvoiceNote) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_note(language: str, text: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_note(FatooraInvoiceBuilder *self,
            const char *language,
            const char *value,
            FatooraError **error);
        ```

### `allowance`

???+ note "Set allowance reason and amount"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::allowance(reason: impl Into<String>, amount: Decimal) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.set_allowance(reason: str, amount: Decimal | str | int) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_set_allowance(FatooraInvoiceBuilder *self,
            const char *reason,
            const char *amount,
            FatooraError **error);
        ```

### `invoice_level_charge`

???+ note "Set invoice-level charge"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::invoice_level_charge(charge: Decimal) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.invoice_level_charge(charge: Decimal | str | int) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_invoice_level_charge(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `invoice_level_discount`

???+ note "Set invoice-level discount"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::invoice_level_discount(discount: Decimal) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.invoice_level_discount(discount: Decimal | str | int) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_invoice_level_discount(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `allowance_reason`

???+ note "Set allowance reason"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::allowance_reason(reason: impl Into<String>) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.allowance_reason(reason: str) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_allowance_reason(FatooraInvoiceBuilder *self,
            const char *value,
            FatooraError **error);
        ```

### `flags`

???+ note "Set invoice flags"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::flags(flags: InvoiceFlags) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.flags(flags: int) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_flags(FatooraInvoiceBuilder *self,
            int32_t value,
            FatooraError **error);
        ```

### `line_item`

???+ note "Add line item"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::line_item(item: LineItem) -> Self
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.add_line_item(description: str, quantity: Decimal | str | int, unit_code: str, unit_price: Decimal | str | int, vat_rate: Decimal | str | int, vat_category: VatCategory) -> None
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_add_line_item(FatooraInvoiceBuilder *self,
            const FatooraLineItemOptions *options,
            FatooraError **error);
        ```

### `build`

???+ note "Finalize invoice"

    === "{{ lang.rust }}"
        ```rust
        InvoiceBuilder::build(self) -> Result<FinalizedInvoice, InvoiceError>
        ```

    === "{{ lang.python }}"
        ```python
        InvoiceBuilder.build() -> FinalizedInvoice
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_invoice_builder_build(FatooraInvoiceBuilder *self,
            FatooraFinalizedInvoice **out,
            FatooraError **error);
        ```

## Field Groups

### `id`

- Setter: `InvoiceBuilder::id` / `InvoiceBuilder.set_id` / `fatoora_invoice_builder_set_id`
- Getters: `FinalizedInvoice.id`, `SignedInvoice.id`, `fatoora_invoice_data_id`, `fatoora_invoice_data_id`

### `uuid`

- Setter: `InvoiceBuilder::uuid` / `InvoiceBuilder.set_uuid` / `fatoora_invoice_builder_set_uuid`
- Getters: `FinalizedInvoice.uuid`, `SignedInvoice.uuid`, `fatoora_invoice_data_uuid`, `fatoora_invoice_data_uuid`

### `issue_datetime`

- Setter: `InvoiceBuilder::issue_datetime` / `InvoiceBuilder.set_issue_datetime` / `fatoora_invoice_builder_set_issue_datetime`
- Getters: `FinalizedInvoice.issue_datetime`, `SignedInvoice.issue_datetime`, `fatoora_invoice_data_issue_datetime`, `fatoora_invoice_data_issue_datetime`

### `currency`

- Setter: `InvoiceBuilder::currency` / `InvoiceBuilder.set_currency` / `fatoora_invoice_builder_set_currency`
- Getters: `FinalizedInvoice.currency`, `SignedInvoice.currency`, `fatoora_invoice_data_currency`, `fatoora_invoice_data_currency`

### `previous_invoice_hash`

- Setter: `InvoiceBuilder::previous_invoice_hash` / `InvoiceBuilder.set_previous_invoice_hash` / `fatoora_invoice_builder_set_previous_invoice_hash`
- Getters: `FinalizedInvoice.previous_invoice_hash`, `SignedInvoice.previous_invoice_hash`, `fatoora_invoice_data_previous_invoice_hash`, `fatoora_invoice_data_previous_invoice_hash`

### `invoice_counter`

- Setter: `InvoiceBuilder::invoice_counter` / `InvoiceBuilder.set_invoice_counter` / `fatoora_invoice_builder_set_invoice_counter`
- Getters: `FinalizedInvoice.invoice_counter`, `SignedInvoice.invoice_counter`, `fatoora_invoice_data_invoice_counter`, `fatoora_invoice_data_invoice_counter`

### `payment_means_code`

- Setter: `InvoiceBuilder::payment_means_code` / `InvoiceBuilder.set_payment_means_code` / `fatoora_invoice_builder_set_payment_means_code`
- Getters: `FinalizedInvoice.payment_means_code`, `SignedInvoice.payment_means_code`, `fatoora_invoice_data_payment_means_code`, `fatoora_invoice_data_payment_means_code`

### `vat_category`

- Setter: `InvoiceBuilder::vat_category` / `InvoiceBuilder.set_vat_category` / `fatoora_invoice_builder_set_vat_category`
- Getters: `FinalizedInvoice.vat_category`, `SignedInvoice.vat_category`, `fatoora_invoice_data_vat_category`, `fatoora_invoice_data_vat_category`

### Parties (`seller`, `buyer`)

- Setters: `InvoiceBuilder::seller`, `InvoiceBuilder::buyer` and matching Python/C builder methods.
- Getters: `FinalizedInvoice.seller`, `FinalizedInvoice.buyer`, `SignedInvoice.seller`, `SignedInvoice.buyer`, `fatoora_invoice_data_seller`, `fatoora_invoice_data_buyer`, `fatoora_invoice_data_seller`, `fatoora_invoice_data_buyer`

### Notes and Allowance (`note`, `allowance_reason`, charge/discount)

- Setters: `set_note`, `set_allowance`, `invoice_level_charge`, `invoice_level_discount`, `allowance_reason` and matching Python/C builder methods.
- Getters: `note`, `allowance_reason`, `invoice_level_charge`, `invoice_level_discount` on finalized/signed invoices and `fatoora_invoice_data_*` getters on an owned snapshot.

### Flags and Type

- Setter: `InvoiceBuilder::flags` / `InvoiceBuilder.flags` / `fatoora_invoice_builder_flags`
- Getters: `flags`, `is_third_party`, `is_nominal`, `is_export`, `is_summary`, `is_self_billed`, `is_simplified`, `invoice_type_kind`, `invoice_sub_type` on finalized/signed invoices and matching C getters.

### Line Items and Totals

- Setter: `InvoiceBuilder::line_item` / `InvoiceBuilder.add_line_item` / `fatoora_invoice_builder_add_line_item`
- Getters: `line_items`, `totals` on finalized/signed invoices and C getters on owned `InvoiceData`, `InvoiceLineItem`, and `InvoiceTotals` snapshots.

C field getters operate on an owned `InvoiceData` snapshot obtained with
`fatoora_finalized_invoice_data` or `fatoora_signed_invoice_data`. Check the result,
read the fields, then call `fatoora_invoice_data_free`. Totals follow the same
pattern with `InvoiceTotals`. Snapshots survive destruction of their invoice.

## FinalizedInvoice

### `hash_base64`

???+ note "Get finalized invoice hash (Base64)"

    === "{{ lang.rust }}"
        ```rust
        FinalizedInvoice::hash_base64(&self) -> Result<String, SigningError>
        ```

    === "{{ lang.python }}"
        ```python
        FinalizedInvoice.hash_base64() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_finalized_invoice_hash_base64(const FatooraFinalizedInvoice *self,
            char **out,
            FatooraError **error);
        ```

### `sign`

???+ note "Sign finalized invoice"

    === "{{ lang.rust }}"
        ```rust
        FinalizedInvoice::sign(self, signer: &InvoiceSigner) -> Result<SignedInvoice, SigningError>
        ```

    === "{{ lang.python }}"
        ```python
        FinalizedInvoice.sign(signer: Signer) -> SignedInvoice
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signer_sign(const FatooraSigner *self,
            FatooraFinalizedInvoice *invoice,
            FatooraSignedInvoice **out,
            FatooraError **error);
        ```

### field accessors

???+ note "Read finalized invoice fields"
    Accessors for identity, parties, references, flags, line items, and totals are available per field in Python and C.

    === "{{ lang.python }}"
        ```python
        FinalizedInvoice.id() -> str
        FinalizedInvoice.uuid() -> str
        FinalizedInvoice.issue_datetime() -> str
        FinalizedInvoice.currency() -> str
        FinalizedInvoice.previous_invoice_hash() -> str
        FinalizedInvoice.invoice_counter() -> int
        FinalizedInvoice.payment_means_code() -> str
        FinalizedInvoice.vat_category() -> VatCategory
        FinalizedInvoice.invoice_level_charge() -> Decimal
        FinalizedInvoice.invoice_level_discount() -> Decimal
        FinalizedInvoice.allowance_reason() -> Optional[str]
        FinalizedInvoice.invoice_type_kind() -> InvoiceTypeKind
        FinalizedInvoice.invoice_sub_type() -> InvoiceSubType
        FinalizedInvoice.original_invoice_ref() -> Optional[OriginalInvoiceRef]
        FinalizedInvoice.original_invoice_reason() -> Optional[str]
        FinalizedInvoice.seller() -> Party
        FinalizedInvoice.buyer() -> Optional[Party]
        FinalizedInvoice.note() -> Optional[InvoiceNote]
        FinalizedInvoice.line_items() -> list[InvoiceLineItem]
        FinalizedInvoice.totals() -> InvoiceTotals
        FinalizedInvoice.flags() -> set[InvoiceFlag]
        FinalizedInvoice.is_third_party() -> bool
        FinalizedInvoice.is_nominal() -> bool
        FinalizedInvoice.is_export() -> bool
        FinalizedInvoice.is_summary() -> bool
        FinalizedInvoice.is_self_billed() -> bool
        FinalizedInvoice.is_simplified() -> bool
        FinalizedInvoice.xml() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_finalized_invoice_data(const FatooraFinalizedInvoice *self,
            FatooraInvoiceData **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_data_id(const FatooraInvoiceData *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_data_uuid(const FatooraInvoiceData *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_finalized_invoice_xml(const FatooraFinalizedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_data_line_items_len(const FatooraInvoiceData *self,
            size_t *out,
            FatooraError **error);

        FatooraStatus fatoora_finalized_invoice_totals(const FatooraFinalizedInvoice *self,
            FatooraInvoiceTotals **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_totals_tax_inclusive(const FatooraInvoiceTotals *self,
            char **out,
            FatooraError **error);
        ```

## SignedInvoice

For exact XML preservation and consuming `into_xml()` access, see [XML ownership](xml.md#signedinvoice).

### `xml`

???+ note "Get signed invoice XML"

    === "{{ lang.rust }}"
        ```rust
        SignedInvoice::xml(&self) -> &str
        ```

    === "{{ lang.python }}"
        ```python
        SignedInvoice.xml() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signed_invoice_xml(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);
        ```

### `to_xml_base64`

???+ note "Get signed XML as Base64"

    === "{{ lang.rust }}"
        ```rust
        SignedInvoice::to_xml_base64(&self) -> String
        ```

    === "{{ lang.python }}"
        ```python
        SignedInvoice.to_xml_base64() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signed_invoice_to_xml_base64(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);
        ```

### `hash_base64`

???+ note "Get signed invoice hash (Base64)"

    === "{{ lang.rust }}"
        ```rust
        SignedInvoice::hash_base64(&self) -> Result<String, SigningError>
        ```

    === "{{ lang.python }}"
        ```python
        SignedInvoice.hash_base64() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signed_invoice_hash_base64(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);
        ```

### signature metadata accessors

???+ note "Read signature metadata"

    === "{{ lang.rust }}"
        ```rust
        SignedInvoice::invoice_hash(&self) -> &str
        SignedInvoice::signature(&self) -> &str
        SignedInvoice::public_key(&self) -> &str
        SignedInvoice::zatca_key_signature(&self) -> Option<&str>
        SignedProperties::issuer(&self) -> &str
        SignedProperties::serial(&self) -> &str
        ```

    === "{{ lang.python }}"
        ```python
        SignedInvoice.invoice_hash() -> str
        SignedInvoice.signature() -> str
        SignedInvoice.public_key() -> str
        SignedInvoice.zatca_key_signature() -> Optional[str]
        SignedInvoice.cert_hash() -> str
        SignedInvoice.signed_props_hash() -> str
        SignedInvoice.signing_time() -> str
        SignedInvoice.issuer() -> str
        SignedInvoice.serial() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signed_invoice_invoice_hash(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_signature(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_public_key(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_zatca_key_signature(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_cert_hash(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_signed_props_hash(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_signing_time(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_issuer(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_serial(const FatooraSignedInvoice *self,
            char **out,
            FatooraError **error);
        ```

### `Address.new`

`district` is the single city-district field and maps to XML
`cbc:CitySubdivisionName`. Replace the removed `subdivision` field, accessor,
and constructor argument with `district`, including in serialized address JSON.
C callers must rebuild against the matching header and library.

???+ note "Create address value"

    === "{{ lang.python }}"
        ```python
        Address.new(
            country_code: str,
            city: str,
            street: str,
            building_number: str,
            postal_code: str,
            additional_street: Optional[str] = None,
            additional_number: Optional[str] = None,
            district: Optional[str] = None,
        ) -> Address
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_address_new(const FatooraAddressOptions *options,
            FatooraAddress **out,
            FatooraError **error);
        ```

### field accessors

???+ note "Read signed invoice data fields"
    Signed invoices expose the same business-field accessors as finalized invoices, plus signature metadata.

    === "{{ lang.python }}"
        ```python
        SignedInvoice.id() -> str
        SignedInvoice.uuid() -> str
        SignedInvoice.issue_datetime() -> str
        SignedInvoice.currency() -> str
        SignedInvoice.previous_invoice_hash() -> str
        SignedInvoice.invoice_counter() -> int
        SignedInvoice.payment_means_code() -> str
        SignedInvoice.vat_category() -> VatCategory
        SignedInvoice.invoice_level_charge() -> Decimal
        SignedInvoice.invoice_level_discount() -> Decimal
        SignedInvoice.allowance_reason() -> Optional[str]
        SignedInvoice.invoice_type_kind() -> InvoiceTypeKind
        SignedInvoice.invoice_sub_type() -> InvoiceSubType
        SignedInvoice.original_invoice_ref() -> Optional[OriginalInvoiceRef]
        SignedInvoice.original_invoice_reason() -> Optional[str]
        SignedInvoice.seller() -> Party
        SignedInvoice.buyer() -> Optional[Party]
        SignedInvoice.note() -> Optional[InvoiceNote]
        SignedInvoice.line_items() -> list[InvoiceLineItem]
        SignedInvoice.totals() -> InvoiceTotals
        SignedInvoice.flags() -> set[InvoiceFlag]
        SignedInvoice.is_third_party() -> bool
        SignedInvoice.is_nominal() -> bool
        SignedInvoice.is_export() -> bool
        SignedInvoice.is_summary() -> bool
        SignedInvoice.is_self_billed() -> bool
        SignedInvoice.is_simplified() -> bool
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signed_invoice_data(const FatooraSignedInvoice *self,
            FatooraInvoiceData **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_data_id(const FatooraInvoiceData *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_data_uuid(const FatooraInvoiceData *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_data_line_items_len(const FatooraInvoiceData *self,
            size_t *out,
            FatooraError **error);

        FatooraStatus fatoora_signed_invoice_totals(const FatooraSignedInvoice *self,
            FatooraInvoiceTotals **out,
            FatooraError **error);

        FatooraStatus fatoora_invoice_totals_tax_inclusive(const FatooraInvoiceTotals *self,
            char **out,
            FatooraError **error);
        ```

## Supporting Types

!!! note "Types"
    - `CountryCode`, `CurrencyCode`, `InvoiceTimestamp`, `InvoiceDate`, `VatId`, `OtherId`.
    - `InvoiceNote`, `OriginalInvoiceRef`, `LineItem`, `Party` (`Seller`/`Buyer` roles).

See also: [Invoice Signing Reference](invoice-signing.md)
