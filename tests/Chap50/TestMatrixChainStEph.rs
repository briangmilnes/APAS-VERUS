//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Matrix Chain Multiplication StEph implementation.

use apas_verus::Chap50::MatrixChainStEph::MatrixChainStEph::{MatrixChainStEphS, MatrixChainStEphTrait};
use apas_verus::Chap50::MatrixChainStEph::MatrixChainStEph::MatrixDim as MatrixChainStEphMatrixDim;
use apas_verus::MatrixChainStEphLit;
use apas_verus::Types::Types::Pair;

#[test]
fn test_matrix_chain_st_eph_empty() {
    let mut chain = MatrixChainStEphS::new();
    assert_eq!(chain.num_matrices(), 0);
    assert_eq!(chain.optimal_cost(), 0);
}

#[test]
fn test_matrix_chain_st_eph_single_matrix() {
    let dimensions = vec![MatrixChainStEphMatrixDim { rows: 10, cols: 20 }];
    let mut chain = MatrixChainStEphS::from_dimensions(dimensions);
    assert_eq!(chain.num_matrices(), 1);
    assert_eq!(chain.optimal_cost(), 0);
}

#[test]
fn test_matrix_chain_st_eph_mutation() {
    let dimensions = vec![
        MatrixChainStEphMatrixDim { rows: 10, cols: 20 },
        MatrixChainStEphMatrixDim { rows: 20, cols: 30 },
    ];
    let mut chain = MatrixChainStEphS::from_dimensions(dimensions);

    // Test mutation
    chain.set_dimension(0, MatrixChainStEphMatrixDim { rows: 15, cols: 25 });
    assert_eq!(chain.dimensions()[0].rows, 15);
    assert_eq!(chain.dimensions()[0].cols, 25);

    // Test mutable access
    chain.dimensions[1] = MatrixChainStEphMatrixDim { rows: 25, cols: 35 };
    assert_eq!(chain.dimensions()[1].rows, 25);
    assert_eq!(chain.dimensions()[1].cols, 35);
}

#[test]
fn test_matrix_chain_st_eph_iteration() {
    let dimensions = vec![
        MatrixChainStEphMatrixDim { rows: 5, cols: 10 },
        MatrixChainStEphMatrixDim { rows: 10, cols: 15 },
    ];
    let chain = MatrixChainStEphS::from_dimensions(dimensions);

    let collected = chain.into_iter().collect::<Vec<MatrixChainStEphMatrixDim>>();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].rows, 5);
    assert_eq!(collected[0].cols, 10);
    assert_eq!(collected[1].rows, 10);
    assert_eq!(collected[1].cols, 15);
}

#[test]
fn test_from_dim_pairs() {
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)]);
    assert_eq!(chain.num_matrices(), 3);
    assert_eq!(chain.dimensions()[0].rows, 10);
    assert_eq!(chain.dimensions()[0].cols, 20);
    assert_eq!(chain.dimensions()[2].rows, 30);
    assert_eq!(chain.dimensions()[2].cols, 40);

    // Verify optimal_cost works
    let cost = chain.optimal_cost();
    assert!(cost > 0);
}

#[test]
fn test_optimal_cost_two_matrices() {
    // A1: 10×20, A2: 20×30
    // Cost = 10 * 20 * 30 = 6000
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30)]);
    let cost = chain.optimal_cost();
    assert_eq!(cost, 6000);
}

#[test]
fn test_optimal_cost_three_matrices() {
    // A1: 10×20, A2: 20×30, A3: 30×40
    // Best order: (A1 × A2) × A3
    // Cost = (10*20*30) + (10*30*40) = 6000 + 12000 = 18000
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)]);
    let cost = chain.optimal_cost();
    assert_eq!(cost, 18000);
}

#[test]
fn test_optimal_cost_four_matrices() {
    // Classic example: A1: 10×100, A2: 100×5, A3: 5×50, A4: 50×1
    // The algorithm will find the optimal parenthesization
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 100), Pair(100, 5), Pair(5, 50), Pair(50, 1)]);
    let cost = chain.optimal_cost();
    // Verify it computes some cost (the optimal one)
    assert_eq!(cost, 1750);
}

