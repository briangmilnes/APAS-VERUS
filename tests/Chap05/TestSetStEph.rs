//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::{PairLit, SetLit};
use apas_verus::Types::Types::*; // macro import

#[test]
fn test_setlit_macro_functionality() {
    // Test empty set creation
    let empty: SetStEph<i32> = SetLit![];
    assert_eq!(empty.size(), 0);

    // Test set creation with elements
    let with_data: SetStEph<i32> = SetLit![1, 2, 3];
    assert_eq!(with_data.size(), 3);
    assert!(with_data.mem(&1));
    assert!(with_data.mem(&2));
    assert!(with_data.mem(&3));
    assert!(!with_data.mem(&4));
}

#[test]
fn test_setlit_macro_type_safety() {
    // Test empty set creation with explicit type
    let empty: SetStEph<&'static str> = SetLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.mem(&"any"));

    // Test single element set
    let one = SetLit!["only"];
    assert_eq!(one.size(), 1);
    assert!(one.mem(&"only"));
    assert!(!one.mem(&"other"));

    // Test multi-element set
    let many = SetLit!["a", "b", "c"];
    assert_eq!(many.size(), 3);
    assert!(many.mem(&"a"));
    assert!(many.mem(&"b"));
    assert!(many.mem(&"c"));
    assert!(!many.mem(&"d"));
}

#[test]
fn test_cartesian_product_example_5_1() {
    let a: SetStEph<N> = SetLit![0, 1, 2, 3];
    let b: SetStEph<char> = SetLit!['a', 'b'];
    let prod = a.CartesianProduct(&b);

    let expect: SetStEph<Pair<N, char>> = SetLit![
        PairLit!(0, 'a'),
        PairLit!(0, 'b'),
        PairLit!(1, 'a'),
        PairLit!(1, 'b'),
        PairLit!(2, 'a'),
        PairLit!(2, 'b'),
        PairLit!(3, 'a'),
        PairLit!(3, 'b')
    ];
    assert_eq!(prod, expect);
    assert_eq!(prod.size(), 8);
}

#[test]
fn test_partition_example_5_2_true() {
    let a: SetStEph<N> = SetLit![1, 2, 3, 4, 5, 6];
    let odd: SetStEph<N> = SetLit![1, 3, 5];
    let even: SetStEph<N> = SetLit![2, 4, 6];
    let p: SetStEph<SetStEph<N>> = SetLit![odd, even];
    assert!(a.partition(&p));
}

#[test]
fn test_partition_example_5_2_false_due_to_overlap() {
    let a: SetStEph<N> = SetLit![1, 2, 3, 4, 5, 6];
    let odd_with_6: SetStEph<N> = SetLit![1, 3, 5, 6];
    let even_with_6: SetStEph<N> = SetLit![2, 4, 6];
    let q: SetStEph<SetStEph<N>> = SetLit![odd_with_6, even_with_6];
    assert!(!a.partition(&q));
}

#[test]
fn test_partition_false_due_to_missing_element() {
    let a: SetStEph<N> = SetLit![1, 2, 3];
    let s1: SetStEph<N> = SetLit![1];
    let s2: SetStEph<N> = SetLit![2];
    let parts: SetStEph<SetStEph<N>> = SetLit![s1, s2];
    assert!(!a.partition(&parts));
}

#[test]
fn test_set_empty() {
    let empty_set = SetStEph::<i32>::empty();
    assert_eq!(empty_set.size(), 0);
    assert!(!empty_set.mem(&42));
}

#[test]
fn test_set_singleton() {
    let single_set = SetStEph::singleton(42);
    assert_eq!(single_set.size(), 1);
    assert!(single_set.mem(&42));
    assert!(!single_set.mem(&99));
}

#[test]
fn test_set_size_comprehensive() {
    let empty = SetStEph::<i32>::empty();
    assert_eq!(empty.size(), 0);

    let single = SetStEph::singleton(1);
    assert_eq!(single.size(), 1);

    let multi = SetLit![1, 2, 3, 4, 5];
    assert_eq!(multi.size(), 5);
}

#[test]
fn test_set_mem_comprehensive() {
    let set = SetLit![1, 2, 3];
    assert!(set.mem(&1));
    assert!(set.mem(&2));
    assert!(set.mem(&3));
    assert!(!set.mem(&4));
    assert!(!set.mem(&0));
}

#[test]
fn test_set_union() {
    let set1 = SetLit![1, 2, 3];
    let set2 = SetLit![3, 4, 5];
    let union_set = set1.union(&set2);

    assert_eq!(union_set.size(), 5);
    assert!(union_set.mem(&1));
    assert!(union_set.mem(&2));
    assert!(union_set.mem(&3));
    assert!(union_set.mem(&4));
    assert!(union_set.mem(&5));
    assert!(!union_set.mem(&6));
}

