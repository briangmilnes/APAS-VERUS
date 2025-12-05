//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap05::RelationStEph::RelationStEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::{RelationLit, SetLit, PairLit};
use apas_verus::Types::Types::*;

#[test]
fn test_relationlit_macro_functionality() {
    // Test empty relation creation
    let empty: RelationStEph<i32, String> = RelationLit![];
    assert_eq!(empty.size(), 0);

    // Test relation creation with pairs
    let with_data: RelationStEph<i32, String> = RelationLit![(1, "one".to_string()), (2, "two".to_string())];
    assert_eq!(with_data.size(), 2);
    assert!(with_data.mem(&1, &"one".to_string()));
}

#[test]
fn test_relation_domain_range_and_mem() {
    // R subset of A x B
    let pairs = SetLit![PairLit!(1usize, 'a'), PairLit!(2usize, 'b'), PairLit!(1usize, 'b')];
    let r = RelationStEph::from_set(pairs);

    let d = r.domain();
    let e = SetLit![1usize, 2usize];
    assert_eq!(d, e);

    let rg = r.range();
    let rexp = SetLit!['a', 'b'];
    assert_eq!(rg, rexp);

    assert!(r.mem(&1usize, &'a'));
    assert!(!r.mem(&2usize, &'a'));
}

#[test]
fn test_relation_empty() {
    let empty_rel = RelationStEph::<i32, char>::empty();
    assert_eq!(empty_rel.size(), 0);
    assert!(!empty_rel.mem(&1, &'a'));
}

#[test]
fn test_relation_size() {
    let empty_rel = RelationStEph::<i32, char>::empty();
    assert_eq!(empty_rel.size(), 0);

    let single_pair = SetLit![PairLit!(1, 'a')];
    let single_rel = RelationStEph::from_set(single_pair);
    assert_eq!(single_rel.size(), 1);

    let multi_pairs = SetLit![PairLit!(1, 'a'), PairLit!(2, 'b'), PairLit!(3, 'c')];
    let multi_rel = RelationStEph::from_set(multi_pairs);
    assert_eq!(multi_rel.size(), 3);
}

#[test]
fn test_relation_domain_empty_edge() {
    let empty_rel = RelationStEph::<i32, char>::empty();
    let empty_domain = empty_rel.domain();
    assert_eq!(empty_domain.size(), 0);
}

#[test]
fn test_relation_range_empty_edge() {
    let empty_rel = RelationStEph::<i32, char>::empty();
    let empty_range = empty_rel.range();
    assert_eq!(empty_range.size(), 0);
}

#[test]
fn test_relation_mem_empty_edge() {
    let empty_rel = RelationStEph::<i32, char>::empty();
    assert!(!empty_rel.mem(&1, &'a'));
    assert!(!empty_rel.mem(&0, &'z'));
}

#[test]
fn test_relation_iter() {
    let pairs = SetLit![PairLit!(1, 'a'), PairLit!(2, 'b')];
    let rel = RelationStEph::from_set(pairs);

    let collected = rel.iter().cloned().collect::<Vec<_>>();
    assert_eq!(collected.len(), 2);
    assert!(collected.contains(&PairLit!(1, 'a')));
    assert!(collected.contains(&PairLit!(2, 'b')));
}

#[test]
fn test_relation_fromvec() {
    let vec_pairs = vec![PairLit!(1, 'a'), PairLit!(2, 'b'), PairLit!(1, 'c')];
    let rel = RelationStEph::from_vec(vec_pairs);

    assert_eq!(rel.size(), 3);
    assert!(rel.mem(&1, &'a'));
    assert!(rel.mem(&2, &'b'));
    assert!(rel.mem(&1, &'c'));
}

#[test]
fn test_relationlit_macro_direct() {
    let empty_rel: RelationStEph<i32, char> = RelationLit![];
    assert_eq!(empty_rel.size(), 0);

    let single_rel = RelationLit![(1, 'a')];
    assert_eq!(single_rel.size(), 1);
    assert!(single_rel.mem(&1, &'a'));

    let multi_rel = RelationLit![(1, 'a'), (2, 'b'), (3, 'c')];
    assert_eq!(multi_rel.size(), 3);
    assert!(multi_rel.mem(&1, &'a'));
    assert!(multi_rel.mem(&2, &'b'));
    assert!(multi_rel.mem(&3, &'c'));
}

