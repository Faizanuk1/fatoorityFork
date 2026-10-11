# ZATCA API Client

C signatures use the [C API](../development/c-api.md). See [C and C++ bindings](bindings/c.md) for ownership and build instructions.
The declarations below come from the generated headers. C++ exposes the same types
in namespace `fatoora` through `fatoora/Type.hpp` headers.

The HTTP client is the main way to contact ZATCA's official fatoora platform API.
Please do read the [ZATCA Official API Gateway](https://sandbox.zatca.gov.sa/IntegrationSandbox)
for more details.

## ZatcaClient

### `new`

???+ note "Create client"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::new(config: Config) -> Result<ZatcaClient, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient(config: Config)
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_create(const FatooraConfig *config,
            FatooraZatcaClient **out,
            FatooraError **error);
        ```

### `post_csr_for_ccsid`

???+ note "Issue compliance CSID"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::post_csr_for_ccsid(csr: &Csr, otp: &str) -> Result<CsidCredentials<Compliance>, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient.post_csr_for_ccsid(csr: Csr, otp: str) -> CsidCompliance
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_post_csr_for_ccsid(const FatooraZatcaClient *self,
            const FatooraCsr *csr,
            const char *otp,
            FatooraCsidCompliance **out,
            FatooraError **error);
        ```

### `post_ccsid_for_pcsid`

???+ note "Issue production CSID"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::post_ccsid_for_pcsid(ccsid: &CsidCredentials<Compliance>) -> Result<CsidCredentials<Production>, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient.post_ccsid_for_pcsid(ccsid: CsidCompliance) -> CsidProduction
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_post_ccsid_for_pcsid(const FatooraZatcaClient *self,
            const FatooraCsidCompliance *credentials,
            FatooraCsidProduction **out,
            FatooraError **error);
        ```

### `renew_csid`

???+ note "Renew production CSID"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::renew_csid(pcsid: &CsidCredentials<Production>, csr: &Csr, otp: &str, accept_language: Option<&str>) -> Result<CsidCredentials<Production>, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient.renew_csid(pcsid: CsidProduction, csr: Csr, otp: str, accept_language: Optional[str]) -> CsidProduction
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_renew_csid(const FatooraZatcaClient *self,
            const FatooraCsidProduction *credentials,
            const FatooraCsr *csr,
            const char *otp,
            const char *accept_language,
            FatooraCsidProduction **out,
            FatooraError **error);
        ```

### `check_invoice_compliance`

???+ note "Check invoice compliance"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::check_invoice_compliance(invoice: &SignedInvoice, credentials: &CsidCredentials<Compliance>) -> Result<ValidationResponse, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient.check_invoice_compliance(invoice: SignedInvoice, ccsid: CsidCompliance) -> ValidationResponse
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_check_invoice_compliance(const FatooraZatcaClient *self,
            const FatooraSignedInvoice *invoice,
            const FatooraCsidCompliance *credentials,
            FatooraValidationResponse **out,
            FatooraError **error);
        ```

### `report_simplified_invoice`

???+ note "Report simplified invoice"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::report_simplified_invoice(invoice: &SignedInvoice, credentials: &CsidCredentials<Production>, clearance_status: bool, accept_language: Option<&str>) -> Result<ValidationResponse, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient.report_simplified_invoice(invoice: SignedInvoice, pcsid: CsidProduction, clearance_status: bool, accept_language: Optional[str]) -> ValidationResponse
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_report_simplified_invoice(const FatooraZatcaClient *self,
            const FatooraSignedInvoice *invoice,
            const FatooraCsidProduction *credentials,
            bool clearance_status,
            const char *accept_language,
            FatooraValidationResponse **out,
            FatooraError **error);
        ```

### `clear_standard_invoice`

???+ note "Clear standard invoice"

    === "{{ lang.rust }}"
        ```rust
        ZatcaClient::clear_standard_invoice(invoice: &SignedInvoice, credentials: &CsidCredentials<Production>, clearance_status: bool, accept_language: Option<&str>) -> Result<ValidationResponse, ZatcaError>
        ```

    === "{{ lang.python }}"
        ```python
        ZatcaClient.clear_standard_invoice(invoice: SignedInvoice, pcsid: CsidProduction, clearance_status: bool, accept_language: Optional[str]) -> ValidationResponse
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_zatca_client_clear_standard_invoice(const FatooraZatcaClient *self,
            const FatooraSignedInvoice *invoice,
            const FatooraCsidProduction *credentials,
            bool clearance_status,
            const char *accept_language,
            FatooraValidationResponse **out,
            FatooraError **error);
        ```

## CsidCredentials / CsidCompliance / CsidProduction

### `new`

???+ note "Create credential handle"

    === "{{ lang.rust }}"
        ```rust
        CsidCredentials::new(env: EnvironmentType, request_id: Option<String>, token: impl Into<String>, secret: impl Into<String>) -> CsidCredentials<T>
        ```

    === "{{ lang.python }}"
        ```python
        CsidCompliance.new(env: Environment, token: str, secret: str, request_id: Optional[str] = None) -> CsidCompliance
        CsidProduction.new(env: Environment, token: str, secret: str, request_id: Optional[str] = None) -> CsidProduction
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csid_compliance_create(int32_t environment,
            const char *request_id,
            const char *token,
            const char *secret,
            FatooraCsidCompliance **out,
            FatooraError **error);

        FatooraStatus fatoora_csid_production_create(int32_t environment,
            const char *request_id,
            const char *token,
            const char *secret,
            FatooraCsidProduction **out,
            FatooraError **error);
        ```

### `env`

???+ note "Read credential environment"

    === "{{ lang.rust }}"
        ```rust
        CsidCredentials::env(&self) -> EnvironmentType
        ```

    === "{{ lang.python }}"
        ```python
        CsidCompliance.env() -> Environment
        CsidProduction.env() -> Environment
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csid_compliance_env(const FatooraCsidCompliance *self,
            uint8_t *out,
            FatooraError **error);

        FatooraStatus fatoora_csid_production_env(const FatooraCsidProduction *self,
            uint8_t *out,
            FatooraError **error);
        ```

### `request_id`

???+ note "Read optional request id"

    === "{{ lang.rust }}"
        ```rust
        CsidCredentials::request_id(&self) -> Option<&str>
        ```

    === "{{ lang.python }}"
        ```python
        CsidCompliance.request_id() -> str
        CsidProduction.request_id() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csid_compliance_request_id(const FatooraCsidCompliance *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_csid_production_request_id(const FatooraCsidProduction *self,
            char **out,
            FatooraError **error);
        ```

### `binary_security_token`

???+ note "Read token"

    === "{{ lang.rust }}"
        ```rust
        CsidCredentials::binary_security_token(&self) -> &str
        ```

    === "{{ lang.python }}"
        ```python
        CsidCompliance.binary_security_token() -> str
        CsidProduction.binary_security_token() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csid_compliance_binary_security_token(const FatooraCsidCompliance *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_csid_production_binary_security_token(const FatooraCsidProduction *self,
            char **out,
            FatooraError **error);
        ```

### `secret`

???+ note "Read credential secret"

    === "{{ lang.rust }}"
        ```rust
        CsidCredentials::secret(&self) -> &str
        ```

    === "{{ lang.python }}"
        ```python
        CsidCompliance.secret() -> str
        CsidProduction.secret() -> str
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_csid_compliance_secret(const FatooraCsidCompliance *self,
            char **out,
            FatooraError **error);

        FatooraStatus fatoora_csid_production_secret(const FatooraCsidProduction *self,
            char **out,
            FatooraError **error);
        ```

## Invoice response contract

This contract applies to reporting, clearance, and invoice compliance methods.
`Ok` (or a Python return value) means HTTP 2xx with a decoded validation body.
Non-2xx responses, including 400 and 409, return errors. Successful HTTP status
alone does not establish invoice acceptance.

| Accessor | Rust | Python | C result value |
| --- | --- | --- | --- |
| `http_status()` | `Option<u16>` | `Optional[int]` | `uint16_t` output plus `bool has_value` |
| `outcome()` | `InvoiceOutcome` | `InvoiceOutcome` | `FatooraInvoiceOutcome`: unknown, accepted, rejected |
| `ensure_accepted()` | `Result<(), ZatcaError>` | returns `None` or raises `ApiError` | `FATOORA_OK` or an error status |
| `cleared_invoice_base64()` | `Option<&str>` | `Optional[str]` | copied string; null if absent |
| `cleared_invoice_xml()` | `Result<Option<String>, ZatcaError>` | `Optional[str]` or `ParseError` | copied decoded string or error; null if absent |

C accessor names start with `fatoora_validation_response_`. Text getters return
owned strings through `char **out`; optional text returns `NULL` for absence,
distinct from a present empty string. Release text with `fatoora_string_free()`.
Response children are owned copies and survive destruction of the response.
A present but empty base64 field stays an empty string and fails decoding.

Outcome uses the endpoint actually invoked: `REPORTED`/`NOT_REPORTED` for
reporting, `CLEARED`/`NOT_CLEARED` for clearance, and `PASS` or `WARNING`/`ERROR`
for compliance. A positive status alongside validation errors is `Unknown`.
Unknown or missing endpoint status also yields `Unknown`. Standalone JSON
serialization omits HTTP/operation metadata; deserializing a body yields no HTTP
status and an unknown outcome, even if the JSON includes metadata-like fields.
`ensure_accepted()` fails for both rejected and unknown outcomes and retains the
response in `ZatcaError::NotAccepted`.

A compliance pass establishes only the result of that check. Cleared XML decoding
preserves the returned text and performs neither XML parsing nor signature
verification. Missing cleared XML is independent of the acceptance outcome.

### Duplicate submissions and redirects

Reporting HTTP 409 can mean that the same invoice hash was already reported;
this submission returns an error without asserting that the earlier invoice was
rejected. Duplicate clearance HTTP 208 follows the successful-response path and
can carry the cleared invoice. See the [Fatoora Development Team announcement](https://zatca1.discourse.group/t/deployment-notification-duplication-check-response-codes-208-and-409/7954).

The client does not follow HTTP redirects. A clearance-disabled 303 is exposed
as a structured response error; callers decide what to do next. This change does
not add automatic reporting of standard invoices. See the [ZATCA developer manual](https://www.zatca.gov.sa/en/E-Invoicing/SystemsDevelopers/ComplianceEnablementToolbox/Documents/Developer%20Portal%20User%20Manual.pdf).

Credential endpoints keep their existing response rules, including the wrapped
HTTP 428 renewal response.

### Migration

Code that previously handled 400/409 through `Ok(ValidationResponse)` must now
handle `ZatcaError::Response`, or `ApiError` in Python. Invoice HTTP 401 still
classifies as Unauthorized but now uses the status-preserving `Response` variant.
Other non-2xx invoice responses classify as Api. Successful HTTP responses with
malformed bodies classify as Parse; failed body reads classify as Network and
retain the received status. Rust callers can inspect `ZatcaError::http_status()`
and `HttpResponseError::body()` / `validation_response()`.

## Response Types

!!! note "Response shapes"
    - `ValidationResponse` exposes `validation_results()`, `reporting_status()`, `clearance_status()`, and QR status fields.
    - `ValidationResults` exposes `info_messages()`, `warning_messages()`, `error_messages()`, and `status()`.
    - `ValidationMessage` exposes type, code, category, message, and status fields.
    - `MessageList` normalizes info messages that may be one, many, or empty.
    - `UnauthorizedResponse` and `ServerErrorResponse` provide structured error bodies.

## Errors

!!! warning "Errors"
    - `ZatcaError` is returned for HTTP failures, invalid responses, unauthorized/server errors, and client-state mismatches (environment, invoice type, etc.).

See also: [API Client Guide](../guides/api.md)
