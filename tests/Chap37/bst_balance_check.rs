// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Shape checks for Chap37 balanced BSTs, computed from a pre-order traversal.
//!
//! A BST is determined by its pre-order key sequence, so the checks rebuild the
//! shape from `pre_order()` and use no module internals. They test textbook
//! properties (APAS Chap37, Definitions 37.1, 37.3, 37.6):
//! - key order at every node;
//! - red-black: at every node, the maximum leaf depth of its subtree is at most
//!   twice the minimum leaf depth (depths count internal nodes from that node);
//! - the height bound h <= 2·lg(n + 1), checked as 2^h <= (n + 1)^2;
//! - weight balance (BB[α], (Δ, Γ) = (3, 2)): at every node, with weight =
//!   size + 1, 3·w(left) >= w(right) and 3·w(right) >= w(left);
//! - the weight-balanced height bound 4^h <= 3^h · (n + 1).
//!
//! Leaves are empty; an empty tree has height 0 and a single node height 1.

#![allow(dead_code)]

/// Per-subtree measurements: key count, height, minimum leaf depth.
#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub size: usize,
    pub height: usize,
    pub min_leaf_depth: usize,
}

/// Rebuilds the subtree whose pre-order sequence is `keys` and checks key order
/// and, when `check_rb` holds, the factor-2 leaf-depth property at every node.
fn shape_of<T: Ord + std::fmt::Debug>(keys: &[T], check_rb: bool) -> Result<Shape, String> {
    if keys.is_empty() {
        return Ok(Shape { size: 0, height: 0, min_leaf_depth: 0 });
    }
    let root = &keys[0];
    let rest = &keys[1..];
    let split = rest.iter().position(|k| k > root).unwrap_or(rest.len());
    let (left, right) = rest.split_at(split);
    if let Some(bad) = right.iter().find(|k| *k <= root) {
        return Err(format!("key order: {:?} in the right subtree of {:?}", bad, root));
    }
    let l = shape_of(left, check_rb)?;
    let r = shape_of(right, check_rb)?;
    let height = 1 + l.height.max(r.height);
    let min_leaf_depth = 1 + l.min_leaf_depth.min(r.min_leaf_depth);
    if check_rb && height > 2 * min_leaf_depth {
        return Err(format!(
            "leaf depths at {:?}: max {} > 2 * min {}", root, height, min_leaf_depth
        ));
    }
    Ok(Shape { size: 1 + l.size + r.size, height, min_leaf_depth })
}

/// Checks key order only and returns the shape.
pub fn bst_shape<T: Ord + std::fmt::Debug>(pre_order: &[T]) -> Result<Shape, String> {
    shape_of(pre_order, false)
}

/// Checks key order, the red-black leaf-depth property at every node, and
/// h <= 2·lg(n + 1). Returns the shape.
pub fn check_red_black<T: Ord + std::fmt::Debug>(pre_order: &[T]) -> Result<Shape, String> {
    let shape = shape_of(pre_order, true)?;
    if !height_within_two_lg(shape.size, shape.height) {
        return Err(format!(
            "height {} exceeds 2*lg({} + 1)", shape.height, shape.size
        ));
    }
    Ok(shape)
}

/// Returns true iff h <= 2·lg(n + 1), i.e. 2^h <= (n + 1)^2.
pub fn height_within_two_lg(n: usize, h: usize) -> bool {
    let bound = (n as u128 + 1) * (n as u128 + 1);
    h < 128 && (1u128 << h) <= bound
}

/// Deterministic pseudo-random sequence (xorshift64*), for seeded workloads.
pub struct SeededRng(u64);

impl SeededRng {
    pub fn new(seed: u64) -> Self { SeededRng(seed.max(1)) }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn below(&mut self, bound: u64) -> u64 { self.next_u64() % bound }
}

/// Keys 0..n in ascending order.
pub fn ascending(n: i64) -> Vec<i64> { (0..n).collect() }

/// Keys 0..n in descending order.
pub fn descending(n: i64) -> Vec<i64> { (0..n).rev().collect() }

/// Keys 0..n taken alternately from the low and high ends.
pub fn zigzag(n: i64) -> Vec<i64> {
    let (mut lo, mut hi) = (0, n - 1);
    let mut keys = Vec::new();
    while lo <= hi {
        keys.push(lo);
        if lo != hi { keys.push(hi); }
        lo += 1;
        hi -= 1;
    }
    keys
}

/// n keys drawn uniformly from 0..range with a fixed seed (duplicates possible).
pub fn random_keys(n: usize, range: u64, seed: u64) -> Vec<i64> {
    let mut rng = SeededRng::new(seed);
    (0..n).map(|_| rng.below(range) as i64).collect()
}

/// A random permutation of 0..n with a fixed seed.
pub fn random_permutation(n: i64, seed: u64) -> Vec<i64> {
    let mut rng = SeededRng::new(seed);
    let mut keys: Vec<i64> = (0..n).collect();
    for i in (1..keys.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        keys.swap(i, j);
    }
    keys
}

/// Rebuilds the subtree whose pre-order sequence is `keys` and checks key order
/// and the weight condition 3·w(l) >= w(r) and 3·w(r) >= w(l) at every node.
fn weight_shape_of<T: Ord + std::fmt::Debug>(keys: &[T]) -> Result<Shape, String> {
    if keys.is_empty() {
        return Ok(Shape { size: 0, height: 0, min_leaf_depth: 0 });
    }
    let root = &keys[0];
    let rest = &keys[1..];
    let split = rest.iter().position(|k| k > root).unwrap_or(rest.len());
    let (left, right) = rest.split_at(split);
    if let Some(bad) = right.iter().find(|k| *k <= root) {
        return Err(format!("key order: {:?} in the right subtree of {:?}", bad, root));
    }
    let l = weight_shape_of(left)?;
    let r = weight_shape_of(right)?;
    let (wl, wr) = (l.size + 1, r.size + 1);
    if 3 * wl < wr || 3 * wr < wl {
        return Err(format!("weights at {:?}: left {} right {}", root, wl, wr));
    }
    let height = 1 + l.height.max(r.height);
    let min_leaf_depth = 1 + l.min_leaf_depth.min(r.min_leaf_depth);
    Ok(Shape { size: 1 + l.size + r.size, height, min_leaf_depth })
}

/// Checks key order, weight balance at every node, and 4^h <= 3^h · (n + 1).
/// Returns the shape.
pub fn check_weight_balanced<T: Ord + std::fmt::Debug>(pre_order: &[T]) -> Result<Shape, String> {
    let shape = weight_shape_of(pre_order)?;
    if !height_within_wb_bound(shape.size, shape.height) {
        return Err(format!("height {} exceeds the bound for n = {}", shape.height, shape.size));
    }
    Ok(shape)
}

/// Returns true iff 4^h <= 3^h · (n + 1), the weight-balanced height bound
/// (h <= lg(n + 1) / lg(4/3)).
pub fn height_within_wb_bound(n: usize, h: usize) -> bool {
    if h >= 60 { return false; }
    let four = 4u128.pow(h as u32);
    let three = 3u128.pow(h as u32);
    three.checked_mul(n as u128 + 1).map_or(true, |rhs| four <= rhs)
}
