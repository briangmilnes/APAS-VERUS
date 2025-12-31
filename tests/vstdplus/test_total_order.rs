// Copyright (c) 2025 Brian G. Milnes
use apas_verus::vstdplus::total_order::total_order::TotalOrder;
use core::cmp::Ordering;

#[test]
fn test_total_order_u64() {
    assert_eq!(TotalOrder::cmp(&5u64, &3u64), Ordering::Greater);
    assert_eq!(TotalOrder::cmp(&3u64, &5u64), Ordering::Less);
    assert_eq!(TotalOrder::cmp(&5u64, &5u64), Ordering::Equal);
}

#[test]
fn test_total_order_i32() {
    assert_eq!(TotalOrder::cmp(&(-5i32), &(-3i32)), Ordering::Less);
    assert_eq!(TotalOrder::cmp(&(-3i32), &(-5i32)), Ordering::Greater);
    assert_eq!(TotalOrder::cmp(&0i32, &0i32), Ordering::Equal);
}

#[test]
fn test_total_order_u8() {
    assert_eq!(TotalOrder::cmp(&255u8, &0u8), Ordering::Greater);
    assert_eq!(TotalOrder::cmp(&0u8, &255u8), Ordering::Less);
    assert_eq!(TotalOrder::cmp(&42u8, &42u8), Ordering::Equal);
}

#[test]
fn test_total_order_i64() {
    assert_eq!(TotalOrder::cmp(&i64::MAX, &i64::MIN), Ordering::Greater);
    assert_eq!(TotalOrder::cmp(&i64::MIN, &i64::MAX), Ordering::Less);
    assert_eq!(TotalOrder::cmp(&0i64, &0i64), Ordering::Equal);
}
