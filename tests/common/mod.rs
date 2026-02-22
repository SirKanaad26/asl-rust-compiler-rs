use std::fs;
use std::path::Path;
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

/// Compile and run a generated tock instruction file with a custom `fn main()`.
///
/// Steps:
///   1. Run the ASL compiler on `asl_file` to produce the instruction source.
///   2. Append `main_fn` (a `fn main() { ... }` with assert_eq! checks) to it.
///   3. Copy `output/tock/bitvec.rs` and `output/tock/runtime.rs` alongside it.
///   4. Compile with `rustc +nightly` and run; panics in main propagate as test failures.
pub fn run_tock_test(asl_file: &str, main_fn: &str) {
    let binary = env!("CARGO_BIN_EXE_asl-rust-compiler-rs");
    let unique_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();

    let tmp = std::env::temp_dir().join(format!("tock_test_{nanos}_{unique_id}"));
    fs::create_dir_all(&tmp).expect("failed to create temp dir");

    let rs_path = tmp.join("instr.rs");

    // 1. Generate instruction source into the temp dir
    let compile_out = Command::new(binary)
        .args(["instructions", asl_file, rs_path.to_str().unwrap()])
        .output()
        .expect("failed to run compiler binary");
    assert!(
        compile_out.status.success(),
        "compiler failed for {asl_file}\n{}",
        String::from_utf8_lossy(&compile_out.stderr)
    );

    // 2. Append fn main
    let mut src = fs::read_to_string(&rs_path).expect("failed to read generated source");
    src.push('\n');
    src.push_str(main_fn);
    fs::write(&rs_path, &src).expect("failed to write source with main");

    // 3. Copy shared runtime files
    for name in ["bitvec.rs", "runtime.rs"] {
        fs::copy(Path::new("output/tock").join(name), tmp.join(name))
            .unwrap_or_else(|e| panic!("failed to copy {name}: {e}"));
    }

    // 4. Compile with rustc +nightly
    let bin_path = tmp.join("instr");
    let rustc = Command::new("rustc")
        .args(["+nightly", "--edition", "2021",
               rs_path.to_str().unwrap(),
               "--out-dir", tmp.to_str().unwrap()])
        .output()
        .expect("failed to run rustc");
    assert!(
        rustc.status.success(),
        "rustc failed for {asl_file}\n{}",
        String::from_utf8_lossy(&rustc.stderr)
    );

    // 5. Run; assert! failures in main() cause a non-zero exit code
    let run = Command::new(&bin_path).output().expect("failed to run test binary");
    let _ = fs::remove_dir_all(&tmp);
    assert!(
        run.status.success(),
        "test assertions failed for {asl_file}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
}
