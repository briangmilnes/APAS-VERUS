//! Tests for CheckedI128

use apas_verus::vstdplus::checked_int::checked_int::CheckedI128;

#[test]
fn test_new() {
    let c = CheckedI128::new(1000000000000000000000i128);
    assert!(c.is_normal());
}

#[test]
fn test_new_negative() {
    let c = CheckedI128::new(-1000000000000000000000i128);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedI128::new(100000000000000000000i128);
    let b = CheckedI128::new(200000000000000000000i128);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 300000000000000000000i128);
}

#[test]
fn test_add_overflow() {
    let a = CheckedI128::new(170141183460469231731687303715884105700i128);
    let b = CheckedI128::new(100i128);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_add_underflow() {
    let a = CheckedI128::new(-170141183460469231731687303715884105700i128);
    let b = CheckedI128::new(-200i128);
    let c = a.add_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_sub_normal() {
    let a = CheckedI128::new(500000000000000000000i128);
    let b = CheckedI128::new(200000000000000000000i128);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 300000000000000000000i128);
}

#[test]
fn test_mul_normal() {
    let a = CheckedI128::new(1000000000000i128);
    let b = CheckedI128::new(1000000000000i128);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 1000000000000000000000000i128);
}

#[test]
fn test_mul_negative() {
    let a = CheckedI128::new(-1000000000000i128);
    let b = CheckedI128::new(1000000000000i128);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), -1000000000000000000000000i128);
}

#[test]
fn test_unwrap() {
    let c = CheckedI128::new(-12345678901234567890i128);
    let v = c.unwrap();
    assert_eq!(v, -12345678901234567890i128);
}
