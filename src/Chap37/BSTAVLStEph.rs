//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree.
//! Verusified: functional-style AVL with BST invariant + balance specs.

// Table of Contents
// 1. module
// 2. imports
// 6. spec fns
// 9. impls
// 12. macros

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTAVLStEph {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::{tree_contains, tree_is_bst};
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 6. spec fns

    /// AVL balance: for every node, |height(left) - height(right)| <= 1.
    pub open spec fn avl_balanced<T>(tree: BalBinTree<T>) -> bool
        decreases tree.spec_size(),
    {
        match tree {
            BalBinTree::Leaf => true,
            BalBinTree::Node(node) =>
                avl_balanced(node.left)
                && avl_balanced(node.right)
                && {
                    let lh = node.left.spec_height() as int;
                    let rh = node.right.spec_height() as int;
                    -1 <= lh - rh && lh - rh <= 1
                }
        }
    }

    /// Combined AVL tree invariant: BST ordering + AVL balance.
    pub open spec fn tree_is_avl<T: TotalOrder>(tree: BalBinTree<T>) -> bool {
        tree_is_bst(tree) && avl_balanced(tree)
    }

    // 7. proof fns

    /// Decomposes tree_is_bst two levels deep, exposing children and grandchildren BST
    /// facts plus all ordering quantifiers. Used by rotation proofs.
    proof fn lemma_bst_deep<T: TotalOrder>(tree: BalBinTree<T>)
        requires tree_is_bst::<T>(tree),
        ensures
            match tree {
                BalBinTree::Leaf => true,
                BalBinTree::Node(node) =>
                    tree_is_bst::<T>(node.left)
                    && tree_is_bst::<T>(node.right)
                    && (forall|x: T| #![auto] tree_contains(node.left, x) ==>
                        T::le(x, node.value) && x != node.value)
                    && (forall|x: T| #![auto] tree_contains(node.right, x) ==>
                        T::le(node.value, x) && x != node.value)
                    && match node.left {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(lnode) =>
                            tree_is_bst::<T>(lnode.left)
                            && tree_is_bst::<T>(lnode.right)
                            && (forall|x: T| #![auto] tree_contains(lnode.left, x) ==>
                                T::le(x, lnode.value) && x != lnode.value)
                            && (forall|x: T| #![auto] tree_contains(lnode.right, x) ==>
                                T::le(lnode.value, x) && x != lnode.value)
                    }
                    && match node.right {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(rnode) =>
                            tree_is_bst::<T>(rnode.left)
                            && tree_is_bst::<T>(rnode.right)
                            && (forall|x: T| #![auto] tree_contains(rnode.left, x) ==>
                                T::le(x, rnode.value) && x != rnode.value)
                            && (forall|x: T| #![auto] tree_contains(rnode.right, x) ==>
                                T::le(rnode.value, x) && x != rnode.value)
                    }
            }
    {
        reveal_with_fuel(tree_is_bst, 3);
        reveal_with_fuel(tree_contains, 3);
    }

    /// max(a+1, b) <= max(a, b) + 1 for natural numbers.
    proof fn lemma_max_plus_one(a: nat, b: nat)
        ensures
            (if a >= b { a + 1 } else { b }) <= (if a >= b { a } else { b }) + 1,
    {
    }

    // 9. impls

    #[verifier::reject_recursive_types(T)]
    pub struct BSTAVLStEph<T> {
        pub root: BalBinTree<T>,
    }

    fn rotate_right<T: TotalOrder>(tree: BalBinTree<T>) -> (result: BalBinTree<T>)
        requires
            tree_is_bst::<T>(tree),
            !(tree is Leaf),
        ensures
            tree_is_bst::<T>(result),
            forall|x: T| #![auto] tree_contains(result, x) == tree_contains(tree, x),
            match tree {
                BalBinTree::Node(outer) => match outer.left {
                    BalBinTree::Node(l) => {
                        let lr_h = l.right.spec_height();
                        let r_h = outer.right.spec_height();
                        let ll_h = l.left.spec_height();
                        let new_rh: nat = 1 + if lr_h >= r_h { lr_h } else { r_h };
                        &&& result.spec_height() == (1 + if ll_h >= new_rh { ll_h } else { new_rh })
                        &&& ((avl_balanced(l.left) && avl_balanced(l.right) && avl_balanced(outer.right)
                             && lr_h as int - r_h as int >= -1 && lr_h as int - r_h as int <= 1
                             && ll_h as int - new_rh as int >= -1 && ll_h as int - new_rh as int <= 1)
                            ==> avl_balanced(result))
                        &&& result is Node
                        &&& match result {
                            BalBinTree::Node(res) => {
                                &&& res.left.spec_height() == ll_h
                                &&& avl_balanced(res.left) == avl_balanced(l.left)
                                &&& res.right.spec_height() == new_rh
                                &&& (avl_balanced(res.right) <==> (avl_balanced(l.right)
                                    && avl_balanced(outer.right) && {
                                    let lh = lr_h as int;
                                    let rh = r_h as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }))
                            },
                            _ => false,
                        }
                    },
                    _ => true,
                },
                _ => true,
            },
    {
        let ghost tree_ghost = tree;
        match tree {
            BalBinTree::Node(y_box) => {
                let BalBinNode { left: left_tree, value: y_val, right: r } = *y_box;
                let ghost old_left = left_tree;
                let ghost old_r = r;

                match left_tree {
                    BalBinTree::Node(x_box) => {
                        let BalBinNode { left: ll, value: x_val, right: lr } = *x_box;
                        let ghost old_ll = ll;
                        let ghost old_lr = lr;

                        let right_sub = BalBinTree::Node(Box::new(BalBinNode {
                            left: lr,
                            value: y_val,
                            right: r,
                        }));

                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: ll,
                            value: x_val,
                            right: right_sub,
                        }));

                        proof {
                            lemma_bst_deep::<T>(tree_ghost);

                            assert forall|z: T| tree_contains(old_lr, z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                assert(tree_contains(old_left, z));
                            };

                            assert(tree_contains(old_left, x_val));
                            assert(x_val != y_val);

                            assert(tree_is_bst::<T>(right_sub));

                            assert forall|z: T| tree_contains(right_sub, z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                if tree_contains(old_lr, z) {
                                } else if z == y_val {
                                    assert(x_val != y_val);
                                } else if tree_contains(old_r, z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                }
                            };

                            assert forall|z: T| tree_contains(r, z) ==
                                tree_contains(tree_ghost, z)
                            by {
                                assert(tree_contains(r, z) ==
                                    (x_val == z
                                    || tree_contains(old_ll, z)
                                    || tree_contains(right_sub, z)));
                                assert(tree_contains(right_sub, z) ==
                                    (y_val == z
                                    || tree_contains(old_lr, z)
                                    || tree_contains(old_r, z)));
                                assert(tree_contains(tree_ghost, z) ==
                                    (y_val == z
                                    || tree_contains(old_left, z)
                                    || tree_contains(old_r, z)));
                                assert(tree_contains(old_left, z) ==
                                    (x_val == z
                                    || tree_contains(old_ll, z)
                                    || tree_contains(old_lr, z)));
                            };

                            // Height: r = Node(x, ll, right_sub) where
                            // right_sub = Node(y, lr, old_r)
                            assert(right_sub.spec_height() == 1 + if old_lr.spec_height()
                                >= old_r.spec_height() { old_lr.spec_height() }
                                else { old_r.spec_height() });
                            assert(r.spec_height() == 1 + if old_ll.spec_height()
                                >= right_sub.spec_height() { old_ll.spec_height() }
                                else { right_sub.spec_height() });

                            // AVL balance: unfold avl_balanced on the constructed nodes
                            assert(avl_balanced(right_sub) <==>
                                (avl_balanced(old_lr) && avl_balanced(old_r) && {
                                    let lh = old_lr.spec_height() as int;
                                    let rh = old_r.spec_height() as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }));
                            assert(avl_balanced(r) <==>
                                (avl_balanced(old_ll) && avl_balanced(right_sub) && {
                                    let lh = old_ll.spec_height() as int;
                                    let rh = right_sub.spec_height() as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }));
                        }
                        r
                    }
                    BalBinTree::Leaf => {
                        BalBinTree::Node(Box::new(BalBinNode {
                            left: BalBinTree::Leaf,
                            value: y_val,
                            right: r,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    fn rotate_left<T: TotalOrder>(tree: BalBinTree<T>) -> (result: BalBinTree<T>)
        requires
            tree_is_bst::<T>(tree),
            !(tree is Leaf),
        ensures
            tree_is_bst::<T>(result),
            forall|x: T| #![auto] tree_contains(result, x) == tree_contains(tree, x),
            match tree {
                BalBinTree::Node(outer) => match outer.right {
                    BalBinTree::Node(r) => {
                        let rl_h = r.left.spec_height();
                        let l_h = outer.left.spec_height();
                        let rr_h = r.right.spec_height();
                        let new_lh: nat = 1 + if l_h >= rl_h { l_h } else { rl_h };
                        &&& result.spec_height() == (1 + if new_lh >= rr_h { new_lh } else { rr_h })
                        &&& ((avl_balanced(outer.left) && avl_balanced(r.left) && avl_balanced(r.right)
                             && l_h as int - rl_h as int >= -1 && l_h as int - rl_h as int <= 1
                             && new_lh as int - rr_h as int >= -1 && new_lh as int - rr_h as int <= 1)
                            ==> avl_balanced(result))
                        &&& result is Node
                        &&& match result {
                            BalBinTree::Node(res) => {
                                &&& res.right.spec_height() == rr_h
                                &&& avl_balanced(res.right) == avl_balanced(r.right)
                                &&& res.left.spec_height() == new_lh
                                &&& (avl_balanced(res.left) <==> (avl_balanced(outer.left)
                                    && avl_balanced(r.left) && {
                                    let lh = l_h as int;
                                    let rh = rl_h as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }))
                            },
                            _ => false,
                        }
                    },
                    _ => true,
                },
                _ => true,
            },
    {
        let ghost tree_ghost = tree;
        match tree {
            BalBinTree::Node(x_box) => {
                let BalBinNode { left: l, value: x_val, right: right_tree } = *x_box;
                let ghost old_right = right_tree;
                let ghost old_l = l;

                match right_tree {
                    BalBinTree::Node(y_box) => {
                        let BalBinNode { left: rl, value: y_val, right: rr } = *y_box;
                        let ghost old_rl = rl;
                        let ghost old_rr = rr;

                        let left_sub = BalBinTree::Node(Box::new(BalBinNode {
                            left: l,
                            value: x_val,
                            right: rl,
                        }));

                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left_sub,
                            value: y_val,
                            right: rr,
                        }));

                        proof {
                            lemma_bst_deep::<T>(tree_ghost);

                            assert forall|z: T| tree_contains(old_rl, z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                assert(tree_contains(old_right, z));
                            };

                            assert(tree_contains(old_right, y_val));
                            assert(x_val != y_val);

                            assert(tree_is_bst::<T>(left_sub));

                            assert forall|z: T| tree_contains(left_sub, z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                if tree_contains(old_l, z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                } else if z == x_val {
                                    assert(x_val != y_val);
                                } else if tree_contains(old_rl, z) {
                                }
                            };

                            assert forall|z: T| tree_contains(r, z) ==
                                tree_contains(tree_ghost, z)
                            by {
                                assert(tree_contains(r, z) ==
                                    (y_val == z
                                    || tree_contains(left_sub, z)
                                    || tree_contains(old_rr, z)));
                                assert(tree_contains(left_sub, z) ==
                                    (x_val == z
                                    || tree_contains(old_l, z)
                                    || tree_contains(old_rl, z)));
                                assert(tree_contains(tree_ghost, z) ==
                                    (x_val == z
                                    || tree_contains(old_l, z)
                                    || tree_contains(old_right, z)));
                                assert(tree_contains(old_right, z) ==
                                    (y_val == z
                                    || tree_contains(old_rl, z)
                                    || tree_contains(old_rr, z)));
                            };

                            // Height: r = Node(y, left_sub, rr) where
                            // left_sub = Node(x, old_l, rl)
                            assert(left_sub.spec_height() == 1 + if old_l.spec_height()
                                >= old_rl.spec_height() { old_l.spec_height() }
                                else { old_rl.spec_height() });
                            assert(r.spec_height() == 1 + if left_sub.spec_height()
                                >= old_rr.spec_height() { left_sub.spec_height() }
                                else { old_rr.spec_height() });

                            // AVL balance: unfold avl_balanced on the constructed nodes
                            assert(avl_balanced(left_sub) <==>
                                (avl_balanced(old_l) && avl_balanced(old_rl) && {
                                    let lh = old_l.spec_height() as int;
                                    let rh = old_rl.spec_height() as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }));
                            assert(avl_balanced(r) <==>
                                (avl_balanced(left_sub) && avl_balanced(old_rr) && {
                                    let lh = left_sub.spec_height() as int;
                                    let rh = old_rr.spec_height() as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }));
                        }
                        r
                    }
                    BalBinTree::Leaf => {
                        BalBinTree::Node(Box::new(BalBinNode {
                            left: l,
                            value: x_val,
                            right: BalBinTree::Leaf,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    // AVL rebalance: check balance factor and rotate if needed.
    // Preserves BST ordering (proved via rotation ensures).
    //
    // Proof gaps (assumes) — full proofs would require:
    // - lemma_rotate_height: rotate_right/rotate_left do not increase height
    // - lemma_rotate_avl: after rotation on left-heavy (balance=2) or right-heavy
    //   (balance=-2) tree, result has |h(left)-h(right)| <= 1 at every node
    fn rebalance<T: TotalOrder>(tree: BalBinTree<T>) -> (result: BalBinTree<T>)
        requires
            tree_is_bst::<T>(tree),
            !(tree is Leaf),
            tree.spec_height() <= usize::MAX,
            match tree {
                BalBinTree::Node(inner) =>
                    avl_balanced(inner.left) && avl_balanced(inner.right)
                    && {
                        let lh = inner.left.spec_height() as int;
                        let rh = inner.right.spec_height() as int;
                        -2 <= lh - rh && lh - rh <= 2
                    },
                _ => false,
            },
        ensures
            tree_is_bst::<T>(result),
            avl_balanced(result),
            result.spec_height() <= tree.spec_height(),
            result.spec_height() + 1 >= tree.spec_height(),
            forall|x: T| #![auto] tree_contains(result, x) == tree_contains(tree, x),
            match tree {
                BalBinTree::Node(inner) => {
                    let lh = inner.left.spec_height() as int;
                    let rh = inner.right.spec_height() as int;
                    (-1 <= lh - rh && lh - rh <= 1) ==> result.spec_height() == tree.spec_height()
                },
                _ => true,
            },
    {
        let ghost tree_ghost = tree;
        match tree {
            BalBinTree::Node(inner) => {
                let lh = inner.left.height();
                let rh = inner.right.height();
                if lh > rh + 1 {
                    // Left-heavy: check for zig-zag case
                    let left_rh = match &inner.left {
                        BalBinTree::Node(ln) => ln.right.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    let left_lh = match &inner.left {
                        BalBinTree::Node(ln) => ln.left.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    if left_rh > left_lh {
                        // Left-right case: rotate left child left, then rotate root right
                        let BalBinNode { left, value: v, right } = *inner;
                        let new_left = rotate_left(left);
                        let intermediate = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: v,
                            right: right,
                        }));
                        proof {
                            assert forall|x: T| tree_contains(intermediate, x) ==
                                tree_contains(tree_ghost, x)
                            by {
                                assert(tree_contains(intermediate, x) ==
                                    (v == x || tree_contains(new_left, x) || tree_contains(right, x)));
                                assert(tree_contains(tree_ghost, x) ==
                                    (v == x || tree_contains(left, x) || tree_contains(right, x)));
                            };
                        }
                        let result = rotate_right(intermediate);
                        proof {
                            match tree_ghost {
                                BalBinTree::Node(tg) => {
                                    match tg.left {
                                        BalBinTree::Node(tg_l) => {
                                            match tg_l.right {
                                                BalBinTree::Node(tg_lr) => {
                                                    let ll_h = tg_l.left.spec_height();
                                                    let lrl_h = tg_lr.left.spec_height();
                                                    let lrr_h = tg_lr.right.spec_height();
                                                    let r_h = tg.right.spec_height();

                                                    assert(ll_h == r_h);

                                                    // lr.h = left_rh = ll_h + 1 = r_h + 1
                                                    assert(tg_l.right.spec_height() == r_h + 1);

                                                    // Unfold lr height: 1 + max(lrl_h, lrr_h) = r_h + 1
                                                    assert(tg_l.right.spec_height() ==
                                                        1 + if lrl_h >= lrr_h { lrl_h } else { lrr_h });

                                                    // max(lrl_h, lrr_h) = r_h; both <= r_h
                                                    assert(lrl_h <= r_h);
                                                    assert(lrr_h <= r_h);

                                                    // avl(lr): |lrl_h - lrr_h| <= 1
                                                    assert(avl_balanced(tg_l.right));

                                                    assert(lrl_h as int >= r_h as int - 1) by {
                                                        if lrl_h >= lrr_h {
                                                            assert(lrl_h == r_h);
                                                        } else {
                                                            assert(lrr_h == r_h);
                                                        }
                                                    };
                                                    assert(lrr_h as int >= r_h as int - 1) by {
                                                        if lrr_h >= lrl_h {
                                                            assert(lrr_h == r_h);
                                                        } else {
                                                            assert(lrl_h == r_h);
                                                        }
                                                    };

                                                    assert(ll_h as int - lrl_h as int >= 0);
                                                    assert(ll_h as int - lrl_h as int <= 1);
                                                    assert(lrr_h as int - r_h as int >= -1);
                                                    assert(lrr_h as int - r_h as int <= 0);
                                                },
                                                _ => { assert(false); },
                                            }
                                        },
                                        _ => { assert(false); },
                                    }
                                },
                                _ => { assert(false); },
                            }
                        }
                        result
                    } else {
                        let result = rotate_right(BalBinTree::Node(inner));
                        proof {
                            match tree_ghost {
                                BalBinTree::Node(tg) => {
                                    match tg.left {
                                        BalBinTree::Node(tg_l) => {
                                            let ll_h = tg_l.left.spec_height();
                                            let lr_h = tg_l.right.spec_height();
                                            let r_h = tg.right.spec_height();

                                            assert(ll_h >= lr_h);
                                            assert(ll_h as int == r_h as int + 1) by {
                                                assert(tg.left.spec_height() as int > tg.right.spec_height() as int + 1);
                                                assert(tg.left.spec_height() as int <= tg.right.spec_height() as int + 2);
                                            };
                                            assert(lr_h >= r_h);
                                            assert(lr_h <= r_h + 1);

                                            let new_rh: nat = 1 + if lr_h >= r_h { lr_h } else { r_h };
                                            assert(lr_h as int - r_h as int >= -1);
                                            assert(lr_h as int - r_h as int <= 1);
                                            assert(ll_h as int - new_rh as int >= -1);
                                            assert(ll_h as int - new_rh as int <= 1);
                                        },
                                        _ => { assert(false); },
                                    }
                                },
                                _ => { assert(false); },
                            }
                        }
                        result
                    }
                } else if rh > lh + 1 {
                    // Right-heavy: check for zig-zag case
                    let right_lh = match &inner.right {
                        BalBinTree::Node(rn) => rn.left.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    let right_rh = match &inner.right {
                        BalBinTree::Node(rn) => rn.right.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    if right_lh > right_rh {
                        // Right-left case: rotate right child right, then rotate root left
                        let BalBinNode { left, value: v, right } = *inner;
                        let new_right = rotate_right(right);
                        let intermediate = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: v,
                            right: new_right,
                        }));
                        proof {
                            assert forall|x: T| tree_contains(intermediate, x) ==
                                tree_contains(tree_ghost, x)
                            by {
                                assert(tree_contains(intermediate, x) ==
                                    (v == x || tree_contains(left, x) || tree_contains(new_right, x)));
                                assert(tree_contains(tree_ghost, x) ==
                                    (v == x || tree_contains(left, x) || tree_contains(right, x)));
                            };
                        }
                        let result = rotate_left(intermediate);
                        proof {
                            match tree_ghost {
                                BalBinTree::Node(tg) => {
                                    match tg.right {
                                        BalBinTree::Node(tg_r) => {
                                            match tg_r.left {
                                                BalBinTree::Node(tg_rl) => {
                                                    let rr_h = tg_r.right.spec_height();
                                                    let rll_h = tg_rl.left.spec_height();
                                                    let rlr_h = tg_rl.right.spec_height();
                                                    let l_h = tg.left.spec_height();

                                                    assert(rr_h == l_h);

                                                    // rl.h = right_lh = rr_h + 1 = l_h + 1
                                                    assert(tg_r.left.spec_height() == l_h + 1);

                                                    // Unfold rl height: 1 + max(rll_h, rlr_h) = l_h + 1
                                                    assert(tg_r.left.spec_height() ==
                                                        1 + if rll_h >= rlr_h { rll_h } else { rlr_h });

                                                    // max(rll_h, rlr_h) = l_h; both <= l_h
                                                    assert(rll_h <= l_h);
                                                    assert(rlr_h <= l_h);

                                                    // avl(rl): |rll_h - rlr_h| <= 1
                                                    assert(avl_balanced(tg_r.left));

                                                    assert(rll_h as int >= l_h as int - 1) by {
                                                        if rll_h >= rlr_h {
                                                            assert(rll_h == l_h);
                                                        } else {
                                                            assert(rlr_h == l_h);
                                                        }
                                                    };
                                                    assert(rlr_h as int >= l_h as int - 1) by {
                                                        if rlr_h >= rll_h {
                                                            assert(rlr_h == l_h);
                                                        } else {
                                                            assert(rll_h == l_h);
                                                        }
                                                    };

                                                    assert(rr_h as int - rlr_h as int >= 0);
                                                    assert(rr_h as int - rlr_h as int <= 1);
                                                    assert(rll_h as int - l_h as int >= -1);
                                                    assert(rll_h as int - l_h as int <= 0);
                                                },
                                                _ => { assert(false); },
                                            }
                                        },
                                        _ => { assert(false); },
                                    }
                                },
                                _ => { assert(false); },
                            }
                        }
                        result
                    } else {
                        let result = rotate_left(BalBinTree::Node(inner));
                        proof {
                            match tree_ghost {
                                BalBinTree::Node(tg) => {
                                    match tg.right {
                                        BalBinTree::Node(tg_r) => {
                                            let rl_h = tg_r.left.spec_height();
                                            let rr_h = tg_r.right.spec_height();
                                            let l_h = tg.left.spec_height();

                                            assert(rr_h >= rl_h);
                                            assert(rr_h as int == l_h as int + 1) by {
                                                assert(tg.right.spec_height() as int > tg.left.spec_height() as int + 1);
                                                assert(tg.right.spec_height() as int <= tg.left.spec_height() as int + 2);
                                            };
                                            assert(rl_h >= l_h);
                                            assert(rl_h <= l_h + 1);

                                            let new_lh: nat = 1 + if l_h >= rl_h { l_h } else { rl_h };
                                            assert(l_h as int - rl_h as int >= -1);
                                            assert(l_h as int - rl_h as int <= 1);
                                            assert(new_lh as int - rr_h as int >= -1);
                                            assert(new_lh as int - rr_h as int <= 1);
                                        },
                                        _ => { assert(false); },
                                    }
                                },
                                _ => { assert(false); },
                            }
                        }
                        result
                    }
                } else {
                    let result = BalBinTree::Node(inner);
                    result
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (result: BalBinTree<T>)
        requires
            tree_is_avl::<T>(node),
            node.spec_height() <= usize::MAX - 1,
        ensures
            tree_is_avl::<T>(result),
            tree_contains(result, value),
            result.spec_height() <= node.spec_height() + 1,
            result.spec_height() >= node.spec_height(),
            forall|x: T| #![auto] tree_contains(result, x) <==>
                (tree_contains(node, x) || x == value),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => {
                BalBinTree::Node(Box::new(BalBinNode {
                    left: BalBinTree::Leaf,
                    value: value,
                    right: BalBinTree::Leaf,
                }))
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;

                match TotalOrder::cmp(&value, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = insert_node(left, value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: node_val,
                            right: right,
                        }));
                        proof {
                            assert(tree_is_bst::<T>(new_left));
                            assert(tree_is_bst::<T>(old_right));

                            assert forall|x: T| tree_contains(new_left, x) implies
                                T::le(x, node_val) && x != node_val
                            by {
                                if tree_contains(old_left, x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| tree_contains(old_right, x) implies
                                T::le(node_val, x) && x != node_val
                            by {};

                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(new_left, x)
                                    || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                            };

                            lemma_max_plus_one(old_left.spec_height(), old_right.spec_height());
                            assert(r.spec_height() <= node.spec_height() + 1);
                            assert(r.spec_height() <= usize::MAX);

                            assert(avl_balanced(new_left));
                            assert(avl_balanced(old_right));

                            assert(r.spec_height() >= node.spec_height());
                        }
                        rebalance(r)
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = insert_node(right, value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: new_right,
                        }));
                        proof {
                            assert(tree_is_bst::<T>(old_left));
                            assert(tree_is_bst::<T>(new_right));

                            assert forall|x: T| tree_contains(old_left, x) implies
                                T::le(x, node_val) && x != node_val
                            by {};

                            assert forall|x: T| tree_contains(new_right, x) implies
                                T::le(node_val, x) && x != node_val
                            by {
                                if tree_contains(old_right, x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(new_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                            };

                            lemma_max_plus_one(old_left.spec_height(), old_right.spec_height());
                            assert(r.spec_height() <= node.spec_height() + 1);
                            assert(r.spec_height() <= usize::MAX);

                            assert(avl_balanced(old_left));
                            assert(avl_balanced(new_right));

                            assert(r.spec_height() >= node.spec_height());
                        }
                        rebalance(r)
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: right,
                        }));
                        proof {
                            assert forall|x: T| tree_contains(r, x) ==
                                (tree_contains(node, x) || x == value)
                            by {
                                assert(tree_contains(r, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                                assert(tree_contains(node, x) ==
                                    (node_val == x
                                    || tree_contains(old_left, x)
                                    || tree_contains(old_right, x)));
                                assert(value == node_val);
                            };
                        }
                        proof {
                            assert(avl_balanced(r));
                        }
                        r
                    }
                }
            }
        }
    }

    fn contains_node<T: TotalOrder>(node: &BalBinTree<T>, target: &T) -> (result: bool)
        requires tree_is_bst::<T>(*node),
        ensures result == tree_contains(*node, *target),
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => false,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => true,
                    core::cmp::Ordering::Less => {
                        let r = contains_node(&inner.left, target);
                        proof {
                            if tree_contains(inner.right, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = contains_node(&inner.right, target);
                        proof {
                            if tree_contains(inner.left, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    fn find_node<'a, T: TotalOrder>(node: &'a BalBinTree<T>, target: &T) -> (result: Option<&'a T>)
        requires tree_is_bst::<T>(*node),
        ensures
            result.is_some() == tree_contains(*node, *target),
            result.is_some() ==> *result.unwrap() == *target,
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => Some(&inner.value),
                    core::cmp::Ordering::Less => {
                        let r = find_node(&inner.left, target);
                        proof {
                            if tree_contains(inner.right, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = find_node(&inner.right, target);
                        proof {
                            if tree_contains(inner.left, *target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (result: Option<&T>)
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.left.is_leaf() {
                    Some(&inner.value)
                } else {
                    min_node(&inner.left)
                }
            }
        }
    }

    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (result: Option<&T>)
        decreases node.spec_size(),
    {
        match node {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() {
                    Some(&inner.value)
                } else {
                    max_node(&inner.right)
                }
            }
        }
    }

    pub fn avl_new<T: TotalOrder>() -> (tree: BSTAVLStEph<T>)
        ensures
            tree_is_avl::<T>(tree.root),
            forall|x: T| !tree_contains(tree.root, x),
    {
        BSTAVLStEph { root: BalBinTree::Leaf }
    }

    pub fn avl_size<T: TotalOrder>(tree: &BSTAVLStEph<T>) -> (n: usize)
        requires tree.root.spec_size() <= usize::MAX,
        ensures n == tree.root.spec_size(),
    {
        tree.root.size()
    }

    pub fn avl_is_empty<T: TotalOrder>(tree: &BSTAVLStEph<T>) -> (b: bool)
        ensures b == (tree.root.spec_size() == 0),
    {
        tree.root.is_leaf()
    }

    pub fn avl_height<T: TotalOrder>(tree: &BSTAVLStEph<T>) -> (h: usize)
        requires tree.root.spec_height() <= usize::MAX,
        ensures h == tree.root.spec_height(),
    {
        tree.root.height()
    }

    pub fn avl_insert<T: TotalOrder>(tree: BSTAVLStEph<T>, value: T) -> (result: BSTAVLStEph<T>)
        requires
            tree_is_avl::<T>(tree.root),
            tree.root.spec_height() <= usize::MAX - 1,
        ensures
            tree_is_avl::<T>(result.root),
            tree_contains(result.root, value),
            forall|x: T| #![auto] tree_contains(result.root, x) <==>
                (tree_contains(tree.root, x) || x == value),
    {
        BSTAVLStEph { root: insert_node(tree.root, value) }
    }

    pub fn avl_contains<T: TotalOrder>(tree: &BSTAVLStEph<T>, target: &T) -> (result: bool)
        requires tree_is_bst::<T>(tree.root),
        ensures result == tree_contains(tree.root, *target),
    {
        contains_node(&tree.root, target)
    }

    pub fn avl_find<'a, T: TotalOrder>(tree: &'a BSTAVLStEph<T>, target: &T) -> (result: Option<&'a T>)
        requires tree_is_bst::<T>(tree.root),
        ensures
            result.is_some() == tree_contains(tree.root, *target),
            result.is_some() ==> *result.unwrap() == *target,
    {
        find_node(&tree.root, target)
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! BSTAVLStEphLit {
        () => { $crate::Chap37::BSTAVLStEph::BSTAVLStEph::avl_new() };
        ($($val:expr),+ $(,)?) => {{
            let mut tree = $crate::Chap37::BSTAVLStEph::BSTAVLStEph::avl_new();
            $(tree = $crate::Chap37::BSTAVLStEph::BSTAVLStEph::avl_insert(tree, $val);)+
            tree
        }};
    }
} // mod

