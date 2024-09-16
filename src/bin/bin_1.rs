use rust_template_output::hello;

fn main() {
    println!("{} (from bin_1)", hello());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bin_1_one() {
        let hello_str = "hello";
        assert!(!hello_str.is_empty());
    }
}
