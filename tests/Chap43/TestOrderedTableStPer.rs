//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Comprehensive tests for OrderedTableStPer - persistent ordered table implementation.

use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap43::OrderedTableStPer::OrderedTableStPer::*;
use apas_verus::OrderedTableStPerLit;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let table = OrderedTableStPer::<i32, String>::empty();
    assert_eq!(table.size(), 0);
    assert_eq!(table.first_key(), None);
    assert_eq!(table.last_key(), None);
}

#[test]
fn test_singleton() {
    let table = OrderedTableStPer::singleton(42, "answer".to_string());
    assert_eq!(table.size(), 1);
    assert_eq!(table.first_key(), Some(42));
    assert_eq!(table.last_key(), Some(42));
    assert_eq!(table.find(&42), Some("answer".to_string()));
    assert_eq!(table.find(&0), None);
}

#[test]
fn test_insert_and_find() {
    let table = OrderedTableStPer::empty()
        .insert(5, "five".to_string())
        .insert(2, "two".to_string())
        .insert(8, "eight".to_string())
        .insert(1, "one".to_string())
        .insert(7, "seven".to_string());

    assert_eq!(table.size(), 5);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&5), Some("five".to_string()));
    assert_eq!(table.find(&7), Some("seven".to_string()));
    assert_eq!(table.find(&8), Some("eight".to_string()));
    assert_eq!(table.find(&0), None);
    assert_eq!(table.find(&10), None);
}

#[test]
fn test_delete() {
    let table = OrderedTableStPer::empty()
        .insert(5, "five".to_string())
        .insert(2, "two".to_string())
        .insert(8, "eight".to_string());

    let table2 = table.delete(&2);
    assert_eq!(table2.size(), 2);
    assert_eq!(table2.find(&2), None);
    assert_eq!(table2.find(&5), Some("five".to_string()));
    assert_eq!(table2.find(&8), Some("eight".to_string()));

    // Original table unchanged (persistent)
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&2), Some("two".to_string()));
}

#[test]
fn test_first_key_and_last_key() {
    let table = OrderedTableStPer::empty()
        .insert(5, "five".to_string())
        .insert(2, "two".to_string())
        .insert(8, "eight".to_string())
        .insert(1, "one".to_string())
        .insert(7, "seven".to_string());

    assert_eq!(table.first_key(), Some(1));
    assert_eq!(table.last_key(), Some(8));

    let empty_table = OrderedTableStPer::<i32, String>::empty();
    assert_eq!(empty_table.first_key(), None);
    assert_eq!(empty_table.last_key(), None);
}

#[test]
fn test_previous_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    assert_eq!(table.previous_key(&0), None); // Before first
    assert_eq!(table.previous_key(&1), None); // At first
    assert_eq!(table.previous_key(&2), Some(1)); // Between keys
    assert_eq!(table.previous_key(&5), Some(3)); // At key
    assert_eq!(table.previous_key(&6), Some(5)); // Between keys
    assert_eq!(table.previous_key(&10), Some(9)); // After last
}

#[test]
fn test_next_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    assert_eq!(table.next_key(&0), Some(1)); // Before first
    assert_eq!(table.next_key(&1), Some(3)); // At first
    assert_eq!(table.next_key(&2), Some(3)); // Between keys
    assert_eq!(table.next_key(&5), Some(7)); // At key
    assert_eq!(table.next_key(&8), Some(9)); // Between keys
    assert_eq!(table.next_key(&9), None); // At last
    assert_eq!(table.next_key(&10), None); // After last
}

#[test]
fn test_split_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    // Split at existing key
    let (left, found_value, right) = table.split_key(&5);
    assert_eq!(found_value, Some("five".to_string()));
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 2);
    assert_eq!(left.find(&1), Some("one".to_string()));
    assert_eq!(left.find(&3), Some("three".to_string()));
    assert_eq!(left.find(&5), None);
    assert_eq!(right.find(&7), Some("seven".to_string()));
    assert_eq!(right.find(&9), Some("nine".to_string()));
    assert_eq!(right.find(&5), None);

    // Split at non-existing key
    let (left2, found_value2, right2) = table.split_key(&4);
    assert_eq!(found_value2, None);
    assert_eq!(left2.size(), 2);
    assert_eq!(right2.size(), 3);
    assert_eq!(left2.find(&1), Some("one".to_string()));
    assert_eq!(left2.find(&3), Some("three".to_string()));
    assert_eq!(right2.find(&5), Some("five".to_string()));
    assert_eq!(right2.find(&7), Some("seven".to_string()));
    assert_eq!(right2.find(&9), Some("nine".to_string()));
}

