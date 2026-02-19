//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OBSTMtEph (Optimal Binary Search Tree).

use apas_verus::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::*;
use apas_verus::Chap50::Probability::Probability::ProbabilityTrait;
use apas_verus::OBSTMtEphLit;
use apas_verus::Types::Types::*;

#[test]
fn test_obstmtephlit_macro_functionality() {
    let obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    assert_eq!(obst.keys().len(), 3);
}

#[test]
fn test_new() {
    let obst: OBSTMtEphS<i32> = OBSTMtEphTrait::new();
    assert_eq!(obst.keys().len(), 0);
}

#[test]
fn test_from_keys_probs() {
    use apas_verus::Chap50::Probability::Probability::*;
    let keys = vec![1, 2, 3];
    let probs = vec![Probability::new(0.1), Probability::new(0.2), Probability::new(0.3)];
    let obst = OBSTMtEphS::from_keys_probs(keys, probs);
    assert_eq!(obst.num_keys(), 3);
}

#[test]
fn test_optimal_cost_single_key() {
    let mut obst = OBSTMtEphLit![keys: [1], probs: [1.0]];
    let cost = obst.optimal_cost();
    assert!(cost.value() >= 0.0);
}

#[test]
fn test_optimal_cost_two_keys() {
    let mut obst = OBSTMtEphLit![keys: [1, 2], probs: [0.4, 0.6]];
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_optimal_cost_three_keys() {
    let mut obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.2, 0.3, 0.5]];
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_keys_method() {
    let obst = OBSTMtEphLit![keys: [10, 20, 30], probs: [0.1, 0.2, 0.3]];
    let keys = obst.keys();
    assert_eq!(keys.len(), 3);
}

#[test]
fn test_num_keys_method() {
    let obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    assert_eq!(obst.num_keys(), 3);
}

#[test]
fn test_num_keys() {
    let obst = OBSTMtEphLit![keys: [1, 2, 3, 4, 5], probs: [0.1, 0.2, 0.3, 0.2, 0.2]];
    assert_eq!(obst.num_keys(), 5);
}

#[test]
fn test_empty_tree() {
    let mut obst: OBSTMtEphS<i32> = OBSTMtEphTrait::new();
    let cost = obst.optimal_cost();
    assert_eq!(cost.value(), 0.0);
}

#[test]
fn test_uniform_probabilities() {
    let mut obst = OBSTMtEphLit![keys: [1, 2, 3, 4], probs: [0.25, 0.25, 0.25, 0.25]];
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_skewed_probabilities() {
    let mut obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.8, 0.1, 0.1]];
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_large_tree() {
    use apas_verus::Chap50::Probability::Probability::*;
    let keys = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let probs = vec![
        Probability::new(0.1),
        Probability::new(0.1),
        Probability::new(0.1),
        Probability::new(0.2),
        Probability::new(0.2),
        Probability::new(0.1),
        Probability::new(0.1),
        Probability::new(0.1),
    ];
    let mut obst = OBSTMtEphS::from_keys_probs(keys, probs);
    let cost = obst.optimal_cost();
    assert!(cost.value() > 0.0);
}

#[test]
fn test_parallel_execution() {
    use std::sync::Arc;
    use std::thread;
    let obst = Arc::new(OBSTMtEphLit![keys: [1, 2, 3, 4], probs: [0.1, 0.2, 0.3, 0.4]]);
    let mut handles = vec![];
    for _ in 0..4 {
        let o = Arc::clone(&obst);
        let handle = thread::spawn(move || o.keys().len());
        handles.push(handle);
    }
    for handle in handles {
        assert_eq!(handle.join().unwrap(), 4);
    }
}

#[test]
fn test_display() {
    let obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    let s = format!("{obst}");
    assert!(!s.is_empty());
}

#[test]
fn test_clone() {
    let obst1 = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    let obst2 = obst1.clone();
    assert_eq!(obst1.num_keys(), obst2.num_keys());
}

#[test]
fn test_set_key_prob() {
    use apas_verus::Chap50::Probability::Probability::*;
    let mut obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    obst.set_key_prob(
        1,
        KeyProb {
            key: 5,
            prob: Probability::new(0.5),
        },
    );
    let keys = obst.keys();
    assert_eq!(keys[1].key, 5);
}

#[test]
fn test_update_prob() {
    use apas_verus::Chap50::Probability::Probability::*;
    let mut obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    obst.update_prob(0, Probability::new(0.9));
    let keys = obst.keys();
    assert!((keys[0].prob.value() - 0.9).abs() < f64::EPSILON);
}

#[test]
fn test_clear_memo() {
    let mut obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    obst.optimal_cost();
    assert!(obst.memo_size() > 0);
    obst.clear_memo();
    assert_eq!(obst.memo_size(), 0);
}

#[test]
fn test_memo_size() {
    let mut obst = OBSTMtEphLit![keys: [1, 2], probs: [0.5, 0.5]];
    assert_eq!(obst.memo_size(), 0);
    obst.optimal_cost();
    assert!(obst.memo_size() > 0);
}

#[test]
fn test_equality() {
    let obst1 = OBSTMtEphLit![keys: [1, 2], probs: [0.5, 0.5]];
    let obst2 = OBSTMtEphLit![keys: [1, 2], probs: [0.5, 0.5]];
    assert_eq!(obst1, obst2);
}

#[test]
fn test_from_key_probs() {
    use apas_verus::Chap50::Probability::Probability::*;
    let key_probs = vec![
        KeyProb {
            key: 1,
            prob: Probability::new(0.2),
        },
        KeyProb {
            key: 2,
            prob: Probability::new(0.8),
        },
    ];
    let obst = OBSTMtEphS::from_key_probs(key_probs);
    assert_eq!(obst.num_keys(), 2);
}

#[test]
fn test_into_iter() {
    let obst = OBSTMtEphLit![keys: [1, 2, 3], probs: [0.1, 0.2, 0.3]];
    let count = obst.into_iter().count();
    assert_eq!(count, 3);
}

#[test]
fn test_into_iter_ref() {
    let obst = OBSTMtEphLit![keys: [1, 2], probs: [0.5, 0.5]];
    let count = (&obst).into_iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_keyprob_display() {
    use apas_verus::Chap50::Probability::Probability::*;
    let kp = KeyProb {
        key: 42,
        prob: Probability::new(0.333),
    };
    let s = format!("{kp}");
    assert!(s.contains("42"));
}
