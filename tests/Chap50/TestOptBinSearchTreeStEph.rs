//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for OptBinSearchTreeStEph.

use apas_verus::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::KeyProb as OBSTStEphKeyProb;
use apas_verus::Chap50::OptBinSearchTreeStEph::OptBinSearchTreeStEph::{OBSTStEphS, OBSTStEphTrait};
use apas_verus::Chap50::Probability::Probability::*;
use apas_verus::{OBSTStEphLit, prob};

#[test]
fn test_obst_st_eph_empty() {
    let mut obst = OBSTStEphS::<char>::new();
    assert_eq!(obst.optimal_cost(), Probability::zero());
    assert_eq!(obst.num_keys(), 0);
}

#[test]
fn test_obst_st_eph_single_key() {
    let keys = vec!['A'];
    let probs = vec![prob!(0.5)];
    let mut obst = OBSTStEphS::from_keys_probs(keys, probs);

    assert_eq!(obst.num_keys(), 1);
    assert_eq!(obst.optimal_cost(), prob!(0.5));
}

#[test]
fn test_obst_st_eph_mutation() {
    let keys1 = vec!['A', 'B'];
    let probs1 = vec![prob!(0.3), prob!(0.7)];
    let mut obst1 = OBSTStEphS::from_keys_probs(keys1, probs1);
    let cost1 = obst1.optimal_cost();

    let keys2 = vec!['A', 'B'];
    let probs2 = vec![prob!(0.5), prob!(0.5)];
    let mut obst2 = OBSTStEphS::from_keys_probs(keys2, probs2);
    let cost2 = obst2.optimal_cost();

    assert!(cost1 != cost2);
}

#[test]
fn test_from_key_probs() {
    let key_probs = vec![
        OBSTStEphKeyProb {
            key: 'A',
            prob: prob!(0.3),
        },
        OBSTStEphKeyProb {
            key: 'B',
            prob: prob!(0.4),
        },
        OBSTStEphKeyProb {
            key: 'C',
            prob: prob!(0.3),
        },
    ];
    let mut obst = OBSTStEphS::from_key_probs(key_probs);
    assert_eq!(obst.num_keys(), 3);

    let cost = obst.optimal_cost();
    assert!(cost > Probability::zero());
}

#[test]
fn test_two_keys() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);
    let cost = obst.optimal_cost();
    assert_eq!(cost, prob!(0.4) + prob!(0.6) + prob!(0.4));
}

#[test]
fn test_three_keys() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B', 'C'], probs: [0.2, 0.3, 0.5]);
    let cost = obst.optimal_cost();
    // Cost should be positive
    assert!(cost > Probability::zero());
}

#[test]
fn test_keys_getter() {
    let obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);
    let keys = obst.keys();
    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0].key, 'A');
    assert_eq!(keys[0].prob, prob!(0.4));
    assert_eq!(keys[1].key, 'B');
    assert_eq!(keys[1].prob, prob!(0.6));
}

#[test]
fn test_keys_mut() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);

    {
        let keys_mut = obst.keys_mut();
        keys_mut[0].prob = prob!(0.5);
    }

    assert_eq!(obst.keys()[0].prob, prob!(0.5));
}

#[test]
fn test_set_key_prob() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);

    let cost1 = obst.optimal_cost();
    let memo_size_before = obst.memo_size();
    assert!(memo_size_before > 0);

    obst.set_key_prob(
        1,
        OBSTStEphKeyProb {
            key: 'C',
            prob: prob!(0.8),
        },
    );
    assert_eq!(obst.memo_size(), 0, "set_key_prob should clear memo");
    assert_eq!(obst.keys()[1].key, 'C');
    assert_eq!(obst.keys()[1].prob, prob!(0.8));

    let cost2 = obst.optimal_cost();
    assert_ne!(cost1, cost2);
}

