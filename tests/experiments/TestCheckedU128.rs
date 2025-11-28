//! Tests for CheckedU128

use apas_verus::vstdplus::checked_int::checked_int::CheckedU128;

#[test]
fn test_new() {
    let c = CheckedU128::new(1000000000000000000000u128);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedU128::new(100000000000000000000u128);
    let b = CheckedU128::new(200000000000000000000u128);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 300000000000000000000u128);
}

#[test]
fn test_add_overflow() {
    let a = CheckedU128::new(340282366920938463463374607431768211400u128);
    let b = CheckedU128::new(100u128);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedU128::new(500000000000000000000u128);
    let b = CheckedU128::new(200000000000000000000u128);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 300000000000000000000u128);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedU128::new(100u128);
    let b = CheckedU128::new(200u128);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedU128::new(1000000000000u128);
    let b = CheckedU128::new(1000000000000u128);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 1000000000000000000000000u128);
}

#[test]
fn test_unwrap() {
    let c = CheckedU128::new(12345678901234567890u128);
    let v = c.unwrap();
    assert_eq!(v, 12345678901234567890u128);
}
