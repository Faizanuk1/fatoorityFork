# QR Codes

Generating QR payloads from finalized/signed invoices and reading embedded QR payloads from signed invoices.

## Example

=== "Rust"
    ```rust
    --8<-- "fatoora-core/examples/qr.rs:example"
    ```

=== "Python"
    ```python
    --8<-- "bindings/python/examples/qr.py:example"
    ```

=== "C# / .NET"
    ```csharp
    --8<-- "bindings/dotnet/examples/SdkExamples.cs:qr"
    ```

=== "C"
    ```c
    --8<-- "bindings/c/examples/qr.c:example"
    ```

=== "CLI"
    ```bash
    fatoora qr --invoice invoice.xml
    fatoora qr --invoice signed.xml
    fatoora qr --invoice signed.xml --fail-on-signed
    fatoora qr-read --invoice signed.xml
    ```

## Notes
- The QR payload is a base64-encoded TLV string. Tag ordering follows ZATCA (seller, VAT, timestamp,
  totals) and includes signing tags when available.

See also: [QR Reference](../reference/qr.md)

C examples include `fatoora.h`; compile the adapter once with your application.
See the [C build instructions](../development/c-api.md#build-and-run).

See the [C# / .NET reference](../reference/bindings/csharp.md) for installation, imports, and ownership rules.
