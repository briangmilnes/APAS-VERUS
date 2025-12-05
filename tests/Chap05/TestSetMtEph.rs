//! Tests for SetMtEph - parallel set operations.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap05::SetMtEph::SetMtEph::*;
use apas_verus::Types::Types::Pair;

#[test]
fn test_cartesian_product_mt_basic() {
    let s1: SetStEph<i32> = SetStEph::from_vec(vec![1, 2, 3]);
    let s2: SetStEph<char> = SetStEph::from_vec(vec!['a', 'b']);
    
    let result = cartesian_product_mt(s1, s2);
    
    assert_eq!(result.size(), 6);
    assert!(result.mem(&Pair(1, 'a')));
    assert!(result.mem(&Pair(1, 'b')));
    assert!(result.mem(&Pair(2, 'a')));
    assert!(result.mem(&Pair(2, 'b')));
    assert!(result.mem(&Pair(3, 'a')));
    assert!(result.mem(&Pair(3, 'b')));
}

#[test]
fn test_cartesian_product_mt_large() {
    // Large enough to trigger parallel execution
    let s1: SetStEph<i32> = SetStEph::from_vec((0..20).collect());
    let s2: SetStEph<i32> = SetStEph::from_vec((100..110).collect());
    
    let result = cartesian_product_mt(s1, s2);
    
    assert_eq!(result.size(), 200);
}

#[test]
fn test_parallel_equals_sequential() {
    let s1: SetStEph<i32> = SetStEph::from_vec((0..10).collect());
    let s2: SetStEph<i32> = SetStEph::from_vec((100..105).collect());
    
    let seq_result = s1.cartesian_product(&s2);
    let par_result = cartesian_product_mt(s1.clone(), s2.clone());
    
    assert_eq!(seq_result.size(), par_result.size());
    // Both should have same elements
    for i in 0..10 {
        for j in 100..105 {
            assert!(seq_result.mem(&Pair(i, j)));
            assert!(par_result.mem(&Pair(i, j)));
        }
    }
}
