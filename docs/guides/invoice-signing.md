# Invoice Signing

How to sign UBL invoices with a CSID certificate and private key.

## Example

=== "Rust"
    ```rust
    --8<-- "fatoora-core/examples/invoice_signing.rs:example"
    ```

=== "Python"
    ```python
    --8<-- "bindings/python/examples/invoice_signing.py:example"
    ```

=== "JavaScript / TypeScript"
    ```js
    --8<-- "bindings/js/examples/sdk.mjs:signing"
    ```

=== "C# / .NET"
    ```csharp
    --8<-- "bindings/dotnet/examples/SdkExamples.cs:signing"
    ```

=== "C"
    ```c
    --8<-- "bindings/c/examples/invoice_signing.c:example"
    ```

## Notes
- Signed invoice metadata (signature/public key/signed props hashes) are exposed as getters.
- The C example reads certificate/key DER in the application and uses the
  invoice file helper. Signing consumes the finalized invoice; all handles are
  released during cleanup.

See also: [Invoice Signing Reference](../reference/invoice-signing.md)

C examples include `fatoora.h`; compile the adapter once with your application.
See the [C build instructions](../development/c-api.md#build-and-run).

See the [JavaScript / TypeScript reference](../reference/bindings/javascript.md) for installation and runtime contracts.

See the [C# / .NET reference](../reference/bindings/csharp.md) for installation, imports, and ownership rules.
