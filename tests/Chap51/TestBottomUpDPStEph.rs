//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BottomUpDPStEph - Bottom-Up Dynamic Programming Single-Threaded Ephemeral

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap51::BottomUpDPStEph::BottomUpDPStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = BottomUpDPStEphS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_bottom_up_empty() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let mut dp = BottomUpDPStEphS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 0);
}

#[test]
fn test_med_bottom_up_textbook() {
    let s = ArraySeqStEphS::tabulate(&|i| ['t', 'c', 'a', 't'][i], 4);
    let t = ArraySeqStEphS::tabulate(&|i| ['a', 't', 'c'][i], 3);
    let mut dp = BottomUpDPStEphS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 3);
}

#[test]
fn test_med_bottom_up_identical() {
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let mut dp = BottomUpDPStEphS::new(s, t);
    assert_eq!(dp.med_bottom_up(), 0);
}

#[test]
fn test_s_length() {
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c'][i], 3);
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = BottomUpDPStEphS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::tabulate(&|i| ['x', 'y'][i], 2);
    let dp = BottomUpDPStEphS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqStEphS::new(0, ' ');
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = BottomUpDPStEphS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqStEphS::tabulate(&|_| 'a', 1);
    let t = ArraySeqStEphS::new(0, ' ');
    let dp = BottomUpDPStEphS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_set_s() {
    let mut dp = BottomUpDPStEphS::default();
    let new_s = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    dp.set_s(new_s);
    assert_eq!(dp.s_length(), 2);
}

#[test]
fn test_set_t() {
    let mut dp = BottomUpDPStEphS::default();
    let new_t = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    dp.set_t(new_t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_default() {
    let dp = BottomUpDPStEphS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp = BottomUpDPStEphS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("BottomUpDPStEph"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t1 = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp1 = BottomUpDPStEphS::new(s1, t1);

    let s2 = ArraySeqStEphS::tabulate(&|i| ['a', 'b'][i], 2);
    let t2 = ArraySeqStEphS::tabulate(&|i| ['c', 'd'][i], 2);
    let dp2 = BottomUpDPStEphS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
