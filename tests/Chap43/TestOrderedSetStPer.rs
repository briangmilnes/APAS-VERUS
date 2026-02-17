//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Comprehensive tests for OrderedSetStPer - persistent ordered set implementation.

use apas_verus::Chap43::OrderedSetStPer::OrderedSetStPer::*;
use apas_verus::OrderedSetStPerLit;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let set = OrderedSetStPer::<i32>::empty();
    assert_eq!(set.size(), 0);
    assert_eq!(set.first(), None);
    assert_eq!(set.last(), None);
}

#[test]
fn test_singleton() {
    let set = OrderedSetStPer::singleton(42);
    assert_eq!(set.size(), 1);
    assert_eq!(set.first(), Some(42));
    assert_eq!(set.last(), Some(42));
    assert!(set.find(&42));
    assert!(!set.find(&0));
}

#[test]
fn test_insert_and_find() {
    let set = OrderedSetStPer::empty()
        .insert(5)
        .insert(2)
        .insert(8)
        .insert(1)
        .insert(7);

    assert_eq!(set.size(), 5);
    assert!(set.find(&1));
    assert!(set.find(&2));
    assert!(set.find(&5));
    assert!(set.find(&7));
    assert!(set.find(&8));
    assert!(!set.find(&0));
    assert!(!set.find(&10));
}

#[test]
fn test_delete() {
    let set = OrderedSetStPer::empty().insert(5).insert(2).insert(8);

    let set2 = set.delete(&2);
    assert_eq!(set2.size(), 2);
    assert!(!set2.find(&2));
    assert!(set2.find(&5));
    assert!(set2.find(&8));

    // Original set unchanged (persistent)
    assert_eq!(set.size(), 3);
    assert!(set.find(&2));
}

#[test]
fn test_first_and_last() {
    let set = OrderedSetStPer::empty()
        .insert(5)
        .insert(2)
        .insert(8)
        .insert(1)
        .insert(7);

    assert_eq!(set.first(), Some(1));
    assert_eq!(set.last(), Some(8));

    let empty_set = OrderedSetStPer::<i32>::empty();
    assert_eq!(empty_set.first(), None);
    assert_eq!(empty_set.last(), None);
}

#[test]
fn test_previous() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    assert_eq!(set.previous(&0), None); // Before first
    assert_eq!(set.previous(&1), None); // At first
    assert_eq!(set.previous(&2), Some(1)); // Between elements
    assert_eq!(set.previous(&5), Some(3)); // At element
    assert_eq!(set.previous(&6), Some(5)); // Between elements
    assert_eq!(set.previous(&10), Some(9)); // After last
}

#[test]
fn test_next() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    assert_eq!(set.next(&0), Some(1)); // Before first
    assert_eq!(set.next(&1), Some(3)); // At first
    assert_eq!(set.next(&2), Some(3)); // Between elements
    assert_eq!(set.next(&5), Some(7)); // At element
    assert_eq!(set.next(&8), Some(9)); // Between elements
    assert_eq!(set.next(&9), None); // At last
    assert_eq!(set.next(&10), None); // After last
}

#[test]
fn test_split() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    // Split at existing element
    let (left, found, right) = set.split(&5);
    assert!(found);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 2);
    assert!(left.find(&1));
    assert!(left.find(&3));
    assert!(!left.find(&5));
    assert!(right.find(&7));
    assert!(right.find(&9));
    assert!(!right.find(&5));

    // Split at non-existing element
    let (left2, found2, right2) = set.split(&4);
    assert!(!found2);
    assert_eq!(left2.size(), 2);
    assert_eq!(right2.size(), 3);
    assert!(left2.find(&1));
    assert!(left2.find(&3));
    assert!(right2.find(&5));
    assert!(right2.find(&7));
    assert!(right2.find(&9));
}

#[test]
fn test_join() {
    let left = OrderedSetStPer::empty().insert(1).insert(3);

    let right = OrderedSetStPer::empty().insert(7).insert(9);

    let joined = OrderedSetStPer::join(&left, &right);
    assert_eq!(joined.size(), 4);
    assert!(joined.find(&1));
    assert!(joined.find(&3));
    assert!(joined.find(&7));
    assert!(joined.find(&9));
}

#[test]
fn test_get_range() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    let range = set.get_range(&3, &7);
    assert_eq!(range.size(), 3);
    assert!(range.find(&3));
    assert!(range.find(&5));
    assert!(range.find(&7));
    assert!(!range.find(&1));
    assert!(!range.find(&9));

    // Empty range
    let empty_range = set.get_range(&10, &20);
    assert_eq!(empty_range.size(), 0);
}

#[test]
fn test_rank() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    assert_eq!(set.rank(&0), 0); // Before first
    assert_eq!(set.rank(&1), 0); // At first
    assert_eq!(set.rank(&2), 1); // Between elements
    assert_eq!(set.rank(&5), 2); // At element
    assert_eq!(set.rank(&6), 3); // Between elements
    assert_eq!(set.rank(&10), 5); // After last
}

