#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Example56_3 - Negative Weight Cycles.

use apas_verus::Chap56::Example56_3::Example56_3::*;
use apas_verus::Types::Types::*;

#[test]
fn test_example_negative_cycle() { example_negative_cycle(); }

#[test]
fn test_example_undefined_shortest_path() { example_undefined_shortest_path(); }
