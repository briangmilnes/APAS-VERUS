// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

use apas_verus::Chap41::OrdKeyMap::OrdKeyMap::*;
use apas_verus::Types::Types::*;
use vstd::prelude::Ghost;

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

// first_key tests.

#[test]
fn test_ordkeymap_first_key_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.first_key(), None);
}

#[test]
fn test_ordkeymap_first_key_singleton() {
    let m = make_map(&[(42, 1)]);
    assert_eq!(m.first_key(), Some(42u64));
}

#[test]
fn test_ordkeymap_first_key_multiple() {
    let m = make_map(&[(30, 3), (10, 1), (20, 2)]);
    assert_eq!(m.first_key(), Some(10u64));
}

#[test]
fn test_ordkeymap_first_key_is_minimum() {
    let m = make_map(&[(100, 1), (50, 2), (200, 3), (1, 4), (75, 5)]);
    assert_eq!(m.first_key(), Some(1u64));
}

// last_key tests.

#[test]
fn test_ordkeymap_last_key_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    assert_eq!(m.last_key(), None);
}

#[test]
fn test_ordkeymap_last_key_singleton() {
    let m = make_map(&[(42, 1)]);
    assert_eq!(m.last_key(), Some(42u64));
}

#[test]
fn test_ordkeymap_last_key_multiple() {
    let m = make_map(&[(30, 3), (10, 1), (20, 2)]);
    assert_eq!(m.last_key(), Some(30u64));
}

#[test]
fn test_ordkeymap_last_key_is_maximum() {
    let m = make_map(&[(100, 1), (50, 2), (200, 3), (1, 4), (75, 5)]);
    assert_eq!(m.last_key(), Some(200u64));
}

#[test]
fn test_ordkeymap_first_last_key_singleton_equal() {
    let m = make_map(&[(77, 99)]);
    assert_eq!(m.first_key(), m.last_key());
    assert_eq!(m.first_key(), Some(77u64));
}

// get_key_range tests.

#[test]
fn test_ordkeymap_get_key_range_all_keys() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    let range = m.get_key_range(&5u64, &55u64);
    assert_eq!(range.size(), 5);
    assert_eq!(range.find(&10u64), Some(1u64));
    assert_eq!(range.find(&50u64), Some(5u64));
}

#[test]
fn test_ordkeymap_get_key_range_subset() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    let range = m.get_key_range(&15u64, &35u64);
    assert_eq!(range.find(&10u64), None);
    assert_eq!(range.find(&20u64), Some(2u64));
    assert_eq!(range.find(&30u64), Some(3u64));
    assert_eq!(range.find(&40u64), None);
    assert_eq!(range.find(&50u64), None);
}

#[test]
fn test_ordkeymap_get_key_range_no_keys_in_bounds() {
    let m = make_map(&[(10, 1), (50, 5)]);
    let range = m.get_key_range(&20u64, &40u64);
    assert_eq!(range.size(), 0);
    assert!(range.is_empty());
}

#[test]
fn test_ordkeymap_get_key_range_exact_boundaries() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    let range = m.get_key_range(&10u64, &30u64);
    assert_eq!(range.find(&10u64), Some(1u64));
    assert_eq!(range.find(&20u64), Some(2u64));
    assert_eq!(range.find(&30u64), Some(3u64));
}

#[test]
fn test_ordkeymap_get_key_range_empty_map() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let range = m.get_key_range(&0u64, &100u64);
    assert!(range.is_empty());
}

#[test]
fn test_ordkeymap_get_key_range_values_preserved() {
    let m = make_map(&[(10, 111), (20, 222), (30, 333)]);
    let range = m.get_key_range(&10u64, &20u64);
    assert_eq!(range.find(&10u64), Some(111u64));
    assert_eq!(range.find(&20u64), Some(222u64));
    assert_eq!(range.find(&30u64), None);
}

// split_rank_key tests.

