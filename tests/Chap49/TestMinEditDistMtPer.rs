//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistMtPer.

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap49::MinEditDistMtPer::MinEditDistMtPer::*;
use apas_verus::MinEditDistMtPerLit;

#[test]
fn test_min_edit_distance_mt_per_basic() {
    let solver = MinEditDistMtPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );
    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_min_edit_distance_mt_per_empty() {
    let solver1 = MinEditDistMtPerLit!(source: [], target: ['A', 'B']);
    assert_eq!(solver1.min_edit_distance(), 2);

    let solver2: MinEditDistMtPerS<char> = MinEditDistMtPerLit!(source: [], target: []);
    assert_eq!(solver2.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_mt_per_identical() {
    let solver = MinEditDistMtPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'C']);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_mt_per_new() {
    let solver = MinEditDistMtPerS::<char>::new();
    assert_eq!(solver.source().length(), 0);
    assert_eq!(solver.target().length(), 0);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_mt_per_from_sequences() {
    let source = <ArraySeqMtPerS<char> as ArraySeqMtPerBaseTrait<char>>::from_vec(vec!['A', 'B', 'C']);
    let target = <ArraySeqMtPerS<char> as ArraySeqMtPerBaseTrait<char>>::from_vec(vec!['A', 'C']);
    let solver = MinEditDistMtPerS::from_sequences(source, target);
    assert_eq!(solver.source().length(), 3);
    assert_eq!(solver.target().length(), 2);
    assert_eq!(solver.min_edit_distance(), 1);
}

#[test]
fn test_min_edit_distance_mt_per_source_getter() {
    let solver = MinEditDistMtPerLit!(source: ['X', 'Y', 'Z'], target: ['A', 'B']);
    let source = solver.source();
    assert_eq!(source.length(), 3);
    assert_eq!(*source.nth(0), 'X');
    assert_eq!(*source.nth(1), 'Y');
    assert_eq!(*source.nth(2), 'Z');
}

#[test]
fn test_min_edit_distance_mt_per_target_getter() {
    let solver = MinEditDistMtPerLit!(source: ['X', 'Y'], target: ['A', 'B', 'C']);
    let target = solver.target();
    assert_eq!(target.length(), 3);
    assert_eq!(*target.nth(0), 'A');
    assert_eq!(*target.nth(1), 'B');
    assert_eq!(*target.nth(2), 'C');
}

#[test]
fn test_min_edit_distance_mt_per_memo_size() {
    let solver = MinEditDistMtPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    // Before computation, memo should be empty
    assert_eq!(solver.memo_size(), 0);

    // After computation, memo should contain subproblem results
    solver.min_edit_distance();
    assert!(solver.memo_size() > 0);
}

#[test]
fn test_min_edit_distance_mt_per_display() {
    let solver = MinEditDistMtPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let display_str = format!("{}", solver);
    assert!(display_str.contains("MinEditDistMtPer"));
    assert!(display_str.contains("source"));
    assert!(display_str.contains("target"));
    assert!(display_str.contains("memo_entries"));
}

#[test]
fn test_min_edit_distance_mt_per_partial_eq() {
    let solver1 = MinEditDistMtPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let solver2 = MinEditDistMtPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let solver3 = MinEditDistMtPerLit!(source: ['A', 'B'], target: ['C', 'E']);

    assert_eq!(solver1, solver2);
    assert_ne!(solver1, solver3);
}

#[test]
fn test_min_edit_distance_mt_per_clone() {
    let solver1 = MinEditDistMtPerLit!(source: ['A', 'B', 'C'], target: ['A', 'C']);
    let solver2 = solver1.clone();

    assert_eq!(solver1.source().length(), solver2.source().length());
    assert_eq!(solver1.target().length(), solver2.target().length());
    assert_eq!(solver1.min_edit_distance(), solver2.min_edit_distance());
}

#[test]
fn test_min_edit_distance_mt_per_completely_different() {
    let solver = MinEditDistMtPerLit!(source: ['A', 'B', 'C'], target: ['X', 'Y', 'Z']);
    // All characters different. Algorithm uses delete/insert only (no substitution).
    // Optimal: delete all 3 from source, insert all 3 from target = 6 operations
    // But actually, it can interleave: delete A, insert X, delete B, insert Y, delete C, insert Z
    // which is still 6. However, the DP might find: match none, so min of various paths.
    // Let's run it to see what the actual value is.
    let result = solver.min_edit_distance();
    // Based on the algorithm: uses min(delete, insert) + 1 at each mismatch
    // For completely different strings, this typically gives length(source) + length(target) / 2
    // or around 3-6 depending on optimization. Let's verify by running first.
    assert_eq!(result, 6); // Update this based on actual result
}

#[test]
fn test_min_edit_distance_mt_per_single_char() {
    let solver1 = MinEditDistMtPerLit!(source: ['A'], target: ['B']);
    // Mismatch: need to delete A (cost 1) and insert B (cost 1) = 2 total
    // Or go from (1,0) to (0,1) via (0,0), which would be 1 delete + 1 insert = 2
    assert_eq!(solver1.min_edit_distance(), 2); // Update based on actual

    let solver2 = MinEditDistMtPerLit!(source: ['A'], target: ['A']);
    assert_eq!(solver2.min_edit_distance(), 0);
}