#[test]
fn test_join_key() {
    let left = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string());

    let right = OrderedTableStPer::empty()
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    let joined = OrderedTableStPer::join_key(&left, &right);
    assert_eq!(joined.size(), 4);
    assert_eq!(joined.find(&1), Some("one".to_string()));
    assert_eq!(joined.find(&3), Some("three".to_string()));
    assert_eq!(joined.find(&7), Some("seven".to_string()));
    assert_eq!(joined.find(&9), Some("nine".to_string()));
}

#[test]
fn test_get_key_range() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    let range = table.get_key_range(&3, &7);
    assert_eq!(range.size(), 3);
    assert_eq!(range.find(&3), Some("three".to_string()));
    assert_eq!(range.find(&5), Some("five".to_string()));
    assert_eq!(range.find(&7), Some("seven".to_string()));
    assert_eq!(range.find(&1), None);
    assert_eq!(range.find(&9), None);

    // Empty range
    let empty_range = table.get_key_range(&10, &20);
    assert_eq!(empty_range.size(), 0);
}

#[test]
fn test_rank_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    assert_eq!(table.rank_key(&0), 0); // Before first
    assert_eq!(table.rank_key(&1), 0); // At first
    assert_eq!(table.rank_key(&2), 1); // Between keys
    assert_eq!(table.rank_key(&5), 2); // At key
    assert_eq!(table.rank_key(&6), 3); // Between keys
    assert_eq!(table.rank_key(&10), 5); // After last
}

#[test]
fn test_select_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    assert_eq!(table.select_key(0), Some(1));
    assert_eq!(table.select_key(1), Some(3));
    assert_eq!(table.select_key(2), Some(5));
    assert_eq!(table.select_key(3), Some(7));
    assert_eq!(table.select_key(4), Some(9));
    assert_eq!(table.select_key(5), None); // Out of bounds
}

#[test]
fn test_split_rank_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string())
        .insert(9, "nine".to_string());

    let (left, right) = table.split_rank_key(2);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 3);
    assert_eq!(left.find(&1), Some("one".to_string()));
    assert_eq!(left.find(&3), Some("three".to_string()));
    assert_eq!(right.find(&5), Some("five".to_string()));
    assert_eq!(right.find(&7), Some("seven".to_string()));
    assert_eq!(right.find(&9), Some("nine".to_string()));

    // Split at end
    let (left2, right2) = table.split_rank_key(5);
    assert_eq!(left2.size(), 5);
    assert_eq!(right2.size(), 0);

    // Split at beginning
    let (left3, right3) = table.split_rank_key(0);
    assert_eq!(left3.size(), 0);
    assert_eq!(right3.size(), 5);
}

#[test]
fn test_domain() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string());

    let domain = table.domain();
    assert_eq!(domain.size(), 3);
    assert!(domain.find(&1));
    assert!(domain.find(&3));
    assert!(domain.find(&5));
    assert!(!domain.find(&2));
}

#[test]
fn test_tabulate() {
    let mut keys = ArraySetStEph::empty();
    keys.insert(1);
    keys.insert(2);
    keys.insert(3);

    let table = OrderedTableStPer::tabulate(|k| format!("value_{k}"), &keys);
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some("value_1".to_string()));
    assert_eq!(table.find(&2), Some("value_2".to_string()));
    assert_eq!(table.find(&3), Some("value_3".to_string()));
}

#[test]
fn test_map() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(2, "two".to_string())
        .insert(3, "three".to_string());

    let mapped = table.map(|v| v.to_uppercase());
    assert_eq!(mapped.size(), 3);
    assert_eq!(mapped.find(&1), Some("ONE".to_string()));
    assert_eq!(mapped.find(&2), Some("TWO".to_string()));
    assert_eq!(mapped.find(&3), Some("THREE".to_string()));
}

