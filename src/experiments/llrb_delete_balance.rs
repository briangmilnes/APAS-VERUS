// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Experiment: which `balance` step keeps Sedgewick's left-leaning red-black
//! (LLRB, 2-3 variant) delete correct?
//!
//! Question. LLRB delete (move_red_left, move_red_right, delete_min, delete)
//! repairs each node on the way up with `balance`. Two versions of its first step
//! appear in the literature:
//!   (C) conditional:   if red(h.right) && !red(h.left) { rotate_left(h) }
//!   (U) unconditional: if red(h.right)                 { rotate_left(h) }
//! The rest of `balance` is: rotate right if red(h.left) && red(h.left.left);
//! flip if red(h.left) && red(h.right). Insert uses (C). Does delete need (U)?
//!
//! Method. Plain Rust, no Verus. Insert keys, then delete in several orders,
//! checking the full LLRB invariant (BST order, no red right link, no red node
//! with a red left child, equal black height on every path, black root) after
//! every operation, for both variants.
//!
//! Run: rustc --edition 2021 --test -O src/experiments/llrb_delete_balance.rs
//!        -o target/experiments/llrb_delete_balance && target/experiments/llrb_delete_balance --nocapture
//!
//! RESULT: SUCCEEDS
//! DATE: 2026-09-24
//! Toolchain: rustc 1.98.1 (48a229cea 2026-09-01), as the logs record.
//!
//! Evidence (logs/experiment-llrb_delete_balance.*.log, all tests pass):
//! - Both variants keep the full LLRB invariant after every insert and delete
//!   (first failure: None for each), so delete does not need (U); the verified
//!   code uses (C), the same `fix_up` as insert.
//! - 26965 interleaved deletes: every `balance` input satisfies `fix_up`'s
//!   precondition (0 outside it).
//! - delete sees three input classes: an LLRB tree with a red root; an LLRB tree
//!   with a black root and a red left child; a black node with a red right child,
//!   reached only when the key is at least the node key. Every output is an LLRB
//!   tree of the input's black height, and a black input gives a black output.
//!   These are the pre- and postconditions of `delete_link` in
//!   src/Chap37/BSTRBMtEph.rs.

#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color { Red, Black }

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node { key: i64, color: Color, left: Link, right: Link }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Variant { Conditional, Unconditional }

fn is_red(l: &Link) -> bool { matches!(l, Some(n) if n.color == Color::Red) }

fn toggle(c: Color) -> Color { if c == Color::Red { Color::Black } else { Color::Red } }

fn rotate_left(mut h: Box<Node>) -> Box<Node> {
    let mut x = h.right.take().unwrap();
    h.right = x.left.take();
    x.color = h.color;
    h.color = Color::Red;
    x.left = Some(h);
    x
}

fn rotate_right(mut h: Box<Node>) -> Box<Node> {
    let mut x = h.left.take().unwrap();
    h.left = x.right.take();
    x.color = h.color;
    h.color = Color::Red;
    x.right = Some(h);
    x
}

fn flip(h: &mut Box<Node>) {
    h.color = toggle(h.color);
    if let Some(l) = h.left.as_mut() { l.color = toggle(l.color); }
    if let Some(r) = h.right.as_mut() { r.color = toggle(r.color); }
}

fn balance(mut h: Box<Node>, v: Variant) -> Box<Node> {
    let rl = match v {
        Variant::Conditional => is_red(&h.right) && !is_red(&h.left),
        Variant::Unconditional => is_red(&h.right),
    };
    if rl { h = rotate_left(h); }
    if is_red(&h.left) && is_red(&h.left.as_ref().unwrap().left) { h = rotate_right(h); }
    if is_red(&h.left) && is_red(&h.right) { flip(&mut h); }
    h
}

fn insert_rec(h: Link, key: i64) -> Box<Node> {
    match h {
        None => Box::new(Node { key, color: Color::Red, left: None, right: None }),
        Some(mut n) => {
            if key < n.key { n.left = Some(insert_rec(n.left.take(), key)); }
            else if key > n.key { n.right = Some(insert_rec(n.right.take(), key)); }
            else { return n; }
            balance(n, Variant::Conditional)
        }
    }
}

fn insert(root: Link, key: i64) -> Link {
    let mut r = insert_rec(root, key);
    r.color = Color::Black;
    Some(r)
}

