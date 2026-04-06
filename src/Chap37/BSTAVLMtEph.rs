//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Ephemeral AVL-balanced binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on BalBinTree) in sections 7/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//  1. module
//  2. imports
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  11. top level coarse locking
//  13. macros
//  14. derive impls outside verus!

// 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTAVLMtEph {

    use core::marker::PhantomData;

    use vstd::prelude::*;
    use vstd::rwlock::{ReadHandle, RwLock, RwLockPredicate, WriteHandle};

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;

    verus! {

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use crate::Chap37::BSTAVLStEph::BSTAVLStEph::{avl_balanced, tree_is_avl};
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    // 7. proof fns/broadcast groups

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
                assert(node.left.tree_is_bst());
                assert(node.right.tree_is_bst());
                match node.left {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(lnode) => {
                        assert(lnode.left.tree_is_bst());
                        assert(lnode.right.tree_is_bst());
                    },
                }
                match node.right {
                    BalBinTree::Leaf => {},
                    BalBinTree::Node(rnode) => {
                        assert(rnode.left.tree_is_bst());
                        assert(rnode.right.tree_is_bst());
                    },
                }
            },
        }
    }

    proof fn lemma_max_plus_one(a: nat, b: nat)
        ensures
            (if a >= b { a + 1 } else { b }) <= (if a >= b { a } else { b }) + 1,
    {
    }


    // 9. impls

    // 8. traits

    /// Exec AVL BST operations on BalBinTree nodes (Mt variant).
    pub trait BSTAVLMtNodeFns<T: TotalOrder>: Sized + BSTSpecFns<T> + BalBinTreeTrait<T> {
        // Spec accessors for abstract ensures.
        spec fn avl_balanced_spec(self) -> bool;
        spec fn tree_is_avl_spec(self) -> bool;
        spec fn spec_left(self) -> Self;
        spec fn spec_right(self) -> Self;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_right(self) -> (rotated: Self)
            requires self.tree_is_bst(), !self.spec_is_leaf(),
            ensures
                rotated.tree_is_bst(),
                rotated.spec_size() == self.spec_size(),
                forall|x: T| (#[trigger] rotated.tree_contains(x)) == self.tree_contains(x),
                !self.spec_left().spec_is_leaf() ==> ({
                    let sl = self.spec_left();
                    let sr = self.spec_right();
                    let sll = sl.spec_left();
                    let slr = sl.spec_right();
                    let lr_h = slr.spec_height();
                    let r_h = sr.spec_height();
                    let ll_h = sll.spec_height();
                    let new_rh: nat = 1 + if lr_h >= r_h { lr_h } else { r_h };
                    &&& rotated.spec_height() == (1 + if ll_h >= new_rh { ll_h } else { new_rh })
                    &&& ((sll.avl_balanced_spec() && slr.avl_balanced_spec() && sr.avl_balanced_spec()
                         && lr_h as int - r_h as int >= -1 && lr_h as int - r_h as int <= 1
                         && ll_h as int - new_rh as int >= -1 && ll_h as int - new_rh as int <= 1)
                        ==> rotated.avl_balanced_spec())
                    &&& !rotated.spec_is_leaf()
                    &&& rotated.spec_left().spec_height() == ll_h
                    &&& rotated.spec_left().avl_balanced_spec() == sll.avl_balanced_spec()
                    &&& rotated.spec_right().spec_height() == new_rh
                    &&& (rotated.spec_right().avl_balanced_spec() <==> (slr.avl_balanced_spec()
                        && sr.avl_balanced_spec() && {
                        let lh = lr_h as int;
                        let rh = r_h as int;
                        -1 <= lh - rh && lh - rh <= 1
                    }))
                }),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rotate_left(self) -> (rotated: Self)
            requires self.tree_is_bst(), !self.spec_is_leaf(),
            ensures
                rotated.tree_is_bst(),
                rotated.spec_size() == self.spec_size(),
                forall|x: T| (#[trigger] rotated.tree_contains(x)) == self.tree_contains(x),
                !self.spec_right().spec_is_leaf() ==> ({
                    let sl = self.spec_left();
                    let sr = self.spec_right();
                    let srl = sr.spec_left();
                    let srr = sr.spec_right();
                    let rl_h = srl.spec_height();
                    let l_h = sl.spec_height();
                    let rr_h = srr.spec_height();
                    let new_lh: nat = 1 + if l_h >= rl_h { l_h } else { rl_h };
                    &&& rotated.spec_height() == (1 + if new_lh >= rr_h { new_lh } else { rr_h })
                    &&& ((sl.avl_balanced_spec() && srl.avl_balanced_spec() && srr.avl_balanced_spec()
                         && l_h as int - rl_h as int >= -1 && l_h as int - rl_h as int <= 1
                         && new_lh as int - rr_h as int >= -1 && new_lh as int - rr_h as int <= 1)
                        ==> rotated.avl_balanced_spec())
                    &&& !rotated.spec_is_leaf()
                    &&& rotated.spec_right().spec_height() == rr_h
                    &&& rotated.spec_right().avl_balanced_spec() == srr.avl_balanced_spec()
                    &&& rotated.spec_left().spec_height() == new_lh
                    &&& (rotated.spec_left().avl_balanced_spec() <==> (sl.avl_balanced_spec()
                        && srl.avl_balanced_spec() && {
                        let lh = l_h as int;
                        let rh = rl_h as int;
                        -1 <= lh - rh && lh - rh <= 1
                    }))
                }),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn rebalance(self) -> (balanced: Self)
            requires
                self.tree_is_bst(),
                !self.spec_is_leaf(),
                self.spec_height() <= usize::MAX,
                self.spec_left().avl_balanced_spec() && self.spec_right().avl_balanced_spec()
                && {
                    let lh = self.spec_left().spec_height() as int;
                    let rh = self.spec_right().spec_height() as int;
                    -2 <= lh - rh && lh - rh <= 2
                },
            ensures
                balanced.tree_is_bst(),
                balanced.avl_balanced_spec(),
                balanced.spec_size() == self.spec_size(),
                balanced.spec_height() <= self.spec_height(),
                balanced.spec_height() + 1 >= self.spec_height(),
                forall|x: T| (#[trigger] balanced.tree_contains(x)) == self.tree_contains(x),
                {
                    let lh = self.spec_left().spec_height() as int;
                    let rh = self.spec_right().spec_height() as int;
                    (-1 <= lh - rh && lh - rh <= 1) ==> balanced.spec_height() == self.spec_height()
                },
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert_node(self, value: T) -> (inserted: Self)
            requires
                self.tree_is_avl_spec(),
                self.spec_height() <= usize::MAX - 1,
            ensures
                inserted.tree_is_avl_spec(),
                inserted.tree_contains(value),
                inserted.spec_size() <= self.spec_size() + 1,
                inserted.spec_height() <= self.spec_height() + 1,
                inserted.spec_height() >= self.spec_height(),
                forall|x: T| (#[trigger] inserted.tree_contains(x)) <==>
                    (self.tree_contains(x) || x == value),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn contains_node(&self, target: &T) -> (found: bool)
            requires (*self).tree_is_bst(),
            ensures found == (*self).tree_contains(*target),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find_node(&self, target: &T) -> (found: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                found.is_some() == (*self).tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target,
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn min_node(&self) -> (min: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> min.is_none(),
                (*self).spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> (*self).tree_contains(*min.unwrap()),
            ;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn max_node(&self) -> (max: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> max.is_none(),
                (*self).spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> (*self).tree_contains(*max.unwrap()),
            ;
    }

    impl<T: TotalOrder> BSTAVLMtNodeFns<T> for BalBinTree<T> {

    open spec fn avl_balanced_spec(self) -> bool { avl_balanced(self) }
    open spec fn tree_is_avl_spec(self) -> bool { tree_is_avl(self) }
    open spec fn spec_left(self) -> Self {
        match self {
            BalBinTree::Leaf => BalBinTree::Leaf,
            BalBinTree::Node(n) => n.left,
        }
    }
    open spec fn spec_right(self) -> Self {
        match self {
            BalBinTree::Leaf => BalBinTree::Leaf,
            BalBinTree::Node(n) => n.right,
        }
    }

    fn rotate_right(self) -> (rotated: Self)
    {
        let ghost tree_ghost = self;
        match self {
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
                            left: lr, value: y_val, right: r,
                        }));
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: ll, value: x_val, right: right_sub,
                        }));
                        proof {
                            lemma_bst_deep::<T>(tree_ghost);
                            assert forall|z: T| #[trigger] old_lr.tree_contains(z) implies
                                T::le(z, y_val) && z != y_val
                            by { assert(old_left.tree_contains(z)); };
                            assert(old_left.tree_contains(x_val));
                            assert(x_val != y_val) by {
                                assert(old_left.tree_contains(x_val));
                            };
                            assert(right_sub.tree_is_bst());
                            assert forall|z: T| #[trigger] right_sub.tree_contains(z) implies
                                T::le(x_val, z) && z != x_val
                            by {
                                if old_lr.tree_contains(z) {}
                                else if z == y_val { assert(x_val != y_val); }
                                else if old_r.tree_contains(z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val { T::antisymmetric(x_val, y_val); }
                                }
                            };
                            assert forall|z: T| r.tree_contains(z) == tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) == (x_val == z
                                    || old_ll.tree_contains(z) || right_sub.tree_contains(z)));
                                assert(right_sub.tree_contains(z) == (y_val == z
                                    || old_lr.tree_contains(z) || old_r.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) == (y_val == z
                                    || old_left.tree_contains(z) || old_r.tree_contains(z)));
                                assert(old_left.tree_contains(z) == (x_val == z
                                    || old_ll.tree_contains(z) || old_lr.tree_contains(z)));
                            };
                            assert(right_sub.spec_size() == 1 + old_lr.spec_size() + old_r.spec_size());
                            assert(r.spec_size() == 1 + old_ll.spec_size() + right_sub.spec_size());
                            assert(old_left.spec_size() == 1 + old_ll.spec_size() + old_lr.spec_size());
                            assert(tree_ghost.spec_size() == 1 + old_left.spec_size() + old_r.spec_size());
                            assert(right_sub.spec_height() == 1 + if old_lr.spec_height()
                                >= old_r.spec_height() { old_lr.spec_height() }
                                else { old_r.spec_height() });
                            assert(r.spec_height() == 1 + if old_ll.spec_height()
                                >= right_sub.spec_height() { old_ll.spec_height() }
                                else { right_sub.spec_height() });
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
                            left: BalBinTree::Leaf, value: y_val, right: r,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    fn rotate_left(self) -> (rotated: Self)
    {
        let ghost tree_ghost = self;
        match self {
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
                            left: l, value: x_val, right: rl,
                        }));
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left_sub, value: y_val, right: rr,
                        }));
                        proof {
                            lemma_bst_deep::<T>(tree_ghost);
                            assert forall|z: T| #[trigger] old_rl.tree_contains(z) implies
                                T::le(x_val, z) && z != x_val
                            by { assert(old_right.tree_contains(z)); };
                            assert(old_right.tree_contains(y_val));
                            assert(x_val != y_val) by {
                                assert(old_right.tree_contains(y_val));
                            };
                            assert(left_sub.tree_is_bst());
                            assert forall|z: T| #[trigger] left_sub.tree_contains(z) implies
                                T::le(z, y_val) && z != y_val
                            by {
                                if old_l.tree_contains(z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val { T::antisymmetric(x_val, y_val); }
                                } else if z == x_val { assert(x_val != y_val); }
                                else if old_rl.tree_contains(z) {}
                            };
                            assert forall|z: T| r.tree_contains(z) == tree_ghost.tree_contains(z)
                            by {
                                assert(r.tree_contains(z) == (y_val == z
                                    || left_sub.tree_contains(z) || old_rr.tree_contains(z)));
                                assert(left_sub.tree_contains(z) == (x_val == z
                                    || old_l.tree_contains(z) || old_rl.tree_contains(z)));
                                assert(tree_ghost.tree_contains(z) == (x_val == z
                                    || old_l.tree_contains(z) || old_right.tree_contains(z)));
                                assert(old_right.tree_contains(z) == (y_val == z
                                    || old_rl.tree_contains(z) || old_rr.tree_contains(z)));
                            };
                            assert(left_sub.spec_size() == 1 + old_l.spec_size() + old_rl.spec_size());
                            assert(r.spec_size() == 1 + left_sub.spec_size() + old_rr.spec_size());
                            assert(old_right.spec_size() == 1 + old_rl.spec_size() + old_rr.spec_size());
                            assert(tree_ghost.spec_size() == 1 + old_l.spec_size() + old_right.spec_size());
                            assert(left_sub.spec_height() == 1 + if old_l.spec_height()
                                >= old_rl.spec_height() { old_l.spec_height() }
                                else { old_rl.spec_height() });
                            assert(r.spec_height() == 1 + if left_sub.spec_height()
                                >= old_rr.spec_height() { left_sub.spec_height() }
                                else { old_rr.spec_height() });
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
                            left: l, value: x_val, right: BalBinTree::Leaf,
                        }))
                    }
                }
            }
            BalBinTree::Leaf => { proof { assert(false); } BalBinTree::Leaf }
        }
    }

    // Verified AVL rebalance (Layer 1).

    fn rebalance(self) -> (balanced: Self)
    {
        let ghost tree_ghost = self;
        match self {
            BalBinTree::Node(inner) => {
                let lh = inner.left.height();
                let rh = inner.right.height();
                if lh > rh + 1 {
                    let left_rh = match &inner.left {
                        BalBinTree::Node(ln) => ln.right.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    let left_lh = match &inner.left {
                        BalBinTree::Node(ln) => ln.left.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    if left_rh > left_lh {
                        let BalBinNode { left, value: v, right } = *inner;
                        let new_left = left.rotate_left();
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
                        let result = intermediate.rotate_right();
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
                                                    assert(tg_l.right.spec_height() == r_h + 1);
                                                    assert(tg_l.right.spec_height() ==
                                                        1 + if lrl_h >= lrr_h { lrl_h } else { lrr_h });
                                                    assert(lrl_h <= r_h);
                                                    assert(lrr_h <= r_h);
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
                        let result = BalBinTree::Node(inner).rotate_right();
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
                    let right_lh = match &inner.right {
                        BalBinTree::Node(rn) => rn.left.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    let right_rh = match &inner.right {
                        BalBinTree::Node(rn) => rn.right.height(),
                        BalBinTree::Leaf => 0usize,
                    };
                    if right_lh > right_rh {
                        let BalBinNode { left, value: v, right } = *inner;
                        let new_right = right.rotate_right();
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
                        let result = intermediate.rotate_left();
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
                                                    assert(tg_r.left.spec_height() == l_h + 1);
                                                    assert(tg_r.left.spec_height() ==
                                                        1 + if rll_h >= rlr_h { rll_h } else { rlr_h });
                                                    assert(rll_h <= l_h);
                                                    assert(rlr_h <= l_h);
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
                        let result = BalBinTree::Node(inner).rotate_left();
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

    // Verified AVL insert (Layer 1).

    fn insert_node(self, value: T) -> (inserted: Self)
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
            BalBinTree::Leaf => {
                BalBinTree::Node(Box::new(BalBinNode {
                    left: BalBinTree::Leaf, value: value, right: BalBinTree::Leaf,
                }))
            }
            BalBinTree::Node(inner) => {
                let BalBinNode { left, value: node_val, right } = *inner;
                let ghost old_left = left;
                let ghost old_right = right;
                match TotalOrder::cmp(&value, &node_val) {
                    core::cmp::Ordering::Less => {
                        let new_left = left.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left, value: node_val, right: right,
                        }));
                        proof {
                            assert(new_left.tree_is_bst());
                            assert(old_right.tree_is_bst());
                            assert forall|x: T| #[trigger] new_left.tree_contains(x) implies
                                T::le(x, node_val) && x != node_val
                            by { if old_left.tree_contains(x) {} else { assert(x == value); } };
                            assert forall|x: T| #[trigger] old_right.tree_contains(x) implies
                                T::le(node_val, x) && x != node_val by {};
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) == (node_val == x
                                    || new_left.tree_contains(x) || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                            };
                            lemma_max_plus_one(old_left.spec_height(), old_right.spec_height());
                            assert(r.spec_height() <= node.spec_height() + 1);
                            assert(r.spec_height() <= usize::MAX);
                            assert(avl_balanced(new_left));
                            assert(avl_balanced(old_right));
                            assert(r.spec_height() >= node.spec_height());
                        }
                        r.rebalance()
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = right.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: new_right,
                        }));
                        proof {
                            assert(old_left.tree_is_bst());
                            assert(new_right.tree_is_bst());
                            assert forall|x: T| #[trigger] old_left.tree_contains(x) implies
                                T::le(x, node_val) && x != node_val by {};
                            assert forall|x: T| #[trigger] new_right.tree_contains(x) implies
                                T::le(node_val, x) && x != node_val
                            by { if old_right.tree_contains(x) {} else { assert(x == value); } };
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || new_right.tree_contains(x)));
                                assert(node.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                            };
                            lemma_max_plus_one(old_right.spec_height(), old_left.spec_height());
                            assert(r.spec_height() <= node.spec_height() + 1);
                            assert(r.spec_height() <= usize::MAX);
                            assert(avl_balanced(old_left));
                            assert(avl_balanced(new_right));
                            assert(r.spec_height() >= node.spec_height());
                        }
                        r.rebalance()
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left, value: node_val, right: right,
                        }));
                        proof {
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                assert(r.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                                assert(node.tree_contains(x) == (node_val == x
                                    || old_left.tree_contains(x) || old_right.tree_contains(x)));
                                assert(value == node_val);
                            };
                        }
                        r
                    }
                }
            }
        }
    }

    fn contains_node(&self, target: &T) -> (found: bool)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => false,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => true,
                    core::cmp::Ordering::Less => {
                        let r = inner.left.contains_node(target);
                        proof { if inner.right.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.contains_node(target);
                        proof { if inner.left.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                }
            }
        }
    }

    fn find_node(&self, target: &T) -> (found: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                match TotalOrder::cmp(target, &inner.value) {
                    core::cmp::Ordering::Equal => Some(&inner.value),
                    core::cmp::Ordering::Less => {
                        let r = inner.left.find_node(target);
                        proof { if inner.right.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.find_node(target);
                        proof { if inner.left.tree_contains(*target) { T::antisymmetric(*target, inner.value); } }
                        r
                    }
                }
            }
        }
    }

    fn min_node(&self) -> (min: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.left.is_leaf() { Some(&inner.value) }
                else { inner.left.min_node() }
            }
        }
    }

    fn max_node(&self) -> (max: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() { Some(&inner.value) }
                else { inner.right.max_node() }
            }
        }
    }

    } // impl BSTAVLMtNodeFns


    // 11. top level coarse locking

    /// Lock predicate: the inner tree satisfies AVL invariant and fits in usize.
    pub(crate) struct BSTAVLMtEphInv<T> {
        _phantom: PhantomData<T>,
    }

    impl<T: TotalOrder> RwLockPredicate<BalBinTree<T>> for BSTAVLMtEphInv<T> {
        open spec fn inv(self, tree: BalBinTree<T>) -> bool {
            tree_is_avl::<T>(tree)
                && tree.spec_size() <= usize::MAX
                && tree.spec_height() <= usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTAVLMtEph<T: TotalOrder> {
        pub(crate) root: RwLock<BalBinTree<T>, BSTAVLMtEphInv<T>>,
        pub(crate) ghost_root: Ghost<BalBinTree<T>>,
    }

    impl<T: TotalOrder> BSTAVLMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            tree_is_avl::<T>(self.ghost_root@)
            && self.ghost_root@.spec_size() <= usize::MAX
            && self.ghost_root@.spec_height() <= usize::MAX
        }

        pub closed spec fn spec_ghost_root(self) -> BalBinTree<T> {
            self.ghost_root@
        }
    }

    impl<T: TotalOrder> View for BSTAVLMtEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.spec_ghost_root() }
    }

    pub trait BSTAVLMtEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_bstavlmteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures tree.spec_bstavlmteph_wf(),
                    tree@ is Leaf,
                    tree_is_avl::<T>(tree@),
                    tree@.tree_is_bst(),
                    forall|x: T| !tree@.tree_contains(x);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstavlmteph_wf(),
            ensures self.spec_bstavlmteph_wf(),
                    match r {
                        Ok(_) => tree_is_avl::<T>(self@)
                            && self@.tree_contains(value)
                            && forall|x: T| (#[trigger] self@.tree_contains(x)) <==>
                                (old(self)@.tree_contains(x) || x == value),
                        Err(_) => self@ == old(self)@,
                    };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn contains(&self, target: &T) -> (found: bool)
            requires self.spec_bstavlmteph_wf(),
            ensures found == self@.tree_contains(*target);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize)
            requires self.spec_bstavlmteph_wf(),
            ensures n as nat == self@.spec_size();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstavlmteph_wf(),
            ensures b == (self@ is Leaf);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires self.spec_bstavlmteph_wf(),
            ensures h as nat == self@.spec_height();

        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T)) — matches APAS
        fn find(&self, target: &T) -> (found: Option<T>) where T: Clone + Eq
            requires self.spec_bstavlmteph_wf(),
            ensures
                found.is_some() == self@.tree_contains(*target),
                found.is_some() ==> found.unwrap() == *target;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn minimum(&self) -> (min: Option<T>) where T: Clone + Eq
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn maximum(&self) -> (max: Option<T>) where T: Clone + Eq
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>) where T: Clone + Eq
            requires self.spec_bstavlmteph_wf(), obeys_feq_clone::<T>(),
            ensures true;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>) where T: Clone + Eq
            requires self.spec_bstavlmteph_wf(), obeys_feq_clone::<T>(),
            ensures true;
    }

    impl<T: TotalOrder> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {
        open spec fn spec_bstavlmteph_wf(&self) -> bool {
            tree_is_avl::<T>(self@)
            && self@.spec_size() <= usize::MAX
            && self@.spec_height() <= usize::MAX
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self) {
            BSTAVLMtEph {
                root: RwLock::new(
                    BalBinTree::Leaf,
                    Ghost(BSTAVLMtEphInv { _phantom: PhantomData }),
                ),
                ghost_root: Ghost(BalBinTree::Leaf),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, value: T) -> (r: Result<(), ()>) {
            let (tree, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == tree); }
            let current_size = tree.size();
            let current_height = tree.height();
            if current_size < usize::MAX && current_height < usize::MAX {
                let new_tree = tree.insert_node(value);
                proof {
                    assert(tree_is_avl::<T>(new_tree));
                    assert(new_tree.spec_size() <= usize::MAX);
                    assert(new_tree.spec_height() <= usize::MAX);
                }
                let ghost new_root = new_tree;
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(new_tree);
                Ok(())
            } else {
                write_handle.release_write(tree);
                Err(())
            }
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn contains(&self, target: &T) -> (found: bool) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = tree_ref.contains_node(target);
            proof { assume(found == self@.tree_contains(*target)); }
            read_handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_size() <= usize::MAX);
            let n = tree_ref.size();
            proof { assume(n as nat == self@.spec_size()); }
            read_handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let b = tree_ref.is_leaf();
            proof { assume(b == (self@ is Leaf)); }
            read_handle.release_read();
            b
        }

        // Reader: assume return value matches ghost.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize) {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            assert(tree_ref.spec_height() <= usize::MAX);
            let h = tree_ref.height();
            proof { assume(h as nat == self@.spec_height()); }
            read_handle.release_read();
            h
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find(&self, target: &T) -> (found: Option<T>) where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let found = tree_ref.find_node(target).cloned();
            proof {
                assume(found.is_some() == self@.tree_contains(*target));
                accept(found.is_some() ==> found.unwrap() == *target);
            }
            read_handle.release_read();
            found
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn minimum(&self) -> Option<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let min = tree_ref.min_node().cloned();
            read_handle.release_read();
            min
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn maximum(&self) -> Option<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let max = tree_ref.max_node().cloned();
            read_handle.release_read();
            max
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn in_order(&self) -> ArraySeqStPerS<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let out = tree_ref.in_order();
            read_handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn pre_order(&self) -> ArraySeqStPerS<T> where T: Clone + Eq {
            let read_handle = self.root.acquire_read();
            let tree_ref = read_handle.borrow();
            let out = tree_ref.pre_order();
            read_handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }
    }

    } // verus!

    // 14. derive impls outside verus!

    impl<T> std::fmt::Debug for BSTAVLMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTAVLMtEphInv").finish()
        }
    }

    impl<T> std::fmt::Display for BSTAVLMtEphInv<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTAVLMtEphInv")
        }
    }

    impl<T: TotalOrder> std::fmt::Debug for BSTAVLMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTAVLMtEph").finish()
        }
    }

    impl<T: TotalOrder> std::fmt::Display for BSTAVLMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTAVLMtEph(size={})", self.size())
        }
    }

    // 13. macros

    #[macro_export]
    macro_rules! BSTAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }
} // mod
