use rust_template_output::hello;

#[test]
fn test_integration_one() {
    let hello_str = hello();
    assert!(!hello_str.is_empty());
}
