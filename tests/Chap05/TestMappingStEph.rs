//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MappingStEphChap5_5 ephemeral mappings.

use apas_ai::Chap05::MappingStEph::MappingStEph::*;
use apas_ai::Chap05::RelationStEph::RelationStEph::*;
use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::{MappingLit, PairLit, SetLit};
use apas_ai::Types::Types::*;

#[test]
fn test_mappinglit_macro_functionality() {
    // Test empty mapping creation
    let empty: MappingStEph<i32, String> = MappingLit![];
    assert_eq!(empty.size(), 0);

    // Test mapping creation with key-value pairs
    let with_data: MappingStEph<i32, String> = MappingLit![(1, "one".to_string()), (2, "two".to_string())];
    assert_eq!(with_data.size(), 2);
    //        assert_eq!(with_data.apply(&1), Some("one".to_string()));
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
    // Test calling empty() directly through the trait
    let m: MappingStEph<N, &str> = <MappingStEph<N, &str> as MappingStEphTrait<N, &str>>::empty();
    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
}

#[test]
fn test_from_vec_basic() {
    let m = MappingLit![(1, "one"), (2, "two"), (3, "three")];
    assert_eq!(m.size(), 3);
    assert!(m.mem(&1, &"one"));
    assert!(m.mem(&2, &"two"));
    assert!(m.mem(&3, &"three"));
    assert!(!m.mem(&1, &"wrong"));
    assert!(!m.mem(&99, &"one"));
}

#[test]
#[should_panic(expected = "MappingLit!: duplicate domain element")]
fn test_from_vec_duplicate_keys() {
    // Mappings should panic on duplicate domain elements
    let _m = MappingLit![(1, "first"), (2, "two"), (1, "second")];
}

#[test]
fn test_from_relation() {
    let pairs_set = SetLit![PairLit!(1, "one"), PairLit!(2, "two"), PairLit!(1, "uno")];
    let rel = <RelationStEph<N, &str> as RelationStEphTrait<N, &str>>::FromSet(pairs_set);
    let m = <MappingStEph<N, &str> as MappingStEphTrait<N, &str>>::FromRelation(&rel);

    // Mapping should convert relation to function (one value per key)
    assert!(m.size() <= 2); // At most 2 keys (1 and 2)
    // Either "one" or "uno" for key 1, depending on implementation
    assert!(m.mem(&1, &"one") || m.mem(&1, &"uno"));
    assert!(m.mem(&2, &"two"));
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
    assert_eq!(range.size(), 2); // "one" and "two"
    assert!(range.mem(&"one"));
    assert!(range.mem(&"two"));
    assert!(!range.mem(&"three"));
}

#[test]
fn test_iter() {
    let m = MappingLit![(1, "one"), (2, "two")];

    let collected = m.iter().cloned().collect::<Vec<_>>();
    assert_eq!(collected.len(), 2);

    // Check that all expected pairs are present (order may vary)
    assert!(collected.contains(&PairLit!(1, "one")));
    assert!(collected.contains(&PairLit!(2, "two")));
}

#[test]
fn test_mem_comprehensive() {
    let m = MappingLit![("a", 1), ("b", 2), ("c", 3)];

    // Test existing pairs
    assert!(m.mem(&"a", &1));
    assert!(m.mem(&"b", &2));
    assert!(m.mem(&"c", &3));

    // Test wrong key-value combinations
    assert!(!m.mem(&"a", &2));
    assert!(!m.mem(&"b", &3));

    // Test non-existent keys/values
    assert!(!m.mem(&"d", &1));
    assert!(!m.mem(&"a", &99));
}

#[test]
fn test_empty_mapping_operations() {
    let m: MappingStEph<N, &str> = MappingLit![];

    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
    assert!(!m.mem(&1, &"anything"));

    let collected = m.iter().collect::<Vec<_>>();
    assert_eq!(collected.len(), 0);
}

#[test]
fn test_from_relation_empty_edge() {
    // Test FromRelation with empty relation
    let empty_rel = RelationStEph::<i32, String>::empty();
    let m = <MappingStEph<i32, String> as MappingStEphTrait<i32, String>>::FromRelation(&empty_rel);

    assert_eq!(m.size(), 0);
    assert_eq!(m.domain().size(), 0);
    assert_eq!(m.range().size(), 0);
    assert!(!m.mem(&42, &"test".to_string()));
}

#[test]
fn test_mapping_extreme_values_graceful() {
    // Test with extreme values to verify no panics occur
    // APAS style: bad arguments produce empty sequences/sets, not panics

    // Test with very large keys
    let large_key = i32::MAX;
    let small_key = i32::MIN;
    let m = MappingLit![(large_key, "max"), (small_key, "min"), (0, "zero")];

    assert_eq!(m.size(), 3);
    assert!(m.mem(&large_key, &"max"));
    assert!(m.mem(&small_key, &"min"));
    assert!(m.mem(&0, &"zero"));

    // Test domain and range operations with extreme values
    let domain = m.domain();
    assert_eq!(domain.size(), 3);
    assert!(domain.mem(&large_key));
    assert!(domain.mem(&small_key));

    let range = m.range();
    assert_eq!(range.size(), 3);
    assert!(range.mem(&"max"));
    assert!(range.mem(&"min"));

    // Test with non-existent extreme keys - should return False, not panic
    assert!(!m.mem(&(large_key - 1), &"max"));
    assert!(!m.mem(&(small_key + 1), &"min"));
}

#[test]
fn test_mapping_large_dataset_stress() {
    // Test with large mapping to verify no panics occur
    let large_pairs = (0..10000).map(|i| Pair(i, format!("value_{i}"))).collect::<Vec<Pair<i32, String>>>();

    let m = <MappingStEph<i32, String> as MappingStEphTrait<i32, String>>::FromVec(large_pairs);

    assert_eq!(m.size(), 10000);
    assert!(m.mem(&5000, &"value_5000".to_string()));
    assert!(!m.mem(&15000, &"value_15000".to_string()));

    // Test domain and range operations on large mapping
    let domain = m.domain();
    assert_eq!(domain.size(), 10000);
    assert!(domain.mem(&9999));
    assert!(!domain.mem(&10000));

    let range = m.range();
    assert_eq!(range.size(), 10000);
    assert!(range.mem(&"value_0".to_string()));
    assert!(!range.mem(&"value_10000".to_string()));

    // Test iteration on large mapping - should not panic
    let mut count = 0;
    for _pair in m.iter() {
        count += 1;
        if count > 10010 {
            // Safety check to prevent infinite loop
            break;
        }
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
    assert!(m2.mem(&1, &"one"));
    assert!(m2.mem(&2, &"two"));
}
