// Copyright (c) 2025 Brian G. Milnes
//! Tests for CheckedI32

use apas_verus::vstdplus::checked_int::checked_int::CheckedI32;

#[test]
fn test_new() {
    let c = CheckedI32::new(1000000i32);
    assert!(c.is_normal());
}

#[test]
fn test_new_negative() {
    let c = CheckedI32::new(-1000000i32);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedI32::new(1000000000i32);
    let b = CheckedI32::new(1000000000i32);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 2000000000i32);
}

#[test]
fn test_add_overflow() {
    let a = CheckedI32::new(2000000000i32);
    let b = CheckedI32::new(500000000i32);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_add_underflow() {
    let a = CheckedI32::new(-2000000000i32);
    let b = CheckedI32::new(-500000000i32);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedI32::new(5000000i32);
    let b = CheckedI32::new(2000000i32);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000i32);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedI32::new(-2000000000i32);
    let b = CheckedI32::new(500000000i32);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedI32::new(10000i32);
    let b = CheckedI32::new(10000i32);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 100000000i32);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedI32::new(100000i32);
    let b = CheckedI32::new(100000i32);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_negative() {
    let a = CheckedI32::new(-10000i32);
    let b = CheckedI32::new(10000i32);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), -100000000i32);
}

#[test]
fn test_unwrap() {
    let c = CheckedI32::new(-123456789i32);
    let v = c.unwrap();
    assert_eq!(v, -123456789i32);
}

#[test]
fn test_chain_operations() {
    let a = CheckedI32::new(1000i32);
    let b = CheckedI32::new(500i32);
    let c = a.add_checked(&b).mul_value(2i32);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000i32);
}
