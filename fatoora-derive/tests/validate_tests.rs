#[test]
fn validate_macro_tests() {
    let t = trybuild::TestCases::new();

    // Should compile
    t.pass("tests/ok/basic_valid.rs");
    t.pass("tests/ok/struct_rules.rs");
    t.pass("tests/ok/skip_field.rs");
    t.pass("tests/ok/generic_borrowed.rs");
    t.pass("tests/ok/generic_error.rs");

    // Should fail compilation
    t.compile_fail("tests/fail/unknown_rule.rs");
    t.compile_fail("tests/fail/wrong_type.rs");
    t.compile_fail("tests/fail/malformed_rules.rs");
    t.compile_fail("tests/fail/qualified_rule.rs");
    t.compile_fail("tests/fail/empty_error.rs");
    t.compile_fail("tests/fail/tuple_struct.rs");
    t.compile_fail("tests/fail/enum.rs");
    t.compile_fail("tests/fail/non_string_types.rs");
}
