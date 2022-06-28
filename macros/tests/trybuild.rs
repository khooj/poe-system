#[test]
fn macros() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/good.rs");
}