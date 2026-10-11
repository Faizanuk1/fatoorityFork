# API parity

The [binding parity map](ffi-parity.md) tracks the Diplomat bridge by capability
and links to its regression tests. The handwritten C symbol matrix has been
replaced by that map.

Use the [C API header](https://github.com/mqqz/fatoority/blob/main/bindings/c/fatoora.h) or
[C++ headers](https://github.com/mqqz/fatoority/tree/main/bindings/cpp) for exact signatures. Python's public API is
checked against its [method inventory](https://github.com/mqqz/fatoority/blob/main/bindings/python/tests/public_api.json).

Local ZATCA validation is exposed as `fatoora_xml_validate_zatca` in C and
`validate_zatca_invoice_from_str` in Python. Both preserve reports for rejection
or incomplete coverage and structured errors for execution failure.