fn move_red_left(mut h: Box<Node>) -> Box<Node> {
    flip(&mut h);
    if is_red(&h.right.as_ref().unwrap().left) {
        h.right = Some(rotate_right(h.right.take().unwrap()));
        h = rotate_left(h);
        flip(&mut h);
    }
    h
}

fn move_red_right(mut h: Box<Node>) -> Box<Node> {
    flip(&mut h);
    if is_red(&h.left.as_ref().unwrap().left) {
        h = rotate_right(h);
        flip(&mut h);
    }
    h
}

fn delete_min_rec(mut h: Box<Node>, v: Variant) -> (Link, i64) {
    if h.left.is_none() { return (None, h.key); }
    if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) { h = move_red_left(h); }
    let (l, m) = delete_min_rec(h.left.take().unwrap(), v);
    h.left = l;
    (Some(balance(h, v)), m)
}

fn delete_rec(mut h: Box<Node>, key: i64, v: Variant) -> Link {
    if key < h.key {
        if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) { h = move_red_left(h); }
        h.left = delete_rec(h.left.take().unwrap(), key, v);
    } else {
        if is_red(&h.left) { h = rotate_right(h); }
        if key == h.key && h.right.is_none() { return None; }
        if !is_red(&h.right) && !is_red(&h.right.as_ref().unwrap().left) { h = move_red_right(h); }
        if key == h.key {
            let (r, m) = delete_min_rec(h.right.take().unwrap(), v);
            h.key = m;
            h.right = r;
        } else {
            h.right = delete_rec(h.right.take().unwrap(), key, v);
        }
    }
    Some(balance(h, v))
}

fn contains(h: &Link, key: i64) -> bool {
    match h {
        None => false,
        Some(n) => if key < n.key { contains(&n.left, key) }
                   else if key > n.key { contains(&n.right, key) } else { true },
    }
}

fn delete(root: Link, key: i64, v: Variant) -> Link {
    if !contains(&root, key) { return root; }
    let mut r = root.unwrap();
    if !is_red(&r.left) && !is_red(&r.right) { r.color = Color::Red; }
    let mut out = delete_rec(r, key, v);
    if let Some(n) = out.as_mut() { n.color = Color::Black; }
    out
}

/// Returns the black height, or an error naming the violated LLRB condition.
fn check(h: &Link, lo: Option<i64>, hi: Option<i64>) -> Result<usize, String> {
    match h {
        None => Ok(0),
        Some(n) => {
            if lo.map_or(false, |l| n.key <= l) || hi.map_or(false, |u| n.key >= u) {
                return Err(format!("order at {}", n.key));
            }
            if is_red(&n.right) { return Err(format!("red right link at {}", n.key)); }
            if n.color == Color::Red && is_red(&n.left) { return Err(format!("red-red at {}", n.key)); }
            let bl = check(&n.left, lo, Some(n.key))?;
            let br = check(&n.right, Some(n.key), hi)?;
            if bl != br { return Err(format!("black heights {} != {} at {}", bl, br, n.key)); }
            Ok(bl + if n.color == Color::Black { 1 } else { 0 })
        }
    }
}

fn check_root(h: &Link) -> Result<usize, String> {
    if is_red(h) { return Err("red root".into()); }
    check(h, None, None)
}

fn size(h: &Link) -> usize { match h { None => 0, Some(n) => 1 + size(&n.left) + size(&n.right) } }

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0; x ^= x >> 12; x ^= x << 25; x ^= x >> 27; self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// Runs insert-then-delete workloads; returns the first failure, if any.
fn run(v: Variant) -> Option<String> {
    for n in 1..=200i64 {
        for seed in 1..=10u64 {
            let mut rng = Rng(seed * 7919 + n as u64);
            let mut keys: Vec<i64> = (0..n).collect();
            for i in (1..keys.len()).rev() { let j = (rng.next() % (i as u64 + 1)) as usize; keys.swap(i, j); }
            let mut root: Link = None;
            for &k in &keys { root = insert(root, k); }
            if let Err(e) = check_root(&root) { return Some(format!("insert n={} seed={}: {}", n, seed, e)); }
            let mut order = keys.clone();
            for i in (1..order.len()).rev() { let j = (rng.next() % (i as u64 + 1)) as usize; order.swap(i, j); }
            if seed == 1 { order.sort(); }
            if seed == 2 { order.sort(); order.reverse(); }
            let mut remaining = n as usize;
            for &k in &order {
                root = delete(root, k, v);
                remaining -= 1;
                if contains(&root, k) { return Some(format!("n={} seed={}: {} still present", n, seed, k)); }
                if size(&root) != remaining { return Some(format!("n={} seed={}: size", n, seed)); }
                if let Err(e) = check_root(&root) {
                    return Some(format!("delete {} n={} seed={}: {}", k, n, seed, e));
                }
            }
        }
    }
    None
}

