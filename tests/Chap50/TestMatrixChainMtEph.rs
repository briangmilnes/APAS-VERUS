//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MatrixChainMtEph.

use apas_verus::Chap50::MatrixChainMtEph::MatrixChainMtEph::*;
use apas_verus::MatrixChainMtEphLit;
use apas_verus::Types::Types::*;

#[test]
fn test_matrixchainmtephlit_macro_functionality() {
    let chain = MatrixChainMtEphLit![dims: [(10, 20), (20, 30), (30, 40)]];
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_new() {
    let chain: MatrixChainMtEphS = MatrixChainMtEphTrait::new();
    assert_eq!(chain.num_matrices(), 0);
}

#[test]
fn test_from_dimensions() {
    let dims = vec![MatrixDim { rows: 10, cols: 20 }, MatrixDim { rows: 20, cols: 30 }];
    let chain = MatrixChainMtEphS::from_dimensions(dims);
    assert_eq!(chain.num_matrices(), 2);
}

#[test]
fn test_from_dim_pairs() {
    let pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let chain = MatrixChainMtEphS::from_dim_pairs(pairs);
    assert_eq!(chain.num_matrices(), 3);
}

#[test]
fn test_optimal_cost_two_matrices() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 10 * 20 * 30);
}

#[test]
fn test_optimal_cost_three_matrices() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_dimensions() {
    let chain = MatrixChainMtEphLit![dims: [(5,10), (10,15)]];
    let dims = chain.dimensions();
    assert_eq!(dims.len(), 2);
    assert_eq!(dims[0].rows, 5);
    assert_eq!(dims[0].cols, 10);
}

#[test]
fn test_set_dimension() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    chain.set_dimension(0, MatrixDim { rows: 5, cols: 10 });
    let dims = chain.dimensions();
    assert_eq!(dims[0].rows, 5);
}

#[test]
fn test_update_dimension() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    chain.update_dimension(1, 25, 35);
    let dims = chain.dimensions();
    assert_eq!(dims[1].rows, 25);
    assert_eq!(dims[1].cols, 35);
}

#[test]
fn test_clear_memo() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    chain.optimal_cost();
    assert!(chain.memo_size() > 0);
    chain.clear_memo();
    assert_eq!(chain.memo_size(), 0);
}

#[test]
fn test_memo_size() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40)]];
    assert_eq!(chain.memo_size(), 0);
    chain.optimal_cost();
    assert!(chain.memo_size() > 0);
}

#[test]
fn test_empty_chain() {
    let mut chain: MatrixChainMtEphS = MatrixChainMtEphTrait::new();
    assert_eq!(chain.num_matrices(), 0);
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_single_matrix() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20)]];
    let cost = chain.optimal_cost();
    assert_eq!(cost, 0);
}

#[test]
fn test_four_matrices() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40), (40,50)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_large_chain() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40), (40,50), (50,60), (60,70)]];
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_memoization_reuse() {
    let mut chain = MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40)]];
    let cost1 = chain.optimal_cost();
    let size1 = chain.memo_size();
    let cost2 = chain.optimal_cost();
    assert_eq!(cost1, cost2);
    assert_eq!(chain.memo_size(), size1);
}

#[test]
fn test_parallel_execution() {
    use std::sync::Arc;
    use std::thread;
    let chain = Arc::new(MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40), (40,50)]]);
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
    let chain = MatrixChainMtEphLit![dims: [(10,20), (20,30), (30,40)]];
    let dims = chain.dimensions();
    assert_eq!(dims[0].cols, dims[1].rows);
    assert_eq!(dims[1].cols, dims[2].rows);
}

#[test]
fn test_display() {
    let chain = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    let s = format!("{chain}");
    assert!(!s.is_empty());
}

#[test]
fn test_clone() {
    let chain1 = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    let chain2 = chain1.clone();
    assert_eq!(chain1.num_matrices(), chain2.num_matrices());
}

#[test]
fn test_equality() {
    let chain1 = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    let chain2 = MatrixChainMtEphLit![dims: [(10,20), (20,30)]];
    assert_eq!(chain1, chain2);
}

#[test]
fn test_matrixdim_equality() {
    let dim1 = MatrixDim { rows: 10, cols: 20 };
    let dim2 = MatrixDim { rows: 10, cols: 20 };
    assert_eq!(dim1, dim2);
}
