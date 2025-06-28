use rust_template_output::{hello, iterate_maybe};

fn main() {
    println!("{}", hello());

    if cfg!(feature = "test_feature_1") {
        println!("test-feature-1 is enabled");
    } else {
        println!("test-feature-1 is disabled");
    }

    let v: Vec<_> = iterate_maybe(Some("Hello, World!")).collect();
    if let Some(&s) = v.first() {
        println!("{s}");
    }

    let v: Vec<&str> = iterate_maybe(None).collect();
    if let Some(&s) = v.first() {
        println!("{s}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_one() {
        let hello_str = "hello";
        assert!(!hello_str.is_empty());
    }
}
