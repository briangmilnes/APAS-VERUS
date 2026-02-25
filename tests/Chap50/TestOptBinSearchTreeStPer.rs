//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OptBinSearchTreeStPer.

use apas_verus::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::KeyProb as OBSTStPerKeyProb;
use apas_verus::Chap50::OptBinSearchTreeStPer::OptBinSearchTreeStPer::{OBSTStPerS, OBSTStPerTrait};
use apas_verus::Chap30::Probability::Probability::*;
use apas_verus::{OBSTStPerLit, prob};

#[test]
fn test_obst_st_per_empty() {
    let obst = OBSTStPerS::<char>::new();
    assert_eq!(obst.optimal_cost(), Probability::zero());
    assert_eq!(obst.num_keys(), 0);
}

#[test]
fn test_obst_st_per_single_key() {
    let keys = vec!['A'];
    let probs = vec![prob!(0.5)];
    let obst = OBSTStPerS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 1);
    assert_eq!(obst.optimal_cost(), prob!(0.5));
}

#[test]
fn test_obst_st_per_two_keys() {
    let keys = vec!['A', 'B'];
    let probs = vec![prob!(0.3), prob!(0.7)];
    let obst = OBSTStPerS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 2);
    let cost = obst.optimal_cost();
    assert!(cost.0 >= 1.0);
    assert!(cost.0 <= 3.0);
}

#[test]
fn test_obst_st_per_three_keys() {
    let keys = vec!['A', 'B', 'C'];
    let probs = vec![prob!(0.1), prob!(0.2), prob!(0.4)];
    let obst = OBSTStPerS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    assert!(cost.0 > 0.0);
    assert!(cost.0 < 10.0);
}

#[test]
fn test_obst_macro() {
    let obst = OBSTStPerLit![
        keys: ['A', 'B'],
        probs: [0.3, 0.7]
    ];
    assert_eq!(obst.num_keys(), 2);
}

#[test]
fn test_obst_st_per_from_key_probs() {
    let key_probs = vec![
        OBSTStPerKeyProb {
            key: 'A',
            prob: prob!(0.2),
        },
        OBSTStPerKeyProb {
            key: 'B',
            prob: prob!(0.5),
        },
        OBSTStPerKeyProb {
            key: 'C',
            prob: prob!(0.3),
        },
    ];
    let obst = OBSTStPerS::from_key_probs(key_probs);
    assert_eq!(obst.num_keys(), 3);
    let cost = obst.optimal_cost();
    assert!(cost.0 > 0.0);
}

#[test]
fn test_obst_st_per_keys_getter() {
    let keys = vec!['X', 'Y', 'Z'];
    let probs = vec![prob!(0.1), prob!(0.3), prob!(0.6)];
    let obst = OBSTStPerS::from_keys_probs(keys.clone(), probs);

    let retrieved_keys = obst.keys();
    assert_eq!(retrieved_keys.len(), 3);
    assert_eq!(retrieved_keys[0].key, 'X');
    assert_eq!(retrieved_keys[1].key, 'Y');
    assert_eq!(retrieved_keys[2].key, 'Z');
}

#[test]
fn test_obst_st_per_memo_size() {
    let obst = OBSTStPerS::from_keys_probs(vec!['A', 'B'], vec![prob!(0.4), prob!(0.6)]);
    // Memo size should be 0 initially (optimal_cost creates a clone, doesn't mutate original)
    assert_eq!(obst.memo_size(), 0);

    // Even after computing optimal cost, original's memo stays empty
    obst.optimal_cost();
    assert_eq!(obst.memo_size(), 0);
}

#[test]
fn test_obst_st_per_display() {
    let obst = OBSTStPerS::from_keys_probs(vec!['A', 'B'], vec![prob!(0.3), prob!(0.7)]);
    let display_str = format!("{}", obst);
    assert!(display_str.contains("OBSTStPer"));
    assert!(display_str.contains("keys"));
}

#[test]
fn test_obst_st_per_key_prob_display() {
    let key_prob = OBSTStPerKeyProb {
        key: 'X',
        prob: prob!(0.123),
    };
    let display_str = format!("{}", key_prob);
    assert!(display_str.contains("X"));
    assert!(display_str.contains("0.123"));
}

#[test]
fn test_obst_st_per_into_iter_owned() {
    let obst = OBSTStPerS::from_keys_probs(vec!['A', 'B', 'C'], vec![prob!(0.2), prob!(0.3), prob!(0.5)]);
    let keys = obst.into_iter().map(|kp| kp.key).collect::<Vec<char>>();
    assert_eq!(keys, vec!['A', 'B', 'C']);
}

#[test]
fn test_obst_st_per_into_iter_ref() {
    let obst = OBSTStPerS::from_keys_probs(vec!['A', 'B'], vec![prob!(0.4), prob!(0.6)]);
    let keys = (&obst).into_iter().map(|kp| kp.key).collect::<Vec<char>>();
    assert_eq!(keys, vec!['A', 'B']);
    // OBST should still be usable after borrowing
    assert_eq!(obst.num_keys(), 2);
}
