#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Example56_1 - Path Weight Computation.

use apas_verus::Chap56::Example56_1::Example56_1::*;
use apas_verus::Types::Types::*;

#[test]
fn test_example_path_weight_int() { example_path_weight_int(); }

#[test]
fn test_example_path_weight_float() { example_path_weight_float(); }

#[test]
fn test_example_negative_weights() { example_negative_weights(); }
