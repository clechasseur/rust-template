/// Returns hello string
/// 
/// ```
/// use rust_template_output::hello;
///
/// let foo = hello();
/// assert!(!foo.is_empty());
/// ```
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_one() {
        let foo = hello();
        assert!(!foo.is_empty());
    }
}
