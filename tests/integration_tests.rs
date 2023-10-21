use rust_template_output::hello;

#[test]
fn test_integration_one() {
    let foo = hello();
    assert!(!foo.is_empty());
}
