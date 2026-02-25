//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap50 OptBinSearchTreeMtPer.

use apas_verus::Chap50::OptBinSearchTreeMtPer::OptBinSearchTreeMtPer::*;
use apas_verus::Chap30::Probability::Probability::*;
use apas_verus::OBSTMtPerLit;
use apas_verus::Types::Types::*;

#[test]
fn test_obst_empty() {
    let obst: OBSTMtPerS<char> = OBSTMtPerLit!();
    assert_eq!(obst.num_keys(), 0);
    assert_eq!(obst.optimal_cost(), Probability::zero());
}

#[test]
fn test_obst_single_key() {
    let obst = OBSTMtPerLit!(
        keys: ['a'],
        probs: [1.0]
    );
    assert_eq!(obst.num_keys(), 1);
    let cost = obst.optimal_cost();
    assert!((cost.value() - 1.0).abs() < 0.001);
}

#[test]
fn test_obst_two_keys() {
    let obst = OBSTMtPerLit!(
        keys: ['a', 'b'],
        probs: [0.5, 0.5]
    );
    assert_eq!(obst.num_keys(), 2);
    let cost = obst.optimal_cost();
    // Expected cost: 1.5 (optimal tree has one key at root, other at depth 1)
    assert!((cost.value() - 1.5).abs() < 0.001);
}

#[test]
fn test_obst_three_keys() {
    let obst = OBSTMtPerLit!(
        keys: ['a', 'b', 'c'],
        probs: [0.25, 0.5, 0.25]
    );
    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    // With probabilities 0.25, 0.5, 0.25, optimal tree has 'b' at root
    // Cost = 0.25*2 + 0.5*1 + 0.25*2 = 1.5
    assert!((cost.value() - 1.5).abs() < 0.001);
}

#[test]
fn test_obst_four_keys_equal_probs() {
    let obst = OBSTMtPerLit!(
        keys: [1, 2, 3, 4],
        probs: [0.25, 0.25, 0.25, 0.25]
    );
    assert_eq!(obst.num_keys(), 4);
    let cost = obst.optimal_cost();
    // With equal probabilities, optimal tree is balanced
    // Expected cost should be reasonable
    assert!(cost.value() > 1.0 && cost.value() < 4.0);
}

#[test]
fn test_obst_five_keys() {
    let obst = OBSTMtPerLit!(
        keys: [1, 2, 3, 4, 5],
        probs: [0.1, 0.2, 0.4, 0.2, 0.1]
    );
    assert_eq!(obst.num_keys(), 5);
    let cost = obst.optimal_cost();
    // With skewed probabilities favoring middle key
    // Expected cost should be relatively low
    assert!(cost.value() > 1.0 && cost.value() < 3.0);
}

#[test]
fn test_obst_skewed_probabilities() {
    let obst = OBSTMtPerLit!(
        keys: ['a', 'b', 'c'],
        probs: [0.1, 0.1, 0.8]
    );
    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    // With highly skewed probability, 'c' should be at or near root
    // Expected cost should be reasonable for skewed distribution
    assert!(cost.value() > 1.0 && cost.value() < 2.0);
}

#[test]
fn test_obst_from_keys_probs() {
    let keys = vec!['x', 'y', 'z'];
    let probs = vec![Probability::new(0.3), Probability::new(0.4), Probability::new(0.3)];
    let obst = OBSTMtPerS::from_keys_probs(keys, probs);
    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    assert!(cost.value() > 1.0 && cost.value() < 3.0);
}

#[test]
fn test_obst_from_key_probs() {
    let key_probs = vec![
        KeyProb {
            key: 1,
            prob: Probability::new(0.25),
        },
        KeyProb {
            key: 2,
            prob: Probability::new(0.75),
        },
    ];
    let obst = OBSTMtPerS::from_key_probs(key_probs);
    assert_eq!(obst.num_keys(), 2);
    let cost = obst.optimal_cost();
    // Key 2 has higher probability, should be at root
    // Cost = 0.25*2 + 0.75*1 = 1.25
    assert!((cost.value() - 1.25).abs() < 0.001);
}

#[test]
fn test_obst_memo_cache() {
    let obst = OBSTMtPerLit!(
        keys: [1, 2, 3, 4],
        probs: [0.25, 0.25, 0.25, 0.25]
    );
    let _ = obst.optimal_cost();
    // After computing optimal cost, memo should have entries
    assert!(obst.memo_size() > 0);
}

#[test]
fn test_obst_display() {
    let obst = OBSTMtPerLit!(
        keys: ['a', 'b'],
        probs: [0.5, 0.5]
    );
    let display_str = format!("{obst}");
    assert!(display_str.contains("OBSTMtPer"));
    assert!(display_str.contains("keys: 2"));
}

#[test]
fn test_obst_equality() {
    let obst1 = OBSTMtPerLit!(
        keys: ['a', 'b'],
        probs: [0.5, 0.5]
    );
    let obst2 = OBSTMtPerLit!(
        keys: ['a', 'b'],
        probs: [0.5, 0.5]
    );
    assert_eq!(obst1, obst2);
}

#[test]
fn test_obst_iterability() {
    let obst = OBSTMtPerLit!(
        keys: [1, 2, 3],
        probs: [0.3, 0.4, 0.3]
    );
    let mut count = 0;
    for _key_prob in &obst {
        count += 1;
    }
    assert_eq!(count, 3);
}

#[test]
fn test_keyprob_display() {
    let kp = KeyProb {
        key: 42,
        prob: Probability::new(0.75),
    };
    let display_str = format!("{}", kp);
    assert!(display_str.contains("42"));
    assert!(display_str.contains("0.75"));
}