#[test]
fn test_set_key_prob_out_of_bounds() {
    let mut obst = OBSTStEphLit!(keys: ['A'], probs: [0.5]);

    // Should not panic
    obst.set_key_prob(
        5,
        OBSTStEphKeyProb {
            key: 'Z',
            prob: prob!(0.9),
        },
    );
    assert_eq!(obst.num_keys(), 1);
    assert_eq!(obst.keys()[0].key, 'A');
}

#[test]
fn test_update_prob() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);

    let cost1 = obst.optimal_cost();
    let memo_size_before = obst.memo_size();
    assert!(memo_size_before > 0);

    obst.update_prob(0, prob!(0.7));
    assert_eq!(obst.memo_size(), 0, "update_prob should clear memo");
    assert_eq!(obst.keys()[0].prob, prob!(0.7));

    let cost2 = obst.optimal_cost();
    assert_ne!(cost1, cost2);
}

#[test]
fn test_update_prob_out_of_bounds() {
    let mut obst = OBSTStEphLit!(keys: ['A'], probs: [0.5]);

    // Should not panic
    obst.update_prob(5, prob!(0.9));
    assert_eq!(obst.num_keys(), 1);
    assert_eq!(obst.keys()[0].prob, prob!(0.5));
}

#[test]
fn test_memoization() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B', 'C'], probs: [0.2, 0.3, 0.5]);

    assert_eq!(obst.memo_size(), 0);

    let cost1 = obst.optimal_cost();
    let memo_size1 = obst.memo_size();
    assert!(memo_size1 > 0, "Memoization should have cached results");

    // Second call should use cached results
    let cost2 = obst.optimal_cost();
    assert_eq!(cost1, cost2);

    // Clear memo
    obst.clear_memo();
    assert_eq!(obst.memo_size(), 0);

    // Recompute after clear
    let cost3 = obst.optimal_cost();
    assert_eq!(cost1, cost3);
    assert!(obst.memo_size() > 0);
}

#[test]
fn test_display_obst() {
    let obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);
    let display_str = format!("{}", obst);
    assert!(display_str.contains("OBSTStEph"));
    assert!(display_str.contains("keys: 2"));
}

#[test]
fn test_display_key_prob() {
    let kp = OBSTStEphKeyProb {
        key: 'A',
        prob: prob!(0.5),
    };
    let display_str = format!("{}", kp);
    assert!(display_str.contains("A"));
    assert!(display_str.contains("0.5"));
}

#[test]
fn test_into_iterator() {
    let obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);
    let key_probs = obst.into_iter().collect::<Vec<_>>();
    assert_eq!(key_probs.len(), 2);
    assert_eq!(key_probs[0].key, 'A');
    assert_eq!(key_probs[0].prob, prob!(0.4));
    assert_eq!(key_probs[1].key, 'B');
    assert_eq!(key_probs[1].prob, prob!(0.6));
}

#[test]
fn test_into_iterator_ref() {
    let obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);
    let key_probs = (&obst).into_iter().collect::<Vec<_>>();
    assert_eq!(key_probs.len(), 2);
    assert_eq!(key_probs[0].key, 'A');
    assert_eq!(key_probs[1].prob, prob!(0.6));
}

#[test]
fn test_into_iterator_mut_ref() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B'], probs: [0.4, 0.6]);
    let key_probs = (&mut obst).into_iter().collect::<Vec<_>>();
    assert_eq!(key_probs.len(), 2);
    assert_eq!(key_probs[0].key, 'A');
    assert_eq!(key_probs[1].prob, prob!(0.6));
}

#[test]
fn test_macro_empty() {
    let mut obst: OBSTStEphS<char> = OBSTStEphLit!();
    assert_eq!(obst.num_keys(), 0);
    assert_eq!(obst.optimal_cost(), Probability::zero());
}

#[test]
fn test_four_keys() {
    let mut obst = OBSTStEphLit!(keys: ['A', 'B', 'C', 'D'], probs: [0.1, 0.2, 0.3, 0.4]);
    let cost = obst.optimal_cost();
    assert!(cost > Probability::zero());
    assert!(obst.memo_size() > 0);
}
