//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainMtPer.

use apas_verus::Chap50::MatrixChainMtPer::MatrixChainMtPer::*;
use apas_verus::MatrixChainMtPerLit;
use apas_verus::Types::Types::*;

#[test]
fn test_matrixchainmtperlit_macro_functionality() {
    let chain = MatrixChainMtPerLit![dims: [(10, 20), (20, 30), (30, 40)]];
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_new() {
    let chain: MatrixChainMtPerS = MatrixChainMtPerTrait::new();
    assert_eq!(chain.num_matrices(), 0);
}

#[test]
fn test_from_dimensions() {
    let dims = vec![MatrixDim { rows: 10, cols: 20 }, MatrixDim { rows: 20, cols: 30 }];
    let chain = MatrixChainMtPerS::from_dimensions(dims);
    assert_eq!(chain.num_matrices(), 2);
}

#[test]
fn test_from_dim_pairs() {
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let chain = MatrixChainMtPerS::from_dim_pairs(pairs);
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_optimal_cost_two_matrices() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 10 * 20 * 30);
}

#[test]
fn test_optimal_cost_three_matrices() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_dimensions() {
    let chain = MatrixChainMtPerLit![dims: [(5,10), (10,15)]];
    let dims = chain.dimensions();
    assert_eq!(dims.len(), 2);
    assert_eq!(dims[0].rows, 5);
    assert_eq!(dims[0].cols, 10);
}

#[test]
fn test_num_matrices() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40)]];
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_empty_chain() {
    let chain: MatrixChainMtPerS = MatrixChainMtPerTrait::new();
    assert_eq!(chain.num_matrices(), 0);
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_single_matrix() {
    let chain = MatrixChainMtPerLit![dims: [(10,20)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_four_matrices() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40), (40,50)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_large_chain() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40), (40,50), (50,60), (60,70)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_persistent_semantics() {
    let chain1 = MatrixChainMtPerLit![dims: [(10,20), (20,30)]];
    let cost1 = chain1.optimal_cost();
    let chain2 = chain1.clone();
    let cost2 = chain2.optimal_cost();
    assert_eq!(cost1, cost2);
}

#[test]
fn test_parallel_execution() {
    use std::sync::Arc;
    use std::thread;
    let chain = Arc::new(MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40), (40,50)]]);
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&chain);
        let handle = thread::spawn(move || c.dimensions().len());
        handles.push(handle);
    }
    for handle in handles {
        assert_eq!(handle.join().unwrap(), 4);
    }
}

#[test]
fn test_dimension_consistency() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40)]];
    let dims = chain.dimensions();
    assert_eq!(dims[0].cols, dims[1].rows);
    assert_eq!(dims[1].cols, dims[2].rows);
}

#[test]
fn test_display() {
    let chain = MatrixChainMtPerLit![dims: [(10,20), (20,30)]];
    let s = format!("{chain}");
    assert!(!s.is_empty());
}

#[test]
fn test_clone() {
    let chain1 = MatrixChainMtPerLit![dims: [(10,20), (20,30)]];
    let chain2 = chain1.clone();
    assert_eq!(chain1.num_matrices(), chain2.num_matrices());
}

#[test]
fn test_concurrent_reads() {
    use std::sync::Arc;
    use std::thread;
    let chain = Arc::new(MatrixChainMtPerLit![dims: [(10,20), (20,30), (30,40)]]);
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&chain);
        let handle = thread::spawn(move || (c.num_matrices(), c.optimal_cost()));
        handles.push(handle);
    }
    for handle in handles {
        let (num, cost) = handle.join().unwrap();
        assert_eq!(num, 3);
        assert!(cost > 0);
    }
}

#[test]
fn test_equality() {
    let chain1 = MatrixChainMtPerLit![dims: [(10,20), (20,30)]];
    let chain2 = MatrixChainMtPerLit![dims: [(10,20), (20,30)]];
    assert_eq!(chain1, chain2);
}

#[test]
fn test_matrixdim_clone() {
    let dim1 = MatrixDim { rows: 10, cols: 20 };
    let dim2 = dim1.clone();
    assert_eq!(dim1, dim2);
}
