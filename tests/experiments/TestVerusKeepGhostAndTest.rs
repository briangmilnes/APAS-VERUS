//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Run Time Test: verify that exec code from verus! compiles and runs under cargo test,
//! with ghost imports erased by #[cfg(verus_keep_ghost)].

use apas_verus::experiments::verus_keep_ghost_and_test::verus_keep_ghost_and_test::*;

#[test]
fn test_empty_bag() {
    let bag = Bag::<i32>::empty();
    assert_eq!(bag.len(), 0);
}

#[test]
fn test_push_and_len() {
    let mut bag = Bag::<i32>::empty();
    bag.push(10);
    bag.push(20);
    bag.push(30);
    assert_eq!(bag.len(), 3);
}

#[test]
fn test_sum_empty() {
    let bag = Bag::<i32>::empty();
    assert_eq!(Bag::sum(&bag), 0);
}

#[test]
fn test_sum_values() {
    let mut bag = Bag::<i32>::empty();
    bag.push(1);
    bag.push(2);
    bag.push(3);
    bag.push(4);
    assert_eq!(Bag::sum(&bag), 10);
}

#[test]
fn test_sum_negative() {
    let mut bag = Bag::<i32>::empty();
    bag.push(-5);
    bag.push(10);
    bag.push(-3);
    assert_eq!(Bag::sum(&bag), 2);
}