#[test]
fn test_filter() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(2, "two".to_string())
        .insert(3, "three".to_string())
        .insert(4, "four".to_string())
        .insert(5, "five".to_string());

    let evens = table.filter(|k, _v| *k % 2 == 0);
    assert_eq!(evens.size(), 2);
    assert_eq!(evens.find(&2), Some("two".to_string()));
    assert_eq!(evens.find(&4), Some("four".to_string()));
    assert_eq!(evens.find(&1), None);
    assert_eq!(evens.find(&3), None);
    assert_eq!(evens.find(&5), None);
}

#[test]
fn test_intersection() {
    let table1 = OrderedTableStPer::empty()
        .insert(1, "one_a".to_string())
        .insert(3, "three_a".to_string())
        .insert(5, "five_a".to_string())
        .insert(7, "seven_a".to_string());

    let table2 = OrderedTableStPer::empty()
        .insert(3, "three_b".to_string())
        .insert(4, "four_b".to_string())
        .insert(5, "five_b".to_string())
        .insert(6, "six_b".to_string());

    let intersection = table1.intersection(&table2, |v1, _v2| v1.clone());
    assert_eq!(intersection.size(), 2);
    assert_eq!(intersection.find(&3), Some("three_a".to_string()));
    assert_eq!(intersection.find(&5), Some("five_a".to_string()));
    assert_eq!(intersection.find(&1), None);
    assert_eq!(intersection.find(&4), None);
}

#[test]
fn test_union() {
    let table1 = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three_a".to_string())
        .insert(5, "five".to_string());

    let table2 = OrderedTableStPer::empty()
        .insert(3, "three_b".to_string())
        .insert(4, "four".to_string())
        .insert(6, "six".to_string());

    let union = table1.union(&table2, |v1, _v2| v1.clone());
    assert_eq!(union.size(), 5);
    assert_eq!(union.find(&1), Some("one".to_string()));
    assert_eq!(union.find(&3), Some("three_a".to_string())); // First value wins
    assert_eq!(union.find(&4), Some("four".to_string()));
    assert_eq!(union.find(&5), Some("five".to_string()));
    assert_eq!(union.find(&6), Some("six".to_string()));
}

#[test]
fn test_difference() {
    let table1 = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string())
        .insert(5, "five".to_string())
        .insert(7, "seven".to_string());

    let table2 = OrderedTableStPer::empty()
        .insert(3, "three_b".to_string())
        .insert(5, "five_b".to_string());

    let difference = table1.difference(&table2);
    assert_eq!(difference.size(), 2);
    assert_eq!(difference.find(&1), Some("one".to_string()));
    assert_eq!(difference.find(&7), Some("seven".to_string()));
    assert_eq!(difference.find(&3), None);
    assert_eq!(difference.find(&5), None);
}

#[test]
fn test_restrict() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(2, "two".to_string())
        .insert(3, "three".to_string())
        .insert(4, "four".to_string())
        .insert(5, "five".to_string());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    let restricted = table.restrict(&keys);
    assert_eq!(restricted.size(), 2);
    assert_eq!(restricted.find(&2), Some("two".to_string()));
    assert_eq!(restricted.find(&4), Some("four".to_string()));
    assert_eq!(restricted.find(&1), None);
    assert_eq!(restricted.find(&3), None);
    assert_eq!(restricted.find(&5), None);
}

#[test]
fn test_subtract() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(2, "two".to_string())
        .insert(3, "three".to_string())
        .insert(4, "four".to_string())
        .insert(5, "five".to_string());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    let subtracted = table.subtract(&keys);
    assert_eq!(subtracted.size(), 3);
    assert_eq!(subtracted.find(&1), Some("one".to_string()));
    assert_eq!(subtracted.find(&3), Some("three".to_string()));
    assert_eq!(subtracted.find(&5), Some("five".to_string()));
    assert_eq!(subtracted.find(&2), None);
    assert_eq!(subtracted.find(&4), None);
}