#[test]
fn llrb_delete_balance_variants() {
    let c = run(Variant::Conditional);
    let u = run(Variant::Unconditional);
    println!("Conditional   first failure: {:?}", c);
    println!("Unconditional first failure: {:?}", u);
}

// Instrumentation for the second question: during delete, does every `balance`
// input satisfy insert's fix_up precondition (spec_fix_up_pre in
// src/Chap37/BSTRBMtEph.rs)? Children LLRB, or the left child relaxed and red;
// equal black heights; a red node without two red children; and a red left-left
// chain only below a black node whose right child is black.

fn llrb_ok(h: &Link) -> Option<usize> {
    match h {
        None => Some(0),
        Some(n) => {
            if is_red(&n.right) || (n.color == Color::Red && is_red(&n.left)) { return None; }
            let bl = llrb_ok(&n.left)?;
            let br = llrb_ok(&n.right)?;
            if bl != br { return None; }
            Some(bl + if n.color == Color::Black { 1 } else { 0 })
        }
    }
}

fn relaxed_red_ok(h: &Link) -> Option<usize> {
    match h {
        Some(n) if n.color == Color::Red && !is_red(&n.right) => {
            let bl = llrb_ok(&n.left)?;
            let br = llrb_ok(&n.right)?;
            if bl != br { None } else { Some(bl) }
        }
        _ => None,
    }
}

/// Classifies a balance input: None if it satisfies fix_up_pre, else the reason.
fn fix_up_pre_violation(h: &Node) -> Option<&'static str> {
    let br = match llrb_ok(&h.right) { Some(b) => b, None => return Some("right not llrb") };
    let bl = match llrb_ok(&h.left).or_else(|| relaxed_red_ok(&h.left)) {
        Some(b) => b, None => return Some("left neither llrb nor relaxed red") };
    if bl != br { return Some("black heights differ"); }
    if h.color == Color::Red && is_red(&h.left) && is_red(&h.right) { return Some("red with two red children"); }
    let rr = is_red(&h.left) && is_red(&h.left.as_ref().unwrap().left);
    if rr && !(h.color == Color::Black && !is_red(&h.right)) { return Some("left-left chain under red or with red right"); }
    None
}

use std::cell::RefCell;
thread_local!(static VIOLATIONS: RefCell<Vec<&'static str>> = RefCell::new(Vec::new()));

fn balance_traced(h: Box<Node>, v: Variant) -> Box<Node> {
    if let Some(r) = fix_up_pre_violation(&h) { VIOLATIONS.with(|vs| vs.borrow_mut().push(r)); }
    balance(h, v)
}

fn delete_min_traced(mut h: Box<Node>) -> (Link, i64) {
    if h.left.is_none() { return (None, h.key); }
    if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) { h = move_red_left(h); }
    let (l, m) = delete_min_traced(h.left.take().unwrap());
    h.left = l;
    (Some(balance_traced(h, Variant::Conditional)), m)
}

fn delete_traced(mut h: Box<Node>, key: i64) -> Link {
    if key < h.key {
        if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) { h = move_red_left(h); }
        h.left = delete_traced(h.left.take().unwrap(), key);
    } else {
        if is_red(&h.left) { h = rotate_right(h); }
        if key == h.key && h.right.is_none() { return None; }
        if !is_red(&h.right) && !is_red(&h.right.as_ref().unwrap().left) { h = move_red_right(h); }
        if key == h.key {
            let (r, m) = delete_min_traced(h.right.take().unwrap());
            h.key = m;
            h.right = r;
        } else {
            h.right = delete_traced(h.right.take().unwrap(), key);
        }
    }
    Some(balance_traced(h, Variant::Conditional))
}

