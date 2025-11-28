//! Tests for CheckedU64

use apas_verus::vstdplus::checked_int::checked_int::CheckedU64;

#[test]
fn test_new() {
    let c = CheckedU64::new(1000000000000u64);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedU64::new(1000000000000u64);
    let b = CheckedU64::new(2000000000000u64);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000000000u64);
}

#[test]
fn test_add_overflow() {
    let a = CheckedU64::new(18000000000000000000u64);
    let b = CheckedU64::new(1000000000000000000u64);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedU64::new(5000000000000u64);
    let b = CheckedU64::new(2000000000000u64);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000000000u64);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedU64::new(100u64);
    let b = CheckedU64::new(200u64);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedU64::new(1000000u64);
    let b = CheckedU64::new(1000000u64);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 1000000000000u64);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedU64::new(10000000000000000u64);
    let b = CheckedU64::new(10000000000000000u64);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_unwrap() {
    let c = CheckedU64::new(9876543210u64);
    let v = c.unwrap();
    assert_eq!(v, 9876543210u64);
}
