//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for TopDownDPMtEph - Top-Down Dynamic Programming Multi-Threaded Ephemeral

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap51::TopDownDPMtEph::TopDownDPMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = TopDownDPMtEphS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_memoized_concurrent_empty() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let mut dp = TopDownDPMtEphS::new(s, t);
    assert_eq!(dp.med_memoized_concurrent(), 0);
}

#[test]
fn test_med_memoized_concurrent_textbook() {
    let s = ArraySeqMtEphS::from_vec(vec!['t', 'c', 'a', 't']);
    let t = ArraySeqMtEphS::from_vec(vec!['a', 't', 'c']);
    let mut dp = TopDownDPMtEphS::new(s, t);
    assert_eq!(dp.med_memoized_concurrent(), 3);
}

#[test]
fn test_med_memoized_parallel() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtEphS::from_vec(vec!['x', 'y', 'z']);
    let mut dp = TopDownDPMtEphS::new(s, t);
    let result = dp.med_memoized_parallel();
    assert_eq!(result, 6); // APAS MED: insert/delete only, no substitute
}

#[test]
fn test_memo_size() {
    let dp = TopDownDPMtEphS::default();
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_is_memoized() {
    let dp = TopDownDPMtEphS::default();
    assert!(!dp.is_memoized(0, 0));
}

#[test]
fn test_get_memoized() {
    let dp = TopDownDPMtEphS::default();
    assert_eq!(dp.get_memoized(0, 0), None);
}

#[test]
fn test_insert_memo() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t = ArraySeqMtEphS::from_vec(vec!['x', 'y']);
    let mut dp = TopDownDPMtEphS::new(s, t);
    
    // Initially not memoized
    assert!(!dp.is_memoized(1, 1));
    assert_eq!(dp.get_memoized(1, 1), None);
    assert_eq!(dp.memo_size(), 0);
    
    // Insert memo value
    dp.insert_memo(1, 1, 42);
    
    // Now should be memoized
    assert!(dp.is_memoized(1, 1));
    assert_eq!(dp.get_memoized(1, 1), Some(42));
    assert_eq!(dp.memo_size(), 1);
    
    // Insert another
    dp.insert_memo(0, 1, 7);
    assert!(dp.is_memoized(0, 1));
    assert_eq!(dp.get_memoized(0, 1), Some(7));
    assert_eq!(dp.memo_size(), 2);
}

#[test]
fn test_insert_memo_overwrite() {
    let s = ArraySeqMtEphS::from_vec(vec!['a']);
    let t = ArraySeqMtEphS::from_vec(vec!['x']);
    let mut dp = TopDownDPMtEphS::new(s, t);
    
    // Insert initial value
    dp.insert_memo(0, 0, 10);
    assert_eq!(dp.get_memoized(0, 0), Some(10));
    
    // Overwrite with new value
    dp.insert_memo(0, 0, 20);
    assert_eq!(dp.get_memoized(0, 0), Some(20));
    assert_eq!(dp.memo_size(), 1); // Still only one entry
}

#[test]
fn test_s_length() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = TopDownDPMtEphS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::from_vec(vec!['x', 'y']);
    let dp = TopDownDPMtEphS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = TopDownDPMtEphS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqMtEphS::from_vec(vec!['a']);
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = TopDownDPMtEphS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_clear_memo() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let mut dp = TopDownDPMtEphS::new(s, t);
    dp.clear_memo();
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_default() {
    let dp = TopDownDPMtEphS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t = ArraySeqMtEphS::from_vec(vec!['c', 'd']);
    let dp = TopDownDPMtEphS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("TopDownDPMtEph"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t1 = ArraySeqMtEphS::from_vec(vec!['c', 'd']);
    let dp1 = TopDownDPMtEphS::new(s1, t1);

    let s2 = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t2 = ArraySeqMtEphS::from_vec(vec!['c', 'd']);
    let dp2 = TopDownDPMtEphS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