#[test]
fn llrb_delete_balance_inputs() {
    let mut total = 0usize;
    for seed in 1..=40u64 {
        let mut rng = Rng(seed);
        let mut root: Link = None;
        for _ in 0..3000 {
            let k = (rng.next() % 300) as i64;
            if rng.next() % 2 == 0 {
                root = insert(root, k);
            } else if contains(&root, k) {
                let mut r = root.unwrap();
                if !is_red(&r.left) && !is_red(&r.right) { r.color = Color::Red; }
                root = delete_traced(r, k);
                if let Some(n) = root.as_mut() { n.color = Color::Black; }
                total += 1;
            }
            check_root(&root).expect("invariant");
        }
    }
    let mut kinds: Vec<&'static str> = VIOLATIONS.with(|vs| vs.borrow().clone());
    let count = kinds.len();
    kinds.sort();
    kinds.dedup();
    println!("interleaved deletes: {}, balance inputs outside fix_up_pre: {} {:?}", total, count, kinds);
}

// Third question: the pre/post classes of delete_min and delete, to state their
// verified contracts. A class names the root color, whether the link is LLRB or
// relaxed-red, and the colors of the root's children.

fn class(h: &Link) -> String {
    match h {
        None => "empty".into(),
        Some(n) => {
            let kind = if llrb_ok(h).is_some() { "llrb" }
                       else if relaxed_red_ok(h).is_some() { "relaxed" } else { "OTHER" };
            let c = |l: &Link| if l.is_none() { "_" } else if is_red(l) { "R" } else { "B" };
            format!("{}:{}({},{})", kind, if n.color == Color::Red { "R" } else { "B" }, c(&n.left), c(&n.right))
        }
    }
}

fn bh(h: &Link) -> usize {
    match h { None => 0, Some(n) => bh(&n.left) + if n.color == Color::Black { 1 } else { 0 } }
}

thread_local!(static PAIRS: RefCell<Vec<String>> = RefCell::new(Vec::new()));

fn record(f: &str, input: &Link, output: &Link) {
    let same = if bh(input) == bh(output) { "bh=" } else { "bh!" };
    PAIRS.with(|p| p.borrow_mut().push(format!("{} {} -> {} {}", f, class(input), class(output), same)));
}

fn clone_link(h: &Link) -> Link {
    h.as_ref().map(|n| Box::new(Node { key: n.key, color: n.color,
        left: clone_link(&n.left), right: clone_link(&n.right) }))
}

fn delete_min_rec2(h: Box<Node>) -> (Link, i64) {
    let input = Some(h);
    let snapshot = clone_link(&input);
    let mut h = input.unwrap();
    let out = if h.left.is_none() { (None, h.key) } else {
        if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) { h = move_red_left(h); }
        let (l, m) = delete_min_rec2(h.left.take().unwrap());
        h.left = l;
        (Some(balance(h, Variant::Conditional)), m)
    };
    record("delete_min", &snapshot, &out.0);
    out
}

fn delete_rec2(h: Box<Node>, key: i64) -> Link {
    let input = Some(h);
    let snapshot = clone_link(&input);
    let mut h = input.unwrap();
    let out;
    if key < h.key {
        if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) { h = move_red_left(h); }
        h.left = delete_rec2(h.left.take().unwrap(), key);
        out = Some(balance(h, Variant::Conditional));
    } else {
        if is_red(&h.left) { h = rotate_right(h); }
        if key == h.key && h.right.is_none() {
            record("delete", &snapshot, &None);
            return None;
        }
        if !is_red(&h.right) && !is_red(&h.right.as_ref().unwrap().left) { h = move_red_right(h); }
        if key == h.key {
            let (r, m) = delete_min_rec2(h.right.take().unwrap());
            h.key = m;
            h.right = r;
        } else {
            h.right = delete_rec2(h.right.take().unwrap(), key);
        }
        out = Some(balance(h, Variant::Conditional));
    }
    record("delete", &snapshot, &out);
    out
}

