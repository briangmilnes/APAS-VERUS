#![cfg(feature = "all_chapters")]
// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for Example56_3 - Negative Weight Cycles.

use apas_verus::Chap56::Example56_3::Example56_3::*;
use apas_verus::Types::Types::*;

#[test]
fn test_example_negative_cycle() { example_negative_cycle(); }

#[test]
fn test_example_undefined_shortest_path() { example_undefined_shortest_path(); }
