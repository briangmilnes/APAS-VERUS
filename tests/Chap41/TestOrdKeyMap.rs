//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
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

// domain, tabulate, restrict, subtract

fn make_set(vals: &[u64]) -> ArraySetStEph<u64> {
    let mut s = ArraySetStEph::empty();
    for &v in vals {
        s.insert(v);
    }
    s
}

#[test]
fn test_ordkeymap_domain_empty() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let dom = m.domain();
    assert_eq!(dom.size(), 0);
}

#[test]
fn test_ordkeymap_domain() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    let dom = m.domain();
    assert_eq!(dom.size(), 3);
    assert!(dom.find(&10));
    assert!(dom.find(&20));
    assert!(dom.find(&30));
    assert!(!dom.find(&99));
}

#[test]
fn test_ordkeymap_tabulate() {
    let keys = make_set(&[10, 20, 30]);
    let f = |k: &u64| -> u64 { *k * 10 };
    let table: OrdKeyMap<u64, u64> = OrdKeyMap::tabulate(&keys, &f);
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&10), Some(100));
    assert_eq!(table.find(&20), Some(200));
    assert_eq!(table.find(&30), Some(300));
    assert_eq!(table.find(&99), None);
}

#[test]
fn test_ordkeymap_tabulate_empty() {
    let keys: ArraySetStEph<u64> = ArraySetStEph::empty();
    let f = |k: &u64| -> u64 { *k };
    let table: OrdKeyMap<u64, u64> = OrdKeyMap::tabulate(&keys, &f);
    assert_eq!(table.size(), 0);
}

#[test]
fn test_ordkeymap_restrict() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4)]);
    let keep = make_set(&[10, 30, 50]);
    let restricted = m.restrict(&keep);
    assert_eq!(restricted.size(), 2);
    assert_eq!(restricted.find(&10), Some(1));
    assert_eq!(restricted.find(&30), Some(3));
    assert_eq!(restricted.find(&20), None);
    assert_eq!(restricted.find(&40), None);
}

#[test]
fn test_ordkeymap_restrict_empty_keys() {
    let m = make_map(&[(10, 1), (20, 2)]);
    let empty_keys: ArraySetStEph<u64> = ArraySetStEph::empty();
    let restricted = m.restrict(&empty_keys);
    assert_eq!(restricted.size(), 0);
}

#[test]
fn test_ordkeymap_restrict_empty_map() {
    let m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let keys = make_set(&[10, 20]);
    let restricted = m.restrict(&keys);
    assert_eq!(restricted.size(), 0);
}

#[test]
fn test_ordkeymap_subtract() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3), (40, 4)]);
    let remove = make_set(&[10, 30, 50]);
    let remaining = m.subtract(&remove);
    assert_eq!(remaining.size(), 2);
    assert_eq!(remaining.find(&20), Some(2));
    assert_eq!(remaining.find(&40), Some(4));
    assert_eq!(remaining.find(&10), None);
    assert_eq!(remaining.find(&30), None);
}

#[test]
fn test_ordkeymap_subtract_empty_keys() {
    let m = make_map(&[(10, 1), (20, 2)]);
    let empty_keys: ArraySetStEph<u64> = ArraySetStEph::empty();
    let remaining = m.subtract(&empty_keys);
    assert_eq!(remaining.size(), 2);
    assert_eq!(remaining.find(&10), Some(1));
    assert_eq!(remaining.find(&20), Some(2));
}

#[test]
fn test_ordkeymap_subtract_all() {
    let m = make_map(&[(10, 1), (20, 2)]);
    let all_keys = make_set(&[10, 20]);
    let remaining = m.subtract(&all_keys);
    assert_eq!(remaining.size(), 0);
}

#[test]
fn test_ordkeymap_domain_roundtrip() {
    let m = make_map(&[(10, 1), (20, 2), (30, 3)]);
    let dom = m.domain();
    let restricted = m.restrict(&dom);
    assert_eq!(restricted.size(), 3);
    assert_eq!(restricted.find(&10), Some(1));
    assert_eq!(restricted.find(&20), Some(2));
    assert_eq!(restricted.find(&30), Some(3));
}
