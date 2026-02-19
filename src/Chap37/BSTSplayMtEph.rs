//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral splay-style (simple BST) structure with interior locking for multi-threaded access.

pub mod BSTSplayMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    fn new_node<T: StTInMtT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>) { node.size = 1 + size_link(&node.left) + size_link(&node.right); }

    fn splay<T: StTInMtT + Ord>(root: Box<Node<T>>, target: &T) -> Box<Node<T>> {
        let mut root = root;
        match target.cmp(&root.key) {
            std::cmp::Ordering::Equal => root,
            std::cmp::Ordering::Less => {
                let Some(mut left) = root.left.take() else { return root };
                match target.cmp(&left.key) {
                    std::cmp::Ordering::Equal => {
                        root.left = left.right.take();
                        update(&mut root);
                        left.right = Some(root);
                        update(&mut left);
                        left
                    }
                    std::cmp::Ordering::Less => {
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
                        root.right = right.left.take();
                        update(&mut root);
                        right.left = Some(root);
                        update(&mut right);
                        right
                    }
                    std::cmp::Ordering::Greater => {
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

    fn bst_insert<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) -> bool {
        match link {
            | Some(node) => {
                let inserted = if value < node.key {
                    bst_insert(&mut node.left, value)
                } else if value > node.key {
                    bst_insert(&mut node.right, value)
                } else {
                    false
                };
                if inserted { update(node); }
                inserted
            }
            | None => {
                *link = Some(Box::new(new_node(value)));
                true
            }
        }
    }

    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) -> bool {
        let v = value.clone();
        let inserted = bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(splay(root, &v));
            }
        }
        inserted
    }

    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
        match link {
            | None => None,
            | Some(node) => {
                if target == &node.key {
                    Some(&node.key)
                } else if target < &node.key {
                    find_link(&node.left, target)
                } else {
                    find_link(&node.right, target)
                }
            }
        }
    }

    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match node.left {
                | None => Some(&node.key),
                | Some(_) => min_link(&node.left),
            },
        }
    }

    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T> {
        match link {
            | None => None,
            | Some(node) => match node.right {
                | None => Some(&node.key),
                | Some(_) => max_link(&node.right),
            },
        }
    }

    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    // Parallel traversals

    fn in_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> Vec<T> {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                use crate::Types::Types::Pair;
                let Pair(left_vec, right_vec) = crate::ParaPair!(
                    move || in_order_parallel(&node.left),
                    move || in_order_parallel(&node.right)
                );
                let mut result = left_vec;
                result.push(node.key.clone());
                result.extend(right_vec);
                result
            }
        }
    }

    fn pre_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> Vec<T> {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                use crate::Types::Types::Pair;
                let Pair(left_vec, right_vec) = crate::ParaPair!(
                    move || pre_order_parallel(&node.left),
                    move || pre_order_parallel(&node.right)
                );
                let mut result = vec![node.key.clone()];
                result.extend(left_vec);
                result.extend(right_vec);
                result
            }
        }
    }

    // Parallel construction from sorted slice
    fn build_balanced<T: StTInMtT + Ord>(values: &[T]) -> Link<T> {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        
        use crate::Types::Types::Pair;
        let Pair(left, right) = crate::ParaPair!(
            move || build_balanced(&values[..mid]),
            move || build_balanced(&values[mid + 1..])
        );
        
        let mut node = Box::new(new_node(values[mid].clone()));
        node.left = left;
        node.right = right;
        update(&mut node);
        Some(node)
    }

    // Parallel filter
    fn filter_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, predicate: &Arc<F>) -> Vec<T>
    where
        F: Fn(&T) -> bool + Send + Sync,
    {
        match link {
            | None => Vec::new(),
            | Some(node) => {
                let pred_left = Arc::clone(predicate);
                let pred_right = Arc::clone(predicate);
                
                use crate::Types::Types::Pair;
                let Pair(left_vals, right_vals) = crate::ParaPair!(
                    move || filter_parallel(&node.left, &pred_left),
                    move || filter_parallel(&node.right, &pred_right)
                );
                
                let mut result = left_vals;
                if predicate(&node.key) {
                    result.push(node.key.clone());
                }
                result.extend(right_vals);
                result
            }
        }
    }

    // Parallel reduce
    fn reduce_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, op: &Arc<F>, identity: T) -> T
    where
        F: Fn(T, T) -> T + Send + Sync,
    {
        match link {
            | None => identity,
            | Some(node) => {
                let op_left = Arc::clone(op);
                let op_right = Arc::clone(op);
                let id_left = identity.clone();
                
                use crate::Types::Types::Pair;
                let Pair(left_acc, right_acc) = crate::ParaPair!(
                    move || reduce_parallel(&node.left, &op_left, id_left),
                    move || reduce_parallel(&node.right, &op_right, identity)
                );
                
                let with_key = op(left_acc, node.key.clone());
                op(with_key, right_acc)
            }
        }
    }

    verus! {
        #[verifier::reject_recursive_types(T)]
        #[verifier::external_type_specification]
        struct ExNode<T: StTInMtT + Ord>(Node<T>);

        pub struct SplayLinkWf;

        impl<T: StTInMtT + Ord> RwLockPredicate<Link<T>> for SplayLinkWf {
            open spec fn inv(self, v: Link<T>) -> bool { true }
        }

        #[verifier::external_body]
        fn new_splay_link_lock<T: StTInMtT + Ord>(val: Link<T>) -> (lock: RwLock<Link<T>, SplayLinkWf>) {
            RwLock::new(val, Ghost(SplayLinkWf))
        }
    }

    #[derive(Clone)]
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>, SplayLinkWf>>,
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSplayMtEph").finish()
        }
    }

    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;

    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel construction from sorted slice
        fn from_sorted_slice(values: &[T]) -> Self;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn insert(&self, value: T);
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn find(&self, target: &T)     -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn contains(&self, target: &T) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                 -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self)             -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self)               -> N;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn minimum(&self)              -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn maximum(&self)              -> Option<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel traversal
        fn in_order(&self)             -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel traversal
        fn pre_order(&self)            -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel filter
        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel reduce
        fn reduce<F>(&self, op: F, identity: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync;
    }

    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {
        fn new() -> Self {
            BSTSplayMtEph {
                root: Arc::new(new_splay_link_lock(None)),
            }
        }

        fn insert(&self, value: T) {
            let (mut current, write_handle) = self.root.acquire_write();
            insert_link(&mut current, value);
            write_handle.release_write(current);
        }

        fn find(&self, target: &T) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            result
        }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn size(&self) -> N {
            let handle = self.root.acquire_read();
            let result = size_link(handle.borrow());
            handle.release_read();
            result
        }

        fn is_empty(&self) -> B { self.size() == 0 }

        fn height(&self) -> N {
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }
            let handle = self.root.acquire_read();
            let result = height_rec(handle.borrow());
            handle.release_read();
            result
        }

        fn minimum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = min_link(handle.borrow()).cloned();
            handle.release_read();
            result
        }

        fn maximum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let result = max_link(handle.borrow()).cloned();
            handle.release_read();
            result
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let out = in_order_parallel(handle.borrow());
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let handle = self.root.acquire_read();
            let out = pre_order_parallel(handle.borrow());
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn from_sorted_slice(values: &[T]) -> Self {
            BSTSplayMtEph {
                root: Arc::new(new_splay_link_lock(build_balanced(values))),
            }
        }

        fn filter<F>(&self, predicate: F) -> ArraySeqStPerS<T>
        where
            F: Fn(&T) -> bool + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let predicate = Arc::new(predicate);
            let out = filter_parallel(handle.borrow(), &predicate);
            handle.release_read();
            ArraySeqStPerS::from_vec(out)
        }

        fn reduce<F>(&self, op: F, identity: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let op = Arc::new(op);
            let result = reduce_parallel(handle.borrow(), &op, identity);
            handle.release_read();
            result
        }
    }

    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTSplayMtEphLit {
        () => {
            < $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph<_> as $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph<_> as $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
