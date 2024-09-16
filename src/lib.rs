//! A simple lib crate to test the `rust-template`.

use std::iter;

use either::Either;

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

/// Returns an iterator with either one or no item.
///
/// ```
/// use rust_template_output::iterate_maybe;
///
/// let mut it = iterate_maybe(Some("hello!"));
/// assert_eq!(Some("hello!"), it.next());
/// assert_eq!(None, it.next());
///
/// let mut it = iterate_maybe::<&str>(None);
/// assert_eq!(None, it.next());
/// ```
pub fn iterate_maybe<T>(value: Option<T>) -> impl Iterator<Item = T> {
    match value {
        Some(value) => Either::Left(iter::once(value)),
        None => Either::Right(iter::empty()),
    }
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

    #[test]
    fn test_iterate_maybe() {
        let v: Vec<_> = iterate_maybe(Some(42)).collect();
        assert_eq!(vec![42], v);

        let v: Vec<i32> = iterate_maybe(None).collect();
        assert!(v.is_empty());
    }
}
