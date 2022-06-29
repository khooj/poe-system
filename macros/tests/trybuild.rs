#[test]
fn macros() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/good.rs");
    t.compile_fail("tests/ui/expected_identifier.rs");
    t.compile_fail("tests/ui/file_not_exist.rs");
}