#[test]
fn test_persistence() {
    let original = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(2, "two".to_string())
        .insert(3, "three".to_string());

    let modified = original.insert(4, "four".to_string()).delete(&2);

    // Original unchanged
    assert_eq!(original.size(), 3);
    assert_eq!(original.find(&1), Some("one".to_string()));
    assert_eq!(original.find(&2), Some("two".to_string()));
    assert_eq!(original.find(&3), Some("three".to_string()));
    assert_eq!(original.find(&4), None);

    // Modified has changes
    assert_eq!(modified.size(), 3);
    assert_eq!(modified.find(&1), Some("one".to_string()));
    assert_eq!(modified.find(&2), None);
    assert_eq!(modified.find(&3), Some("three".to_string()));
    assert_eq!(modified.find(&4), Some("four".to_string()));
}

#[test]
fn test_ordered_table_st_per_lit_macro() {
    let table: OrderedTableStPer<i32, String> = OrderedTableStPerLit![
        1 => "one".to_string(),
        3 => "three".to_string(),
        5 => "five".to_string()
    ];
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));
    assert_eq!(table.find(&5), Some("five".to_string()));
    assert_eq!(table.find(&2), None);

    let empty_table: OrderedTableStPer<i32, String> = OrderedTableStPerLit![];
    assert_eq!(empty_table.size(), 0);
}

#[test]
fn test_string_key_ordering() {
    let table = OrderedTableStPer::empty()
        .insert("charlie".to_string(), 3)
        .insert("alice".to_string(), 1)
        .insert("bob".to_string(), 2);

    assert_eq!(table.first_key(), Some("alice".to_string()));
    assert_eq!(table.last_key(), Some("charlie".to_string()));
    assert_eq!(table.next_key(&"alice".to_string()), Some("bob".to_string()));
    assert_eq!(table.previous_key(&"charlie".to_string()), Some("bob".to_string()));
}

#[test]
fn test_collect() {
    let table = OrderedTableStPer::empty()
        .insert(3, "three".to_string())
        .insert(1, "one".to_string())
        .insert(2, "two".to_string());

    let collected = table.collect();
    assert_eq!(collected.length(), 3);

    // Should be in sorted order
    assert_eq!(collected.nth(0).0, 1);
    assert_eq!(collected.nth(1).0, 2);
    assert_eq!(collected.nth(2).0, 3);
}

#[test]
fn test_empty_operations() {
    let empty = OrderedTableStPer::<i32, String>::empty();

    assert_eq!(empty.size(), 0);
    assert_eq!(empty.find(&1), None);
    assert_eq!(empty.first_key(), None);
    assert_eq!(empty.last_key(), None);
    assert_eq!(empty.previous_key(&1), None);
    assert_eq!(empty.next_key(&1), None);
    assert_eq!(empty.select_key(0), None);

    let domain = empty.domain();
    assert_eq!(domain.size(), 0);

    let collected = empty.collect();
    assert_eq!(collected.length(), 0);
}

#[test]
fn test_insert_duplicate_key() {
    let table = OrderedTableStPer::empty()
        .insert(1, "first".to_string())
        .insert(2, "two".to_string())
        .insert(1, "second".to_string()); // Duplicate key

    // Should overwrite the first value
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&1), Some("second".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
}

#[test]
fn test_large_dataset() {
    let mut table = OrderedTableStPer::empty();

    for i in 0..100 {
        table = table.insert(i, format!("value_{i}"));
    }

    assert_eq!(table.size(), 100);
    assert_eq!(table.first_key(), Some(0));
    assert_eq!(table.last_key(), Some(99));
    assert_eq!(table.select_key(50), Some(50));
    assert_eq!(table.rank_key(&50), 50);

    // Test filter on large dataset
    let evens = table.filter(|k, _v| k % 2 == 0);
    assert_eq!(evens.size(), 50);
}

#[test]
fn test_delete_nonexistent() {
    let table = OrderedTableStPer::empty()
        .insert(1, "one".to_string())
        .insert(3, "three".to_string());

    let table2 = table.delete(&2);
    assert_eq!(table2.size(), 2);
}
