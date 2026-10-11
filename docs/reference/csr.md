# CSR

C signatures use the [C API](../development/c-api.md). See [C and C++ bindings](bindings/c.md) for ownership and build instructions.
The declarations below come from the generated headers. C++ exposes the same types
in namespace `fatoora` through `fatoora/Type.hpp` headers.

CSR parsing and generation helpers shared by Rust, FFI, and Python.

## CsrProperties

### `new`

???+ note "Create CSR properties"
    Construct `CsrProperties` from explicit fields.

    === "{{ lang.rust }}"
        ```rust
        CsrProperties::new(
            common_name: impl Into<String>,
            serial_number: impl Into<String>,
            organization_identifier: impl Into<String>,
            organization_unit_name: impl Into<String>,
            organization_name: impl Into<String>,
            country_name: impl Into<String>,
            invoice_type: impl Into<String>,
            location_address: impl Into<String>,
            industry_business_category: impl Into<String>,
        ) -> Result<CsrProperties, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        CsrProperties.new(
            common_name: str,
            serial_number: str,
            organization_identifier: str,
            organization_unit_name: str,
            organization_name: str,
            country_name: str,
            invoice_type: str,
            location_address: str,
            industry_business_category: str,
        ) -> CsrProperties
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_properties_new(const FatooraCsrOptions *options,
            FatooraCsrProperties **out,
            FatooraError **error);
        ```

### `from_properties_str`

???+ note "Parse CSR properties string"
    Parse properties text into `CsrProperties`.

    === "{{ lang.rust }}"
        ```rust
        CsrProperties::from_properties_str(properties: &str) -> Result<CsrProperties, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        CsrProperties.from_properties_str(properties: str) -> CsrProperties
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_properties_from_properties_str(const char *properties,
            FatooraCsrProperties **out,
            FatooraError **error);
        ```

### `parse_csr_config`

???+ note "Parse CSR config"
    Parse properties text using the config parser alias.

    === "{{ lang.rust }}"
        ```rust
        CsrProperties::parse_csr_config(properties: &str) -> Result<CsrProperties, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        CsrProperties.parse_csr_config(properties: str) -> CsrProperties
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_properties_from_properties_str(const char *properties,
            FatooraCsrProperties **out,
            FatooraError **error);
        ```

### `parse_csr_config_file`

???+ note "Parse CSR config file"
    Parse properties from a file path.

    === "{{ lang.rust }}"
        ```rust
        CsrProperties::parse_csr_config_file(path: impl AsRef<Path>) -> Result<CsrProperties, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        CsrProperties.parse_csr_config_file(path: str) -> CsrProperties
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_properties_parse_csr_config_file(const char *path,
            FatooraCsrProperties **out,
            FatooraError **error);
        ```

### `build`

???+ note "Build CSR"
    Build a CSR from properties and signing key.

    === "{{ lang.rust }}"
        ```rust
        CsrProperties::build(&self, signer: &SigningKey, env: EnvironmentType) -> Result<Csr, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        CsrProperties.build(key: SigningKey, env: Environment) -> Csr
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_properties_build(const FatooraCsrProperties *self,
            const FatooraSigningKey *key,
            int32_t env,
            FatooraCsr **out,
            FatooraError **error);
        ```

## SigningKey

### `generate`

???+ note "Generate key"
    Generate a new signing key.

    === "{{ lang.rust }}"
        ```rust
        SigningKey::generate() -> SigningKey
        ```

    === "{{ lang.python }}"
        ```python
        SigningKey.generate() -> SigningKey
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signing_key_generate(FatooraSigningKey **out,
            FatooraError **error);
        ```

### `from_pem`

???+ note "Load key from PEM"
    Parse PKCS#8 key material in PEM format.

    === "{{ lang.rust }}"
        ```rust
        SigningKey::from_pem(pem: &str) -> Result<SigningKey, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        SigningKey.from_pem(pem: str) -> SigningKey
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signing_key_from_pem(const char *pem,
            FatooraSigningKey **out,
            FatooraError **error);
        ```

