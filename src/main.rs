use rust_template_output::hello;

fn main() {
    println!("{}", hello());

    if cfg!(feature = "test_feature_1") {
        println!("test-feature-1 is enabled");
    } else {
        println!("test-feature-1 is disabled");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_one() {
        let foo = "foo";
        assert!(!foo.is_empty());
    }
}
