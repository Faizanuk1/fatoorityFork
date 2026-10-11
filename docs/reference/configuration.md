# Configuration

C signatures use the [C API](../development/c-api.md). See [C and C++ bindings](bindings/c.md) for ownership and build instructions.
The declarations below come from the generated headers. C++ exposes the same types
in namespace `fatoora` through `fatoora/Type.hpp` headers.

There are three main "environments" which are required to determine which ZATCA API endpoint to use.
Additionally, the type of environment changes the signing/validation process slightly.

- "Non-Production"
      Sometimes also called the "Sandbox". It's open to anyone to use without registration for
      initial testing. It's a dummy environment with mainly hardcoded values and responses. For
      instance, the OTP "123345" is always considered valid.

      Endpoint: ["https://gw-fatoora.zatca.gov.sa/e-invoicing/developer-portal/"]

- "Simulation":
      The simulation environment requires registration at the developer portal. It's also used for
      testing only.

      Endpoint: ["https://gw-fatoora.zatca.gov.sa/e-invoicing/simulation/"]

- "Production":
      This is the real deal environment for in-production systems where real invoices are to be
      reported.

      Endpoint: ["https://gw-fatoora.zatca.gov.sa/e-invoicing/core/"]

C constructors accept `uint8_t` environment values: `0` for non-production,
`1` for simulation, and `2` for production. Other values return an error.

## Environment

### `from_str`

???+ note "Parse environment"
    Convert a string value to an environment enum.

    === "{{ lang.rust }}"
        ```rust
        EnvironmentType::from_str(env: &str) -> Result<EnvironmentType, EnvironmentParseError>
        ```

    === "{{ lang.python }}"
        ```python
        Environment(value: str) -> Environment
        ```

    === "{{ lang.c }}"
        ```c
        /* Pass uint8_t: 0 = non-production, 1 = simulation, 2 = production. */
        ```

### `as_str`

???+ note "Get canonical string"
    Return the canonical string value of the environment.

    === "{{ lang.rust }}"
        ```rust
        EnvironmentType::as_str(&self) -> &'static str
        ```

    === "{{ lang.python }}"
        ```python
        Environment.value: str
        ```

    === "{{ lang.c }}"
        ```c
        /* not exposed; use enum value directly */
        ```

### `endpoint_url`

???+ note "Resolve API base URL"
    Return the ZATCA API base URL for the selected environment.

    === "{{ lang.rust }}"
        ```rust
        EnvironmentType::endpoint_url(&self) -> &'static str
        ```

    === "{{ lang.python }}"
        ```python
        # not directly exposed
        ```

    === "{{ lang.c }}"
        ```c
        /* not exposed */
        ```

## Config

### `new`

???+ note "Create config"
    Create an environment-aware config handle.

    === "{{ lang.rust }}"
        ```rust
        Config::new(env: EnvironmentType) -> Config
        ```

    === "{{ lang.python }}"
        ```python
        Config(env: Environment = Environment.NON_PRODUCTION)
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_config_new(int32_t env,
            FatooraConfig **out,
            FatooraError **error);
        ```

### `env`

???+ note "Read config environment"
    Return the environment value stored in a config.

    === "{{ lang.rust }}"
        ```rust
        Config::env(&self) -> EnvironmentType
        ```

    === "{{ lang.python }}"
        ```python
        Config.env() -> Environment
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_config_env(const FatooraConfig *self,
            uint8_t *out,
            FatooraError **error);
        ```

### `default`

???+ note "Default config"
    Create a default config using non-production environment.

    === "{{ lang.rust }}"
        ```rust
        Config::default() -> Config
        ```

    === "{{ lang.python }}"
        ```python
        Config() -> Config
        ```

    === "{{ lang.c }}"
        ```c
        #include "fatoora.h"

        FatooraStatus fatoora_config_new(int32_t env,
            FatooraConfig **out,
            FatooraError **error);
        ```
