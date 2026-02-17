//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MinEditDistStPer.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait;
use apas_verus::ArraySeqStPerSLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap49::MinEditDistStPer::MinEditDistStPer::*;
use apas_verus::MinEditDistStPerLit;

#[test]
fn test_min_edit_distance_st_per_example_49_3() {
    // Example 49.3: Transform S = <A, B, C, A, D, A> to T = <A, B, A, D, C>
    let solver = MinEditDistStPerLit!(
        source: ['A', 'B', 'C', 'A', 'D', 'A'],
        target: ['A', 'B', 'A', 'D', 'C']
    );
    assert_eq!(solver.min_edit_distance(), 3);
}

#[test]
fn test_min_edit_distance_st_per_basic() {
    let solver1 = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'C']);
    assert_eq!(solver1.min_edit_distance(), 0);

    let solver2 = MinEditDistStPerLit!(source: [], target: ['A', 'B', 'C']);
    assert_eq!(solver2.min_edit_distance(), 3);

    let solver3 = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: []);
    assert_eq!(solver3.min_edit_distance(), 3);

    let solver4: MinEditDistStPerS<char> = MinEditDistStPerLit!(source: [], target: []);
    assert_eq!(solver4.min_edit_distance(), 0);
}

#[test]
fn test_min_edit_distance_st_per_single_operations() {
    let solver1 = MinEditDistStPerLit!(source: ['A', 'B'], target: ['A', 'X', 'B']);
    assert_eq!(solver1.min_edit_distance(), 1);

    let solver2 = MinEditDistStPerLit!(source: ['A', 'X', 'B'], target: ['A', 'B']);
    assert_eq!(solver2.min_edit_distance(), 1);

    let solver3 = MinEditDistStPerLit!(source: ['A', 'X', 'B'], target: ['A', 'Y', 'B']);
    assert_eq!(solver3.min_edit_distance(), 2);
}

#[test]
fn test_new() {
    let solver = MinEditDistStPerS::<i32>::new();
    assert_eq!(solver.source().length(), 0);
    assert_eq!(solver.target().length(), 0);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_from_sequences() {
    let source = ArraySeqStPerSLit![1, 2, 3];
    let target = ArraySeqStPerSLit![1, 2, 4];
    let solver = MinEditDistStPerS::from_sequences(source, target);
    assert_eq!(solver.source().length(), 3);
    assert_eq!(solver.target().length(), 3);
    let dist = solver.min_edit_distance();
    assert_eq!(dist, 2);
}

#[test]
fn test_identical_sequences() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B', 'C', 'D'], target: ['A', 'B', 'C', 'D']);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_completely_different() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['X', 'Y', 'Z']);
    assert_eq!(solver.min_edit_distance(), 6);
}

#[test]
fn test_source_target_getters() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let source = solver.source();
    let target = solver.target();
    assert_eq!(source.length(), 2);
    assert_eq!(target.length(), 2);
    assert_eq!(*source.nth(0), 'A');
    assert_eq!(*target.nth(1), 'D');
}

#[test]
fn test_persistent_immutability() {
    let solver1 = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);
    let dist1 = solver1.min_edit_distance();

    // Create new solver with same data - persistent means original unchanged
    let solver2 = solver1.clone();
    let dist2 = solver2.min_edit_distance();

    assert_eq!(dist1, dist2);
    assert_eq!(solver1.source().length(), 3);
    assert_eq!(solver2.source().length(), 3);
}

#[test]
fn test_memo_size() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);

    // Initially memo is empty
    assert_eq!(solver.memo_size(), 0);

    // After computing distance, memo might be populated in internal copy
    let dist = solver.min_edit_distance();
    assert_eq!(dist, 2);

    // Original remains unchanged (persistent)
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_multiple_calls() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['X', 'Y', 'Z']);

    // Multiple calls should give same result (persistent)
    let dist1 = solver.min_edit_distance();
    let dist2 = solver.min_edit_distance();
    let dist3 = solver.min_edit_distance();

    assert_eq!(dist1, 6);
    assert_eq!(dist2, 6);
    assert_eq!(dist3, 6);
}

#[test]
fn test_display() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let display_str = format!("{}", solver);
    assert!(display_str.contains("MinEditDistStPer"));
    assert!(display_str.contains("source:"));
    assert!(display_str.contains("target:"));
}

#[test]
fn test_into_iterator() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let pairs = solver.into_iter().collect::<Vec<_>>();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, 'A');
    assert_eq!(pairs[0].1, 'C');
    assert_eq!(pairs[1].0, 'B');
    assert_eq!(pairs[1].1, 'D');
}

#[test]
fn test_into_iterator_ref() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B'], target: ['C', 'D']);
    let pairs = (&solver).into_iter().collect::<Vec<_>>();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, 'A');
    assert_eq!(pairs[1].1, 'D');

    // Original still usable (persistent)
    assert_eq!(solver.min_edit_distance(), 4);
}

#[test]
fn test_macro_empty() {
    let solver: MinEditDistStPerS<char> = MinEditDistStPerLit!();
    assert_eq!(solver.source().length(), 0);
    assert_eq!(solver.target().length(), 0);
    assert_eq!(solver.min_edit_distance(), 0);
}

#[test]
fn test_longer_sequences() {
    // "kitten" -> "sitting"
    let solver = MinEditDistStPerLit!(
        source: ['k', 'i', 't', 't', 'e', 'n'],
        target: ['s', 'i', 't', 't', 'i', 'n', 'g']
    );
    let dist = solver.min_edit_distance();
    assert_eq!(dist, 5);
}

#[test]
fn test_prefix_suffix() {
    // Test when one is prefix of other
    let solver1 = MinEditDistStPerLit!(source: ['A', 'B'], target: ['A', 'B', 'C', 'D']);
    assert_eq!(solver1.min_edit_distance(), 2);

    let solver2 = MinEditDistStPerLit!(source: ['A', 'B', 'C', 'D'], target: ['A', 'B']);
    assert_eq!(solver2.min_edit_distance(), 2);
}

#[test]
fn test_single_char_difference() {
    let solver = MinEditDistStPerLit!(source: ['A', 'B', 'C'], target: ['A', 'B', 'D']);
    assert_eq!(solver.min_edit_distance(), 2);
}

#[test]
fn test_with_integers() {
    let solver = MinEditDistStPerLit!(source: [1, 2, 3, 4], target: [1, 3, 4, 5]);
    let dist = solver.min_edit_distance();
    assert_eq!(dist, 2); // delete 2, insert 5
}
