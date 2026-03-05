//! Runtime tests for standards::mod_standard.

use apas_verus::standards::mod_standard::mod_standard::*;

#[test]
fn test_new() {
    let s: ExampleS<u64> = ExampleS::new(3, 42);
    assert_eq!(s.length(), 3);
}

#[test]
fn test_new_empty() {
    let s: ExampleS<u64> = ExampleS::new(0, 0);
    assert_eq!(s.length(), 0);
}