#[test]
fn test_set_intersection() {
    let set1 = SetLit![1, 2, 3, 4];
    let set2 = SetLit![3, 4, 5, 6];
    let intersect_set = set1.intersection(&set2);

    assert_eq!(intersect_set.size(), 2);
    assert!(intersect_set.mem(&3));
    assert!(intersect_set.mem(&4));
    assert!(!intersect_set.mem(&1));
    assert!(!intersect_set.mem(&5));
}

#[test]
fn test_set_insert() {
    let mut set = SetStEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(1); // duplicate

    assert_eq!(set.size(), 2);
    assert!(set.mem(&1));
    assert!(set.mem(&2));
}

#[test]
fn test_set_iter() {
    let set = SetLit![1, 2, 3];
    let mut collected = set.iter().cloned().collect::<Vec<i32>>();
    collected.sort(); // HashSet order is not guaranteed

    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_set_fromvec() {
    let vec_data = vec![1, 2, 3, 2, 1]; // with duplicates
    let set = SetStEph::FromVec(vec_data);

    assert_eq!(set.size(), 3);
    assert!(set.mem(&1));
    assert!(set.mem(&2));
    assert!(set.mem(&3));
}

#[test]
fn test_cartesian_product_empty_edge() {
    let empty_set = SetStEph::<i32>::empty();
    let normal_set = SetLit![1, 2];

    let prod1 = empty_set.CartesianProduct(&normal_set);
    assert_eq!(prod1.size(), 0);

    let prod2 = normal_set.CartesianProduct(&empty_set);
    assert_eq!(prod2.size(), 0);
}

#[test]
fn test_setlit_macro_direct() {
    let empty: SetStEph<i32> = SetLit![];
    assert_eq!(empty.size(), 0);

    let single = SetLit![42];
    assert_eq!(single.size(), 1);
    assert!(single.mem(&42));

    let multi = SetLit![1, 2, 3];
    assert_eq!(multi.size(), 3);
}

#[test]
fn test_empty_set_union() {
    let empty = SetStEph::<i32>::empty();
    let normal = SetLit![1, 2, 3];

    let union1 = empty.union(&normal);
    assert_eq!(union1.size(), 3);

    let union2 = normal.union(&empty);
    assert_eq!(union2.size(), 3);

    let union_empty = empty.union(&empty);
    assert_eq!(union_empty.size(), 0);
}

#[test]
fn test_empty_set_intersection() {
    let empty = SetStEph::<i32>::empty();
    let normal = SetLit![1, 2, 3];

    let intersect1 = empty.intersection(&normal);
    assert_eq!(intersect1.size(), 0);

    let intersect2 = normal.intersection(&empty);
    assert_eq!(intersect2.size(), 0);

    let intersect_empty = empty.intersection(&empty);
    assert_eq!(intersect_empty.size(), 0);
}

#[test]
fn test_set_large_operations_stress() {
    // Test with large sets to verify no panics occur
    let large_vec = (0..10000).collect::<Vec<i32>>();
    let large_set = SetStEph::FromVec(large_vec);

    assert_eq!(large_set.size(), 10000);
    assert!(large_set.mem(&5000));
    assert!(!large_set.mem(&15000));

    // Test union with another large set
    let large_vec2 = (5000..15000).collect::<Vec<i32>>();
    let large_set2 = SetStEph::FromVec(large_vec2);

    let union_result = large_set.union(&large_set2);
    assert_eq!(union_result.size(), 15000); // 0-4999 + 5000-14999 = 15000 unique elements

    // Test intersection
    let intersection_result = large_set.intersection(&large_set2);
    assert_eq!(intersection_result.size(), 5000); // 5000-9999 overlap

    // Verify no panics on extreme operations
    let empty_set = SetStEph::<i32>::empty();
    let union_with_empty = large_set.union(&empty_set);
    assert_eq!(union_with_empty.size(), 10000);

    let intersection_with_empty = large_set.intersection(&empty_set);
    assert_eq!(intersection_with_empty.size(), 0);
}

#[test]
fn test_set_single_element_boundary() {
    // Test single element set operations
    let single = SetStEph::singleton(42);
    assert_eq!(single.size(), 1);
    assert!(single.mem(&42));
    assert!(!single.mem(&43));

    // Operations with single element set
    let empty = SetStEph::<i32>::empty();
    let union_with_empty = single.union(&empty);
    assert_eq!(union_with_empty.size(), 1);
    assert!(union_with_empty.mem(&42));

    let intersection_with_empty = single.intersection(&empty);
    assert_eq!(intersection_with_empty.size(), 0);

    // Union with another single element
    let single2 = SetStEph::singleton(99);
    let union_singles = single.union(&single2);
    assert_eq!(union_singles.size(), 2);
    assert!(union_singles.mem(&42));
    assert!(union_singles.mem(&99));

    // Intersection with same element
    let single_same = SetStEph::singleton(42);
    let intersection_same = single.intersection(&single_same);
    assert_eq!(intersection_same.size(), 1);
    assert!(intersection_same.mem(&42));

    // Intersection with different element
    let intersection_diff = single.intersection(&single2);
    assert_eq!(intersection_diff.size(), 0);

    // Cartesian product with single element
    let single_char = SetStEph::singleton('a');
    let cartesian = single.CartesianProduct(&single_char);
    assert_eq!(cartesian.size(), 1);
    assert!(cartesian.mem(&Pair(42, 'a')));

    // Iterator on single element
    let collected = single.iter().cloned().collect::<Vec<i32>>();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0], 42);
}

