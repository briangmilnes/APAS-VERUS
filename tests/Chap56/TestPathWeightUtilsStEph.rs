// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//!
//! Test suite for PathWeightUtilsStEph.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::PathWeightUtilsStEph::PathWeightUtilsStEph::*;
use apas_verus::vstdplus::float::float::*;

fn dist(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

#[test]
fn test_path_weight_int_simple() {
    let weights = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![0, 1, i64::MAX]),
        ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 2]),
        ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(PathWeightUtilsStEphS::path_weight_int(&path, &weights), Some(3));
}

#[test]
fn test_path_weight_int_negative() {
    let weights = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![0, 5, i64::MAX]),
        ArraySeqStEphS::from_vec(vec![i64::MAX, 0, -3]),
        ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(PathWeightUtilsStEphS::path_weight_int(&path, &weights), Some(2));
}

#[test]
fn test_path_weight_float_simple() {
    let weights = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![dist(0.0), dist(1.5), dist(f64::INFINITY)]),
        ArraySeqStEphS::from_vec(vec![dist(f64::INFINITY), dist(0.0), dist(2.5)]),
        ArraySeqStEphS::from_vec(vec![dist(f64::INFINITY), dist(f64::INFINITY), dist(0.0)]),
    ]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert_eq!(PathWeightUtilsStEphS::path_weight_float(&path, &weights), Some(dist(4.0)));
}

#[test]
fn test_validate_subpath_int() {
    let weights = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![0, 1, i64::MAX]),
        ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 2]),
        ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0]),
    ]);
    let distances = ArraySeqStEphS::from_vec(vec![0, 1, 3]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert!(PathWeightUtilsStEphS::validate_subpath_property_int(&path, &distances, &weights));
}

#[test]
fn test_validate_subpath_float() {
    let weights = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![dist(0.0), dist(1.5), dist(f64::INFINITY)]),
        ArraySeqStEphS::from_vec(vec![dist(f64::INFINITY), dist(0.0), dist(2.5)]),
        ArraySeqStEphS::from_vec(vec![dist(f64::INFINITY), dist(f64::INFINITY), dist(0.0)]),
    ]);
    let distances = ArraySeqStEphS::from_vec(vec![dist(0.0), dist(1.5), dist(4.0)]);
    let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
    assert!(PathWeightUtilsStEphS::validate_subpath_property_float(&path, &distances, &weights));
}
