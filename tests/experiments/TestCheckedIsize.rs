// Copyright (c) 2025 Brian G. Milnes
//! Tests for CheckedIsize

use apas_verus::vstdplus::checked_int::checked_int::CheckedIsize;

#[test]
fn test_new() {
    let c = CheckedIsize::new(1000000isize);
    assert!(c.is_normal());
}

#[test]
fn test_new_negative() {
    let c = CheckedIsize::new(-1000000isize);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedIsize::new(1000000isize);
    let b = CheckedIsize::new(2000000isize);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000isize);
}

#[test]
fn test_sub_normal() {
    let a = CheckedIsize::new(5000000isize);
    let b = CheckedIsize::new(2000000isize);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000isize);
}

#[test]
fn test_mul_normal() {
    let a = CheckedIsize::new(10000isize);
    let b = CheckedIsize::new(10000isize);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 100000000isize);
}

#[test]
fn test_mul_negative() {
    let a = CheckedIsize::new(-10000isize);
    let b = CheckedIsize::new(10000isize);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), -100000000isize);
}

#[test]
fn test_unwrap() {
    let c = CheckedIsize::new(-123456isize);
    let v = c.unwrap();
    assert_eq!(v, -123456isize);
}
