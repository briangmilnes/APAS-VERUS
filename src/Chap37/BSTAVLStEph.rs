//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree.
//! Verusified: functional-style AVL with BST invariant + balance specs.

// Table of Contents
// 1. module
// 2. imports
// 4. type definitions
// 5. view impls
// 6. spec fns
// 7. proof fns
// 8. traits
// 9. impls
// 12. macros
// 13. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTAVLStEph {

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTAVLStEph<T> {
        pub root: BalBinTree<T>,
    }

    // 5. view impls

    impl<T> View for BSTAVLStEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.root }
    }

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
        tree.tree_is_bst() && avl_balanced(tree)
    }

    // 7. proof fns

    /// Decomposes tree_is_bst two levels deep, exposing children and grandchildren BST
    /// facts plus all ordering quantifiers. Used by rotation proofs.
    proof fn lemma_bst_deep<T: TotalOrder>(tree: BalBinTree<T>)
        requires tree.tree_is_bst(),
        ensures
            match tree {
                BalBinTree::Leaf => true,
                BalBinTree::Node(node) =>
                    node.left.tree_is_bst()
                    && node.right.tree_is_bst()
                    && (forall|x: T| (#[trigger] node.left.tree_contains(x)) ==>
                        T::le(x, node.value) && x != node.value)
                    && (forall|x: T| (#[trigger] node.right.tree_contains(x)) ==>
                        T::le(node.value, x) && x != node.value)
                    && match node.left {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(lnode) =>
                            lnode.left.tree_is_bst()
                            && lnode.right.tree_is_bst()
                            && (forall|x: T| (#[trigger] lnode.left.tree_contains(x)) ==>
                                T::le(x, lnode.value) && x != lnode.value)
                            && (forall|x: T| (#[trigger] lnode.right.tree_contains(x)) ==>
                                T::le(lnode.value, x) && x != lnode.value)
                    }
                    && match node.right {
                        BalBinTree::Leaf => true,
                        BalBinTree::Node(rnode) =>
                            rnode.left.tree_is_bst()
                            && rnode.right.tree_is_bst()
                            && (forall|x: T| (#[trigger] rnode.left.tree_contains(x)) ==>
                                T::le(x, rnode.value) && x != rnode.value)
                            && (forall|x: T| (#[trigger] rnode.right.tree_contains(x)) ==>
                                T::le(rnode.value, x) && x != rnode.value)
                    }
            }
    {
        match tree {
            BalBinTree::Leaf => {},
            BalBinTree::Node(node) => {
                match node.left {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(_) => {},
                }
                match node.right {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(_) => {},
                }
            },
        }
    }

    /// max(a+1, b) <= max(a, b) + 1 for natural numbers.
    proof fn lemma_max_plus_one(a: nat, b: nat)
        ensures
            (if a >= b { a + 1 } else { b }) <= (if a >= b { a } else { b }) + 1,
    {
    }

    // 8. traits

    pub trait BSTAVLStEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_root(self) -> BalBinTree<T>;
        spec fn spec_bstavlsteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures
                tree.spec_bstavlsteph_wf(),
                tree_is_avl::<T>(tree.spec_root()),
                forall|x: T| !tree.spec_root().tree_contains(x);
        fn size(&self) -> (n: usize)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstavlsteph_wf(),
            ensures b == (self.spec_root().spec_size() == 0);
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().spec_height() <= usize::MAX,
            ensures h == self.spec_root().spec_height();
        fn insert(self, value: T) -> (inserted: Self)
            requires
                self.spec_bstavlsteph_wf(),
                tree_is_avl::<T>(self.spec_root()),
                self.spec_root().spec_height() <= usize::MAX - 1,
            ensures
                inserted.spec_bstavlsteph_wf(),
                tree_is_avl::<T>(inserted.spec_root()),
                inserted.spec_root().tree_contains(value),
                forall|x: T| (#[trigger] inserted.spec_root().tree_contains(x)) <==>
                    (self.spec_root().tree_contains(x) || x == value);
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures found == self.spec_root().tree_contains(*target);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                found.is_some() == self.spec_root().tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
    }

    // 9. impls

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
    fn rotate_right<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires
            tree.tree_is_bst(),
            !(tree is Leaf),
        ensures
            rotated.tree_is_bst(),
            forall|x: T| (#[trigger] rotated.tree_contains(x)) == tree.tree_contains(x),
            match tree {
                BalBinTree::Node(outer) => match outer.left {
                    BalBinTree::Node(l) => {
                        let lr_h = l.right.spec_height();
                        let r_h = outer.right.spec_height();
                        let ll_h = l.left.spec_height();
                        let new_rh: nat = 1 + if lr_h >= r_h { lr_h } else { r_h };
                        &&& rotated.spec_height() == (1 + if ll_h >= new_rh { ll_h } else { new_rh })
                        &&& ((avl_balanced(l.left) && avl_balanced(l.right) && avl_balanced(outer.right)
                             && lr_h as int - r_h as int >= -1 && lr_h as int - r_h as int <= 1
                             && ll_h as int - new_rh as int >= -1 && ll_h as int - new_rh as int <= 1)
                            ==> avl_balanced(rotated))
                        &&& rotated is Node
                        &&& match rotated {
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

                            assert forall|z: T| old_lr.tree_contains(z) implies
                                #[trigger] T::le(z, y_val) && z != y_val
                            by {
                                assert(old_left.tree_contains(z));
                            };

                            assert(old_left.tree_contains(x_val));
                            assert(x_val != y_val) by {
                                assert(old_left.tree_contains(x_val));
                            };

                            assert(right_sub.tree_is_bst());

                            assert forall|z: T| right_sub.tree_contains(z) implies
                                #[trigger] T::le(x_val, z) && z != x_val
                            by {
                                if old_lr.tree_contains(z) {
                                } else if z == y_val {
                                    assert(x_val != y_val);
                                } else if old_r.tree_contains(z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                }
                            };

                            assert forall|z: T| r.tree_contains(z) ==
                                tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) ==
                                    (x_val == z
                                    || old_ll.tree_contains(z)
                                    || right_sub.tree_contains(z)));
                                assert(right_sub.tree_contains(z) ==
                                    (y_val == z
                                    || old_lr.tree_contains(z)
                                    || old_r.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) ==
                                    (y_val == z
                                    || old_left.tree_contains(z)
                                    || old_r.tree_contains(z)));
                                assert(old_left.tree_contains(z) ==
                                    (x_val == z
                                    || old_ll.tree_contains(z)
                                    || old_lr.tree_contains(z)));
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

                            assert(r.tree_is_bst());
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

    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
    fn rotate_left<T: TotalOrder>(tree: BalBinTree<T>) -> (rotated: BalBinTree<T>)
        requires
            tree.tree_is_bst(),
            !(tree is Leaf),
        ensures
            rotated.tree_is_bst(),
            forall|x: T| (#[trigger] rotated.tree_contains(x)) == tree.tree_contains(x),
            match tree {
                BalBinTree::Node(outer) => match outer.right {
                    BalBinTree::Node(r) => {
                        let rl_h = r.left.spec_height();
                        let l_h = outer.left.spec_height();
                        let rr_h = r.right.spec_height();
                        let new_lh: nat = 1 + if l_h >= rl_h { l_h } else { rl_h };
                        &&& rotated.spec_height() == (1 + if new_lh >= rr_h { new_lh } else { rr_h })
                        &&& ((avl_balanced(outer.left) && avl_balanced(r.left) && avl_balanced(r.right)
                             && l_h as int - rl_h as int >= -1 && l_h as int - rl_h as int <= 1
                             && new_lh as int - rr_h as int >= -1 && new_lh as int - rr_h as int <= 1)
                            ==> avl_balanced(rotated))
                        &&& rotated is Node
                        &&& match rotated {
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

                            assert forall|z: T| old_rl.tree_contains(z) implies
                                #[trigger] T::le(x_val, z) && z != x_val
                            by {
                                assert(old_right.tree_contains(z));
                            };

                            assert(old_right.tree_contains(y_val));
                            assert(x_val != y_val) by {
                                assert(old_right.tree_contains(y_val));
                            };

                            assert(left_sub.tree_is_bst());

                            assert forall|z: T| left_sub.tree_contains(z) implies
                                #[trigger] T::le(z, y_val) && z != y_val
                            by {
                                if old_l.tree_contains(z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                } else if z == x_val {
                                    assert(x_val != y_val);
                                } else if old_rl.tree_contains(z) {
                                }
                            };

                            assert forall|z: T| r.tree_contains(z) ==
                                tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) ==
                                    (y_val == z
                                    || left_sub.tree_contains(z)
                                    || old_rr.tree_contains(z)));
                                assert(left_sub.tree_contains(z) ==
                                    (x_val == z
                                    || old_l.tree_contains(z)
                                    || old_rl.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) ==
                                    (x_val == z
                                    || old_l.tree_contains(z)
                                    || old_right.tree_contains(z)));
                                assert(old_right.tree_contains(z) ==
                                    (y_val == z
                                    || old_rl.tree_contains(z)
                                    || old_rr.tree_contains(z)));
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
    /// - APAS: Work O(1), Span O(1)
    /// - Claude-Opus-4.6: Work O(1), Span O(1) -- constant-number of rotations.
    fn rebalance<T: TotalOrder>(tree: BalBinTree<T>) -> (balanced: BalBinTree<T>)
        requires
            tree.tree_is_bst(),
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
            balanced.tree_is_bst(),
            avl_balanced(balanced),
            balanced.spec_height() <= tree.spec_height(),
            balanced.spec_height() + 1 >= tree.spec_height(),
            forall|x: T| (#[trigger] balanced.tree_contains(x)) == tree.tree_contains(x),
            match tree {
                BalBinTree::Node(inner) => {
                    let lh = inner.left.spec_height() as int;
                    let rh = inner.right.spec_height() as int;
                    (-1 <= lh - rh && lh - rh <= 1) ==> balanced.spec_height() == tree.spec_height()
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
                            assert forall|x: T| intermediate.tree_contains(x) ==
                                tree_ghost.tree_contains(x)
                            by {
                                assert(intermediate.tree_contains(x) ==
                                    (v == x || new_left.tree_contains(x) || right.tree_contains(x)));
                                assert(tree_ghost.tree_contains(x) ==
                                    (v == x || left.tree_contains(x) || right.tree_contains(x)));
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
                            assert forall|x: T| intermediate.tree_contains(x) ==
                                tree_ghost.tree_contains(x)
                            by {
                                assert(intermediate.tree_contains(x) ==
                                    (v == x || left.tree_contains(x) || new_right.tree_contains(x)));
                                assert(tree_ghost.tree_contains(x) ==
                                    (v == x || left.tree_contains(x) || right.tree_contains(x)));
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

    /// - APAS: Work O(lg n), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- agrees with APAS; AVL height is O(lg n).
    fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
        requires
            tree_is_avl::<T>(node),
            node.spec_height() <= usize::MAX - 1,
        ensures
            tree_is_avl::<T>(inserted),
            inserted.tree_contains(value),
            inserted.spec_height() <= node.spec_height() + 1,
            inserted.spec_height() >= node.spec_height(),
            forall|x: T| (#[trigger] inserted.tree_contains(x)) <==>
                (node.tree_contains(x) || x == value),
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
                            assert(new_left.tree_is_bst());
                            assert(old_right.tree_is_bst());

                            assert forall|x: T| new_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {
                                if old_left.tree_contains(x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| old_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {};

                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || new_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
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
                            assert(old_left.tree_is_bst());
                            assert(new_right.tree_is_bst());

                            assert forall|x: T| old_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {};

                            assert forall|x: T| new_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {
                                if old_right.tree_contains(x) {
                                } else {
                                    assert(x == value);
                                }
                            };

                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || new_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
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
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
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

    /// - APAS: Work O(lg n), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- agrees with APAS.
    fn contains_node<T: TotalOrder>(node: &BalBinTree<T>, target: &T) -> (found: bool)
        requires (*node).tree_is_bst(),
        ensures found == (*node).tree_contains(*target),
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
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = contains_node(&inner.right, target);
                        proof {
                            if inner.left.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    /// - APAS: Work O(lg n), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- agrees with APAS.
    fn find_node<'a, T: TotalOrder>(node: &'a BalBinTree<T>, target: &T) -> (found: Option<&'a T>)
        requires (*node).tree_is_bst(),
        ensures
            found.is_some() == (*node).tree_contains(*target),
            found.is_some() ==> *found.unwrap() == *target,
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
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = find_node(&inner.right, target);
                        proof {
                            if inner.left.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                }
            }
        }
    }

    /// - APAS: (no cost stated)
    /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- descends leftmost path; AVL balanced.
    fn min_node<T: TotalOrder>(node: &BalBinTree<T>) -> (min: Option<&T>)
        requires (*node).tree_is_bst(),
        ensures
            node.spec_size() == 0 ==> min.is_none(),
            node.spec_size() > 0 ==> min.is_some(),
            min.is_some() ==> (*node).tree_contains(*min.unwrap()),
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

    /// - APAS: (no cost stated)
    /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- descends rightmost path; AVL balanced.
    fn max_node<T: TotalOrder>(node: &BalBinTree<T>) -> (max: Option<&T>)
        requires (*node).tree_is_bst(),
        ensures
            node.spec_size() == 0 ==> max.is_none(),
            node.spec_size() > 0 ==> max.is_some(),
            max.is_some() ==> (*node).tree_contains(*max.unwrap()),
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

    impl<T: TotalOrder> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { self.root }
        open spec fn spec_bstavlsteph_wf(&self) -> bool { tree_is_avl::<T>(self.spec_root()) }

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- agrees with APAS.
        fn new() -> (tree: Self) {
            BSTAVLStEph { root: BalBinTree::Leaf }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to BalBinTree::size.
        fn size(&self) -> (n: usize) {
            self.root.size()
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- leaf check.
        fn is_empty(&self) -> (b: bool) {
            self.root.is_leaf()
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) -- delegates to BalBinTree::height.
        fn height(&self) -> (h: usize) {
            self.root.height()
        }

        /// - APAS: Work O(lg n), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- agrees with APAS; AVL guarantees h = O(lg n).
        fn insert(self, value: T) -> (inserted: Self) {
            BSTAVLStEph { root: insert_node(self.root, value) }
        }

        /// - APAS: Work O(lg n), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- agrees with APAS; AVL guarantees h = O(lg n).
        fn contains(&self, target: &T) -> (found: bool) {
            contains_node(&self.root, target)
        }

        /// - APAS: Work O(lg n), Span O(lg n)
        /// - Claude-Opus-4.6: Work O(lg n), Span O(lg n) -- agrees with APAS; AVL guarantees h = O(lg n).
        fn find(&self, target: &T) -> (found: Option<&T>) {
            find_node(&self.root, target)
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! BSTAVLStEphLit {
        () => { <$crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEph<_> as $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEphTrait<_>>::new() };
        ($($val:expr),+ $(,)?) => {{
            use $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEphTrait;
            let mut tree = $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEph::new();
            $(tree = tree.insert($val);)+
            tree
        }};
    }

    // 13. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for BSTAVLStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTAVLStEph")
                .field("root", &self.root)
                .finish()
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for BSTAVLStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTAVLStEph({:?})", &self.root)
        }
    }
} // mod
