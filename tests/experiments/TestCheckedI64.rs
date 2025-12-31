// Copyright (c) 2025 Brian G. Milnes
//! Tests for CheckedI64

use apas_verus::vstdplus::checked_int::checked_int::CheckedI64;

#[test]
fn test_new() {
    let c = CheckedI64::new(1000000000000i64);
    assert!(c.is_normal());
}

#[test]
fn test_new_negative() {
    let c = CheckedI64::new(-1000000000000i64);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedI64::new(1000000000000i64);
    let b = CheckedI64::new(2000000000000i64);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000000000i64);
}

#[test]
fn test_add_overflow() {
    let a = CheckedI64::new(9000000000000000000i64);
    let b = CheckedI64::new(1000000000000000000i64);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_add_underflow() {
    let a = CheckedI64::new(-9000000000000000000i64);
    let b = CheckedI64::new(-1000000000000000000i64);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedI64::new(5000000000000i64);
    let b = CheckedI64::new(2000000000000i64);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000000000i64);
}

#[test]
fn test_mul_normal() {
    let a = CheckedI64::new(1000000i64);
    let b = CheckedI64::new(1000000i64);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 1000000000000i64);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedI64::new(10000000000i64);
    let b = CheckedI64::new(10000000000i64);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_unwrap() {
    let c = CheckedI64::new(-9876543210i64);
    let v = c.unwrap();
    assert_eq!(v, -9876543210i64);
}
