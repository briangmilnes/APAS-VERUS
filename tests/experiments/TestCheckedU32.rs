//! Tests for CheckedU32

use apas_verus::vstdplus::checked_int::checked_int::CheckedU32;

#[test]
fn test_new() {
    let c = CheckedU32::new(1000000u32);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedU32::new(1000000000u32);
    let b = CheckedU32::new(2000000000u32);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000000u32);
}

#[test]
fn test_add_overflow() {
    let a = CheckedU32::new(4000000000u32);
    let b = CheckedU32::new(1000000000u32);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedU32::new(5000000u32);
    let b = CheckedU32::new(2000000u32);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000u32);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedU32::new(100u32);
    let b = CheckedU32::new(200u32);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedU32::new(10000u32);
    let b = CheckedU32::new(10000u32);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 100000000u32);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedU32::new(100000u32);
    let b = CheckedU32::new(100000u32);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_unwrap() {
    let c = CheckedU32::new(123456789u32);
    let v = c.unwrap();
    assert_eq!(v, 123456789u32);
}