#[test]
fn test_set_iterator_boundaries() {
    // Test iterator at beginning/end boundaries
    let set = SetLit![10, 20, 30, 40, 50];

    // Test iterator starting from beginning
    let mut iter = set.iter();
    let first = iter.next();
    assert!(first.is_some()); // Should have first element
    let second = iter.next();
    assert!(second.is_some()); // Should have second element

    // Test iterator ending at end
    let iter_end = set.iter();
    let collected = iter_end.cloned().collect::<Vec<i32>>();
    assert_eq!(collected.len(), 5);
    // Note: HashSet iteration order is not guaranteed, so we check membership
    for val in &collected {
        assert!(set.mem(val));
    }

    // Test iterator on single element - both beginning and end
    let single = SetStEph::singleton(99);
    let mut single_iter = single.iter();
    assert_eq!(single_iter.next(), Some(&99)); // Beginning = end
    assert_eq!(single_iter.next(), None); // Past end

    // Test iterator on empty set - no boundaries
    let empty = SetStEph::<i32>::empty();
    let mut empty_iter = empty.iter();
    assert_eq!(empty_iter.next(), None); // No beginning

    // Test iterator exhaustion and reset
    let set_reset = SetLit![1, 2];
    let mut iter1 = set_reset.iter();
    // Exhaust iterator by consuming all elements
    let first_elem = iter1.next();
    let second_elem = iter1.next();
    assert!(first_elem.is_some());
    assert!(second_elem.is_some());
    assert_eq!(iter1.next(), None); // Should be exhausted

    // New iterator should start fresh at beginning
    let mut iter2 = set_reset.iter();
    let fresh_first = iter2.next();
    assert!(fresh_first.is_some()); // Fresh start at beginning

    // Test iterator with functional operations at boundaries
    let set_func = SetLit![100, 200, 300];

    // First element via iterator (order not guaranteed)
    let first = set_func.iter().next();
    assert!(first.is_some());
    assert!(set_func.mem(first.unwrap()));

    // Count elements via iterator
    let count = set_func.iter().count();
    assert_eq!(count, 3);

    // Test iterator chain boundaries
    let set1 = SetLit![1, 2];
    let set2 = SetLit![3, 4];
    let chained = set1.iter().chain(set2.iter()).cloned().collect::<Vec<i32>>();
    assert_eq!(chained.len(), 4);
    // Check all elements are present
    for val in &chained {
        assert!(set1.mem(val) || set2.mem(val));
    }

    // Test iterator skip/take boundaries
    let set_skip = SetLit![10, 20, 30, 40, 50];
    let skipped = set_skip.iter().skip(2).cloned().collect::<Vec<i32>>();
    assert_eq!(skipped.len(), 3);
    // All skipped elements should be in original set
    for val in &skipped {
        assert!(set_skip.mem(val));
    }

    let taken = set_skip.iter().take(3).cloned().collect::<Vec<i32>>();
    assert_eq!(taken.len(), 3);
    // All taken elements should be in original set
    for val in &taken {
        assert!(set_skip.mem(val));
    }

    // Test iterator collect and verify completeness
    let original = SetLit![1, 2, 3, 4, 5];
    let collected_all = original.iter().cloned().collect::<Vec<i32>>();
    assert_eq!(collected_all.len(), 5);

    // Create new set from collected elements and verify equality
    let reconstructed = SetStEph::FromVec(collected_all);
    assert_eq!(reconstructed.size(), original.size());
    for i in 1..=5 {
        assert_eq!(original.mem(&i), reconstructed.mem(&i));
    }
}

