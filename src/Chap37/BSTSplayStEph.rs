//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Splay Tree (standard BST semantics) with public methods.

pub mod BSTSplayStEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    type Link<T> = Option<Box<Node<T>>>;

    pub struct Node<T: StT + Ord> {
        pub key: T,
        pub size: N,
        pub left: Link<T>,
        pub right: Link<T>,
    }

    impl<T: StT + Ord> Clone for Node<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result == *self
        {
            Node {
                key: self.key.clone(),
                size: self.size,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    fn new_node<T: StT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    pub open spec fn spec_size_link<T: StT + Ord>(link: &Link<T>) -> nat {
        match link {
            None => 0,
            Some(node) => node.size as nat,
        }
    }

    pub open spec fn spec_height_link<T: StT + Ord>(link: &Link<T>) -> nat
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

    fn size_link<T: StT + Ord>(link: &Link<T>) -> (result: N)
        ensures result as nat == spec_size_link(link),
    {
        proof { reveal(spec_size_link); }
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn height_link<T: StT + Ord>(link: &Link<T>) -> (result: N)
        requires spec_height_link(link) < usize::MAX as nat,
        ensures result as nat == spec_height_link(link),
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

    fn update<T: StT + Ord>(node: &mut Node<T>)
        ensures
            node.size as nat == 1 + spec_size_link(&old(node).left) + spec_size_link(&old(node).right),
            node.key == old(node).key,
            node.left == old(node).left,
            node.right == old(node).right,
    {
        proof { reveal(spec_size_link); }
        let ls = size_link(&node.left);
        let rs = size_link(&node.right);
        assume(ls as int + rs as int + 1 <= usize::MAX as int);
        node.size = 1 + ls + rs;
    }

    // Bottom-up splay: bring target (or nearest key) toward the root using
    // zig, zig-zig, and zig-zag rotations (Sleator & Tarjan).
    fn splay<T: StT + Ord>(root: Box<Node<T>>, target: &T) -> Box<Node<T>>
        decreases root,
    {
        let mut root = root;
        match target.cmp(&root.key) {
            std::cmp::Ordering::Equal => root,
            std::cmp::Ordering::Less => {
                let Some(mut left) = root.left.take() else { return root };
                match target.cmp(&left.key) {
                    std::cmp::Ordering::Equal => {
                        // Zig
                        root.left = left.right.take();
                        update(&mut root);
                        left.right = Some(root);
                        update(&mut left);
                        left
                    }
                    std::cmp::Ordering::Less => {
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
                    std::cmp::Ordering::Greater => {
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
            std::cmp::Ordering::Greater => {
                let Some(mut right) = root.right.take() else { return root };
                match target.cmp(&right.key) {
                    std::cmp::Ordering::Equal => {
                        // Zag
                        root.right = right.left.take();
                        update(&mut root);
                        right.left = Some(root);
                        update(&mut right);
                        right
                    }
                    std::cmp::Ordering::Greater => {
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
                    std::cmp::Ordering::Less => {
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

    fn bst_insert<T: StT + Ord>(link: &mut Link<T>, value: T) -> (inserted: bool)
        decreases old(link),
    {
        let cur = link.take();
        match cur {
            | None => {
                *link = Some(Box::new(new_node(value)));
                true
            }
            | Some(mut node) => {
                let inserted = if value < node.key {
                    bst_insert(&mut node.left, value)
                } else if value > node.key {
                    bst_insert(&mut node.right, value)
                } else {
                    false
                };
                if inserted { update(&mut node); }
                *link = Some(node);
                inserted
            }
        }
    }

    fn insert_link<T: StT + Ord>(link: &mut Link<T>, value: T) -> (inserted: bool) {
        let v = value.clone();
        let inserted = bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(splay(root, &v));
            }
        }
        inserted
    }

    fn find_link<'a, T: StT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T>
        decreases *link,
    {
        match link {
            | None => None,
            | Some(node) => {
                if *target == node.key {
                    Some(&node.key)
                } else if *target < node.key {
                    find_link(&node.left, target)
                } else {
                    find_link(&node.right, target)
                }
            }
        }
    }

    fn min_link<T: StT + Ord>(link: &Link<T>) -> Option<&T>
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

    fn max_link<T: StT + Ord>(link: &Link<T>) -> Option<&T>
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

    fn in_order_collect<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    pub struct BSTSplayStEph<T: StT + Ord> {
        pub root: Link<T>,
    }

    impl<T: StT + Ord> Clone for BSTSplayStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures true,
        {
            BSTSplayStEph { root: self.root.clone() }
        }
    }

    pub type BSTreeSplay<T> = BSTSplayStEph<T>;

    pub trait BSTSplayStEphTrait<T: StT + Ord> {
        spec fn spec_size(self) -> nat;
        spec fn spec_height(self) -> nat;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self
        where
            Self: Sized;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N
            requires self.spec_height() < usize::MAX as nat;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized, Parallelism Θ(1)
        fn insert(&mut self, value: T);
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized, Parallelism Θ(1)
        fn find(&self, target: &T)     -> Option<&T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized, Parallelism Θ(1)
        fn contains(&self, target: &T) -> B;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized, Parallelism Θ(1)
        fn minimum(&self)              -> Option<&T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized, Parallelism Θ(1)
        fn maximum(&self)              -> Option<&T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {
        open spec fn spec_size(self) -> nat { spec_size_link(&self.root) }
        open spec fn spec_height(self) -> nat { spec_height_link(&self.root) }

        fn new() -> Self { BSTSplayStEph { root: None } }

        fn size(&self) -> N { size_link(&self.root) }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N {
            height_link(&self.root)
        }

        fn insert(&mut self, value: T) { insert_link(&mut self.root, value); }

        fn find(&self, target: &T) -> Option<&T> { find_link(&self.root, target) }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { max_link(&self.root) }

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

    impl<T: StT + Ord> Default for BSTSplayStEph<T> {
        fn default() -> Self { Self::new() }
    }

    }

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("size", &self.size)
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }

    impl<T: StT + Ord + fmt::Debug> fmt::Debug for BSTSplayStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSplayStEph").field("root", &self.root).finish()
        }
    }

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
