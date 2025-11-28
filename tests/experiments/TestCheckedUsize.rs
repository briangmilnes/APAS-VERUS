//! Tests for CheckedUsize

use apas_verus::vstdplus::checked_int::checked_int::CheckedUsize;

#[test]
fn test_new() {
    let c = CheckedUsize::new(1000000usize);
    assert!(c.is_normal());
}

#[test]
fn test_add_normal() {
    let a = CheckedUsize::new(1000000usize);
    let b = CheckedUsize::new(2000000usize);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000usize);
}

#[test]
fn test_sub_normal() {
    let a = CheckedUsize::new(5000000usize);
    let b = CheckedUsize::new(2000000usize);
    let c = a.sub_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3000000usize);
}

#[test]
fn test_sub_underflow() {
    let a = CheckedUsize::new(100usize);
    let b = CheckedUsize::new(200usize);
    let c = a.sub_checked(&b);
    assert!(c.is_out_of_range());
}

#[test]
fn test_mul_normal() {
    let a = CheckedUsize::new(10000usize);
    let b = CheckedUsize::new(10000usize);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 100000000usize);
}

#[test]
fn test_unwrap() {
    let c = CheckedUsize::new(123456usize);
    let v = c.unwrap();
    assert_eq!(v, 123456usize);
}
