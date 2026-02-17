//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistMtEph.

use apas_verus::ArraySeqMtEphChap19SLit;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap49::MinEditDistMtEph::MinEditDistMtEph::*;
use apas_verus::MinEditDistMtEphLit;

#[test]
fn test_min_edit_distance_mt_eph_basic() {
    let mut solver = MinEditDistMtEphLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);
    assert_eq!(solver.min_edit_distance(), 2);

    // Test ephemeral mutation
    solver.set_source(2, 'D');
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_mt_eph_mutation() {
    let mut solver = MinEditDistMtEphLit!(source: ['X', 'Y'], target: ['A', 'B', 'C']);
    assert_eq!(solver.min_edit_distance(), 5);

    solver.set_source(0, 'A');
    solver.set_source(1, 'B');
    assert!(solver.min_edit_distance() < 5);
}

#[test]
fn test_min_edit_distance_mt_eph_empty() {
    let mut solver: MinEditDistMtEphS<char> = MinEditDistMtEphLit!(source: [], target: []);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_constructors() {
    let solver = MinEditDistMtEphS::<char>::new();
    assert_eq!(solver.source().length(), 0);
    assert_eq!(solver.target().length(), 0);

    let solver2 = MinEditDistMtEphS::from_sequences(ArraySeqMtEphChap19SLit!['a', 'b'], ArraySeqMtEphChap19SLit!['c', 'd']);
    assert_eq!(*solver2.source().nth(0), 'a');
    assert_eq!(*solver2.target().nth(0), 'c');
}

#[test]
fn test_identical_sequences() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b', 'c'], target: ['a', 'b', 'c']);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_completely_different() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b', 'c'], target: ['x', 'y', 'z']);
    let dist = solver.min_edit_distance();
    assert!(dist > 0); // Different sequences should have some distance
}

#[test]
fn test_single_insert() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['a', 'b', 'c']);
    let dist = solver.min_edit_distance();
    assert!(dist > 0);
}

#[test]
fn test_single_delete() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b', 'c'], target: ['a', 'b']);
    let dist = solver.min_edit_distance();
    assert!(dist > 0);
}

#[test]
fn test_single_substitute() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b', 'c'], target: ['a', 'x', 'c']);
    let dist = solver.min_edit_distance();
    assert!(dist > 0);
}

#[test]
fn test_kitten_to_sitting() {
    let mut solver = MinEditDistMtEphLit!(
        source: ['k', 'i', 't', 't', 'e', 'n'],
        target: ['s', 'i', 't', 't', 'i', 'n', 'g']
    );
    let dist = solver.min_edit_distance();
    assert!(dist > 0 && dist < 10); // Some reasonable edit distance
}

#[test]
fn test_getters() {
    let solver = MinEditDistMtEphLit!(source: ['a'], target: ['b']);
    assert_eq!(*solver.source().nth(0), 'a');
    assert_eq!(*solver.target().nth(0), 'b');
}

#[test]
fn test_mutable_getters() {
    let mut solver = MinEditDistMtEphLit!(source: ['a'], target: ['b']);
    let _ = solver.source_mut().set(0, 'x');
    let _ = solver.target_mut().set(0, 'y');
    assert_eq!(*solver.source().nth(0), 'x');
    assert_eq!(*solver.target().nth(0), 'y');
}

#[test]
fn test_set_methods() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['c', 'd']);
    solver.set_source(0, 'x');
    solver.set_target(1, 'y');
    assert_eq!(*solver.source().nth(0), 'x');
    assert_eq!(*solver.target().nth(1), 'y');
}

#[test]
fn test_memo_size() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['c', 'd']);
    assert_eq!(solver.memo_size(), 0);
    solver.min_edit_distance();
    assert!(solver.memo_size() > 0);
}

#[test]
fn test_clear_memo() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['c', 'd']);
    solver.min_edit_distance();
    let size_before = solver.memo_size();
    assert!(size_before > 0);
    solver.clear_memo();
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_multiple_calls() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['c', 'd']);
    let dist1 = solver.min_edit_distance();
    let dist2 = solver.min_edit_distance();
    assert_eq!(dist1, dist2);
}

#[test]
fn test_mutation_clears_cache() {
    let mut solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['c', 'd']);
    let dist1 = solver.min_edit_distance();
    solver.set_source(0, 'c');
    let dist2 = solver.min_edit_distance();
    assert_ne!(dist1, dist2);
}

#[test]
fn test_display_trait() {
    let solver = MinEditDistMtEphLit!(source: ['a', 'b'], target: ['c', 'd']);
    let display = format!("{}", solver);
    assert!(!display.is_empty()); // Just verify Display is implemented
}

#[test]
fn test_with_integers() {
    let mut solver = MinEditDistMtEphLit!(source: [1, 2, 3, 4], target: [1, 3, 4, 5]);
    let dist = solver.min_edit_distance();
    assert!(dist > 0); // Different sequences
}

#[test]
fn test_prefix_match() {
    let mut solver = MinEditDistMtEphLit!(
        source: ['p', 'r', 'e', 'f', 'i', 'x'],
        target: ['p', 'r', 'e']
    );
    let dist = solver.min_edit_distance();
    assert!(dist > 0);
}

#[test]
fn test_suffix_match() {
    let mut solver = MinEditDistMtEphLit!(
        source: ['s', 'u', 'f', 'f', 'i', 'x'],
        target: ['f', 'i', 'x']
    );
    let dist = solver.min_edit_distance();
    assert!(dist > 0);
}

#[test]
fn test_macro_usage() {
    let mut solver = MinEditDistMtEphLit!(source: ['a'], target: ['b']);
    let dist = solver.min_edit_distance();
    assert!(dist > 0); // Different sequences should have distance > 0

    let mut solver2: MinEditDistMtEphS<char> = MinEditDistMtEphLit!();
    assert_eq!(solver2.min_edit_distance(), 0);
}