#[test]
fn test_select() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    assert_eq!(set.select(0), Some(1));
    assert_eq!(set.select(1), Some(3));
    assert_eq!(set.select(2), Some(5));
    assert_eq!(set.select(3), Some(7));
    assert_eq!(set.select(4), Some(9));
    assert_eq!(set.select(5), None); // Out of bounds
}

#[test]
fn test_split_rank() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(3)
        .insert(5)
        .insert(7)
        .insert(9);

    let (left, right) = set.split_rank(2);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 3);
    assert!(left.find(&1));
    assert!(left.find(&3));
    assert!(right.find(&5));
    assert!(right.find(&7));
    assert!(right.find(&9));

    // Split at end
    let (left2, right2) = set.split_rank(5);
    assert_eq!(left2.size(), 5);
    assert_eq!(right2.size(), 0);

    // Split at beginning
    let (left3, right3) = set.split_rank(0);
    assert_eq!(left3.size(), 0);
    assert_eq!(right3.size(), 5);
}

#[test]
fn test_filter() {
    let set = OrderedSetStPer::empty()
        .insert(1)
        .insert(2)
        .insert(3)
        .insert(4)
        .insert(5);

    let evens = set.filter(|x| *x % 2 == 0);
    assert_eq!(evens.size(), 2);
    assert!(evens.find(&2));
    assert!(evens.find(&4));
    assert!(!evens.find(&1));
    assert!(!evens.find(&3));
    assert!(!evens.find(&5));
}

#[test]
fn test_intersection() {
    let set1 = OrderedSetStPer::empty().insert(1).insert(3).insert(5).insert(7);

    let set2 = OrderedSetStPer::empty().insert(3).insert(4).insert(5).insert(6);

    let intersection = set1.intersection(&set2);
    assert_eq!(intersection.size(), 2);
    assert!(intersection.find(&3));
    assert!(intersection.find(&5));
    assert!(!intersection.find(&1));
    assert!(!intersection.find(&4));
}

#[test]
fn test_union() {
    let set1 = OrderedSetStPer::empty().insert(1).insert(3).insert(5);

    let set2 = OrderedSetStPer::empty().insert(3).insert(4).insert(6);

    let union = set1.union(&set2);
    assert_eq!(union.size(), 5);
    assert!(union.find(&1));
    assert!(union.find(&3));
    assert!(union.find(&4));
    assert!(union.find(&5));
    assert!(union.find(&6));
}

#[test]
fn test_difference() {
    let set1 = OrderedSetStPer::empty().insert(1).insert(3).insert(5).insert(7);

    let set2 = OrderedSetStPer::empty().insert(3).insert(5);

    let difference = set1.difference(&set2);
    assert_eq!(difference.size(), 2);
    assert!(difference.find(&1));
    assert!(difference.find(&7));
    assert!(!difference.find(&3));
    assert!(!difference.find(&5));
}

#[test]
fn test_persistence() {
    let original = OrderedSetStPer::empty().insert(1).insert(2).insert(3);

    let modified = original.insert(4).delete(&2);

    // Original unchanged
    assert_eq!(original.size(), 3);
    assert!(original.find(&1));
    assert!(original.find(&2));
    assert!(original.find(&3));
    assert!(!original.find(&4));

    // Modified has changes
    assert_eq!(modified.size(), 3);
    assert!(modified.find(&1));
    assert!(!modified.find(&2));
    assert!(modified.find(&3));
    assert!(modified.find(&4));
}

#[test]
fn test_ordered_set_st_per_lit_macro() {
    let set: OrderedSetStPer<i32> = OrderedSetStPerLit![1, 3, 5, 7, 9];
    assert_eq!(set.size(), 5);
    assert!(set.find(&1));
    assert!(set.find(&3));
    assert!(set.find(&5));
    assert!(set.find(&7));
    assert!(set.find(&9));
    assert!(!set.find(&2));

    let empty_set: OrderedSetStPer<i32> = OrderedSetStPerLit![];
    assert_eq!(empty_set.size(), 0);
}

#[test]
fn test_string_ordering() {
    let set = OrderedSetStPer::empty()
        .insert("charlie".to_string())
        .insert("alice".to_string())
        .insert("bob".to_string());

    assert_eq!(set.first(), Some("alice".to_string()));
    assert_eq!(set.last(), Some("charlie".to_string()));
    assert_eq!(set.next(&"alice".to_string()), Some("bob".to_string()));
    assert_eq!(set.previous(&"charlie".to_string()), Some("bob".to_string()));
}

#[test]
fn test_from_sorted_elements() {
    let set = from_sorted_elements(vec![1, 3, 5, 7, 9]);
    assert_eq!(set.size(), 5);
    assert!(set.find(&1));
    assert!(set.find(&3));
    assert!(set.find(&5));
    assert!(set.find(&7));
    assert!(set.find(&9));
    assert_eq!(set.first(), Some(1));
    assert_eq!(set.last(), Some(9));

    let empty_set = from_sorted_elements::<i32>(vec![]);
    assert_eq!(empty_set.size(), 0);

    let single = from_sorted_elements(vec![42]);
    assert_eq!(single.size(), 1);
    assert!(single.find(&42));
}