#[test]
fn test_ordkeymap_split_rank_key_at_zero() {
    let mut m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    let (left, right) = m.split_rank_key(0);
    assert_eq!(left.size(), 0);
    assert!(left.is_empty());
    assert_eq!(right.size(), 3);
    assert_eq!(right.find(&10u64), Some(1u64));
    assert_eq!(right.find(&20u64), Some(2u64));
    assert_eq!(right.find(&30u64), Some(3u64));
}

#[test]
fn test_ordkeymap_split_rank_key_at_size() {
    let mut m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    let (left, right) = m.split_rank_key(3);
    assert_eq!(left.size(), 3);
    assert_eq!(right.size(), 0);
    assert!(right.is_empty());
    assert_eq!(left.find(&10u64), Some(1u64));
    assert_eq!(left.find(&20u64), Some(2u64));
    assert_eq!(left.find(&30u64), Some(3u64));
}

#[test]
fn test_ordkeymap_split_rank_key_at_middle() {
    let mut m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    let (left, right) = m.split_rank_key(2);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 3);
    assert_eq!(left.find(&10u64), Some(1u64));
    assert_eq!(left.find(&20u64), Some(2u64));
    assert_eq!(left.find(&30u64), None);
    assert_eq!(right.find(&30u64), Some(3u64));
    assert_eq!(right.find(&40u64), Some(4u64));
    assert_eq!(right.find(&50u64), Some(5u64));
}

#[test]
fn test_ordkeymap_split_rank_key_sizes_sum() {
    let mut m = make_map(&[(5, 50), (15, 150), (25, 250), (35, 350), (45, 450)]);
    let original_size = 5;
    let split_point = 3;
    let (left, right) = m.split_rank_key(split_point);
    assert_eq!(left.size() + right.size(), original_size);
    assert_eq!(left.size(), split_point);
    assert_eq!(right.size(), original_size - split_point);
}

#[test]
fn test_ordkeymap_split_rank_key_disjoint_doms() {
    let mut m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4)]);
    let (left, right) = m.split_rank_key(2);
    // Keys in left must not appear in right.
    if let Some(k) = left.first_key() {
        assert_eq!(right.find(&k), None);
    }
    if let Some(k) = left.last_key() {
        assert_eq!(right.find(&k), None);
    }
    // Keys in right must not appear in left.
    if let Some(k) = right.first_key() {
        assert_eq!(left.find(&k), None);
    }
    if let Some(k) = right.last_key() {
        assert_eq!(left.find(&k), None);
    }
}

#[test]
fn test_ordkeymap_split_rank_key_ordering() {
    let mut m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4), (50, 5)]);
    let (left, right) = m.split_rank_key(3);
    // All keys in left must be less than all keys in right.
    if let (Some(left_max), Some(right_min)) = (left.last_key(), right.first_key()) {
        assert!(left_max < right_min);
    }
}

#[test]
fn test_ordkeymap_split_rank_key_singleton_left() {
    let mut m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    let (left, right) = m.split_rank_key(1);
    assert_eq!(left.size(), 1);
    assert_eq!(left.first_key(), Some(10u64));
    assert_eq!(right.size(), 2);
}

// Stress tests.

#[test]
fn test_ordkeymap_stress_200_entries() {
    let mut m = OrdKeyMap::new();
    // Insert 200 entries.
    for i in 0u64..200 {
        m.insert(i * 2, i * 10); // even keys 0,2,...,398
    }
    assert_eq!(m.size(), 200);
    // Verify all present.
    for i in 0u64..200 {
        assert_eq!(m.find(&(i * 2)), Some(i * 10));
    }
    // Verify odd keys absent.
    for i in 0u64..10 {
        assert_eq!(m.find(&(i * 2 + 1)), None);
    }
    // Check first and last.
    assert_eq!(m.first_key(), Some(0u64));
    assert_eq!(m.last_key(), Some(398u64));
    // rank/select roundtrip for a few indices.
    for i in 0usize..10 {
        let k = m.select_key(i).unwrap();
        assert_eq!(m.rank_key(&k), i);
        assert_eq!(k, (i as u64) * 2);
    }
    // get_key_range for middle 100 keys.
    let range = m.get_key_range(&100u64, &298u64);
    assert_eq!(range.find(&100u64), Some(500u64));
    assert_eq!(range.find(&298u64), Some(1490u64));
    assert_eq!(range.find(&98u64), None);
    assert_eq!(range.find(&300u64), None);
}