#[test]
fn test_empty_relation_domain_range() {
    let empty_rel: RelationStEph<i32, String> = RelationLit![];

    let domain = empty_rel.domain();
    assert_eq!(domain.size(), 0);

    let range = empty_rel.range();
    assert_eq!(range.size(), 0);

    assert!(!empty_rel.mem(&42, &"test".to_string()));
}

#[test]
fn test_relation_iterator_boundaries() {
    // Test iterator at beginning/end boundaries
    let rel = RelationLit![(10, 'a'), (20, 'b'), (30, 'c'), (40, 'd'), (50, 'e')];

    // Test iterator starting from beginning
    let mut iter = rel.iter();
    let first = iter.next();
    assert!(first.is_some()); // Should have first element
    let second = iter.next();
    assert!(second.is_some()); // Should have second element

    // Test iterator ending at end
    let iter_end = rel.iter();
    let collected = iter_end.cloned().collect::<Vec<Pair<i32, char>>>();
    assert_eq!(collected.len(), 5);
    // Note: HashSet iteration order is not guaranteed, so we check membership
    for pair in &collected {
        assert!(rel.mem(&pair.0, &pair.1));
    }

    // Test iterator on single element - both beginning and end
    let single_rel = RelationLit![(99, 'z')];
    let mut single_iter = single_rel.iter();
    assert_eq!(single_iter.next(), Some(&Pair(99, 'z'))); // Beginning = end
    assert_eq!(single_iter.next(), None); // Past end

    // Test iterator on empty relation - no boundaries
    let empty_rel: RelationStEph<i32, char> = RelationLit![];
    let mut empty_iter = empty_rel.iter();
    assert_eq!(empty_iter.next(), None); // No beginning

    // Test iterator exhaustion and reset
    let rel_reset = RelationLit![(1, 'x'), (2, 'y')];
    let mut iter1 = rel_reset.iter();
    // Exhaust iterator by consuming all elements
    let first_elem = iter1.next();
    let second_elem = iter1.next();
    assert!(first_elem.is_some());
    assert!(second_elem.is_some());
    assert_eq!(iter1.next(), None); // Should be exhausted

    // New iterator should start fresh at beginning
    let mut iter2 = rel_reset.iter();
    let fresh_first = iter2.next();
    assert!(fresh_first.is_some()); // Fresh start at beginning

    // Test iterator with functional operations at boundaries
    let rel_func = RelationLit![(100, 'p'), (200, 'q'), (300, 'r')];

    // First element via iterator (order not guaranteed)
    let first = rel_func.iter().next();
    assert!(first.is_some());
    assert!(rel_func.mem(&first.unwrap().0, &first.unwrap().1));

    // Count elements via iterator
    let count = rel_func.iter().count();
    assert_eq!(count, 3);

    // Test iterator chain boundaries
    let rel1 = RelationLit![(1, 'a'), (2, 'b')];
    let rel2 = RelationLit![(3, 'c'), (4, 'd')];
    let chained = rel1.iter().chain(rel2.iter()).cloned().collect::<Vec<Pair<i32, char>>>();
    assert_eq!(chained.len(), 4);
    // Check all elements are present
    for pair in &chained {
        assert!(rel1.mem(&pair.0, &pair.1) || rel2.mem(&pair.0, &pair.1));
    }

    // Test iterator skip/take boundaries
    let rel_skip = RelationLit![(10, 'a'), (20, 'b'), (30, 'c'), (40, 'd'), (50, 'e')];
    let skipped = rel_skip.iter().skip(2).cloned().collect::<Vec<Pair<i32, char>>>();
    assert_eq!(skipped.len(), 3);
    // All skipped elements should be in original relation
    for pair in &skipped {
        assert!(rel_skip.mem(&pair.0, &pair.1));
    }

    let taken = rel_skip.iter().take(3).cloned().collect::<Vec<Pair<i32, char>>>();
    assert_eq!(taken.len(), 3);
    // All taken elements should be in original relation
    for pair in &taken {
        assert!(rel_skip.mem(&pair.0, &pair.1));
    }

    // Test iterator collect and verify completeness
    let original = RelationLit![(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd'), (5, 'e')];
    let collected_all = original.iter().cloned().collect::<Vec<Pair<i32, char>>>();
    assert_eq!(collected_all.len(), 5);

    // Create new relation from collected elements and verify equality
    let reconstructed = RelationStEph::from_vec(collected_all);
    assert_eq!(reconstructed.size(), original.size());
    for i in 1..=5 {
        let expected_char = match i {
            | 1 => 'a',
            | 2 => 'b',
            | 3 => 'c',
            | 4 => 'd',
            | 5 => 'e',
            | _ => unreachable!(),
        };
        assert_eq!(original.mem(&i, &expected_char), reconstructed.mem(&i, &expected_char));
    }
}

#[test]
fn test_relation_maximum_size_boundary() {
    // Test maximum size collection boundary - use reasonably large size
    // to verify graceful handling without causing memory issues
    let large_size = 50_000usize;
    let large_vec = (0..large_size as i32).map(|i| Pair(i, i * 2)).collect::<Vec<Pair<i32, i32>>>();
    let large_rel = RelationStEph::from_vec(large_vec);

    // Verify basic operations work on large relation
    assert_eq!(large_rel.size(), large_size);
    assert!(large_rel.mem(&0, &0));
    assert!(large_rel.mem(&((large_size - 1) as i32), &(((large_size - 1) * 2) as i32)));
    assert!(!large_rel.mem(&(large_size as i32), &0));

    // Test domain and range on large relation
    let domain = large_rel.domain();
    assert_eq!(domain.size(), large_size);
    assert!(domain.mem(&0));
    assert!(domain.mem(&((large_size - 1) as i32)));
    assert!(!domain.mem(&(large_size as i32)));

    let range = large_rel.range();
    assert_eq!(range.size(), large_size);
    assert!(range.mem(&0));
    assert!(range.mem(&(((large_size - 1) * 2) as i32)));
    assert!(!range.mem(&(((large_size * 2) + 1) as i32))); // Value not in range

    // Test iterator on large relation (sample check)
    let mut count = 0;
    for pair in large_rel.iter() {
        if count < 10 {
            assert!(large_rel.mem(&pair.0, &pair.1));
            assert_eq!(pair.1, pair.0 * 2); // Verify relationship
        }
        count += 1;
        if count > large_size + 100 {
            // Safety check
            break;
        }
    }
    assert_eq!(count, large_size);

    // Test with another large relation (partial overlap)
    let large_vec2 = (25_000..75_000).map(|i| Pair(i, i * 3)).collect::<Vec<Pair<i32, i32>>>();
    let large_rel2 = RelationStEph::from_vec(large_vec2);

    // Verify the second relation
    assert_eq!(large_rel2.size(), 50_000);
    assert!(large_rel2.mem(&25_000, &75_000));
    assert!(large_rel2.mem(&74_999, &224_997));

    // Test operations between large relations would be memory intensive,
    // so we test with smaller subsets
    let small_rel1 = RelationLit![(1, 2), (2, 4), (3, 6)];
    let small_rel2 = RelationLit![(2, 4), (4, 8), (5, 10)];

    // Verify these work as expected (proxy for large relation operations)
    assert_eq!(small_rel1.size(), 3);
    assert_eq!(small_rel2.size(), 3);
    assert!(small_rel1.mem(&2, &4));
    assert!(small_rel2.mem(&2, &4));
}

#[test]
fn test_relation_debug_display() {
    let rel = RelationLit![(1, 'a'), (2, 'b')];

    let debug_str = format!("{:?}", rel);
    assert!(!debug_str.is_empty());

    let display_str = format!("{}", rel);
    assert!(!display_str.is_empty());
}

#[test]
fn test_relation_equality() {
    let rel1 = RelationLit![(1, 'a'), (2, 'b')];
    let rel2 = RelationLit![(1, 'a'), (2, 'b')];
    let rel3 = RelationLit![(1, 'a'), (3, 'c')];

    assert_eq!(rel1, rel2);
    assert_ne!(rel1, rel3);
}

#[test]
fn test_relation_clone() {
    let rel1 = RelationLit![(1, 'a'), (2, 'b')];
    let rel2 = rel1.clone();

    assert_eq!(rel1, rel2);
    assert_eq!(rel1.size(), rel2.size());
}
