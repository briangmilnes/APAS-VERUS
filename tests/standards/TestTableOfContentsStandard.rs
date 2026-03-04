//! Runtime tests for standards::table_of_contents_standard.

use apas_verus::standards::table_of_contents_standard::table_of_contents_standard::*;
use apas_verus::ExampleLit;

#[test]
fn test_new() {
    let s: ExampleS<u64> = ExampleS::new(3, 42);
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 42);
    assert_eq!(*s.nth(2), 42);
}

#[test]
fn test_is_empty() {
    let s: ExampleS<u64> = ExampleS::new(0, 0);
    assert!(s.is_empty());
    let s2: ExampleS<u64> = ExampleS::new(1, 5);
    assert!(!s2.is_empty());
}

#[test]
fn test_iter() {
    let s: ExampleS<u64> = ExampleS { seq: vec![10, 20, 30] };
    let mut it = s.iter();
    assert_eq!(it.next(), Some(&10));
    assert_eq!(it.next(), Some(&20));
    assert_eq!(it.next(), Some(&30));
    assert_eq!(it.next(), None);
}

#[test]
fn test_into_iter_borrow() {
    let s: ExampleS<u64> = ExampleS { seq: vec![1, 2, 3] };
    let collected: Vec<&u64> = (&s).into_iter().collect();
    assert_eq!(collected, vec![&1, &2, &3]);
}

#[test]
fn test_into_iter_consume() {
    let s: ExampleS<u64> = ExampleS { seq: vec![4, 5] };
    let collected: Vec<u64> = s.into_iter().collect();
    assert_eq!(collected, vec![4, 5]);
}

#[test]
fn test_clone() {
    let s: ExampleS<u64> = ExampleS { seq: vec![1, 2, 3] };
    let c = s.clone();
    assert_eq!(s.seq, c.seq);
}

#[test]
fn test_partial_eq() {
    let a: ExampleS<u64> = ExampleS { seq: vec![1, 2] };
    let b: ExampleS<u64> = ExampleS { seq: vec![1, 2] };
    let c: ExampleS<u64> = ExampleS { seq: vec![1, 3] };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_display() {
    let s: ExampleS<u64> = ExampleS { seq: vec![10, 20, 30] };
    assert_eq!(format!("{}", s), "[10, 20, 30]");
}

#[test]
fn test_debug() {
    let s: ExampleS<u64> = ExampleS { seq: vec![1] };
    let d = format!("{:?}", s);
    assert!(d.contains("ExampleS"));
}

#[test]
fn test_lit_empty() {
    let s: ExampleS<u64> = ExampleLit!();
    assert!(s.is_empty());
}

#[test]
fn test_lit_values() {
    let s: ExampleS<u64> = ExampleLit!(10, 20, 30);
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 10);
    assert_eq!(*s.nth(2), 30);
}