#[test]
fn test_ordkeymap_stress_insert_delete_find() {
    let mut m = OrdKeyMap::new();
    // Insert 100 entries.
    for i in 0u64..100 {
        m.insert(i, i * 5);
    }
    assert_eq!(m.size(), 100);
    // Delete every even key.
    for i in (0u64..100).step_by(2) {
        m.delete(&i);
    }
    assert_eq!(m.size(), 50);
    // Odd keys remain, even keys gone.
    for i in 0u64..100 {
        if i % 2 == 0 {
            assert_eq!(m.find(&i), None);
        } else {
            assert_eq!(m.find(&i), Some(i * 5));
        }
    }
    // first_key = 1, last_key = 99.
    assert_eq!(m.first_key(), Some(1u64));
    assert_eq!(m.last_key(), Some(99u64));
}

#[test]
fn test_ordkeymap_stress_union_intersect() {
    let mut pairs_a = vec![];
    let mut pairs_b = vec![];
    for i in 0u64..50 {
        pairs_a.push((i, i));
    }
    for i in 25u64..75 {
        pairs_b.push((i, i + 100));
    }
    let a = make_map(&pairs_a);
    let b = make_map(&pairs_b);
    // Union: left wins on collision.
    let u = a.union_with(&b, &|v1: &u64, _v2: &u64| *v1);
    assert_eq!(u.size(), 75);
    assert_eq!(u.find(&0u64), Some(0u64));    // only in a
    assert_eq!(u.find(&74u64), Some(174u64)); // only in b
    assert_eq!(u.find(&25u64), Some(25u64)); // collision, left wins
    // Intersection: right wins on collision.
    let inter = a.intersect_with(&b, &|_v1: &u64, v2: &u64| *v2);
    assert_eq!(inter.size(), 25); // keys 25..49
    assert_eq!(inter.find(&24u64), None); // only in a
    assert_eq!(inter.find(&50u64), None); // only in b
    assert_eq!(inter.find(&30u64), Some(130u64)); // in both, right wins
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

// collect tests.

#[test]
fn test_ordkeymap_collect_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let entries = m.collect();
    assert!(entries.is_empty());
}

#[test]
fn test_ordkeymap_collect_singleton() {
    let m = make_map(&[(42, 99)]);
    let entries = m.collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, 42u64);
    assert_eq!(entries[0].1, 99u64);
}

#[test]
fn test_ordkeymap_collect_sorted_order() {
    // Insert out of order; collect must return ascending key order.
    let m = make_map(&[(30, 3), (10, 1), (50, 5), (20, 2), (40, 4)]);
    let entries = m.collect();
    assert_eq!(entries.len(), 5);
    let keys: Vec<u64> = entries.iter().map(|p| p.0).collect();
    assert_eq!(keys, vec![10, 20, 30, 40, 50]);
    for e in &entries {
        assert_eq!(m.find(&e.0), Some(e.1));
    }
}

#[test]
fn test_ordkeymap_collect_values_correct() {
    let m = make_map(&[(1, 100), (2, 200), (3, 300)]);
    let entries = m.collect();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0], Pair(1u64, 100u64));
    assert_eq!(entries[1], Pair(2u64, 200u64));
    assert_eq!(entries[2], Pair(3u64, 300u64));
}

// filter tests.

