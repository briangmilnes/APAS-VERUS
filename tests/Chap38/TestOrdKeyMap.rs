//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap38::OrdKeyMap::OrdKeyMap::*;
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

// union tests.

#[test]
fn test_ordkeymap_union_disjoint() {
    let a = make_map(&[(1, 10), (3, 30), (5, 50)]);
    let b = make_map(&[(2, 20), (4, 40), (6, 60)]);
    let c = a.union(&b);
    assert_eq!(c.size(), 6);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&2u64), Some(20u64));
    assert_eq!(c.find(&3u64), Some(30u64));
    assert_eq!(c.find(&4u64), Some(40u64));
    assert_eq!(c.find(&5u64), Some(50u64));
    assert_eq!(c.find(&6u64), Some(60u64));
}

#[test]
fn test_ordkeymap_union_overlapping_other_wins() {
    // Key 3 in both; other's value (300) should win.
    let a = make_map(&[(1, 10), (3, 30), (5, 50)]);
    let b = make_map(&[(3, 300), (7, 70)]);
    let c = a.union(&b);
    assert_eq!(c.size(), 4);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&3u64), Some(300u64));
    assert_eq!(c.find(&5u64), Some(50u64));
    assert_eq!(c.find(&7u64), Some(70u64));
}

#[test]
fn test_ordkeymap_union_with_empty() {
    let a = make_map(&[(1, 10), (2, 20)]);
    let empty: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    // union with empty right.
    let c1 = a.union(&empty);
    assert_eq!(c1.size(), 2);
    assert_eq!(c1.find(&1u64), Some(10u64));
    assert_eq!(c1.find(&2u64), Some(20u64));
    // union with empty left.
    let c2 = empty.union(&a);
    assert_eq!(c2.size(), 2);
    assert_eq!(c2.find(&1u64), Some(10u64));
    assert_eq!(c2.find(&2u64), Some(20u64));
}

#[test]
fn test_ordkeymap_union_identical() {
    // Same keys in both; values from other.
    let a = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let b = make_map(&[(1, 11), (2, 22), (3, 33)]);
    let c = a.union(&b);
    assert_eq!(c.size(), 3);
    assert_eq!(c.find(&1u64), Some(11u64));
    assert_eq!(c.find(&2u64), Some(22u64));
    assert_eq!(c.find(&3u64), Some(33u64));
}

#[test]
fn test_ordkeymap_union_size_correctness() {
    // 5 keys in a, 5 keys in b, 2 shared: result has 8 keys.
    let a = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]);
    let b = make_map(&[(3, 300), (5, 500), (6, 60), (7, 70), (8, 80)]);
    let c = a.union(&b);
    assert_eq!(c.size(), 8);
}

// intersect tests.

#[test]
fn test_ordkeymap_intersect_disjoint() {
    let a = make_map(&[(1, 10), (3, 30), (5, 50)]);
    let b = make_map(&[(2, 20), (4, 40), (6, 60)]);
    let c = a.intersect(&b);
    assert_eq!(c.size(), 0);
    assert!(c.is_empty());
}

#[test]
fn test_ordkeymap_intersect_overlapping() {
    let a = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40)]);
    let b = make_map(&[(2, 200), (4, 400), (6, 600)]);
    let c = a.intersect(&b);
    assert_eq!(c.size(), 2);
    assert_eq!(c.find(&2u64), Some(20u64));
    assert_eq!(c.find(&4u64), Some(40u64));
    assert_eq!(c.find(&1u64), None);
    assert_eq!(c.find(&6u64), None);
}

#[test]
fn test_ordkeymap_intersect_with_empty() {
    let a = make_map(&[(1, 10), (2, 20)]);
    let empty: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let c1 = a.intersect(&empty);
    assert_eq!(c1.size(), 0);
    let c2 = empty.intersect(&a);
    assert_eq!(c2.size(), 0);
}

