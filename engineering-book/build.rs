use std::process::Command;

fn main() {
    minimal_example();
    compile_time_constants();
}

fn minimal_example() {
    // Only re-run if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");

    // Set a compile-time environment variable
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    println!("cargo::rustc-env=BUILD_TIMESTAMP={timestamp}");
}

fn compile_time_constants() {
    println!("cargo::rerun-if-env-changed=.git/HEAD");
    println!("cargo::rerun-if-env-changed=.git/refs");

    // Git commit hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("git not found");

    let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("cargo::rustc-env=GIT_COMMIT_HASH={git_hash}");

    // Build profile (debug or release)
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".into());
    println!("cargo::rustc-env=BUILD_PROFILE={profile}");

    // Target triple
    let target_triple = std::env::var("TARGET").unwrap_or_else(|_| "unknown".into());
    println!("cargo::rustc-env=TARGET_TRIPLE={target_triple}");
}

fn print_version() {
    println!(
        "{} {} (git:{} target:{} profile:{})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("GIT_COMMIT_HASH"),
        env!("BUILD_TARGET"),
        env!("BUILD_PROFILE")
    )
}