### `from_der`

???+ note "Load key from DER"
    Parse PKCS#8 key material in DER format.

    === "{{ lang.rust }}"
        ```rust
        SigningKey::from_der(der: &[u8]) -> Result<SigningKey, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        SigningKey.from_der(der: bytes) -> SigningKey
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signing_key_from_der(const uint8_t *der,
            size_t der_len,
            FatooraSigningKey **out,
            FatooraError **error);
        ```

### `to_pem`

???+ note "Serialize key to PEM"
    Serialize the key to PEM format.

    === "{{ lang.rust }}"
        ```rust
        SigningKey::to_pem(&self) -> Result<String, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        SigningKey.to_pem() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signing_key_to_pem(const FatooraSigningKey *self,
            char **out,
            FatooraError **error);
        ```

### `to_der`

???+ note "Serialize key to DER"
    Serialize the key to DER format.

    === "{{ lang.rust }}"
        ```rust
        SigningKey::to_der(&self) -> Result<Vec<u8>, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        SigningKey.to_der() -> bytes
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_signing_key_to_der(const FatooraSigningKey *self,
            FatooraBytes **out,
            FatooraError **error);
        ```

## Csr

### `from_der`

???+ note "Load CSR from DER"
    Parse CSR bytes in DER format.

    === "{{ lang.rust }}"
        ```rust
        Csr::from_der(der: &[u8]) -> Result<Csr, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        Csr.from_der(der: bytes) -> Csr
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_from_der(const uint8_t *der,
            size_t der_len,
            FatooraCsr **out,
            FatooraError **error);
        ```

### `to_pem`

???+ note "Serialize CSR to PEM"

    === "{{ lang.rust }}"
        ```rust
        Csr::to_pem(&self) -> Result<String, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        Csr.to_pem() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_to_pem(const FatooraCsr *self,
            char **out,
            FatooraError **error);
        ```

### `to_der`

???+ note "Serialize CSR to DER"

    === "{{ lang.rust }}"
        ```rust
        Csr::to_der(&self) -> Result<Vec<u8>, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        Csr.to_der() -> bytes
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_to_der(const FatooraCsr *self,
            FatooraBytes **out,
            FatooraError **error);
        ```

### `to_base64`

???+ note "Serialize CSR DER to Base64"

    === "{{ lang.rust }}"
        ```rust
        Csr::to_base64(&self) -> Result<String, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        Csr.to_base64() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_to_base64(const FatooraCsr *self,
            char **out,
            FatooraError **error);
        ```

### `to_pem_base64`

???+ note "Serialize PEM CSR to Base64"

    === "{{ lang.rust }}"
        ```rust
        Csr::to_pem_base64(&self) -> Result<String, CsrError>
        ```

    === "{{ lang.python }}"
        ```python
        Csr.to_pem_base64() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_to_pem_base64(const FatooraCsr *self,
            char **out,
            FatooraError **error);
        ```

### `subject_string`

???+ note "Get CSR subject string"

    === "{{ lang.rust }}"
        ```rust
        Csr::subject_string(&self) -> String
        ```

    === "{{ lang.python }}"
        ```python
        Csr.subject_string() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_subject_string(const FatooraCsr *self,
            char **out,
            FatooraError **error);
        ```

### `extension_values_der`

???+ note "Get extension DER values"

    === "{{ lang.rust }}"
        ```rust
        Csr::extension_values_der(&self) -> Vec<Vec<u8>>
        ```

    === "{{ lang.python }}"
        ```python
        Csr.extension_values_der() -> list[bytes]
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csr_extension_values_der(const FatooraCsr *self,
            FatooraBytesList **out,
            FatooraError **error);
        ```

## Errors

!!! warning "Errors"
    - `CsrError` covers parsing, missing fields, subject/SAN build failures, encoding issues, and IO.

## Notes

!!! note "Notes"
    - The template name extension is selected from `EnvironmentType`.
    - `Csr.to_pem_base64()` is the value expected by the ZATCA compliance endpoint.

See also: [CSR Guide](../guides/csr.md)
