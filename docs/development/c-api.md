# C API

Include `fatoora.h` from `bindings/c`. The C API covers the same
invoice, crypto, XML, and ZATCA operations as the generated bindings. Public
signatures use ordinary C strings, opaque owners, named-field options, and a
shared status convention. Diplomat writers, views, and result unions stay inside
the implementation.

The implementation is a C source adapter over `fatoora_ffi`.
Compile `fatoora.c` once into your application or a static library, and link the
existing Rust library. Its allocations use that C translation unit's allocator;
Rust-owned objects are always released through their native destructors. This
also avoids mixing C and Rust allocators or Windows CRT heaps.

## Build and run

From a checkout:

```sh
cargo build -p fatoora-ffi --locked
cc -std=c11 -Ibindings/c \
  bindings/c/examples/build_invoice.c bindings/c/fatoora.c \
  -Ltarget/debug -lfatoora_ffi -Wl,-rpath,"$PWD/target/debug" \
  -o /tmp/build-invoice
/tmp/build-invoice
```

The example constructs a simplified invoice with an exact `0.3333` unit price,
builds it, and prints its XML. It demonstrates cleanup on both success and error.
This is invoice construction; the example does not establish ZATCA compliance or
submit an invoice.

The other guide examples use the same compile command: replace
`examples/build_invoice.c` and the executable name with the selected example.

| Example | Arguments | Output |
| --- | --- | --- |
| `csr` | `csr.properties` | Private key PEM and CSR PEM |
| `invoice_signing` | `invoice.xml certificate.der private-key.der` | Signed XML |
| `qr` | `signed-invoice.xml` | Embedded base64 QR payload |
| `validation` | `invoice.xml` | XSD validation result for the original XML |
| `api` | `signed-invoice.xml token secret` | Reports a simplified invoice to the non-production gateway |

See the [CSR](../guides/csr.md), [signing](../guides/invoice-signing.md),
[QR](../guides/qr.md), [validation](../guides/validation.md), and
[API](../guides/api.md) guides for complete source examples. Calling `api` with
no arguments prints help and performs no request.

Native release archives contain `include/fatoora.h`, `src/fatoora.c`,
`examples/c`, and the underlying headers in `include/generated`. Compile
`src/fatoora.c` with `-Iinclude`; it locates the generated headers internally.
For C++ applications, compile the
adapter as C and link that object; the public header supports C++17.

## Common conventions

All operations return `FatooraStatus`: `FATOORA_OK` on success, otherwise the
native numeric error classification. Unknown future codes remain representable.
The optional last parameter receives an owned `FatooraError`; pass `NULL` when
only the status matters. Error message and JSON details are borrowed strings
valid until `fatoora_error_free()`. Structured native error details are preserved.

```c
FatooraError *error = NULL;
char *xml = NULL;
FatooraStatus status = fatoora_finalized_invoice_xml(invoice, &xml, &error);
if (status == FATOORA_OK) {
    puts(xml);
    fatoora_string_free(xml);
} else {
    fprintf(stderr, "%s\n", fatoora_error_message(error));
    fatoora_error_free(error);
}
```

Output slots are required and cleared on entry, including on failure. Release
previous values before reusing a slot: clearing a slot does not free its previous
owner. An error slot follows the same rule. Allocation failure can leave an error
object or its diagnostic strings unavailable, so always use the returned status
to decide success. `fatoora_error_message(NULL)` returns an empty string, and
`fatoora_error_details_json(NULL)` returns `{}`. Native error classifications
survive diagnostic allocation failure; output allocation failure returns
`FATOORA_INTERNAL`, never truncated success.

Text inputs are borrowed, NUL-terminated UTF-8 strings. Required strings reject
`NULL`; optional strings use `NULL` for absence and `""` for a present empty
value. The C string API ends at the first NUL. Use the raw length-delimited ABI
if you need to validate a buffer that may contain embedded NUL. Input text is
copied or processed within the call; it is not retained as a borrowed pointer.

Decimal inputs and outputs are decimal strings, preserving the Rust/Python exact
decimal contract. Enum-like inputs accept named `FATOORA_*` constants; unknown
values are validated before mutation, including values that would otherwise wrap
when narrowed to a byte. Invoice flags combine `FATOORA_FLAG_*` constants with `|`.

Options structs support C designated initializers. Zero-initialize them, then
set required fields:

```c
FatooraInvoiceOptions options = {
    .kind = FATOORA_TAX,
    .subtype = FATOORA_SIMPLIFIED
};
FatooraInvoiceBuilder *builder = NULL;
FatooraStatus status = fatoora_invoice_builder_new(&options, &builder, &error);
```

- `FatooraInvoiceOptions`: tax/prepayment require kind/subtype; credit/debit notes
  additionally require `original_id` and `reason`. UUID/date are optional.
- `FatooraAddressOptions`: country, city, street, building number, and postal code
  are required; additional street/number and district are optional.
- `FatooraPartyOptions`: name/address are required; seller VAT ID is required,
  buyer VAT ID is optional, and other ID/scheme are optional.
- `FatooraLineItemOptions`: description, quantity, unit code, unit price, and VAT
  rate are required; category uses a `FATOORA_VAT_*` constant.
- `FatooraCsrOptions`: all text fields are required, matching the CSR constructor.

The native validation rules still apply to values supplied in these fields.

## Ownership and consumption

Every returned handle is owned and has a matching `fatoora_<type>_free()`.
Strings are owned NUL-terminated copies released by `fatoora_string_free()`.
Optional text getters return a nullable owned string directly, without exposing
an intermediate `Text` owner. Optional handle getters likewise return `NULL` for
absence. HTTP status uses a separate `has_value` flag, since zero is not absence.
Every free function accepts `NULL`.

Invoice data, totals, parties, addresses, line items, and response getters return
independent snapshots. They survive releasing their source owner.
`fatoora_bytes_as_slice()` returns a read-only `FatooraBytesView` borrowed from its
immutable bytes owner; copy its contents before freeing that owner. Binary input
uses pointer + length; `NULL` is accepted only with a zero length.

Consumption follows the existing Rust bridge:

- `fatoora_invoice_builder_build()` consumes inner state on success and invoice
  validation failure. Invalid arguments rejected by the facade do not consume it.
- `fatoora_signer_sign()` consumes the finalized invoice when native signing
  starts, including signing failure.
- `fatoora_signed_invoice_into_xml()` consumes the signed invoice when the native
  call starts. Prefer `fatoora_signed_invoice_xml()` for a non-consuming copy.

The outer handle always remains yours to free. Further operations after
consumption return an error. Builder setters validate before updating state, so
an invalid decimal can be corrected. If output allocation fails after a
consuming native call, consumption is not rolled back.

ZATCA calls are synchronous. Callers must synchronize shared owners, with
exclusive access during mutation, consumption, or freeing. Null checks cannot
validate dangling pointers, wrong handle types, or unterminated input strings.

## Maintenance

The facade generator reads the generated C headers. `generate_bindings.py` also
regenerates/checks the facade, so native signature changes cannot silently leave
stale wrappers. Edit `scripts/generate_c_facade.py` or its runtime template
`scripts/c_facade_runtime.c`, not the generated facade files.

```sh
python scripts/generate_c_facade.py --check
python scripts/check_native_bindings.py
python scripts/check_c_examples.py
```

The native checks run the generated C/C++ contracts, C ownership/error
and allocation-failure contracts, and the complete invoice example. The raw ABI
remains available for generated C++, Python, and .NET bindings and advanced C
callers.
