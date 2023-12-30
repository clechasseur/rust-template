/// Returns hello string
///
/// ```
/// use rust_template_output::hello;
///
/// let hello_str = hello();
/// assert!(!hello_str.is_empty());
/// ```
pub fn hello() -> &'static str {
    "Hello, World!"
}

/// Returns a string for feature!
///
/// ```
/// use rust_template_output::for_feature;
///
/// let feature_str = for_feature();
/// assert!(!feature_str.is_empty());
/// ```
#[cfg(feature = "test_feature_1")]
pub fn for_feature() -> &'static str {
    "For feature!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_one() {
        let hello_str = hello();
        assert!(!hello_str.is_empty());
    }

    #[cfg(feature = "test_feature_1")]
    mod for_feature {
        use super::*;

        #[test]
        fn test_for_feature_one() {
            let feature_str = for_feature();
            assert!(!feature_str.is_empty());
        }
    }
}