#[test]
fn test_ordkeymap_intersect_preserves_self_values() {
    // Keys 2 and 4 are in both; values come from self (a).
    let a = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]);
    let b = make_map(&[(2, 999), (4, 888)]);
    let c = a.intersect(&b);
    assert_eq!(c.size(), 2);
    // Self's values, not other's.
    assert_eq!(c.find(&2u64), Some(20u64));
    assert_eq!(c.find(&4u64), Some(40u64));
}

// difference tests.

#[test]
fn test_ordkeymap_difference_disjoint() {
    // No shared keys — difference equals self.
    let a = make_map(&[(1, 10), (3, 30), (5, 50)]);
    let b = make_map(&[(2, 20), (4, 40), (6, 60)]);
    let c = a.difference(&b);
    assert_eq!(c.size(), 3);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&3u64), Some(30u64));
    assert_eq!(c.find(&5u64), Some(50u64));
}

#[test]
fn test_ordkeymap_difference_overlapping() {
    let a = make_map(&[(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]);
    let b = make_map(&[(2, 200), (4, 400)]);
    let c = a.difference(&b);
    assert_eq!(c.size(), 3);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&2u64), None);
    assert_eq!(c.find(&3u64), Some(30u64));
    assert_eq!(c.find(&4u64), None);
    assert_eq!(c.find(&5u64), Some(50u64));
}

#[test]
fn test_ordkeymap_difference_from_empty() {
    // Subtracting empty changes nothing.
    let a = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let empty: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let c = a.difference(&empty);
    assert_eq!(c.size(), 3);
    assert_eq!(c.find(&1u64), Some(10u64));
    assert_eq!(c.find(&2u64), Some(20u64));
    assert_eq!(c.find(&3u64), Some(30u64));
}

#[test]
fn test_ordkeymap_difference_identical() {
    // Subtracting self leaves nothing.
    let a = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let b = make_map(&[(1, 10), (2, 20), (3, 30)]);
    let c = a.difference(&b);
    assert_eq!(c.size(), 0);
    assert!(c.is_empty());
}

// additional split tests.

#[test]
fn test_ordkeymap_split_at_min() {
    let m = make_map(&[(1, 10), (3, 30), (5, 50), (7, 70)]);
    let (left, mid, right) = m.split(&1u64);
    assert_eq!(mid, Some(10u64));
    // Left of min is empty.
    assert_eq!(left.size(), 0);
    // Right has all remaining keys.
    assert_eq!(right.size(), 3);
    assert_eq!(right.find(&3u64), Some(30u64));
    assert_eq!(right.find(&5u64), Some(50u64));
    assert_eq!(right.find(&7u64), Some(70u64));
}

#[test]
fn test_ordkeymap_split_at_max() {
    let m = make_map(&[(1, 10), (3, 30), (5, 50), (7, 70)]);
    let (left, mid, right) = m.split(&7u64);
    assert_eq!(mid, Some(70u64));
    // Left has all keys below max.
    assert_eq!(left.size(), 3);
    assert_eq!(left.find(&1u64), Some(10u64));
    assert_eq!(left.find(&3u64), Some(30u64));
    assert_eq!(left.find(&5u64), Some(50u64));
    // Right of max is empty.
    assert_eq!(right.size(), 0);
}

#[test]
fn test_ordkeymap_split_partition_correctness() {
    // No key leaks between left, mid, and right.
    let m = make_map(&[(2, 20), (5, 50), (8, 80), (11, 110), (14, 140)]);
    let (left, mid, right) = m.split(&8u64);
    assert_eq!(mid, Some(80u64));
    // Left has only keys strictly less than 8.
    assert_eq!(left.find(&2u64), Some(20u64));
    assert_eq!(left.find(&5u64), Some(50u64));
    assert_eq!(left.find(&8u64), None);
    assert_eq!(left.find(&11u64), None);
    assert_eq!(left.find(&14u64), None);
    // Right has only keys strictly greater than 8.
    assert_eq!(right.find(&2u64), None);
    assert_eq!(right.find(&5u64), None);
    assert_eq!(right.find(&8u64), None);
    assert_eq!(right.find(&11u64), Some(110u64));
    assert_eq!(right.find(&14u64), Some(140u64));
    // Total coverage: left + 1 (mid) + right = original size.
    assert_eq!(left.size() + 1 + right.size(), 5);
}