#[test]
fn test_ordkeymap_filter_keep_some() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]);
    // Keep only entries with even keys.
    let filtered = m.filter(|k: &u64, _v: &u64| *k % 2 == 0, Ghost::assume_new());
    assert_eq!(filtered.size(), 2);
    assert_eq!(filtered.find(&2u64), Some(20u64));
    assert_eq!(filtered.find(&4u64), Some(40u64));
    assert_eq!(filtered.find(&1u64), None);
    assert_eq!(filtered.find(&3u64), None);
    assert_eq!(filtered.find(&5u64), None);
}

#[test]
fn test_ordkeymap_filter_keep_none() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30)]);
    // Keep nothing.
    let filtered = m.filter(|_k: &u64, _v: &u64| false, Ghost::assume_new());
    assert!(filtered.is_empty());
}

#[test]
fn test_ordkeymap_filter_keep_all() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30)]);
    // Keep everything.
    let filtered = m.filter(|_k: &u64, _v: &u64| true, Ghost::assume_new());
    assert_eq!(filtered.size(), 3);
    assert_eq!(filtered.find(&1u64), Some(10u64));
    assert_eq!(filtered.find(&2u64), Some(20u64));
    assert_eq!(filtered.find(&3u64), Some(30u64));
}

#[test]
fn test_ordkeymap_filter_by_value() {
    let m = make_map(&[(1, 5), (2, 15), (3, 25), (4, 35)]);
    // Keep only entries with value > 20.
    let filtered = m.filter(|_k: &u64, v: &u64| *v > 20, Ghost::assume_new());
    assert_eq!(filtered.size(), 2);
    assert_eq!(filtered.find(&3u64), Some(25u64));
    assert_eq!(filtered.find(&4u64), Some(35u64));
    assert_eq!(filtered.find(&1u64), None);
    assert_eq!(filtered.find(&2u64), None);
}

#[test]
fn test_ordkeymap_filter_empty_map() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let filtered = m.filter(|_k: &u64, _v: &u64| true, Ghost::assume_new());
    assert!(filtered.is_empty());
}

// map_values tests.

#[test]
fn test_ordkeymap_map_values_double() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let mapped = m.map_values(|_k: &u64, v: &u64| *v * 2);
    assert_eq!(mapped.size(), 3);
    assert_eq!(mapped.find(&1u64), Some(20u64));
    assert_eq!(mapped.find(&2u64), Some(40u64));
    assert_eq!(mapped.find(&3u64), Some(60u64));
}

#[test]
fn test_ordkeymap_map_values_keys_unchanged() {
    let m = make_map(&[(5, 50), (10, 100), (15, 150)]);
    let mapped = m.map_values(|_k: &u64, v: &u64| *v + 1);
    assert_eq!(mapped.size(), 3);
    // Keys must be exactly the same set.
    assert_eq!(mapped.first_key(), Some(5u64));
    assert_eq!(mapped.last_key(), Some(15u64));
    assert_eq!(mapped.find(&5u64), Some(51u64));
    assert_eq!(mapped.find(&10u64), Some(101u64));
    assert_eq!(mapped.find(&15u64), Some(151u64));
    // Original map unchanged.
    assert_eq!(m.find(&5u64), Some(50u64));
}

#[test]
fn test_ordkeymap_map_values_key_dependent() {
    // Value becomes key + old_value.
    let m = make_map(&[(1, 100), (2, 200), (3, 300)]);
    let mapped = m.map_values(|k: &u64, v: &u64| *k + *v);
    assert_eq!(mapped.find(&1u64), Some(101u64));
    assert_eq!(mapped.find(&2u64), Some(202u64));
    assert_eq!(mapped.find(&3u64), Some(303u64));
}

#[test]
fn test_ordkeymap_map_values_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let mapped = m.map_values(|_k: &u64, v: &u64| *v * 10);
    assert!(mapped.is_empty());
}

// reduce tests.

#[test]
fn test_ordkeymap_reduce_sum() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40)]);
    let total = m.reduce(|acc: &u64, v: &u64| acc + v, &0u64);
    assert_eq!(total, 100u64);
}

#[test]
fn test_ordkeymap_reduce_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let result = m.reduce(|acc: &u64, v: &u64| acc + v, &0u64);
    assert_eq!(result, 0u64);
}

