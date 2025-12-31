// Copyright (c) 2025 Brian G. Milnes
//! Tests for CheckedI8

use apas_verus::vstdplus::checked_int::checked_int::CheckedI8;

#[test]
fn test_new() {
    let c = CheckedI8::new(42i8);
    assert!(c.is_normal());
}

#[test]
fn test_new_negative() {
    let c = CheckedI8::new(-50i8);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedI8::new(10i8);
    let b = CheckedI8::new(20i8);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 30i8);
}

#[test]
fn test_add_overflow() {
    let a = CheckedI8::new(100i8);
    let b = CheckedI8::new(50i8);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_add_underflow() {
    let a = CheckedI8::new(-100i8);
    let b = CheckedI8::new(-50i8);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedI8::new(50i8);
    let b = CheckedI8::new(20i8);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 30i8);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedI8::new(-100i8);
    let b = CheckedI8::new(50i8);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_overflow() {
    let a = CheckedI8::new(100i8);
    let b = CheckedI8::new(-50i8);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedI8::new(5i8);
    let b = CheckedI8::new(10i8);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 50i8);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedI8::new(20i8);
    let b = CheckedI8::new(10i8);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_negative() {
    let a = CheckedI8::new(-5i8);
    let b = CheckedI8::new(10i8);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), -50i8);
}

#[test]
fn test_unwrap() {
    let c = CheckedI8::new(-100i8);
    let v = c.unwrap();
    assert_eq!(v, -100i8);
}
