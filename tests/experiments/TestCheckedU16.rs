//! Tests for CheckedU16

use apas_verus::vstdplus::checked_int::checked_int::CheckedU16;

#[test]
fn test_new() {
    let c = CheckedU16::new(1000u16);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedU16::new(10000u16);
    let b = CheckedU16::new(20000u16);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 30000u16);
}

#[test]
fn test_add_overflow() {
    let a = CheckedU16::new(60000u16);
    let b = CheckedU16::new(10000u16);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedU16::new(5000u16);
    let b = CheckedU16::new(2000u16);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000u16);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedU16::new(100u16);
    let b = CheckedU16::new(200u16);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedU16::new(100u16);
    let b = CheckedU16::new(50u16);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 5000u16);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedU16::new(1000u16);
    let b = CheckedU16::new(1000u16);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_unwrap() {
    let c = CheckedU16::new(12345u16);
    let v = c.unwrap();
    assert_eq!(v, 12345u16);
}
