// Copyright (c) 2025 Brian G. Milnes
//! Tests for CheckedU8

use apas_verus::vstdplus::checked_int::checked_int::CheckedU8;

#[test]
fn test_new() {
    let c = CheckedU8::new(42u8);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedU8::new(10u8);
    let b = CheckedU8::new(20u8);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 30u8);
}

#[test]
fn test_add_overflow() {
    let a = CheckedU8::new(200u8);
    let b = CheckedU8::new(100u8);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedU8::new(50u8);
    let b = CheckedU8::new(20u8);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 30u8);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedU8::new(10u8);
    let b = CheckedU8::new(20u8);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedU8::new(10u8);
    let b = CheckedU8::new(5u8);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 50u8);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedU8::new(20u8);
    let b = CheckedU8::new(20u8);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_unwrap() {
    let c = CheckedU8::new(100u8);
    let v = c.unwrap();
    assert_eq!(v, 100u8);
}

#[test]
fn test_chain_operations() {
    let a = CheckedU8::new(10u8);
    let b = CheckedU8::new(5u8);
    let c = a.add_checked(&b).mul_value(2u8);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 30u8);
}
