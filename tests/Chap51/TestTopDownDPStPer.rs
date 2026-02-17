//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for TopDownDPStPer - Top-Down Dynamic Programming Single-Threaded Persistent

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap51::TopDownDPStPer::TopDownDPStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = TopDownDPStPerS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_memoized_empty() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = TopDownDPStPerS::new(s, t);
    assert_eq!(dp.med_memoized(), 0);
}

#[test]
fn test_med_memoized_textbook() {
    let s = ArraySeqStPerS::tabulate(&|i| ['t', 'c', 'a', 't'][i], 4);
    let t = ArraySeqStPerS::tabulate(&|i| ['a', 't', 'c'][i], 3);
    let dp = TopDownDPStPerS::new(s, t);
    assert_eq!(dp.med_memoized(), 3);
}

#[test]
fn test_med_memoized_identical() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let dp = TopDownDPStPerS::new(s, t);
    assert_eq!(dp.med_memoized(), 0);
}

#[test]
fn test_memo_size() {
    let dp = TopDownDPStPerS::default();
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_is_memoized() {
    let dp = TopDownDPStPerS::default();
    assert!(!dp.is_memoized(0, 0));
}

#[test]
fn test_get_memoized() {
    let dp = TopDownDPStPerS::default();
    assert_eq!(dp.get_memoized(0, 0), None);
}

#[test]
fn test_s_length() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = TopDownDPStPerS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::tabulate(&|i| ['x', 'y'][i], 2);
    let dp = TopDownDPStPerS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = TopDownDPStPerS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqStPerS::tabulate(&|_| 'a', 1);
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = TopDownDPStPerS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_clear_memo() {
    use std::collections::HashMap;
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = TopDownDPStPerS::new(s, t);

    let mut memo = HashMap::new();
    memo.insert((1, 1), 42);
    let dp_with_memo = dp.with_memo_table(memo);
    assert_eq!(dp_with_memo.memo_size(), 1);

    let dp_cleared = dp_with_memo.clear_memo();
    assert_eq!(dp_cleared.memo_size(), 0);
}

#[test]
fn test_default() {
    let dp = TopDownDPStPerS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp = TopDownDPStPerS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("TopDownDPStPer"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
    assert!(display_str.contains("memo_size=0"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t1 = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp1 = TopDownDPStPerS::new(s1, t1);

    let s2 = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t2 = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp2 = TopDownDPStPerS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
