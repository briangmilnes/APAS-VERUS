#![cfg(feature = "all_chapters")]
// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for Example56_1 - Path Weight Computation.

use apas_verus::Chap56::Example56_1::Example56_1::*;
use apas_verus::Types::Types::*;

#[test]
fn test_example_path_weight_int() { Example56_1S::example_path_weight_int(); }

#[test]
fn test_example_path_weight_float() { Example56_1S::example_path_weight_float(); }

#[test]
fn test_example_negative_weights() { Example56_1S::example_negative_weights(); }
