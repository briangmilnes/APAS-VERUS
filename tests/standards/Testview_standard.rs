//! Runtime tests for standards::view_standard.

use apas_verus::standards::view_standard::view_standard::*;

#[test]
fn test_simple_new_get() {
    let s = SimpleS::new(42);
    assert_eq!(s.get(), 42);
}

#[test]
fn test_collection_length() {
    let c = CollectionS { seq: vec![10u64, 20, 30] };
    assert_eq!(c.length(), 3);
}

#[test]
fn test_collection_nth() {
    let c = CollectionS { seq: vec![10u64, 20, 30] };
    assert_eq!(*c.nth(0), 10);
    assert_eq!(*c.nth(1), 20);
    assert_eq!(*c.nth(2), 30);
}

#[test]
fn test_collection_empty() {
    let c: CollectionS<u64> = CollectionS { seq: vec![] };
    assert_eq!(c.length(), 0);
}
