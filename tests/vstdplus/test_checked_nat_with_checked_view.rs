//! Tests for checked_nat_with_checked_view::CheckedU32
//! Tests addition, multiplication, and overflow cases

// Use the non-verus version (cfg(not(verus_keep_ghost)))
use apas_verus::vstdplus::checked_nat_with_checked_view::checked_nat_with_checked_view::CheckedU32;

// ============================================================
// Addition tests
// ============================================================

#[test]
fn test_new() {
    let c = CheckedU32::new(42u32);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 42u32);
}

#[test]
fn test_add_value_normal() {
    let a = CheckedU32::new(100u32);
    let b = a.add_value(200u32);
    assert!(b.is_normal());
    assert_eq!(b.unwrap(), 300u32);
}

#[test]
fn test_add_checked_normal() {
    let a = CheckedU32::new(1_000_000_000u32);
    let b = CheckedU32::new(2_000_000_000u32);
    let c = a.add_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 3_000_000_000u32);
}

#[test]
fn test_add_value_overflow() {
    let a = CheckedU32::new(u32::MAX);
    let b = a.add_value(1u32);
    assert!(b.is_overflow());
    assert!(!b.is_normal());
}

#[test]
fn test_add_checked_overflow() {
    let a = CheckedU32::new(4_000_000_000u32);
    let b = CheckedU32::new(1_000_000_000u32);
    let c = a.add_checked(&b);
    assert!(c.is_overflow());
}

#[test]
fn test_add_overflow_propagates() {
    // Once overflow, adding more still overflows
    let a = CheckedU32::new(u32::MAX);
    let b = a.add_value(1u32);  // overflow
    assert!(b.is_overflow());
    let c = b.add_value(100u32);  // still overflow
    assert!(c.is_overflow());
}

// ============================================================
// Multiplication tests
// ============================================================

#[test]
fn test_mul_value_normal() {
    let a = CheckedU32::new(1000u32);
    let b = a.mul_value(1000u32);
    assert!(b.is_normal());
    assert_eq!(b.unwrap(), 1_000_000u32);
}

#[test]
fn test_mul_checked_normal() {
    let a = CheckedU32::new(10_000u32);
    let b = CheckedU32::new(10_000u32);
    let c = a.mul_checked(&b);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 100_000_000u32);
}

#[test]
fn test_mul_value_overflow() {
    let a = CheckedU32::new(100_000u32);
    let b = a.mul_value(100_000u32);  // 10^10 > u32::MAX
    assert!(b.is_overflow());
}

#[test]
fn test_mul_checked_overflow() {
    let a = CheckedU32::new(100_000u32);
    let b = CheckedU32::new(100_000u32);
    let c = a.mul_checked(&b);
    assert!(c.is_overflow());
}

#[test]
fn test_mul_by_zero_normal() {
    // Multiplying by zero should give normal 0, even if other operand is large
    let a = CheckedU32::new(u32::MAX);
    let b = a.mul_value(0u32);
    assert!(b.is_normal());
    assert_eq!(b.unwrap(), 0u32);
}

#[test]
fn test_mul_overflow_by_zero() {
    // Multiplying overflow by zero gives 0 (special case)
    let a = CheckedU32::new(u32::MAX);
    let b = a.add_value(1u32);  // overflow
    assert!(b.is_overflow());
    let c = b.mul_value(0u32);
    assert!(c.is_normal());
    assert_eq!(c.unwrap(), 0u32);
}

// ============================================================
// Sum sequence tests (simulating total_weight)
// ============================================================

#[test]
fn test_sum_sequence_normal() {
    let weights = vec![100u32, 200u32, 300u32, 400u32];
    let mut sum = CheckedU32::new(0u32);
    for w in weights {
        sum = sum.add_value(w);
    }
    assert!(sum.is_normal());
    assert_eq!(sum.unwrap(), 1000u32);
}

#[test]
fn test_sum_sequence_overflow() {
    let weights = vec![u32::MAX, 1u32, 2u32, 3u32];
    let mut sum = CheckedU32::new(0u32);
    for w in weights {
        sum = sum.add_value(w);
    }
    assert!(sum.is_overflow());
}

#[test]
fn test_sum_large_sequence_normal() {
    // Sum of 1000 values of 1_000_000 each = 1_000_000_000 (fits in u32)
    let mut sum = CheckedU32::new(0u32);
    for _ in 0..1000 {
        sum = sum.add_value(1_000_000u32);
    }
    assert!(sum.is_normal());
    assert_eq!(sum.unwrap(), 1_000_000_000u32);
}

#[test]
fn test_sum_large_sequence_overflow() {
    // Sum of 5000 values of 1_000_000 each = 5_000_000_000 (overflows u32)
    let mut sum = CheckedU32::new(0u32);
    for _ in 0..5000 {
        sum = sum.add_value(1_000_000u32);
    }
    assert!(sum.is_overflow());
}

// ============================================================
// Edge cases
// ============================================================

#[test]
fn test_max_value() {
    let a = CheckedU32::new(u32::MAX);
    assert!(a.is_normal());
    assert_eq!(a.unwrap(), u32::MAX);
}

#[test]
fn test_zero() {
    let a = CheckedU32::new(0u32);
    assert!(a.is_normal());
    assert_eq!(a.unwrap(), 0u32);
}

#[test]
fn test_add_zero() {
    let a = CheckedU32::new(12345u32);
    let b = a.add_value(0u32);
    assert!(b.is_normal());
    assert_eq!(b.unwrap(), 12345u32);
}

#[test]
fn test_mul_one() {
    let a = CheckedU32::new(12345u32);
    let b = a.mul_value(1u32);
    assert!(b.is_normal());
    assert_eq!(b.unwrap(), 12345u32);
}

#[test]
fn test_to_option_normal() {
    let a = CheckedU32::new(42u32);
    let opt = a.to_option();
    assert!(opt.is_some());
    assert_eq!(opt.unwrap(), 42u32);
}

#[test]
fn test_to_option_overflow() {
    let a = CheckedU32::new(u32::MAX);
    let b = a.add_value(1u32);
    let opt = b.to_option();
    assert!(opt.is_none());
}

