//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BottomUpDPStPer - Bottom-Up Dynamic Programming Single-Threaded Persistent

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap51::BottomUpDPStPer::BottomUpDPStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_bottom_up_empty() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 0);
}

#[test]
fn test_med_bottom_up_textbook() {
    // "tcat" -> "atc" example from textbook
    let s = ArraySeqStPerS::tabulate(&|i| ['t', 'c', 'a', 't'][i], 4);
    let t = ArraySeqStPerS::tabulate(&|i| ['a', 't', 'c'][i], 3);
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 3);
}

#[test]
fn test_med_bottom_up_identical() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 0);
}

#[test]
fn test_med_bottom_up_one_empty() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 3);
}

#[test]
fn test_med_bottom_up_single_char_same() {
    let s = ArraySeqStPerS::tabulate(&|_| 'x', 1);
    let t = ArraySeqStPerS::tabulate(&|_| 'x', 1);
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 0);
}

#[test]
fn test_med_bottom_up_single_char_different() {
    let s = ArraySeqStPerS::tabulate(&|_| 'a', 1);
    let t = ArraySeqStPerS::tabulate(&|_| 'b', 1);
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 2); // Algorithm uses delete+insert, not substitute
}

#[test]
fn test_s_length() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::tabulate(&|i| ['x', 'y'][i], 2);
    let dp = BottomUpDPStPerS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqStPerS::new(0, ' ');
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = BottomUpDPStPerS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqStPerS::tabulate(&|_| 'a', 1);
    let t = ArraySeqStPerS::new(0, ' ');
    let dp = BottomUpDPStPerS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_default() {
    let dp = BottomUpDPStPerS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp = BottomUpDPStPerS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("BottomUpDPStPer"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq_true() {
    let s1 = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t1 = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp1 = BottomUpDPStPerS::new(s1.clone(), t1.clone());

    let s2 = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t2 = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp2 = BottomUpDPStPerS::new(s2, t2);

    assert_eq!(dp1, dp2);
}

#[test]
fn test_partial_eq_false() {
    let s1 = ArraySeqStPerS::tabulate(&|i| ['a', 'b'][i], 2);
    let t1 = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp1 = BottomUpDPStPerS::new(s1, t1);

    let s2 = ArraySeqStPerS::tabulate(&|i| ['x', 'y'][i], 2);
    let t2 = ArraySeqStPerS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp2 = BottomUpDPStPerS::new(s2, t2);

    assert_ne!(dp1, dp2);
}