#[test]
fn llrb_delete_classes() {
    for seed in 1..=40u64 {
        let mut rng = Rng(seed + 1000);
        let mut root: Link = None;
        for _ in 0..3000 {
            let k = (rng.next() % 300) as i64;
            if rng.next() % 2 == 0 {
                root = insert(root, k);
            } else if contains(&root, k) {
                let mut r = root.unwrap();
                if !is_red(&r.left) && !is_red(&r.right) { r.color = Color::Red; }
                root = delete_rec2(r, k);
                if let Some(n) = root.as_mut() { n.color = Color::Black; }
            }
            check_root(&root).expect("invariant");
        }
    }
    let mut pairs: Vec<String> = PAIRS.with(|p| p.borrow().clone());
    pairs.sort();
    let mut counted: Vec<(String, usize)> = Vec::new();
    for p in pairs {
        match counted.last_mut() { Some((q, c)) if *q == p => *c += 1, _ => counted.push((p, 1)) }
    }
    for (p, c) in counted { println!("{:>8}  {}", c, p); }
}

// Fourth question: which branches of delete run on which input classes.

thread_local!(static BRANCHES: RefCell<Vec<String>> = RefCell::new(Vec::new()));

fn delete_rec3(mut h: Box<Node>, key: i64) -> Link {
    let cls = class(&Some(Box::new(Node { key: h.key, color: h.color,
        left: clone_link(&h.left), right: clone_link(&h.right) })));
    let mut path = String::new();
    let out;
    if key < h.key {
        if !is_red(&h.left) && !is_red(&h.left.as_ref().unwrap().left) {
            let rot = is_red(&h.right.as_ref().unwrap().left);
            path.push_str(if rot { "lt-moveRL-rot" } else { "lt-moveRL" });
            h = move_red_left(h);
        } else {
            path.push_str("lt-nomove");
        }
        let child_cls = class(&h.left.as_ref().map(|n| Box::new(Node { key: n.key, color: n.color,
            left: clone_link(&n.left), right: clone_link(&n.right) })));
        path.push_str(&format!(" child={}", child_cls));
        h.left = delete_rec3(h.left.take().unwrap(), key);
        out = Some(balance(h, Variant::Conditional));
    } else {
        if is_red(&h.left) { h = rotate_right(h); path.push_str("ge-rotR "); }
        if key == h.key && h.right.is_none() {
            BRANCHES.with(|b| b.borrow_mut().push(format!("{} :: {}leaf", cls, path)));
            return None;
        }
        if !is_red(&h.right) && !is_red(&h.right.as_ref().unwrap().left) {
            let rot = is_red(&h.left.as_ref().unwrap().left);
            path.push_str(if rot { "moveRR-rot" } else { "moveRR" });
            h = move_red_right(h);
        } else {
            path.push_str("nomove");
        }
        let child_cls = class(&h.right.as_ref().map(|n| Box::new(Node { key: n.key, color: n.color,
            left: clone_link(&n.left), right: clone_link(&n.right) })));
        if key == h.key {
            path.push_str(&format!(" eq child={}", child_cls));
            let (r, m) = delete_min_rec(h.right.take().unwrap(), Variant::Conditional);
            h.key = m;
            h.right = r;
        } else {
            path.push_str(&format!(" gt child={}", child_cls));
            h.right = delete_rec3(h.right.take().unwrap(), key);
        }
        out = Some(balance(h, Variant::Conditional));
    }
    BRANCHES.with(|b| b.borrow_mut().push(format!("{} :: {} => {}", cls, path, class(&out))));
    out
}

#[test]
fn llrb_delete_branches() {
    for seed in 1..=40u64 {
        let mut rng = Rng(seed + 2000);
        let mut root: Link = None;
        for _ in 0..3000 {
            let k = (rng.next() % 300) as i64;
            if rng.next() % 2 == 0 {
                root = insert(root, k);
            } else if contains(&root, k) {
                let mut r = root.unwrap();
                if !is_red(&r.left) && !is_red(&r.right) { r.color = Color::Red; }
                root = delete_rec3(r, k);
                if let Some(n) = root.as_mut() { n.color = Color::Black; }
            }
            check_root(&root).expect("invariant");
        }
    }
    let mut rows: Vec<String> = BRANCHES.with(|b| b.borrow().clone());
    rows.sort();
    let mut counted: Vec<(String, usize)> = Vec::new();
    for p in rows {
        match counted.last_mut() { Some((q, c)) if *q == p => *c += 1, _ => counted.push((p, 1)) }
    }
    for (p, c) in counted { println!("{:>8}  {}", c, p); }
}
