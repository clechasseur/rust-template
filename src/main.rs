fn main() {
    println!("Hello, world!");

    if cfg!(feature = "test_feature_1") {
        println!("test-feature-1 is enabled");
    } else {
        println!("test-feature-1 is disabled");
    }
}
