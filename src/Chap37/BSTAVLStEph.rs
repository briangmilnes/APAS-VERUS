//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Ephemeral AVL-balanced binary search tree.
//! Verusified: functional-style AVL with BST invariant + balance specs.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

#[allow(non_shorthand_field_patterns)]
pub mod BSTAVLStEph {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! 
{


    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;
    use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTSpecFns;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct BSTAVLStEph<T> {
        pub root: BalBinTree<T>,
    }

    //		Section 5. view impls


    impl<T> View for BSTAVLStEph<T> {
        type V = BalBinTree<T>;
        open spec fn view(&self) -> BalBinTree<T> { self.root }
    }

    //		Section 6. spec fns


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

    //		Section 7. proof fns/broadcast groups


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

    //		Section 8. traits


    pub trait BSTAVLStEphTrait<T: TotalOrder>: Sized + View<V = BalBinTree<T>> {
        spec fn spec_root(self) -> BalBinTree<T>;
        spec fn spec_bstavlsteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (tree: Self)
            ensures
                tree.spec_bstavlsteph_wf(),
                tree_is_avl::<T>(tree.spec_root()),
                forall|x: T| !tree.spec_root().tree_contains(x);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn size(&self) -> (n: usize)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().spec_size() <= usize::MAX,
            ensures n == self.spec_root().spec_size();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (b: bool)
            requires self.spec_bstavlsteph_wf(),
            ensures b == (self.spec_root().spec_size() == 0);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn height(&self) -> (h: usize)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().spec_height() <= usize::MAX,
            ensures h == self.spec_root().spec_height();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn contains(&self, target: &T) -> (found: bool)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures found == self.spec_root().tree_contains(*target);
        /// - Alg Analysis: APAS (Ch37 Alg 37.4): Work O(h(T)), Span O(h(T))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(h(T)), Span O(h(T))
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires
                self.spec_bstavlsteph_wf(),
                self.spec_root().tree_is_bst(),
            ensures
                found.is_some() == self.spec_root().tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target;
    }

