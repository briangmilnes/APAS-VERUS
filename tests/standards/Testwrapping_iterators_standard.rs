//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Runtime tests for standards::wrapping_iterators_standard.

use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

#[test]
fn test_inner_new_and_iter() {
    let s: InnerS<u64> = InnerS::new(3, 42);
    assert_eq!(s.seq.len(), 3);
    let items: Vec<&u64> = s.iter().collect();
    assert_eq!(items, vec![&42, &42, &42]);
}

#[test]
fn test_outer_new_and_iter() {
    let s: OuterS<u64> = OuterS::new(4, 10);
    assert_eq!(s.data.seq.len(), 4);
    let items: Vec<&u64> = s.iter().collect();
    assert_eq!(items, vec![&10, &10, &10, &10]);
}

#[test]
fn test_outer_into_iter() {
    let s: OuterS<u64> = OuterS::new(2, 7);
    let items: Vec<&u64> = (&s).into_iter().collect();
    assert_eq!(items, vec![&7, &7]);
}

#[test]
fn test_display() {
    let s: OuterS<u64> = OuterS::new(3, 1);
    assert_eq!(format!("{}", s), "[1, 1, 1]");
    assert_eq!(format!("{:?}", s), "OuterS(InnerS([1, 1, 1]))");
}
