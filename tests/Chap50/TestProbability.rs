//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Probability.

use std::cmp::Ordering::{Equal, Greater, Less};

use apas_verus::Chap30::Probability::Probability::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let p = Probability::new(0.5);
    assert_eq!(p.value(), 0.5);
}

#[test]
fn test_zero() {
    let p = Probability::zero();
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_infinity() {
    let p = Probability::infinity();
    assert!(p.value().is_infinite());
}

#[test]
fn test_add() {
    let p1 = Probability::new(0.3);
    let p2 = Probability::new(0.2);
    let sum = p1 + p2;
    assert!((sum.value() - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_sub() {
    let p1 = Probability::new(0.7);
    let p2 = Probability::new(0.3);
    let diff = p1 - p2;
    assert!((diff.value() - 0.4).abs() < f64::EPSILON);
}

#[test]
fn test_mul() {
    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.4);
    let prod = p1 * p2;
    assert!((prod.value() - 0.2).abs() < f64::EPSILON);
}

#[test]
fn test_div() {
    let p1 = Probability::new(0.6);
    let p2 = Probability::new(0.3);
    let quot = p1 / p2;
    assert!((quot.value() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn test_eq() {
    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.5);
    let p3 = Probability::new(0.3);
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}

#[test]
fn test_ord() {
    let p1 = Probability::new(0.3);
    let p2 = Probability::new(0.5);
    assert!(p1 < p2);
    assert!(p2 > p1);
}

#[test]
fn test_clone() {
    let p1 = Probability::new(0.7);
    let p2 = p1;
    assert_eq!(p1, p2);
}

#[test]
fn test_display() {
    let p = Probability::new(0.75);
    let s = format!("{p}");
    assert!(s.contains("0.75"));
}

#[test]
fn test_debug() {
    let p = Probability::new(0.5);
    let s = format!("{p:?}");
    assert!(!s.is_empty());
}

#[test]
fn test_zero_operations() {
    let p = Probability::zero();
    let p2 = Probability::new(0.5);
    let sum = p + p2;
    assert_eq!(sum.value(), 0.5);
}

#[test]
fn test_infinity_operations() {
    let p = Probability::infinity();
    let p2 = Probability::new(0.5);
    let sum = p + p2;
    assert!(sum.value().is_infinite());
}

#[test]
fn test_from_f64() {
    let p: Probability = 0.75.into();
    assert_eq!(p.value(), 0.75);
}

#[test]
fn test_into_f64() {
    let p = Probability::new(0.75);
    let f: f64 = p.into();
    assert_eq!(f, 0.75);
}

#[test]
fn test_default() {
    let p: Probability = Default::default();
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_partial_cmp() {
    let p1 = Probability::new(0.3);
    let p2 = Probability::new(0.5);
    assert_eq!(p1.partial_cmp(&p2), Some(Less));
}

#[test]
fn test_copy_trait() {
    let p1 = Probability::new(0.5);
    let p2 = p1; // Copy, not move
    assert_eq!(p1.value(), p2.value());
    // p1 is still valid
    assert_eq!(p1.value(), 0.5);
}

#[test]
fn test_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.5);

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    p1.hash(&mut hasher1);
    p2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_prob_macro() {
    use apas_verus::prob;

    let p = prob!(0.75);
    assert_eq!(p.value(), 0.75);
}

#[test]
fn test_ordering_less() {
    let p1 = Probability::new(0.1);
    let p2 = Probability::new(0.9);
    assert_eq!(p1.cmp(&p2), Less);
}

#[test]
fn test_ordering_greater() {
    let p1 = Probability::new(0.9);
    let p2 = Probability::new(0.1);
    assert_eq!(p1.cmp(&p2), Greater);
}

#[test]
fn test_ordering_equal() {
    let p1 = Probability::new(0.5);
    let p2 = Probability::new(0.5);
    assert_eq!(p1.cmp(&p2), Equal);
}
