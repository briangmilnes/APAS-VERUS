//! Runtime tests for standards::iterators_standard.

use apas_verus::standards::iterators_standard::iterators_standard::*;

#[test]
fn test_new() {
    let s: ExampleS<u64> = ExampleS { seq: vec![1, 2, 3] };
    assert_eq!(s.seq.len(), 3);
    assert_eq!(s.seq[0], 1);
    assert_eq!(s.seq[2], 3);
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
    let s: ExampleS<u64> = ExampleS { seq: vec![5, 6, 7] };
    let collected: Vec<&u64> = (&s).into_iter().collect();
    assert_eq!(collected, vec![&5, &6, &7]);
}

#[test]
fn test_into_iter_consume() {
    let s: ExampleS<u64> = ExampleS { seq: vec![100, 200] };
    let collected: Vec<u64> = s.into_iter().collect();
    assert_eq!(collected, vec![100, 200]);
}

#[test]
fn test_iter_empty() {
    let s: ExampleS<u64> = ExampleS { seq: vec![] };
    let mut it = s.iter();
    assert_eq!(it.next(), None);
}
