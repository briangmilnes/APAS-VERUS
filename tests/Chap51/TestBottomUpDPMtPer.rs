//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BottomUpDPMtPer - Bottom-Up Dynamic Programming Multi-Threaded Persistent

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap51::BottomUpDPMtPer::BottomUpDPMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_med_bottom_up_parallel_empty() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 0);
}

#[test]
fn test_med_bottom_up_parallel_textbook() {
    let s = ArraySeqMtPerS::from_vec(vec!['t', 'c', 'a', 't']);
    let t = ArraySeqMtPerS::from_vec(vec!['a', 't', 'c']);
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 3);
}

#[test]
fn test_med_bottom_up_parallel_identical() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 0);
}

#[test]
fn test_med_bottom_up_parallel_one_empty() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.med_bottom_up_parallel(), 3);
}

#[test]
fn test_s_length() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c']);
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.s_length(), 3);
}

#[test]
fn test_t_length() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::from_vec(vec!['x', 'y']);
    let dp = BottomUpDPMtPerS::new(s, t);
    assert_eq!(dp.t_length(), 2);
}

#[test]
fn test_is_empty_true() {
    let s = ArraySeqMtPerS::new(0, ' ');
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = BottomUpDPMtPerS::new(s, t);
    assert!(dp.is_empty());
}

#[test]
fn test_is_empty_false() {
    let s = ArraySeqMtPerS::from_vec(vec!['a']);
    let t = ArraySeqMtPerS::new(0, ' ');
    let dp = BottomUpDPMtPerS::new(s, t);
    assert!(!dp.is_empty());
}

#[test]
fn test_default() {
    let dp = BottomUpDPMtPerS::default();
    assert_eq!(dp.s_length(), 0);
    assert_eq!(dp.t_length(), 0);
}

#[test]
fn test_display() {
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b']);
    let t = ArraySeqMtPerS::from_vec(vec!['c', 'd']);
    let dp = BottomUpDPMtPerS::new(s, t);
    let display_str = format!("{dp}");
    assert!(display_str.contains("BottomUpDPMtPer"));
    assert!(display_str.contains("s_len=2"));
    assert!(display_str.contains("t_len=2"));
}

#[test]
fn test_partial_eq() {
    let s1 = ArraySeqMtPerS::from_vec(vec!['a', 'b']);
    let t1 = ArraySeqMtPerS::from_vec(vec!['c', 'd']);
    let dp1 = BottomUpDPMtPerS::new(s1, t1);

    let s2 = ArraySeqMtPerS::from_vec(vec!['a', 'b']);
    let t2 = ArraySeqMtPerS::from_vec(vec!['c', 'd']);
    let dp2 = BottomUpDPMtPerS::new(s2, t2);

    assert_eq!(dp1, dp2);
}
