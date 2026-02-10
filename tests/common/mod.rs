use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Run the compiler with given mode/input, return generated Rust output as a string
pub fn run_compiler(mode: &str, input: &str) -> String {
    let binary = env!("CARGO_BIN_EXE_asl-rust-compiler-rs");
    let unique_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let out_path = std::env::temp_dir().join(format!("asl_codegen_test_{nanos}_{unique_id}.rs"));
    let out_file = out_path.to_string_lossy().into_owned();

    let output = Command::new(binary)
        .args([mode, input, out_file.as_str()])
        .output()
        .expect("failed to run compiler binary");

    assert!(
        output.status.success(),
        "compiler failed for mode={mode}, input={input}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let generated = fs::read_to_string(&out_path)
        .unwrap_or_else(|e| panic!("failed to read generated file {}: {e}", out_path.display()));
    let _ = fs::remove_file(&out_path);
    generated
}
