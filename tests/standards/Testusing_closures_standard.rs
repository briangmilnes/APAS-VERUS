//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Runtime tests for standards::using_closures_standard.

use apas_verus::standards::using_closures_standard::using_closures_standard::*;

#[test]
fn test_tabulate() {
    let s: ExampleS<u64> = ExampleS::tabulate(&|i| (i * i) as u64, 5);
    assert_eq!(s.seq.len(), 5);
    assert_eq!(s.seq[0], 0);
    assert_eq!(s.seq[1], 1);
    assert_eq!(s.seq[2], 4);
    assert_eq!(s.seq[3], 9);
    assert_eq!(s.seq[4], 16);
}

#[test]
fn test_map_apply() {
    let s: ExampleS<u64> = ExampleS::tabulate(&|i| i as u64, 3);
    let doubled: ExampleS<u64> = s.map_apply(&|x| x * 2);
    assert_eq!(doubled.seq.len(), 3);
    assert_eq!(doubled.seq[0], 0);
    assert_eq!(doubled.seq[1], 2);
    assert_eq!(doubled.seq[2], 4);
}

#[test]
fn test_display() {
    let s: ExampleS<u64> = ExampleS::tabulate(&|i| i as u64, 3);
    assert_eq!(format!("{}", s), "[0, 1, 2]");
    assert_eq!(format!("{:?}", s), "ExampleS([0, 1, 2])");
}
