//! Tests for feq - Full Equality specification
//!
//! The feq module provides spec-only functions for Verus verification.
//! These runtime tests verify basic Rust equality properties that feq specifies.

// Test reflexivity at runtime
#[test]
fn test_reflexive_u64() {
    let x: u64 = 42;
    assert!(x == x);
}

// Test symmetry at runtime
#[test]
fn test_symmetric_u64() {
    let x: u64 = 42;
    let y: u64 = 42;
    assert!((x == y) == (y == x));
}

// Test transitivity at runtime
#[test]
fn test_transitive_u64() {
    let x: u64 = 42;
    let y: u64 = 42;
    let z: u64 = 42;
    if x == y && y == z {
        assert!(x == z);
    }
}

// Test clone equality at runtime
#[test]
fn test_clone_u64() {
    let x: u64 = 42;
    let x_clone = x.clone();
    assert_eq!(x, x_clone);
}

#[test]
fn test_clone_i32() {
    let x: i32 = -100;
    let x_clone = x.clone();
    assert_eq!(x, x_clone);
}

#[test]
fn test_clone_bool() {
    let x: bool = true;
    let x_clone = x.clone();
    assert_eq!(x, x_clone);
}

#[test]
fn test_clone_vec() {
    let v: Vec<i64> = vec![1, 2, 3];
    let v_clone = v.clone();
    assert_eq!(v, v_clone);
}

fn is_sorted<T: Ord>(v: &Vec<T>) -> bool {
    for i in 0..v.len().saturating_sub(1) {
        if v[i] > v[i + 1] {
            return false;
        }
    }
    true
}

#[test]
fn test_is_sorted_empty() {
    let v: Vec<i64> = vec![];
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_single() {
    let v: Vec<i64> = vec![42];
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_true() {
    let v: Vec<i64> = vec![1, 2, 3, 4, 5];
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_equal_elements() {
    let v: Vec<i64> = vec![3, 3, 3, 3];
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_false_beginning() {
    let v: Vec<i64> = vec![5, 1, 2, 3, 4];
    assert!(!is_sorted(&v));
}

#[test]
fn test_is_sorted_false_middle() {
    let v: Vec<i64> = vec![1, 2, 5, 3, 4];
    assert!(!is_sorted(&v));
}

#[test]
fn test_is_sorted_false_end() {
    let v: Vec<i64> = vec![1, 2, 3, 4, 0];
    assert!(!is_sorted(&v));
}

#[test]
fn test_is_sorted_two_elements_sorted() {
    let v: Vec<i64> = vec![1, 2];
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_two_elements_unsorted() {
    let v: Vec<i64> = vec![2, 1];
    assert!(!is_sorted(&v));
}

#[test]
fn test_is_sorted_negative_numbers() {
    let v: Vec<i64> = vec![-5, -3, -1, 0, 2, 4];
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_large_vec() {
    let v: Vec<i64> = (0..10000).collect();
    assert!(is_sorted(&v));
}

#[test]
fn test_is_sorted_large_vec_unsorted() {
    let mut v: Vec<i64> = (0..10000).collect();
    v[5000] = -1;
    assert!(!is_sorted(&v));
}
