//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for TopDownDPMtPer - Top-Down Dynamic Programming Multi-Threaded Persistent

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap51::TopDownDPMtPer::TopDownDPMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = TopDownDPMtPerS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_memoized_concurrent_empty() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = TopDownDPMtPerS::new(s, t);
    assert_eq!(dp.med_memoized_concurrent(), 0);
}

#[test]
fn test_med_memoized_concurrent_textbook() {
    let s = ArraySeqMtPerS::from_vec(vec!['t', 'c', 'a', 't']);
    let t = ArraySeqMtPerS::from_vec(vec!['a', 't', 'c']);
    let dp = TopDownDPMtPerS::new(s, t);
    assert_eq!(dp.med_memoized_concurrent(), 3);
}

#[test]
fn test_med_memoized_parallel() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtPerS::from_vec(vec!['x', 'y', 'z']);
    let dp = TopDownDPMtPerS::new(s, t);
    let result = dp.med_memoized_parallel();
    assert_eq!(result, 6); // APAS MED: insert/delete only, no substitute
}

#[test]
fn test_memo_size() {
    let dp = TopDownDPMtPerS::default();
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_is_memoized() {
    let dp = TopDownDPMtPerS::default();
    assert!(!dp.is_memoized(0, 0));
}

#[test]
fn test_get_memoized() {
    let dp = TopDownDPMtPerS::default();
    assert_eq!(dp.get_memoized(0, 0), None);
}

#[test]
fn test_s_length() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = TopDownDPMtPerS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::from_vec(vec!['x', 'y']);
    let dp = TopDownDPMtPerS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = TopDownDPMtPerS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqMtPerS::from_vec(vec!['a']);
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = TopDownDPMtPerS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_clear_memo() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = TopDownDPMtPerS::new(s, t);
    let dp_cleared = dp.clear_memo();
    assert_eq!(dp_cleared.memo_size(), 0);
}

#[test]
fn test_default() {
    let dp = TopDownDPMtPerS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b']);
    let t = ArraySeqMtPerS::from_vec(vec!['c', 'd']);
    let dp = TopDownDPMtPerS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("TopDownDPMtPer"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqMtPerS::from_vec(vec!['a', 'b']);
    let t1 = ArraySeqMtPerS::from_vec(vec!['c', 'd']);
    let dp1 = TopDownDPMtPerS::new(s1, t1);

    let s2 = ArraySeqMtPerS::from_vec(vec!['a', 'b']);
    let t2 = ArraySeqMtPerS::from_vec(vec!['c', 'd']);
    let dp2 = TopDownDPMtPerS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