    /// Exec AVL BST operations on BalBinTree nodes.
    pub trait BSTAVLNodeFns<T: TotalOrder>: Sized + BSTSpecFns<T> + BalBinTreeTrait<T> {
        // Spec accessors for abstract ensures.
        spec fn avl_balanced_spec(self) -> bool;
        spec fn tree_is_avl_spec(self) -> bool;
        spec fn spec_left(self) -> Self;
        spec fn spec_right(self) -> Self;

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        fn rotate_right(self) -> (rotated: Self)
            requires
                self.tree_is_bst(),
                !self.spec_is_leaf(),
            ensures
                rotated.tree_is_bst(),
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
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        fn rotate_left(self) -> (rotated: Self)
            requires
                self.tree_is_bst(),
                !self.spec_is_leaf(),
            ensures
                rotated.tree_is_bst(),
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
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
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
                balanced.spec_height() <= self.spec_height(),
                balanced.spec_height() + 1 >= self.spec_height(),
                forall|x: T| (#[trigger] balanced.tree_contains(x)) == self.tree_contains(x),
                {
                    let lh = self.spec_left().spec_height() as int;
                    let rh = self.spec_right().spec_height() as int;
                    (-1 <= lh - rh && lh - rh <= 1) ==> balanced.spec_height() == self.spec_height()
                },
            ;
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n), Span O(lg n)
        fn insert_node(self, value: T) -> (inserted: Self)
            requires
                self.tree_is_avl_spec(),
                self.spec_height() <= usize::MAX - 1,
            ensures
                inserted.tree_is_avl_spec(),
                inserted.tree_contains(value),
                inserted.spec_height() <= self.spec_height() + 1,
                inserted.spec_height() >= self.spec_height(),
                forall|x: T| (#[trigger] inserted.tree_contains(x)) <==>
                    (self.tree_contains(x) || x == value),
            ;
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n), Span O(lg n)
        fn contains_node(&self, target: &T) -> (found: bool)
            requires (*self).tree_is_bst(),
            ensures found == (*self).tree_contains(*target),
            ;
        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n), Span O(lg n)
        fn find_node(&self, target: &T) -> (found: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                found.is_some() == (*self).tree_contains(*target),
                found.is_some() ==> *found.unwrap() == *target,
            ;
        fn min_node(&self) -> (min: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> min.is_none(),
                (*self).spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> (*self).tree_contains(*min.unwrap()),
            ;
        fn max_node(&self) -> (max: Option<&T>)
            requires (*self).tree_is_bst(),
            ensures
                (*self).spec_size() == 0 ==> max.is_none(),
                (*self).spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> (*self).tree_contains(*max.unwrap()),
            ;
    }

    //		Section 9. impls


    impl<T: TotalOrder> BSTAVLNodeFns<T> for BalBinTree<T> {

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
                            left: lr,
                            value: y_val,
                            right: r,
                        }));

                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: ll,
                            value: x_val,
                            right: right_sub,
                        }));

                        // Veracity: NEEDED proof block
                        proof {
                            lemma_bst_deep::<T>(tree_ghost);

                            // Veracity: NEEDED assert
                            assert forall|z: T| old_lr.tree_contains(z) implies
                                #[trigger] T::le(z, y_val) && z != y_val
                            by {
                                // Veracity: NEEDED assert
                                assert(old_left.tree_contains(z));
                            };

                            // Veracity: NEEDED assert
                            assert(old_left.tree_contains(x_val));

                            // Veracity: NEEDED assert
                            assert(right_sub.tree_is_bst());

                            // Veracity: NEEDED assert
                            assert forall|z: T| right_sub.tree_contains(z) implies
                                #[trigger] T::le(x_val, z) && z != x_val
                            by {
                                if old_lr.tree_contains(z) {
                                } else if z == y_val {
                                } else if old_r.tree_contains(z) {
                                    T::transitive(x_val, y_val, z);
                                    if z == x_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                }
                            };

                            // Veracity: NEEDED assert
                            assert forall|z: T| r.tree_contains(z) ==
                                tree_ghost.tree_contains(z)
                            by {
                                // Veracity: NEEDED assert
                                assert(right_sub.tree_contains(z) ==
                                    (y_val == z
                                    || old_lr.tree_contains(z)
                                    || old_r.tree_contains(z)));
                                // Veracity: NEEDED assert
                                assert(old_left.tree_contains(z) ==
                                    (x_val == z
                                    || old_ll.tree_contains(z)
                                    || old_lr.tree_contains(z)));
                            };

                            // Height: r = Node(x, ll, right_sub) where
                            // right_sub = Node(y, lr, old_r)

                            // AVL balance: unfold avl_balanced on the constructed nodes
                            // Veracity: NEEDED assert
                            assert(avl_balanced(right_sub) <==>
                                (avl_balanced(old_lr) && avl_balanced(old_r) && {
                                    let lh = old_lr.spec_height() as int;
                                    let rh = old_r.spec_height() as int;
                                    -1 <= lh - rh && lh - rh <= 1
                                }));
                            // Veracity: NEEDED assert
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
                            left: l,
                            value: x_val,
                            right: rl,
                        }));

                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left_sub,
                            value: y_val,
                            right: rr,
                        }));

                        // Veracity: NEEDED proof block
                        proof {
                            lemma_bst_deep::<T>(tree_ghost);

                            // Veracity: NEEDED assert
                            assert forall|z: T| old_rl.tree_contains(z) implies
                                #[trigger] T::le(x_val, z) && z != x_val
                            by {
                                // Veracity: NEEDED assert
                                assert(old_right.tree_contains(z));
                            };

                            // Veracity: NEEDED assert
                            assert(old_right.tree_contains(y_val));

                            // Veracity: NEEDED assert
                            assert(left_sub.tree_is_bst());

                            // Veracity: NEEDED assert
                            assert forall|z: T| left_sub.tree_contains(z) implies
                                #[trigger] T::le(z, y_val) && z != y_val
                            by {
                                if old_l.tree_contains(z) {
                                    T::transitive(z, x_val, y_val);
                                    if z == y_val {
                                        T::antisymmetric(x_val, y_val);
                                    }
                                } else if z == x_val {
                                } else if old_rl.tree_contains(z) {
                                }
                            };

                            // Veracity: NEEDED assert
                            assert forall|z: T| r.tree_contains(z) ==
                                tree_ghost.tree_contains(z)
                            by {
                                // Veracity: NEEDED assert
                                assert(left_sub.tree_contains(z) ==
                                    (x_val == z
                                    || old_l.tree_contains(z)
                                    || old_rl.tree_contains(z)));
                                // Veracity: NEEDED assert
                                assert(old_right.tree_contains(z) ==
                                    (y_val == z
                                    || old_rl.tree_contains(z)
                                    || old_rr.tree_contains(z)));
                            };

                            // Height: r = Node(y, left_sub, rr) where
                            // left_sub = Node(x, old_l, rl)

                            // AVL balance: unfold avl_balanced on the constructed nodes
                            // Veracity: NEEDED assert
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
    /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-number of rotations.
    fn rebalance(self) -> (balanced: Self)
    {
        let ghost tree_ghost = self;
        match self {
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
                        let new_left = left.rotate_left();
                        let intermediate = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: v,
                            right: right,
                        }));
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall|x: T| intermediate.tree_contains(x) ==
                                tree_ghost.tree_contains(x)
                            by {
                                // Veracity: NEEDED assert
                                assert(intermediate.tree_contains(x) ==
                                    (v == x || new_left.tree_contains(x) || right.tree_contains(x)));
                            };
                        }
                        let result = intermediate.rotate_right();
                        // Veracity: NEEDED proof block
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


                                                    // lr.h = left_rh = ll_h + 1 = r_h + 1

                                                    // Unfold lr height: 1 + max(lrl_h, lrr_h) = r_h + 1

                                                    // max(lrl_h, lrr_h) = r_h; both <= r_h

                                                    // avl(lr): |lrl_h - lrr_h| <= 1
                                                    // Veracity: NEEDED assert
                                                    assert(avl_balanced(tg_l.right));


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
                        // Veracity: NEEDED proof block
                        proof {
                            match tree_ghost {
                                BalBinTree::Node(tg) => {
                                    match tg.left {
                                        BalBinTree::Node(tg_l) => {
                                            let ll_h = tg_l.left.spec_height();
                                            let lr_h = tg_l.right.spec_height();
                                            let r_h = tg.right.spec_height();


                                            let new_rh: nat = 1 + if lr_h >= r_h { lr_h } else { r_h };
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
                        let new_right = right.rotate_right();
                        let intermediate = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: v,
                            right: new_right,
                        }));
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall|x: T| intermediate.tree_contains(x) ==
                                tree_ghost.tree_contains(x)
                            by {
                                // Veracity: NEEDED assert
                                assert(intermediate.tree_contains(x) ==
                                    (v == x || left.tree_contains(x) || new_right.tree_contains(x)));
                            };
                        }
                        let result = intermediate.rotate_left();
                        // Veracity: NEEDED proof block
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


                                                    // rl.h = right_lh = rr_h + 1 = l_h + 1

                                                    // Unfold rl height: 1 + max(rll_h, rlr_h) = l_h + 1

                                                    // max(rll_h, rlr_h) = l_h; both <= l_h

                                                    // avl(rl): |rll_h - rlr_h| <= 1
                                                    // Veracity: NEEDED assert
                                                    assert(avl_balanced(tg_r.left));


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
                        // Veracity: NEEDED proof block
                        proof {
                            match tree_ghost {
                                BalBinTree::Node(tg) => {
                                    match tg.right {
                                        BalBinTree::Node(tg_r) => {
                                            let rl_h = tg_r.left.spec_height();
                                            let rr_h = tg_r.right.spec_height();
                                            let l_h = tg.left.spec_height();


                                            let new_lh: nat = 1 + if l_h >= rl_h { l_h } else { rl_h };
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

    fn insert_node(self, value: T) -> (inserted: Self)
        decreases self.spec_size(),
    {
        let ghost node = self;
        match self {
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
                        let new_left = left.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: new_left,
                            value: node_val,
                            right: right,
                        }));
                        // Veracity: NEEDED proof block
                        proof {

                            // Veracity: NEEDED assert
                            assert forall|x: T| new_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {
                                if old_left.tree_contains(x) {
                                } else {
                                }
                            };

                            // Veracity: NEEDED assert
                            assert forall|x: T| old_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {};

                            // Veracity: NEEDED assert
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                // Veracity: NEEDED assert
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || new_left.tree_contains(x)
                                    || old_right.tree_contains(x)));
                            };

                            lemma_max_plus_one(old_left.spec_height(), old_right.spec_height());


                        }
                        r.rebalance()
                    }
                    core::cmp::Ordering::Greater => {
                        let new_right = right.insert_node(value);
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: new_right,
                        }));
                        // Veracity: NEEDED proof block
                        proof {

                            // Veracity: NEEDED assert
                            assert forall|x: T| old_left.tree_contains(x) implies
                                #[trigger] T::le(x, node_val) && x != node_val
                            by {};

                            // Veracity: NEEDED assert
                            assert forall|x: T| new_right.tree_contains(x) implies
                                #[trigger] T::le(node_val, x) && x != node_val
                            by {
                                if old_right.tree_contains(x) {
                                } else {
                                }
                            };

                            // Veracity: NEEDED assert
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                                // Veracity: NEEDED assert
                                assert(r.tree_contains(x) ==
                                    (node_val == x
                                    || old_left.tree_contains(x)
                                    || new_right.tree_contains(x)));
                            };

                            lemma_max_plus_one(old_left.spec_height(), old_right.spec_height());


                        }
                        r.rebalance()
                    }
                    core::cmp::Ordering::Equal => {
                        let r = BalBinTree::Node(Box::new(BalBinNode {
                            left: left,
                            value: node_val,
                            right: right,
                        }));
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall|x: T| r.tree_contains(x) ==
                                (node.tree_contains(x) || x == value)
                            by {
                            };
                        }
                        // Veracity: NEEDED proof block
                        proof {
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
                        // Veracity: NEEDED proof block
                        proof {
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.contains_node(target);
                        // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED proof block
                        proof {
                            if inner.right.tree_contains(*target) {
                                T::antisymmetric(*target, inner.value);
                            }
                        }
                        r
                    }
                    core::cmp::Ordering::Greater => {
                        let r = inner.right.find_node(target);
                        // Veracity: NEEDED proof block
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

    fn min_node(&self) -> (min: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.left.is_leaf() {
                    Some(&inner.value)
                } else {
                    inner.left.min_node()
                }
            }
        }
    }

    fn max_node(&self) -> (max: Option<&T>)
        decreases self.spec_size(),
    {
        match self {
            BalBinTree::Leaf => None,
            BalBinTree::Node(inner) => {
                if inner.right.is_leaf() {
                    Some(&inner.value)
                } else {
                    inner.right.max_node()
                }
            }
        }
    }

    } // impl BSTAVLNodeFns

    impl<T: TotalOrder> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {
        open spec fn spec_root(self) -> BalBinTree<T> { self.root }
        open spec fn spec_bstavlsteph_wf(&self) -> bool { tree_is_avl::<T>(self.spec_root()) }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS.
        fn new() -> (tree: Self) {
            BSTAVLStEph { root: BalBinTree::Leaf }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to BalBinTree::size.
        fn size(&self) -> (n: usize) {
            self.root.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — leaf check.
        fn is_empty(&self) -> (b: bool) {
            self.root.is_leaf()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to BalBinTree::height.
        fn height(&self) -> (h: usize) {
            self.root.height()
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — agrees with APAS; AVL guarantees h = O(lg n).
        fn insert(self, value: T) -> (inserted: Self) {
            BSTAVLStEph { root: self.root.insert_node(value) }
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — agrees with APAS; AVL guarantees h = O(lg n).
        fn contains(&self, target: &T) -> (found: bool) {
            self.root.contains_node(target)
        }

        /// - Alg Analysis: APAS (Ch37 CS 38.11): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — agrees with APAS; AVL guarantees h = O(lg n).
        fn find(&self, target: &T) -> (found: Option<&T>) {
            self.root.find_node(target)
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

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