#[test]
fn test_memoization() {
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30), Pair(30, 40)]);

    assert_eq!(chain.memo_size(), 0);

    let cost1 = chain.optimal_cost();
    let memo_size1 = chain.memo_size();
    assert!(memo_size1 > 0, "Memoization should have cached results");

    // Second call should use cached results
    let cost2 = chain.optimal_cost();
    assert_eq!(cost1, cost2);

    // Clear memo
    chain.clear_memo();
    assert_eq!(chain.memo_size(), 0);

    // Recompute after clear
    let cost3 = chain.optimal_cost();
    assert_eq!(cost1, cost3);
    assert!(chain.memo_size() > 0);
}

#[test]
fn test_update_dimension() {
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30)]);

    let cost1 = chain.optimal_cost();
    let memo_size_before = chain.memo_size();
    assert!(memo_size_before > 0);

    // Update dimension should clear memo
    chain.update_dimension(0, 15, 25);
    assert_eq!(chain.memo_size(), 0, "update_dimension should clear memo");
    assert_eq!(chain.dimensions()[0].rows, 15);
    assert_eq!(chain.dimensions()[0].cols, 25);

    // Cost should be different now
    let cost2 = chain.optimal_cost();
    assert_ne!(cost1, cost2);
}

#[test]
fn test_update_dimension_out_of_bounds() {
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30)]);

    // Should not panic or change anything
    chain.update_dimension(5, 100, 200);
    assert_eq!(chain.num_matrices(), 2);
    assert_eq!(chain.dimensions()[0].rows, 10);
}

#[test]
fn test_set_dimension_out_of_bounds() {
    let mut chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20)]);

    // Should not panic
    chain.set_dimension(5, MatrixChainStEphMatrixDim { rows: 100, cols: 200 });
    assert_eq!(chain.num_matrices(), 1);
}

#[test]
fn test_display_matrix_chain() {
    let chain = MatrixChainStEphS::from_dim_pairs(vec![Pair(10, 20), Pair(20, 30)]);

    let display_str = format!("{}", chain);
    assert!(display_str.contains("MatrixChainStEph"));
    assert!(display_str.contains("matrices: 2"));
}

#[test]
fn test_display_matrix_dim() {
    let dim = MatrixChainStEphMatrixDim { rows: 10, cols: 20 };
    let display_str = format!("{}", dim);
    assert_eq!(display_str, "10×20");
}

#[test]
fn test_into_iterator_ref() {
    let dimensions = vec![
        MatrixChainStEphMatrixDim { rows: 5, cols: 10 },
        MatrixChainStEphMatrixDim { rows: 10, cols: 15 },
    ];
    let chain = MatrixChainStEphS::from_dimensions(dimensions);

    let collected = (&chain).into_iter().collect::<Vec<MatrixChainStEphMatrixDim>>();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].rows, 5);
    assert_eq!(collected[1].rows, 10);
}

#[test]
fn test_into_iterator_mut_ref() {
    let dimensions = vec![
        MatrixChainStEphMatrixDim { rows: 5, cols: 10 },
        MatrixChainStEphMatrixDim { rows: 10, cols: 15 },
    ];
    let mut chain = MatrixChainStEphS::from_dimensions(dimensions);

    let collected = (&mut chain).into_iter().collect::<Vec<MatrixChainStEphMatrixDim>>();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].rows, 5);
    assert_eq!(collected[1].rows, 10);
}

#[test]
fn test_macro_with_dims() {
    let mut chain = MatrixChainStEphLit!(dims: [(10, 20), (20, 30), (30, 40)]);
    assert_eq!(chain.num_matrices(), 3);
    assert_eq!(chain.dimensions()[0].rows, 10);
    assert_eq!(chain.dimensions()[2].cols, 40);

    let cost = chain.optimal_cost();
    assert_eq!(cost, 18000);
}

#[test]
fn test_macro_empty() {
    let mut chain = MatrixChainStEphLit!();
    assert_eq!(chain.num_matrices(), 0);
    assert_eq!(chain.optimal_cost(), 0);
}
