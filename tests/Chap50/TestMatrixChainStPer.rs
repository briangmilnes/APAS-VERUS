//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Matrix Chain Multiplication StPer implementation.

use apas_verus::Chap50::MatrixChainStPer::MatrixChainStPer::*;
use apas_verus::MatrixChainStPerLit;
use apas_verus::Types::Types::*;

#[test]
fn test_matrix_chain_st_per_empty() {
    let chain = MatrixChainStPerS::new();
    assert_eq!(chain.num_matrices(), 0);
    assert_eq!(chain.optimal_cost(), 0);
}

#[test]
fn test_matrix_chain_st_per_single_matrix() {
    let dimensions = vec![MatrixDim { rows: 10, cols: 20 }];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    assert_eq!(chain.num_matrices(), 1);
    assert_eq!(chain.optimal_cost(), 0); // No multiplication needed for single matrix
}

#[test]
fn test_matrix_chain_st_per_two_matrices() {
    let dimensions = vec![MatrixDim { rows: 10, cols: 20 }, MatrixDim { rows: 20, cols: 30 }];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    assert_eq!(chain.num_matrices(), 2);
    // Cost should be 10 * 20 * 30 = 6000
    assert_eq!(chain.optimal_cost(), 6000);
}

#[test]
fn test_matrix_chain_st_per_three_matrices() {
    let dimensions = vec![
        MatrixDim { rows: 10, cols: 20 },
        MatrixDim { rows: 20, cols: 30 },
        MatrixDim { rows: 30, cols: 40 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    assert_eq!(chain.num_matrices(), 3);
    // Optimal cost should be minimum of:
    // (A*B)*C: 10*20*30 + 10*30*40 = 6000 + 12000 = 18000
    // A*(B*C): 20*30*40 + 10*20*40 = 24000 + 8000 = 32000
    // So optimal is 18000
    assert_eq!(chain.optimal_cost(), 18000);
}

#[test]
fn test_matrix_chain_st_per_from_dim_pairs() {
    let dim_pairs = vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)];
    let chain = MatrixChainStPerS::from_dim_pairs(dim_pairs);
    assert_eq!(chain.num_matrices(), 3);
    assert_eq!(chain.optimal_cost(), 18000);
}

#[test]
fn test_matrix_chain_st_per_iteration() {
    let dimensions = vec![MatrixDim { rows: 5, cols: 10 }, MatrixDim { rows: 10, cols: 15 }];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);

    let collected = chain.into_iter().collect::<Vec<MatrixDim>>();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].rows, 5);
    assert_eq!(collected[0].cols, 10);
    assert_eq!(collected[1].rows, 10);
    assert_eq!(collected[1].cols, 15);
}

#[test]
fn test_matrix_dim_display() {
    let dim = MatrixDim { rows: 10, cols: 20 };
    let display_str = format!("{dim}");
    assert!(display_str.contains("10"));
    assert!(display_str.contains("20"));
}

#[test]
fn test_matrix_chain_macro() {
    let chain = MatrixChainStPerLit![
        dims: [(10, 20), (20, 30)]
    ];
    assert_eq!(chain.num_matrices(), 2);
    assert_eq!(chain.optimal_cost(), 6000);
}

#[test]
fn test_matrix_chain_large_example() {
    // Example from textbook: matrices with dimensions
    // A: 40x20, B: 20x30, C: 30x10, D: 10x30
    let dimensions = vec![
        MatrixDim { rows: 40, cols: 20 },
        MatrixDim { rows: 20, cols: 30 },
        MatrixDim { rows: 30, cols: 10 },
        MatrixDim { rows: 10, cols: 30 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    assert_eq!(chain.num_matrices(), 4);

    // The optimal cost should be computed by dynamic programming
    let cost = chain.optimal_cost();
    assert!(cost > 0);
    assert!(cost < 100000); // Reasonable upper bound
}

#[test]
fn test_matrix_chain_multiply_cost() {
    let dimensions = vec![
        MatrixDim { rows: 2, cols: 3 },
        MatrixDim { rows: 3, cols: 4 },
        MatrixDim { rows: 4, cols: 5 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);

    // Test the multiply_cost function indirectly through optimal_cost
    let cost = chain.optimal_cost();
    // For 3 matrices, we should get the minimum of two possible parenthesizations
    assert!(cost > 0);
}

#[test]
fn test_dimensions_accessor() {
    let dimensions = vec![
        MatrixDim { rows: 10, cols: 20 },
        MatrixDim { rows: 20, cols: 30 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions.clone());
    
    let dims = chain.dimensions();
    assert_eq!(dims.len(), 2);
    assert_eq!(dims[0].rows, 10);
    assert_eq!(dims[0].cols, 20);
    assert_eq!(dims[1].rows, 20);
    assert_eq!(dims[1].cols, 30);
}

#[test]
fn test_memo_size() {
    let dimensions = vec![
        MatrixDim { rows: 10, cols: 20 },
        MatrixDim { rows: 20, cols: 30 },
        MatrixDim { rows: 30, cols: 40 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    
    // Before computing, memo should be empty
    assert_eq!(chain.memo_size(), 0);
    
    // After computing optimal cost, memo should have entries
    let _ = chain.optimal_cost();
    // Note: optimal_cost creates a mutable copy, so original memo stays empty
    // This is testing the persistent behavior
    assert_eq!(chain.memo_size(), 0);
}

#[test]
fn test_matrix_chain_display() {
    let dimensions = vec![
        MatrixDim { rows: 10, cols: 20 },
        MatrixDim { rows: 20, cols: 30 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    
    let display_str = format!("{chain}");
    assert!(display_str.contains("MatrixChainStPer"));
    assert!(display_str.contains("2")); // 2 matrices
}

#[test]
fn test_into_iter_by_ref() {
    let dimensions = vec![
        MatrixDim { rows: 5, cols: 10 },
        MatrixDim { rows: 10, cols: 15 },
    ];
    let chain = MatrixChainStPerS::from_dimensions(dimensions);
    
    // Test iterating by reference
    let collected = (&chain).into_iter().collect::<Vec<MatrixDim>>();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].rows, 5);
    assert_eq!(collected[0].cols, 10);
    
    // Chain should still be valid after reference iteration
    assert_eq!(chain.num_matrices(), 2);
}
