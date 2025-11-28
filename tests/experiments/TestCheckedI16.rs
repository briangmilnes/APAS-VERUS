//! Tests for CheckedI16

use apas_verus::vstdplus::checked_int::checked_int::CheckedI16;

#[test]
fn test_new() {
    let c = CheckedI16::new(1000i16);
    assert!(c.is_normal());
}

#[test]
fn test_new_negative() {
    let c = CheckedI16::new(-5000i16);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedI16::new(10000i16);
    let b = CheckedI16::new(10000i16);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 20000i16);
}

#[test]
fn test_add_overflow() {
    let a = CheckedI16::new(30000i16);
    let b = CheckedI16::new(10000i16);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_add_underflow() {
    let a = CheckedI16::new(-30000i16);
    let b = CheckedI16::new(-10000i16);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedI16::new(5000i16);
    let b = CheckedI16::new(2000i16);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000i16);
}

#[test]
fn test_mul_normal() {
    let a = CheckedI16::new(100i16);
    let b = CheckedI16::new(100i16);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 10000i16);
}

#[test]
fn test_mul_overflow() {
    let a = CheckedI16::new(200i16);
    let b = CheckedI16::new(200i16);
    let c = a.mul_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_unwrap() {
    let c = CheckedI16::new(-12345i16);
    let v = c.unwrap();
    assert_eq!(v, -12345i16);
}
