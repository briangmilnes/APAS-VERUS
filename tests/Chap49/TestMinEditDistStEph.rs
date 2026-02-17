//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistStEph.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap49::MinEditDistStEph::MinEditDistStEph::*;
use apas_verus::MinEditDistStEphLit;

#[test]
fn test_min_edit_distance_st_eph_basic() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);
    assert_eq!(solver.min_edit_distance(), 2);

    // Test ephemeral mutation
    solver.set_target(2, 'C');
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_eph_mutation() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: ['X', 'Y', 'Z']);
    assert_eq!(solver.min_edit_distance(), 6);

    // Mutate source to match target
    solver.set_source(0, 'X');
    solver.set_source(1, 'Y');
    solver.set_source(2, 'Z');
    assert_eq!(solver.min_edit_distance(), 0);

    // Mutate target back
    solver.set_target(0, 'A');
    solver.set_target(1, 'B');
    solver.set_target(2, 'C');
    assert_eq!(solver.min_edit_distance(), 6);
}

#[test]
fn test_new() {
    let mut solver = MinEditDistStEphS::<i32>::new();
    assert_eq!(solver.source().length(), 0);
    assert_eq!(solver.target().length(), 0);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_from_sequences() {
    let source = ArraySeqStEphSLit![1, 2, 3];
    let target = ArraySeqStEphSLit![1, 2, 4];
    let mut solver = MinEditDistStEphS::from_sequences(source, target);
    assert_eq!(solver.source().length(), 3);
    assert_eq!(solver.target().length(), 3);
    let dist = solver.min_edit_distance();
    assert_eq!(dist, 2);
}

#[test]
fn test_empty_sequences() {
    let mut solver: MinEditDistStEphS<char> = MinEditDistStEphLit!(source: [], target: []);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_empty_source() {
    let mut solver = MinEditDistStEphLit!(source: [], target: ['A', 'B', 'C']);
    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_empty_target() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: []);
    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_identical_sequences() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C', 'D'], target: ['A', 'B', 'C', 'D']);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_completely_different() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: ['X', 'Y', 'Z']);
    assert_eq!(solver.min_edit_distance(), 6);
}

#[test]
fn test_source_target_getters() {
    let solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['C', 'D']);
    let source = solver.source();
    let target = solver.target();
    assert_eq!(source.length(), 2);
    assert_eq!(target.length(), 2);
    assert_eq!(*source.nth(0), 'A');
    assert_eq!(*target.nth(1), 'D');
}

#[test]
fn test_source_target_mut() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['C', 'D']);

    {
        let source_mut = solver.source_mut();
        let _ = source_mut.set(0, 'X');
    }

    {
        let target_mut = solver.target_mut();
        let _ = target_mut.set(1, 'Y');
    }

    assert_eq!(*solver.source().nth(0), 'X');
    assert_eq!(*solver.target().nth(1), 'Y');
}

#[test]
fn test_memoization() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);

    assert_eq!(solver.memo_size(), 0);

    let dist1 = solver.min_edit_distance();
    let memo_size1 = solver.memo_size();
    assert!(memo_size1 > 0, "Memoization should have cached results");

    // Second call should use cached results
    let dist2 = solver.min_edit_distance();
    assert_eq!(dist1, dist2);

    // Clear memo
    solver.clear_memo();
    assert_eq!(solver.memo_size(), 0);

    // Recompute after clear
    let dist3 = solver.min_edit_distance();
    assert_eq!(dist1, dist3);
    assert!(solver.memo_size() > 0);
}

#[test]
fn test_set_source_clears_memo() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['A', 'C']);

    let _ = solver.min_edit_distance();
    assert!(solver.memo_size() > 0);

    solver.set_source(1, 'C');
    assert_eq!(solver.memo_size(), 0, "set_source should clear memo");
}

#[test]
fn test_set_target_clears_memo() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['A', 'C']);

    let _ = solver.min_edit_distance();
    assert!(solver.memo_size() > 0);

    solver.set_target(1, 'B');
    assert_eq!(solver.memo_size(), 0, "set_target should clear memo");
}

#[test]
fn test_display() {
    let solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['C', 'D']);
    let display_str = format!("{}", solver);
    assert!(display_str.contains("MinEditDistStEph"));
    assert!(display_str.contains("source:"));
    assert!(display_str.contains("target:"));
}

#[test]
fn test_into_iterator() {
    let solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['C', 'D']);
    let pairs = solver.into_iter().collect::<Vec<_>>();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, 'A');
    assert_eq!(pairs[0].1, 'C');
    assert_eq!(pairs[1].0, 'B');
    assert_eq!(pairs[1].1, 'D');
}

#[test]
fn test_into_iterator_ref() {
    let solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['C', 'D']);
    let pairs = (&solver).into_iter().collect::<Vec<_>>();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, 'A');
    assert_eq!(pairs[1].1, 'D');
}

#[test]
fn test_into_iterator_mut_ref() {
    let mut solver = MinEditDistStEphLit!(source: ['A', 'B'], target: ['C', 'D']);
    let pairs = (&mut solver).into_iter().collect::<Vec<_>>();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, 'A');
    assert_eq!(pairs[1].1, 'D');
}

#[test]
fn test_macro_empty() {
    let mut solver: MinEditDistStEphS<char> = MinEditDistStEphLit!();
    assert_eq!(solver.source().length(), 0);
    assert_eq!(solver.target().length(), 0);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_longer_sequences() {
    // "kitten" -> "sitting": 3 edits
    let mut solver = MinEditDistStEphLit!(
        source: ['k', 'i', 't', 't', 'e', 'n'],
        target: ['s', 'i', 't', 't', 'i', 'n', 'g']
    );
    let dist = solver.min_edit_distance();
    assert_eq!(dist, 5);
}
