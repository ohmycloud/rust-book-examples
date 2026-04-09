mod version;

// src/lib.rs - include the generated code
// OUT_DIR is a Cargo-provided directory where build scripts should place generated files.
// Each crate gets its own OUT_DIR under target/.
pub mod diagnostics {
    include!(concat!(env!("OUT_DIR"), "/diagnostics.rs"));
}

pub mod telemetry {
    include!(concat!(env!("OUT_DIR"), "/telemetry.rs"));
}
