// Copyright (c) 2025 Brian G. Milnes
use apas_verus::vstdplus::partial_order::partial_order::PartialOrder;
use core::cmp::Ordering;

#[test]
fn test_partial_order_u64() {
    assert_eq!(PartialOrder::compare(&5u64, &3u64), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&3u64, &5u64), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&5u64, &5u64), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_i32() {
    assert_eq!(PartialOrder::compare(&(-5i32), &(-3i32)), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&(-3i32), &(-5i32)), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&0i32, &0i32), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_u8() {
    assert_eq!(PartialOrder::compare(&255u8, &0u8), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&0u8, &255u8), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&42u8, &42u8), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_i64() {
    assert_eq!(PartialOrder::compare(&i64::MAX, &i64::MIN), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&i64::MIN, &i64::MAX), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&0i64, &0i64), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_returns_option() {
    // Demonstrate that partial_cmp returns Option (even though integers are always Some)
    let result = PartialOrder::compare(&5u64, &3u64);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), Ordering::Greater);
}

#[test]
fn test_partial_order_f32() {
    assert_eq!(PartialOrder::compare(&5.0f32, &3.0f32), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&3.0f32, &5.0f32), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&5.0f32, &5.0f32), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_f64() {
    assert_eq!(PartialOrder::compare(&5.0f64, &3.0f64), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&3.0f64, &5.0f64), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&5.0f64, &5.0f64), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_f32_nan() {
    // NaN comparisons return None (incomparable)
    let nan = f32::NAN;
    let x = 5.0f32;
    
    assert_eq!(PartialOrder::compare(&nan, &x), None);
    assert_eq!(PartialOrder::compare(&x, &nan), None);
    assert_eq!(PartialOrder::compare(&nan, &nan), None);
}

#[test]
fn test_partial_order_f64_nan() {
    // NaN comparisons return None (incomparable)
    let nan = f64::NAN;
    let x = 5.0f64;
    
    assert_eq!(PartialOrder::compare(&nan, &x), None);
    assert_eq!(PartialOrder::compare(&x, &nan), None);
    assert_eq!(PartialOrder::compare(&nan, &nan), None);
}

#[test]
fn test_partial_order_f32_infinity() {
    assert_eq!(PartialOrder::compare(&f32::INFINITY, &1.0f32), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&f32::NEG_INFINITY, &1.0f32), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&f32::INFINITY, &f32::INFINITY), Some(Ordering::Equal));
}

#[test]
fn test_partial_order_f64_infinity() {
    assert_eq!(PartialOrder::compare(&f64::INFINITY, &1.0f64), Some(Ordering::Greater));
    assert_eq!(PartialOrder::compare(&f64::NEG_INFINITY, &1.0f64), Some(Ordering::Less));
    assert_eq!(PartialOrder::compare(&f64::INFINITY, &f64::INFINITY), Some(Ordering::Equal));
}