#[test]
fn test_ordkeymap_reduce_max() {
    let m = make_map(&[(1, 5), (2, 3), (3, 8), (4, 1), (5, 6)]);
    let max = m.reduce(|acc: &u64, v: &u64| if *v > *acc { *v } else { *acc }, &0u64);
    assert_eq!(max, 8u64);
}

#[test]
fn test_ordkeymap_reduce_singleton() {
    let m = make_map(&[(42, 99)]);
    let result = m.reduce(|acc: &u64, v: &u64| acc + v, &0u64);
    assert_eq!(result, 99u64);
}

// Clone tests.

#[test]
fn test_ordkeymap_clone_equals_original() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let c = m.clone();
    assert_eq!(c.size(), m.size());
    assert_eq!(c.find(&1u64), m.find(&1u64));
    assert_eq!(c.find(&2u64), m.find(&2u64));
    assert_eq!(c.find(&3u64), m.find(&3u64));
    assert_eq!(c.first_key(), m.first_key());
    assert_eq!(c.last_key(), m.last_key());
}

#[test]
fn test_ordkeymap_clone_modify_clone_no_effect_on_original() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let mut c = m.clone();
    c.insert(4u64, 40u64);
    c.delete(&1u64);
    // Original unchanged.
    assert_eq!(m.size(), 3);
    assert_eq!(m.find(&1u64), Some(10u64));
    assert_eq!(m.find(&4u64), None);
    // Clone has the changes.
    assert_eq!(c.size(), 3);
    assert_eq!(c.find(&4u64), Some(40u64));
    assert_eq!(c.find(&1u64), None);
}

#[test]
fn test_ordkeymap_clone_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let c = m.clone();
    assert!(c.is_empty());
}

// Integration tests.

#[test]
fn test_ordkeymap_collect_after_filter() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]);
    let filtered = m.filter(|k: &u64, _v: &u64| *k % 2 != 0, Ghost::assume_new());
    let entries = filtered.collect();
    // Odd keys: 1, 3, 5.
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].0, 1u64);
    assert_eq!(entries[1].0, 3u64);
    assert_eq!(entries[2].0, 5u64);
}

#[test]
fn test_ordkeymap_clone_filter_map_values_independent() {
    let m = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40)]);
    let c = m.clone();
    // Filter even keys from clone, double values.
    let filtered = c.filter(|k: &u64, _v: &u64| *k % 2 == 0, Ghost::assume_new());
    let mapped = filtered.map_values(|_k: &u64, v: &u64| *v * 2);
    // Verify original untouched.
    assert_eq!(m.size(), 4);
    assert_eq!(m.find(&1u64), Some(10u64));
    // Verify derived maps correct.
    assert_eq!(filtered.size(), 2);
    assert_eq!(mapped.find(&2u64), Some(40u64));
    assert_eq!(mapped.find(&4u64), Some(80u64));
    assert_eq!(mapped.find(&1u64), None);
}

#[test]
fn test_ordkeymap_reduce_after_map_values() {
    let m = make_map(&[(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]);
    // Square each value, then sum.
    let squared = m.map_values(|_k: &u64, v: &u64| *v * *v);
    let total = squared.reduce(|acc: &u64, v: &u64| acc + v, &0u64);
    assert_eq!(total, 55u64); // 1 + 4 + 9 + 16 + 25.
}

#[test]
fn test_ordkeymap_collect_roundtrip() {
    // Build from collect output using make_map logic.
    let m = make_map(&[(3, 30), (1, 10), (2, 20)]);
    let entries = m.collect();
    let mut m2 = OrdKeyMap::new();
    for e in &entries {
        m2.insert(e.0, e.1);
    }
    assert_eq!(m2.size(), m.size());
    assert_eq!(m2.find(&1u64), m.find(&1u64));
    assert_eq!(m2.find(&2u64), m.find(&2u64));
    assert_eq!(m2.find(&3u64), m.find(&3u64));
}