#[test]
fn test_ordkeymap_split_union_roundtrip() {
    // Splitting and rejoining via union + insert reconstructs the original.
    let m = make_map(&[(1, 10), (3, 30), (5, 50), (7, 70), (9, 90)]);
    let (left, mid_val, right) = m.split(&5u64);
    assert_eq!(mid_val, Some(50u64));
    let mut reconstructed = left.union(&right);
    if let Some(v) = mid_val {
        reconstructed.insert(5u64, v);
    }
    assert_eq!(reconstructed.size(), 5);
    assert_eq!(reconstructed.find(&1u64), Some(10u64));
    assert_eq!(reconstructed.find(&3u64), Some(30u64));
    assert_eq!(reconstructed.find(&5u64), Some(50u64));
    assert_eq!(reconstructed.find(&7u64), Some(70u64));
    assert_eq!(reconstructed.find(&9u64), Some(90u64));
}

// Integration tests.

#[test]
fn test_ordkeymap_large_map_operations() {
    // Build a map with 110 entries.
    let mut m: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    for i in 0u64..110 {
        m.insert(i * 2, i * 10);
    }
    assert_eq!(m.size(), 110);
    // Verify every inserted key is findable with the correct value.
    for i in 0u64..110 {
        assert_eq!(m.find(&(i * 2)), Some(i * 10));
        // Odd keys not inserted.
        assert_eq!(m.find(&(i * 2 + 1)), None);
    }
    // Split at key 100 (even, so it exists).
    let (left, mid, right) = m.split(&100u64);
    assert_eq!(mid, Some(500u64));  // key 100 = index 50, value = 50 * 10 = 500
    assert_eq!(left.size(), 50);
    assert_eq!(right.size(), 59);
}

#[test]
fn test_ordkeymap_chain_insert_union_split() {
    // Insert 20 keys into two separate maps, union them, split, verify.
    let mut a: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    let mut b: OrdKeyMap<u64, u64> = OrdKeyMap::new();
    // a: keys 0, 2, 4, ..., 18 with value = key + 1.
    for i in 0u64..10 {
        a.insert(i * 2, i * 2 + 1);
    }
    // b: keys 21, 23, 25, ..., 39 with value = key * 2.
    for i in 0u64..10 {
        b.insert(i * 2 + 21, (i * 2 + 21) * 2);
    }
    assert_eq!(a.size(), 10);
    assert_eq!(b.size(), 10);
    // Union is disjoint, so combined has 20 keys.
    let combined = a.union(&b);
    assert_eq!(combined.size(), 20);
    // Split at key 20 (absent from both maps).
    let (left, mid, right) = combined.split(&20u64);
    assert!(mid.is_none());
    assert_eq!(left.size(), 10);
    assert_eq!(right.size(), 10);
    // Left side matches a.
    for i in 0u64..10 {
        assert_eq!(left.find(&(i * 2)), Some(i * 2 + 1));
    }
    // Right side matches b.
    for i in 0u64..10 {
        assert_eq!(right.find(&(i * 2 + 21)), Some((i * 2 + 21) * 2));
    }
    // Difference: combined minus b = a.
    let diff = combined.difference(&b);
    assert_eq!(diff.size(), 10);
    for i in 0u64..10 {
        assert_eq!(diff.find(&(i * 2)), Some(i * 2 + 1));
    }
    // Intersect: combined intersect b = b (values from combined = from b since no overlap).
    let common = combined.intersect(&b);
    assert_eq!(common.size(), 10);
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
