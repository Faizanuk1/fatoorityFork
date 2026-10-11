# FFI workflow

1. Add the capability and its behavior tests in `fatoora-core`.
2. Expose it in the appropriate `fatoora-ffi` bridge module. Keep core errors
   structured through `common::core_error` and fallible operations within
   `common::boundary`. Validate text and enum inputs before mutating state.
3. Run `python scripts/generate_bindings.py`; generated files are checked in.
   This also regenerates the C API from the C headers. Review any new
   argument shapes and ownership rules in [the C API](c-api.md).
4. Adapt the public Python facade where conversion or ownership requires it.
5. Test Rust contracts, real C/C++ callers, and the installed Python wheel.
   Include failure, consumption, child lifetime, and concurrency cases relevant
   to the change.

See [Diplomat bindings](diplomat.md) for commands, ownership rules, pinned tools,
and packaging requirements, and [binding parity](ffi-parity.md) for coverage.
