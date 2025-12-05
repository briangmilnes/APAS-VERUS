//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MappingStEph ephemeral mappings.

use apas_verus::Chap05::MappingStEph::MappingStEph::*;
use apas_verus::Chap05::RelationStEph::RelationStEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::{MappingLit, PairLit, SetLit};
use apas_verus::Types::Types::*;

#[test]
fn test_mappinglit_macro_functionality() {
    let empty: MappingStEph<i32, String> = MappingLit![];
    assert_eq!(empty.size(), 0);

    let with_data: MappingStEph<i32, String> = MappingLit![(1, "one".to_string()), (2, "two".to_string())];
    assert_eq!(with_data.size(), 2);
}

#[test]
fn test_empty_mapping() {
    let m: MappingStEph<N, &str> = MappingLit![];
    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
}

#[test]
fn test_empty_mapping_trait() {
    let m: MappingStEph<N, &str> = <MappingStEph<N, &str> as MappingStEphTrait<N, &str>>::empty();
    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
}

#[test]
fn test_from_vec_basic() {
    let m = MappingLit![(1, "one"), (2, "two"), (3, "three")];
    assert_eq!(m.size(), 3);
    assert!(m.mem(&Pair(1, "one")));
    assert!(m.mem(&Pair(2, "two")));
    assert!(m.mem(&Pair(3, "three")));
    assert!(!m.mem(&Pair(1, "wrong")));
    assert!(!m.mem(&Pair(99, "one")));
}

#[test]
#[should_panic(expected = "MappingLit!: duplicate domain element")]
fn test_from_vec_duplicate_keys() {
    let _m = MappingLit![(1, "first"), (2, "two"), (1, "second")];
}

#[test]
fn test_from_relation() {
    // Relation with unique keys
    let pairs_set = SetLit![PairLit!(1, "one"), PairLit!(2, "two"), PairLit!(3, "three")];
    let rel = <RelationStEph<N, &str> as RelationStEphTrait<N, &str>>::from_set(pairs_set);
    let m = <MappingStEph<N, &str> as MappingStEphTrait<N, &str>>::from_relation(&rel);

    assert_eq!(m.size(), 3);
    assert!(m.mem(&Pair(1, "one")));
    assert!(m.mem(&Pair(2, "two")));
    assert!(m.mem(&Pair(3, "three")));
}

#[test]
fn test_domain_and_range() {
    let m = MappingLit![(1, "one"), (2, "two"), (3, "one")];

    let domain = m.domain();
    assert_eq!(domain.size(), 3);
    assert!(domain.mem(&1));
    assert!(domain.mem(&2));
    assert!(domain.mem(&3));
    assert!(!domain.mem(&4));

    let range = m.range();
    assert_eq!(range.size(), 2);
    assert!(range.mem(&"one"));
    assert!(range.mem(&"two"));
    assert!(!range.mem(&"three"));
}

#[test]
fn test_iter() {
    let m = MappingLit![(1, "one"), (2, "two")];

    let collected = m.iter().cloned().collect::<Vec<_>>();
    assert_eq!(collected.len(), 2);
    assert!(collected.contains(&Pair(1, "one")));
    assert!(collected.contains(&Pair(2, "two")));
}

#[test]
fn test_mem_comprehensive() {
    let m = MappingLit![("a", 1), ("b", 2), ("c", 3)];

    assert!(m.mem(&Pair("a", 1)));
    assert!(m.mem(&Pair("b", 2)));
    assert!(m.mem(&Pair("c", 3)));

    assert!(!m.mem(&Pair("a", 2)));
    assert!(!m.mem(&Pair("b", 3)));

    assert!(!m.mem(&Pair("d", 1)));
    assert!(!m.mem(&Pair("a", 99)));
}

#[test]
fn test_empty_mapping_operations() {
    let m: MappingStEph<N, &str> = MappingLit![];

    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
    assert!(!m.mem(&Pair(1, "anything")));

    let collected = m.iter().collect::<Vec<_>>();
    assert_eq!(collected.len(), 0);
}

#[test]
fn test_from_relation_empty_edge() {
    let empty_rel = RelationStEph::<i32, String>::empty();
    let m = <MappingStEph<i32, String> as MappingStEphTrait<i32, String>>::from_relation(&empty_rel);

    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
    assert!(!m.mem(&Pair(42, "test".to_string())));
}

#[test]
fn test_mapping_extreme_values_graceful() {
    let large_key = i32::MAX;
    let small_key = i32::MIN;
    let m = MappingLit![(large_key, "max"), (small_key, "min"), (0, "zero")];

    assert_eq!(m.size(), 3);
    assert!(m.mem(&Pair(large_key, "max")));
    assert!(m.mem(&Pair(small_key, "min")));
    assert!(m.mem(&Pair(0, "zero")));

    let domain = m.domain();
    assert_eq!(domain.size(), 3);
    assert!(domain.mem(&large_key));
    assert!(domain.mem(&small_key));

    let range = m.range();
    assert_eq!(range.size(), 3);
    assert!(range.mem(&"max"));
    assert!(range.mem(&"min"));

    assert!(!m.mem(&Pair(large_key - 1, "max")));
    assert!(!m.mem(&Pair(small_key + 1, "min")));
}

#[test]
fn test_mapping_large_dataset_stress() {
    let large_pairs = (0..10000).map(|i| Pair(i, format!("value_{i}"))).collect::<Vec<Pair<i32, String>>>();
    let m = <MappingStEph<i32, String> as MappingStEphTrait<i32, String>>::from_vec(large_pairs);

    assert_eq!(m.size(), 10000);
    assert!(m.mem(&Pair(5000, "value_5000".to_string())));
    assert!(!m.mem(&Pair(15000, "value_15000".to_string())));

    let domain = m.domain();
    assert_eq!(domain.size(), 10000);
    assert!(domain.mem(&9999));
    assert!(!domain.mem(&10000));

    let range = m.range();
    assert_eq!(range.size(), 10000);
    assert!(range.mem(&"value_0".to_string()));
    assert!(!range.mem(&"value_10000".to_string()));

    let mut count = 0;
    for _pair in m.iter() {
        count += 1;
        if count > 10010 { break; }
    }
    assert_eq!(count, 10000);
}

#[test]
fn test_mapping_debug_display() {
    let m = MappingLit![(1, "one"), (2, "two")];
    let debug_str = format!("{:?}", m);
    assert!(!debug_str.is_empty());
    let display_str = format!("{}", m);
    assert!(!display_str.is_empty());
}

#[test]
fn test_mapping_equality() {
    let m1 = MappingLit![(1, "one"), (2, "two")];
    let m2 = MappingLit![(1, "one"), (2, "two")];
    let m3 = MappingLit![(1, "one"), (3, "three")];

    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}

#[test]
fn test_mapping_clone() {
    let m1 = MappingLit![(1, "one"), (2, "two")];
    let m2 = m1.clone();

    assert_eq!(m1, m2);
    assert_eq!(m1.size(), m2.size());
    assert!(m2.mem(&Pair(1, "one")));
    assert!(m2.mem(&Pair(2, "two")));
}
