//! Test: Can we use rust_verify_test_macros from APAS-VERUS?

use rust_verify_test_macros::verus_code;

// The verus_code! macro converts code to a string wrapped in verus!{}
// Let's see what it produces:

#[test]
fn test_verus_code_macro() {
    let code_string = verus_code! {
        fn test_simple() {
            assert(1 + 1 == 2);
        }
    };
    
    println!("Generated code:\n{}", code_string);
    
    // Now we need to write this to a file and run verus on it
    // This is what verify_one_file() does in the verus test harness
}

// Minimal harness to run verus on a snippet
fn run_verus_on_snippet(code: &str) -> Result<(), String> {
    use std::process::Command;
    use std::io::Write;
    
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("verus_test_snippet.rs");
    
    // Write the code (verus_code! already wraps in verus!{})
    let full_code = format!(
        r#"#![feature(allocator_api)]
use vstd::prelude::*;
{}
"#,
        code
    );
    
    std::fs::write(&temp_file, &full_code).map_err(|e| e.to_string())?;
    
    // Find verus binary
    let verus_path = which::which("verus").map_err(|e| e.to_string())?;
    
    let output = Command::new(verus_path)
        .arg("--crate-type=lib")
        .arg(&temp_file)
        .output()
        .map_err(|e| e.to_string())?;
    
    // Clean up
    let _ = std::fs::remove_file(&temp_file);
    
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[test]
fn test_run_verus_success() {
    let code = verus_code! {
        fn test_passes() {
            assert(1 + 1 == 2);
        }
    };
    
    let result = run_verus_on_snippet(&code);
    assert!(result.is_ok(), "Expected success, got: {:?}", result);
}

#[test]
fn test_run_verus_expected_failure() {
    let code = verus_code! {
        fn test_fails() {
            assert(1 + 1 == 3);
        }
    };
    
    let result = run_verus_on_snippet(&code);
    assert!(result.is_err(), "Expected failure but got success");
}

