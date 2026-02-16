//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap05::KleeneStPer::KleeneStPer::*;
use apas_verus::SetLit;

#[test]
fn test_star_empty_string_always_accepted() {
    let sigma: SetStEph<char> = SetLit!['a', 'b'];
    let k = KleeneStPer::new(sigma);
    let empty: Vec<char> = vec![];
    assert!(k.mem_star(&empty));
}

#[test]
fn test_plus_rejects_empty_string() {
    let sigma: SetStEph<char> = SetLit!['a', 'b'];
    let k = KleeneStPer::new(sigma);
    let empty: Vec<char> = vec![];
    assert!(!k.mem_plus(&empty));
}

#[test]
fn test_star_single_element() {
    let sigma: SetStEph<char> = SetLit!['a', 'b'];
    let k = KleeneStPer::new(sigma);
    assert!(k.mem_star(&['a']));
    assert!(k.mem_star(&['b']));
    assert!(!k.mem_star(&['c']));
}

#[test]
fn test_plus_single_element() {
    let sigma: SetStEph<char> = SetLit!['a', 'b'];
    let k = KleeneStPer::new(sigma);
    assert!(k.mem_plus(&['a']));
    assert!(k.mem_plus(&['b']));
    assert!(!k.mem_plus(&['c']));
}

#[test]
fn test_star_multi_element_strings() {
    let sigma: SetStEph<char> = SetLit!['a', 'b'];
    let k = KleeneStPer::new(sigma);
    assert!(k.mem_star(&['a', 'b', 'a']));
    assert!(k.mem_star(&['b', 'b', 'b', 'b']));
    assert!(!k.mem_star(&['a', 'c', 'b']));
    assert!(!k.mem_star(&['x']));
}

#[test]
fn test_plus_multi_element_strings() {
    let sigma: SetStEph<char> = SetLit!['a', 'b'];
    let k = KleeneStPer::new(sigma);
    assert!(k.mem_plus(&['a', 'b', 'a']));
    assert!(!k.mem_plus(&['a', 'c']));
}

#[test]
fn test_empty_alphabet() {
    let sigma: SetStEph<char> = SetLit![];
    let k = KleeneStPer::new(sigma);
    let empty: Vec<char> = vec![];
    assert!(k.mem_star(&empty));
    assert!(!k.mem_plus(&empty));
    assert!(!k.mem_star(&['a']));
    assert!(!k.mem_plus(&['a']));
}

#[test]
fn test_singleton_alphabet() {
    let sigma: SetStEph<char> = SetLit!['x'];
    let k = KleeneStPer::new(sigma);
    assert!(k.mem_star(&['x', 'x', 'x']));
    assert!(k.mem_plus(&['x']));
    assert!(!k.mem_star(&['x', 'y']));
}

#[test]
fn test_alphabet_accessor() {
    let sigma: SetStEph<char> = SetLit!['a', 'b', 'c'];
    let k = KleeneStPer::new(sigma);
    let a = k.alphabet();
    assert_eq!(a.size(), 3);
    assert!(a.mem(&'a'));
    assert!(a.mem(&'b'));
    assert!(a.mem(&'c'));
    assert!(!a.mem(&'d'));
}

#[test]
fn test_integer_alphabet() {
    let sigma: SetStEph<i32> = SetLit![0, 1];
    let k = KleeneStPer::new(sigma);
    assert!(k.mem_star(&[0, 1, 0, 1]));
    assert!(k.mem_star(&[0, 0, 0]));
    assert!(!k.mem_star(&[0, 2]));
    assert!(k.mem_plus(&[1]));
}
