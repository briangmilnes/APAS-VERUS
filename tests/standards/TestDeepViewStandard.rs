//! Runtime tests for standards::deep_view_standard.

use apas_verus::standards::deep_view_standard::deep_view_standard::*;

#[test]
fn test_simple_some() {
    let s = SimpleS { val: Some(42) };
    assert_eq!(s.val, Some(42));
}

#[test]
fn test_simple_none() {
    let s = SimpleS { val: None };
    assert_eq!(s.val, None);
}

#[test]
fn test_collection_construction() {
    let c = CollectionS { seq: vec![1u64, 2, 3] };
    assert_eq!(c.seq.len(), 3);
    assert_eq!(c.seq[0], 1);
}
