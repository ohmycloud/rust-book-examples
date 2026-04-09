use engineering_book::BuildInfo;

fn main() {
    let timestamp = std::env::var("BUILD_TIMESTAMP").unwrap_or_default();
    println!("Build timestamp: {timestamp}");

    print_version();
}

fn print_version() {
    let build_info = BuildInfo::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("GIT_COMMIT_HASH"),
        env!("APP_BUILD_EPOCH"),
        env!("APP_BUILD_TARGET"),
        env!("APP_BUILD_PROFILE"),
    );
    println!("{build_info}");
}
