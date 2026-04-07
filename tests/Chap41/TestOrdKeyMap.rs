//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap41::OrdKeyMap::OrdKeyMap::*;
use apas_verus::Types::Types::*;

fn make_map(pairs: &[(u64, u64)]) -> OrdKeyMap<u64, u64> {
    let mut m = OrdKeyMap::new();
    for &(k, v) in pairs {
        m.insert(k, v);
    }
    m
}

#[test]
fn test_ordkeymap_new_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.size(), 0);
    assert!(m.is_empty());
}

#[test]
fn test_ordkeymap_insert_find() {
    let mut m = OrdKeyMap::new();
    m.insert(10u64, 100u64);
    m.insert(20u64, 200u64);
    m.insert(5u64, 50u64);
    assert_eq!(m.size(), 3);
    assert_eq!(m.find(&10u64), Some(100u64));
    assert_eq!(m.find(&20u64), Some(200u64));
    assert_eq!(m.find(&5u64), Some(50u64));
    assert_eq!(m.find(&99u64), None);
}

#[test]
fn test_ordkeymap_insert_overwrite() {
    let mut m = OrdKeyMap::new();
    m.insert(10u64, 100u64);
    assert_eq!(m.find(&10u64), Some(100u64));
    m.insert(10u64, 999u64);
    assert_eq!(m.find(&10u64), Some(999u64));
    assert_eq!(m.size(), 1);
}

#[test]
fn test_ordkeymap_delete() {
    let mut m = make_map(&[(1, 10), (2, 20), (3, 30)]);
    assert_eq!(m.size(), 3);
    m.delete(&2u64);
    assert_eq!(m.size(), 2);
    assert_eq!(m.find(&2u64), None);
    assert_eq!(m.find(&1u64), Some(10u64));
    assert_eq!(m.find(&3u64), Some(30u64));
}

#[test]
fn test_ordkeymap_delete_nonexistent() {
    let mut m = make_map(&[(1, 10)]);
    m.delete(&99u64);
    assert_eq!(m.size(), 1);
}

#[test]
fn test_ordkeymap_split() {
    let m = make_map(&[(1, 10), (3, 30), (5, 50), (7, 70), (9, 90)]);
    let (left, mid, right) = m.split(&5u64);
    assert_eq!(mid, Some(50u64));
    assert_eq!(left.find(&1u64), Some(10u64));
    assert_eq!(left.find(&3u64), Some(30u64));
    assert_eq!(left.find(&5u64), None);
    assert_eq!(right.find(&7u64), Some(70u64));
    assert_eq!(right.find(&9u64), Some(90u64));
    assert_eq!(right.find(&5u64), None);
}

#[test]
fn test_ordkeymap_split_missing_key() {
    let m = make_map(&[(1, 10), (3, 30), (5, 50)]);
    let (left, mid, right) = m.split(&4u64);
    assert!(mid.is_none());
    assert_eq!(left.find(&1u64), Some(10u64));
    assert_eq!(left.find(&3u64), Some(30u64));
    assert_eq!(right.find(&5u64), Some(50u64));
}

// next_key tests.

#[test]
fn test_ordkeymap_next_key_basic() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    assert_eq!(m.next_key(&10u64), Some(20u64));
    assert_eq!(m.next_key(&20u64), Some(30u64));
    assert_eq!(m.next_key(&40u64), Some(50u64));
}

#[test]
fn test_ordkeymap_next_key_last_element() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    assert_eq!(m.next_key(&30u64), None);
}

#[test]
fn test_ordkeymap_next_key_before_all() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    assert_eq!(m.next_key(&5u64), Some(10u64));
}

#[test]
fn test_ordkeymap_next_key_between_keys() {
    let m = make_map(&[(10, 1), (30, 3), (50, 5)]);
    assert_eq!(m.next_key(&15u64), Some(30u64));
    assert_eq!(m.next_key(&35u64), Some(50u64));
}

#[test]
fn test_ordkeymap_next_key_after_all() {
    let m = make_map(&[(10, 1), (20, 2)]);
    assert_eq!(m.next_key(&99u64), None);
}

#[test]
fn test_ordkeymap_next_key_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.next_key(&10u64), None);
}

// prev_key tests.

#[test]
fn test_ordkeymap_prev_key_basic() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    assert_eq!(m.prev_key(&50u64), Some(40u64));
    assert_eq!(m.prev_key(&30u64), Some(20u64));
    assert_eq!(m.prev_key(&20u64), Some(10u64));
}

#[test]
fn test_ordkeymap_prev_key_first_element() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    assert_eq!(m.prev_key(&10u64), None);
}

#[test]
fn test_ordkeymap_prev_key_after_all() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    assert_eq!(m.prev_key(&99u64), Some(30u64));
}

#[test]
fn test_ordkeymap_prev_key_between_keys() {
    let m = make_map(&[(10, 1), (30, 3), (50, 5)]);
    assert_eq!(m.prev_key(&25u64), Some(10u64));
    assert_eq!(m.prev_key(&45u64), Some(30u64));
}

#[test]
fn test_ordkeymap_prev_key_before_all() {
    let m = make_map(&[(10, 1), (20, 2)]);
    assert_eq!(m.prev_key(&5u64), None);
}

#[test]
fn test_ordkeymap_prev_key_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.prev_key(&10u64), None);
}

// rank_key tests.

#[test]
fn test_ordkeymap_rank_key_basic() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    assert_eq!(m.rank_key(&10u64), 0);
    assert_eq!(m.rank_key(&20u64), 1);
    assert_eq!(m.rank_key(&30u64), 2);
    assert_eq!(m.rank_key(&40u64), 3);
    assert_eq!(m.rank_key(&50u64), 4);
}

