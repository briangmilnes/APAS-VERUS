//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral Splay Tree (standard BST semantics) with public methods.

//  Table of Contents
//	1. module
//	2. imports
//	4. type definitions
//	6. spec fns
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod BSTSplayStEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    //		2. imports

    use crate::vstdplus::total_order::total_order::TotalOrder;

    //		4. type definitions

    type Link<T> = Option<Box<Node<T>>>;

    pub struct Node<T: TotalOrder + Clone> {
        pub key: T,
        pub size: N,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    pub struct BSTSplayStEph<T: TotalOrder + Clone> {
        pub root: Link<T>,
    }

    pub type BSTreeSplay<T> = BSTSplayStEph<T>;


    //		6. spec fns

    pub open spec fn spec_size_link<T: TotalOrder + Clone>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_height_link<T: TotalOrder + Clone>(link: &Link<T>) -> nat
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let lh = spec_height_link(&node.left);
                let rh = spec_height_link(&node.right);
                1 + if lh >= rh { lh } else { rh }
            }
        }
    }

    /// Recursive membership predicate for a splay tree link.
    pub open spec fn spec_contains_link<T: TotalOrder + Clone>(link: &Link<T>, value: T) -> bool
        decreases *link,
    {
        match link {
            None => false,
            Some(node) =>
                node.key == value
                || spec_contains_link(&node.left, value)
                || spec_contains_link(&node.right, value),
        }
    }


    //		8. traits

    pub trait BSTSplayStEphTrait<T: TotalOrder + Clone> {
        spec fn spec_size(self) -> nat;
        spec fn spec_height(self) -> nat;
        spec fn spec_contains(self, value: T) -> bool;
        spec fn spec_bstsplaysteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
        where
            Self: Sized,
            ensures
                tree.spec_bstsplaysteph_wf(),
                tree.spec_size() == 0,
                forall|x: T| !tree.spec_contains(x);
        fn size(&self) -> (n: N)
            requires self.spec_bstsplaysteph_wf(),
            ensures n as nat == self.spec_size();
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsplaysteph_wf(),
            ensures b == (self.spec_size() == 0);
        fn height(&self) -> (h: N)
            requires
                self.spec_bstsplaysteph_wf(),
                self.spec_height() < usize::MAX as nat,
            ensures h as nat == self.spec_height();
        fn insert(&mut self, value: T)
            requires old(self).spec_bstsplaysteph_wf(),
            ensures
                self.spec_bstsplaysteph_wf();
        fn find(&self, target: &T) -> (found: Option<&T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures found.is_some() ==> *found.unwrap() == *target;
        fn contains(&self, target: &T) -> (found: B)
            requires self.spec_bstsplaysteph_wf(),
            ensures true;
        fn minimum(&self) -> (min: Option<&T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures
                self.spec_size() > 0 ==> min.is_some(),
                min.is_some() ==> self.spec_contains(*min.unwrap());
        fn maximum(&self) -> (max: Option<&T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures
                self.spec_size() > 0 ==> max.is_some(),
                max.is_some() ==> self.spec_contains(*max.unwrap());
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures true;
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsplaysteph_wf(),
            ensures true;
    }


    //		9. impls

    fn new_node<T: TotalOrder + Clone>(key: T) -> (node: Node<T>)
        ensures
            node.key == key,
            node.size == 1,
            node.left is None,
            node.right is None,
    {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn size_link<T: TotalOrder + Clone>(link: &Link<T>) -> (size: N)
        ensures size as nat == spec_size_link(link),
    {
        proof { reveal(spec_size_link); }
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn height_link<T: TotalOrder + Clone>(link: &Link<T>) -> (height: N)
        requires spec_height_link(link) < usize::MAX as nat,
        ensures height as nat == spec_height_link(link),
        decreases *link,
    {
        proof { reveal_with_fuel(spec_height_link, 2); }
        match link {
            | None => 0,
            | Some(node) => {
                let lh = height_link(&node.left);
                let rh = height_link(&node.right);
                let m = if lh >= rh { lh } else { rh };
                1 + m
            }
        }
    }

    fn update<T: TotalOrder + Clone>(node: &mut Node<T>)
        ensures
            node.key == old(node).key,
            node.left == old(node).left,
            node.right == old(node).right,
    {
        proof { reveal(spec_size_link); }
        let ls = size_link(&node.left);
        let rs = size_link(&node.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }

    // Bottom-up splay: bring target (or nearest key) toward the root using
    // zig, zig-zig, and zig-zag rotations (Sleator & Tarjan).
    fn splay<T: TotalOrder + Clone>(root: Box<Node<T>>, target: &T) -> Box<Node<T>>
        decreases root,
    {
        let mut root = root;
        match TotalOrder::cmp(target,&root.key) {
            core::cmp::Ordering::Equal => root,
            core::cmp::Ordering::Less => {
                let Some(mut left) = root.left.take() else { return root };
                match TotalOrder::cmp(target,&left.key) {
                    core::cmp::Ordering::Equal => {
                        // Zig
                        root.left = left.right.take();
                        update(&mut root);
                        left.right = Some(root);
                        update(&mut left);
                        left
                    }
                    core::cmp::Ordering::Less => {
                        // Zig-zig: recurse into left.left, then two right rotations
                        if let Some(ll) = left.left.take() {
                            left.left = Some(splay(ll, target));
                        }
                        root.left = left.right.take();
                        update(&mut root);
                        left.right = Some(root);
                        update(&mut left);
                        if let Some(mut ll) = left.left.take() {
                            left.left = ll.right.take();
                            update(&mut left);
                            ll.right = Some(left);
                            update(&mut ll);
                            ll
                        } else {
                            left
                        }
                    }
                    core::cmp::Ordering::Greater => {
                        // Zig-zag: recurse into left.right, left-rotate left child, right-rotate root
                        if let Some(lr) = left.right.take() {
                            left.right = Some(splay(lr, target));
                        }
                        if left.right.is_some() {
                            let mut lr = left.right.take().unwrap();
                            left.right = lr.left.take();
                            update(&mut left);
                            lr.left = Some(left);
                            update(&mut lr);
                            root.left = lr.right.take();
                            update(&mut root);
                            lr.right = Some(root);
                            update(&mut lr);
                            lr
                        } else {
                            root.left = left.right.take();
                            update(&mut root);
                            left.right = Some(root);
                            update(&mut left);
                            left
                        }
                    }
                }
            }
            core::cmp::Ordering::Greater => {
                let Some(mut right) = root.right.take() else { return root };
                match TotalOrder::cmp(target,&right.key) {
                    core::cmp::Ordering::Equal => {
                        // Zag
                        root.right = right.left.take();
                        update(&mut root);
                        right.left = Some(root);
                        update(&mut right);
                        right
                    }
                    core::cmp::Ordering::Greater => {
                        // Zag-zag: recurse into right.right, then two left rotations
                        if let Some(rr) = right.right.take() {
                            right.right = Some(splay(rr, target));
                        }
                        root.right = right.left.take();
                        update(&mut root);
                        right.left = Some(root);
                        update(&mut right);
                        if let Some(mut rr) = right.right.take() {
                            right.right = rr.left.take();
                            update(&mut right);
                            rr.left = Some(right);
                            update(&mut rr);
                            rr
                        } else {
                            right
                        }
                    }
                    core::cmp::Ordering::Less => {
                        // Zag-zig: recurse into right.left, right-rotate right child, left-rotate root
                        if let Some(rl) = right.left.take() {
                            right.left = Some(splay(rl, target));
                        }
                        if right.left.is_some() {
                            let mut rl = right.left.take().unwrap();
                            right.left = rl.right.take();
                            update(&mut right);
                            rl.right = Some(right);
                            update(&mut rl);
                            root.right = rl.left.take();
                            update(&mut root);
                            rl.left = Some(root);
                            update(&mut rl);
                            rl
                        } else {
                            root.right = right.left.take();
                            update(&mut root);
                            right.left = Some(root);
                            update(&mut right);
                            right
                        }
                    }
                }
            }
        }
    }

    fn bst_insert<T: TotalOrder + Clone>(link: &mut Link<T>, value: T) -> (inserted: bool)
        decreases old(link),
    {
        let cur = link.take();
        match cur {
            | None => {
                *link = Some(Box::new(new_node(value)));
                true
            }
            | Some(mut node) => {
                let inserted = match TotalOrder::cmp(&value, &node.key) {
                    core::cmp::Ordering::Less => bst_insert(&mut node.left, value),
                    core::cmp::Ordering::Greater => bst_insert(&mut node.right, value),
                    core::cmp::Ordering::Equal => false,
                };
                if inserted { update(&mut node); }
                *link = Some(node);
                inserted
            }
        }
    }

    fn insert_link<T: TotalOrder + Clone>(link: &mut Link<T>, value: T) -> (inserted: bool) {
        let v = value.clone();
        let inserted = bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(splay(root, &v));
            }
        }
        inserted
    }

    fn find_link<'a, T: TotalOrder + Clone>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
        ensures
            found.is_some() ==> *found.unwrap() == *target,
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                match TotalOrder::cmp(target, &node.key) {
                    core::cmp::Ordering::Equal => Some(&node.key),
                    core::cmp::Ordering::Less => find_link(&node.left, target),
                    core::cmp::Ordering::Greater => find_link(&node.right, target),
                }
            }
        }
    }

    fn min_link<T: TotalOrder + Clone>(link: &Link<T>) -> (min: Option<&T>)
        ensures
            link.is_some() ==> min.is_some(),
            min.is_some() ==> spec_contains_link(link, *min.unwrap()),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => Some(&node.key),
                | Some(_) => min_link(&node.left),
            },
        }
    }

    fn max_link<T: TotalOrder + Clone>(link: &Link<T>) -> (max: Option<&T>)
        ensures
            link.is_some() ==> max.is_some(),
            max.is_some() ==> spec_contains_link(link, *max.unwrap()),
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => Some(&node.key),
                | Some(_) => max_link(&node.right),
            },
        }
    }

    fn in_order_collect<T: TotalOrder + Clone>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: TotalOrder + Clone>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    impl<T: TotalOrder + Clone> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {
        open spec fn spec_size(self) -> nat { spec_size_link(&self.root) }
        open spec fn spec_height(self) -> nat { spec_height_link(&self.root) }
        open spec fn spec_contains(self, value: T) -> bool { spec_contains_link(&self.root, value) }
        open spec fn spec_bstsplaysteph_wf(&self) -> bool { true }

        fn new() -> (tree: Self) { BSTSplayStEph { root: None } }

        fn size(&self) -> (n: N) { size_link(&self.root) }

        fn is_empty(&self) -> (b: B) { self.size() == 0 }

        fn height(&self) -> (h: N) {
            height_link(&self.root)
        }

        fn insert(&mut self, value: T) { insert_link(&mut self.root, value); }

        fn find(&self, target: &T) -> (found: Option<&T>) { find_link(&self.root, target) }

        fn contains(&self, target: &T) -> (found: B) { self.find(target).is_some() }

        fn minimum(&self) -> (min: Option<&T>) { min_link(&self.root) }

        fn maximum(&self) -> (max: Option<&T>) { max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            pre_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    impl<T: TotalOrder + Clone> Default for BSTSplayStEph<T> {
        fn default() -> Self { Self::new() }
    }


    //		11. derive impls in verus!

    impl<T: TotalOrder + Clone> Clone for Node<T> {
        #[verifier::external_body]
        fn clone(&self) -> (copy: Self)
            ensures copy == *self
        {
            Node {
                key: self.key.clone(),
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    impl<T: TotalOrder + Clone> Clone for BSTSplayStEph<T> {
        fn clone(&self) -> (copy: Self)
            ensures true,
        {
            BSTSplayStEph { root: self.root.clone() }
        }
    }

    }


    //		13. derive impls outside verus!

    impl<T: TotalOrder + Clone + fmt::Debug> fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: TotalOrder + Clone + fmt::Debug> fmt::Debug for BSTSplayStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSplayStEph").field("root", &self.root).finish()
        }
    }

    impl<T: TotalOrder + Clone + fmt::Display> fmt::Display for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    impl<T: TotalOrder + Clone> fmt::Display for BSTSplayStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSplayStEph(size={})", self.size())
        }
    }


    //		12. macros

    #[macro_export]
    macro_rules! BSTSplayStEphLit {
        () => {
            < $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEph<_> as $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEph<_> as $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
