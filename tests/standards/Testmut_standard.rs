//! Runtime tests for standards::mut_standard.

use apas_verus::standards::mut_standard::mut_standard::*;

#[test]
fn test_counter_new() {
    let c = Counter::new();
    assert_eq!(c.val, 0);
}

#[test]
fn test_counter_inc() {
    let mut c = Counter::new();
    c.inc();
    assert_eq!(c.val, 1);
    c.inc();
    assert_eq!(c.val, 2);
}

#[test]
fn test_counter_add() {
    let mut c = Counter::new();
    c.add(5);
    assert_eq!(c.val, 5);
    c.add(3);
    assert_eq!(c.val, 8);
}

#[test]
fn test_pair_new() {
    let p = Pair::new(10, 20);
    assert_eq!(p.fst, 10);
    assert_eq!(p.snd, 20);
}

#[test]
fn test_pair_set_fst() {
    let mut p = Pair::new(10, 20);
    p.set_fst(99);
    assert_eq!(p.fst, 99);
    assert_eq!(p.snd, 20);
}

#[test]
fn test_pair_set_snd() {
    let mut p = Pair::new(10, 20);
    p.set_snd(99);
    assert_eq!(p.fst, 10);
    assert_eq!(p.snd, 99);
}

#[test]
fn test_pair_swap() {
    let mut p = Pair::new(10, 20);
    p.swap();
    assert_eq!(p.fst, 20);
    assert_eq!(p.snd, 10);
}

#[test]
fn test_increment_ref() {
    let mut v: u64 = 100;
    increment_ref(&mut v);
    assert_eq!(v, 101);
}

#[test]
fn test_collect_range() {
    let mut out: Vec<u64> = Vec::new();
    collect_range(&mut out, 0, 5);
    assert_eq!(out.len(), 5);
    collect_range(&mut out, 10, 13);
    assert_eq!(out.len(), 8);
}

#[test]
fn test_replace_item() {
    let mut c = Container { item: None };
    replace_item(&mut c, 42);
    assert_eq!(*c.item.unwrap(), 42);
}
