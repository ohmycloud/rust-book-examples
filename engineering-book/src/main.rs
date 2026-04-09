fn main() {
    let timestamp = std::env::var("BUILD_TIMESTAMP").unwrap_or_default();
    println!("Build timestamp: {timestamp}");
}
