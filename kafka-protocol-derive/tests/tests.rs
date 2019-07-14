#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/version.rs");
    t.pass("tests/structs.rs");
}
