use rust_template_output::{hello, iterate_maybe};
use serial_test::serial;

#[test]
fn test_integration_one() {
    let hello_str = hello();
    assert!(!hello_str.is_empty());
}

#[test]
#[serial]
fn test_iterate_one() {
    let v: Vec<_> = iterate_maybe(Some(42)).collect();
    assert_eq!(vec![42], v);
}

#[test]
#[serial]
fn test_iterate_none() {
    let v: Vec<i32> = iterate_maybe(None).collect();
    assert!(v.is_empty());
}