#[test]
fn test_ordkeymap_rank_key_missing() {
    let m = make_map(&[(10, 1), (30, 3), (50, 5)]);
    assert_eq!(m.rank_key(&5u64), 0);
    assert_eq!(m.rank_key(&15u64), 1);
    assert_eq!(m.rank_key(&35u64), 2);
    assert_eq!(m.rank_key(&99u64), 3);
}

#[test]
fn test_ordkeymap_rank_key_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.rank_key(&10u64), 0);
}

// select_key tests.

#[test]
fn test_ordkeymap_select_key_basic() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    assert_eq!(m.select_key(0), Some(10u64));
    assert_eq!(m.select_key(1), Some(20u64));
    assert_eq!(m.select_key(2), Some(30u64));
    assert_eq!(m.select_key(3), Some(40u64));
    assert_eq!(m.select_key(4), Some(50u64));
}

#[test]
fn test_ordkeymap_select_key_out_of_range() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    assert_eq!(m.select_key(3), None);
    assert_eq!(m.select_key(100), None);
}

#[test]
fn test_ordkeymap_select_key_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.select_key(0), None);
}

// Round-trip: rank(select(i)) == i.

#[test]
fn test_ordkeymap_rank_select_roundtrip() {
    let m = make_map(&[(5, 50), (15, 150), (25, 250), (35, 350), (45, 450)]);
    for i in 0..5 {
        let k = m.select_key(i).unwrap();
        assert_eq!(m.rank_key(&k), i);
    }
}

// union_with tests.

#[test]
fn test_ordkeymap_union_with_disjoint() {
    let a = make_map(&[(1, 10), (3, 30)]);
    let b = make_map(&[(2, 20), (4, 40)]);
    let c = a.union_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(c.size(), 4);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&2u64), Some(20u64));
    assert_eq!(c.find(&3u64), Some(30u64));
    assert_eq!(c.find(&4u64), Some(40u64));
}

#[test]
fn test_ordkeymap_union_with_overlap_sum() {
    let a = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let b = make_map(&[(2, 200), (3, 300), (4, 400)]);
    let c = a.union_with(&b, &|v1: &u64, v2: &u64| *v1 + *v2);
    assert_eq!(c.size(), 4);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&2u64), Some(220u64));
    assert_eq!(c.find(&3u64), Some(330u64));
    assert_eq!(c.find(&4u64), Some(400u64));
}

#[test]
fn test_ordkeymap_union_with_overlap_left_wins() {
    let a = make_map(&[(1, 10), (2, 20)]);
    let b = make_map(&[(2, 200), (3, 300)]);
    let c = a.union_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(c.find(&2u64), Some(20u64));
}

#[test]
fn test_ordkeymap_union_with_overlap_right_wins() {
    let a = make_map(&[(1, 10), (2, 20)]);
    let b = make_map(&[(2, 200), (3, 300)]);
    let c = a.union_with(&b, &|_v1: &u64, v2: &u64| *v2);
    assert_eq!(c.find(&2u64), Some(200u64));
}

#[test]
fn test_ordkeymap_union_with_empty_left() {
    let a: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let b = make_map(&[(1, 10)]);
    let c = a.union_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(c.size(), 1);
    assert_eq!(c.find(&1u64), Some(10u64));
}

#[test]
fn test_ordkeymap_union_with_empty_right() {
    let a = make_map(&[(1, 10)]);
    let b: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let c = a.union_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(c.size(), 1);
    assert_eq!(c.find(&1u64), Some(10u64));
}

// intersect_with tests.

#[test]
fn test_ordkeymap_intersect_with_overlap_sum() {
    let a = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let b = make_map(&[(2, 200), (3, 300), (4, 400)]);
    let c = a.intersect_with(&b, &|v1: &u64, v2: &u64| *v1 + *v2);
    assert_eq!(c.size(), 2);
    assert_eq!(c.find(&2u64), Some(220u64));
    assert_eq!(c.find(&3u64), Some(330u64));
    assert_eq!(c.find(&1u64), None);
    assert_eq!(c.find(&4u64), None);
}

#[test]
fn test_ordkeymap_intersect_with_left_wins() {
    let a = make_map(&[(1, 10), (2, 20)]);
    let b = make_map(&[(2, 200), (3, 300)]);
    let c = a.intersect_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(c.size(), 1);
    assert_eq!(c.find(&2u64), Some(20u64));
}

#[test]
fn test_ordkeymap_intersect_with_disjoint() {
    let a = make_map(&[(1, 10), (3, 30)]);
    let b = make_map(&[(2, 20), (4, 40)]);
    let c = a.intersect_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(c.size(), 0);
    assert!(c.is_empty());
}

#[test]
fn test_ordkeymap_intersect_with_empty() {
    let a = make_map(&[(1, 10)]);
    let b: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let c = a.intersect_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert!(c.is_empty());
}

// Comprehensive next/prev walkthrough.

#[test]
fn test_ordkeymap_next_prev_walk() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    // Walk forward.
    let mut keys_fwd = vec![];
    let mut cur = m.next_key(&0u64);
    while let Some(k) = cur {
        keys_fwd.push(k);
        cur = m.next_key(&k);
    }
    assert_eq!(keys_fwd, vec![10, 20, 30, 40, 50]);
    // Walk backward.
    let mut keys_bwd = vec![];
    cur = m.prev_key(&u64::MAX);
    while let Some(k) = cur {
        keys_bwd.push(k);
        cur = m.prev_key(&k);
    }
    keys_bwd.reverse();
    assert_eq!(keys_bwd, vec![10, 20, 30, 40, 50]);
}
