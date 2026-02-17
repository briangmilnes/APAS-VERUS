//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for TopDownDPStEph - Top-Down Dynamic Programming Single-Threaded Ephemeral

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap51::TopDownDPStEph::TopDownDPStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = TopDownDPStEphS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_memoized_empty() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let mut dp = TopDownDPStEphS::new(s, t);
    assert_eq!(dp.med_memoized(), 0);
}

#[test]
fn test_med_memoized_textbook() {
    let s = ArraySeqStEphS::tabulate(&|i| ['t', 'c', 'a', 't'][i], 4);
    let t = ArraySeqStEphS::tabulate(&|i| ['a', 't', 'c'][i], 3);
    let mut dp = TopDownDPStEphS::new(s, t);
    assert_eq!(dp.med_memoized(), 3);
}

#[test]
fn test_med_memoized_identical() {
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let mut dp = TopDownDPStEphS::new(s, t);
    assert_eq!(dp.med_memoized(), 0);
}

#[test]
fn test_memo_size() {
    let dp = TopDownDPStEphS::default();
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_is_memoized() {
    let dp = TopDownDPStEphS::default();
    assert!(!dp.is_memoized(0, 0));
}

#[test]
fn test_get_memoized() {
    let dp = TopDownDPStEphS::default();
    assert_eq!(dp.get_memoized(0, 0), None);
}

#[test]
fn test_insert_memo() {
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t = ArraySeqStEphS::tabulate(&|i| ['x', 'y'][i], 2);
    let mut dp = TopDownDPStEphS::new(s, t);
    
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
    let s = ArraySeqStEphS::tabulate(&|_| 'a', 1);
    let t = ArraySeqStEphS::tabulate(&|_| 'x', 1);
    let mut dp = TopDownDPStEphS::new(s, t);
    
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
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = TopDownDPStEphS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::tabulate(&|i| ['x', 'y'][i], 2);
    let dp = TopDownDPStEphS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = TopDownDPStEphS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqStEphS::tabulate(&|_| 'a', 1);
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = TopDownDPStEphS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_clear_memo() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let mut dp = TopDownDPStEphS::new(s, t);
    dp.clear_memo();
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_default() {
    let dp = TopDownDPStEphS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
    assert_eq!(dp.memo_size(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp = TopDownDPStEphS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("TopDownDPStEph"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t1 = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp1 = TopDownDPStEphS::new(s1, t1);

    let s2 = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t2 = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp2 = TopDownDPStEphS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
