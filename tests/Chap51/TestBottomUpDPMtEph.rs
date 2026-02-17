//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BottomUpDPMtEph - Bottom-Up Dynamic Programming Multi-Threaded Ephemeral

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap51::BottomUpDPMtEph::BottomUpDPMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = BottomUpDPMtEphS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_bottom_up_parallel_empty() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let mut dp = BottomUpDPMtEphS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 0);
}

#[test]
fn test_med_bottom_up_parallel_textbook() {
    let s = ArraySeqMtEphS::from_vec(vec!['t', 'c', 'a', 't']);
    let t = ArraySeqMtEphS::from_vec(vec!['a', 't', 'c']);
    let mut dp = BottomUpDPMtEphS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 3);
}

#[test]
fn test_med_bottom_up_parallel_identical() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let mut dp = BottomUpDPMtEphS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 0);
}

#[test]
fn test_s_length() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = BottomUpDPMtEphS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::from_vec(vec!['x', 'y']);
    let dp = BottomUpDPMtEphS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqMtEphS::new(0, ' ');
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = BottomUpDPMtEphS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqMtEphS::from_vec(vec!['a']);
    let t = ArraySeqMtEphS::new(0, ' ');
    let dp = BottomUpDPMtEphS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_default() {
    let dp = BottomUpDPMtEphS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t = ArraySeqMtEphS::from_vec(vec!['c', 'd']);
    let dp = BottomUpDPMtEphS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("BottomUpDPMtEph"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t1 = ArraySeqMtEphS::from_vec(vec!['c', 'd']);
    let dp1 = BottomUpDPMtEphS::new(s1, t1);

    let s2 = ArraySeqMtEphS::from_vec(vec!['a', 'b']);
    let t2 = ArraySeqMtEphS::from_vec(vec!['c', 'd']);
    let dp2 = BottomUpDPMtEphS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
