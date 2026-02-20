//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Test suite for PathWeightUtilsStPer.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::PathWeightUtilsStPer::PathWeightUtilsStPer::*;
use apas_verus::vstdplus::float::float::*;

fn dist(v: f64) -> F64Dist { F64Dist { val: v } }

#[test]
fn test_path_weight_int_simple() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![0, 1, i64::MAX]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, 0, 2]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(path_weight_int(&path, &weights), Some(3));
}

#[test]
fn test_path_weight_int_negative() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![0, 5, i64::MAX]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, 0, -3]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(path_weight_int(&path, &weights), Some(2));
}

#[test]
fn test_path_weight_float_simple() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![dist(0.0), dist(1.5), dist(f64::INFINITY)]),
        ArraySeqStPerS::from_vec(vec![dist(f64::INFINITY), dist(0.0), dist(2.5)]),
        ArraySeqStPerS::from_vec(vec![dist(f64::INFINITY), dist(f64::INFINITY), dist(0.0)]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(path_weight_float(&path, &weights), Some(dist(4.0)));
}

#[test]
fn test_validate_subpath_int() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![0, 1, i64::MAX]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, 0, 2]),
        ArraySeqStPerS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let distances = ArraySeqStPerS::from_vec(vec![0, 1, 3]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert!(validate_subpath_property_int(&path, &distances, &weights));
}

#[test]
fn test_validate_subpath_float() {
    let weights = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![dist(0.0), dist(1.5), dist(f64::INFINITY)]),
        ArraySeqStPerS::from_vec(vec![dist(f64::INFINITY), dist(0.0), dist(2.5)]),
        ArraySeqStPerS::from_vec(vec![dist(f64::INFINITY), dist(f64::INFINITY), dist(0.0)]),
    ]);
    let distances = ArraySeqStPerS::from_vec(vec![dist(0.0), dist(1.5), dist(4.0)]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert!(validate_subpath_property_float(&path, &distances, &weights));
}