#[test]
fn test_set_maximum_size_boundary() {
    // Test large set boundary (reduced from 100k to 20k for faster testing)
    let large_size = 20_000usize;
    let large_vec = (0..large_size as i32).collect::<Vec<i32>>();
    let large_set = SetStEph::FromVec(large_vec);

    // Verify basic operations work on large set
    assert_eq!(large_set.size(), large_size);
    assert!(large_set.mem(&0));
    assert!(large_set.mem(&((large_size - 1) as i32)));
    assert!(!large_set.mem(&(large_size as i32)));

    // Test operations on large set
    let empty_set = SetStEph::<i32>::empty();
    let union_with_empty = large_set.union(&empty_set);
    assert_eq!(union_with_empty.size(), large_size);

    let intersection_with_empty = large_set.intersection(&empty_set);
    assert_eq!(intersection_with_empty.size(), 0);

    // Test with another large set
    let large_vec2 = (10_000..30_000).collect::<Vec<i32>>();
    let large_set2 = SetStEph::FromVec(large_vec2);

    let union_large = large_set.union(&large_set2);
    assert_eq!(union_large.size(), 30_000); // 0-9999 + 10000-29999 = 30000 unique

    let intersection_large = large_set.intersection(&large_set2);
    assert_eq!(intersection_large.size(), 10_000); // 10000-19999 overlap

    // Test iterator on large set (sample check)
    let mut count = 0;
    for val in large_set.iter() {
        if count < 10 {
            assert!(large_set.mem(val));
        }
        count += 1;
        if count > large_size + 100 {
            // Safety check
            break;
        }
    }
    assert_eq!(count, large_size);

    // Test Cartesian product with small set to avoid explosion
    let small_set = SetStEph::singleton('a');
    let cartesian_large = large_set.CartesianProduct(&small_set);
    assert_eq!(cartesian_large.size(), large_size);
    assert!(cartesian_large.mem(&Pair(0, 'a')));
    assert!(cartesian_large.mem(&Pair((large_size - 1) as i32, 'a')));
}

#[test]
fn test_trait_empty() {
    let s: SetStEph<i32> = <SetStEph<i32> as SetStEphTrait<i32>>::empty();
    assert_eq!(s.size(), 0);
}

#[test]
fn test_trait_singleton() {
    let s: SetStEph<i32> = <SetStEph<i32> as SetStEphTrait<i32>>::singleton(42);
    assert_eq!(s.size(), 1);
    assert!(s.mem(&42));
}

#[test]
fn test_trait_union() {
    let s1 = SetLit![1, 2];
    let s2 = SetLit![2, 3];
    let u: SetStEph<i32> = <SetStEph<i32> as SetStEphTrait<i32>>::union(&s1, &s2);
    assert_eq!(u.size(), 3);
}

#[test]
fn test_trait_intersection() {
    let s1 = SetLit![1, 2, 3];
    let s2 = SetLit![2, 3, 4];
    let i: SetStEph<i32> = <SetStEph<i32> as SetStEphTrait<i32>>::intersection(&s1, &s2);
    assert_eq!(i.size(), 2);
}

#[test]
fn test_trait_partition() {
    let whole = SetLit![1, 2, 3, 4];
    let part1 = SetLit![1, 2];
    let part2 = SetLit![3, 4];
    let parts = SetLit![part1, part2];
    assert!(<SetStEph<i32> as SetStEphTrait<i32>>::partition(&whole, &parts));
}

#[test]
fn test_trait_cartesian_product() {
    let s1 = SetLit![1, 2];
    let s2 = SetLit!['a', 'b'];
    let cp = <SetStEph<i32> as SetStEphTrait<i32>>::CartesianProduct(&s1, &s2);
    assert_eq!(cp.size(), 4);
}

#[test]
fn test_trait_insert() {
    let mut s = SetStEph::<i32>::empty();
    <SetStEph<i32> as SetStEphTrait<i32>>::insert(&mut s, 42);
    assert_eq!(s.size(), 1);
}

#[test]
fn test_trait_from_vec() {
    let s = <SetStEph<i32> as SetStEphTrait<i32>>::FromVec(vec![1, 2, 3]);
    assert_eq!(s.size(), 3);
}

#[test]
fn test_debug_trait() {
    let s = SetLit![1, 2, 3];
    let debug_str = format!("{:?}", s);
    assert!(debug_str.contains("1") || debug_str.contains("2") || debug_str.contains("3"));
}

#[test]
fn test_display_trait() {
    let s = SetLit![1, 2, 3];
    let display_str = format!("{}", s);
    assert!(display_str.contains("1"));
}
