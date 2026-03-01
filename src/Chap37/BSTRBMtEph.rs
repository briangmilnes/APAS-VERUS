//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Red-Black balanced binary search tree with interior locking for multi-threaded access.

pub mod BSTRBMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Color {
            Red,
            Black,
        }

        #[verifier::reject_recursive_types(T)]
        #[derive(Clone)]
        struct Node<T: StTInMtT + Ord> {
            key: T,
            color: Color,
            size: N,
            left: Option<Box<Node<T>>>,
            right: Option<Box<Node<T>>>,
        }

        type Link<T> = Option<Box<Node<T>>>;
    }

    fn new_node<T: StTInMtT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            color: Color::Red,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn is_red<T: StTInMtT + Ord>(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Red) }

    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>) { node.size = 1 + size_link(&node.left) + size_link(&node.right); }

    fn rotate_left<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(mut h) = link.take() {
            if let Some(mut x) = h.right.take() {
                h.right = x.left.take();
                update(&mut h);
                x.color = h.color;
                h.color = Color::Red;
                x.left = Some(h);
                if let Some(left) = x.left.as_mut() {
                    update(left);
                }
                update(&mut x);
                *link = Some(x);
            } else {
                *link = Some(h);
            }
        }
    }

    fn rotate_right<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(mut h) = link.take() {
            if let Some(mut x) = h.left.take() {
                h.left = x.right.take();
                update(&mut h);
                x.color = h.color;
                h.color = Color::Red;
                x.right = Some(h);
                if let Some(right) = x.right.as_mut() {
                    update(right);
                }
                update(&mut x);
                *link = Some(x);
            } else {
                *link = Some(h);
            }
        }
    }

    fn flip_colors<T: StTInMtT + Ord>(link: &mut Link<T>) {
        if let Some(node) = link.as_mut() {
            node.color = match node.color {
                | Color::Red => Color::Black,
                | Color::Black => Color::Red,
            };
            if let Some(left) = node.left.as_mut() {
                left.color = match left.color {
                    | Color::Red => Color::Black,
                    | Color::Black => Color::Red,
                };
            }
            if let Some(right) = node.right.as_mut() {
                right.color = match right.color {
                    | Color::Red => Color::Black,
                    | Color::Black => Color::Red,
                };
            }
        }
    }

    fn fix_up<T: StTInMtT + Ord>(link: &mut Link<T>) {
        let rotate_left_needed = match link {
            | Some(node) => is_red(&node.right) && !is_red(&node.left),
            | None => false,
        };
        if rotate_left_needed {
            rotate_left(link);
        }

        let rotate_right_needed = match link {
            | Some(node) => {
                if let Some(left) = node.left.as_ref() {
                    is_red(&node.left) && is_red(&left.left)
                } else {
                    false
                }
            }
            | None => false,
        };
        if rotate_right_needed {
            rotate_right(link);
        }

        let flip_needed = match link {
            | Some(node) => is_red(&node.left) && is_red(&node.right),
            | None => false,
        };
        if flip_needed {
            flip_colors(link);
        }

        if let Some(node) = link.as_mut() {
            update(node);
        }
    }

    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) {
        if let Some(node) = link.as_mut() {
            if value < node.key {
                insert_link(&mut node.left, value);
            } else if value > node.key {
                insert_link(&mut node.right, value);
            } else {
                return;
            }
        } else {
            *link = Some(Box::new(new_node(value)));
            return;
        }
        fix_up(link);
    }

    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
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
        node.color = Color::Black;
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
        /// Uninterpreted well-formedness for RB tree links (Node is outside verus!).
        pub open spec fn link_wf<T: StTInMtT + Ord>(link: Link<T>) -> bool;

        pub struct BSTRBMtEphInv;

        impl<T: StTInMtT + Ord> RwLockPredicate<Link<T>> for BSTRBMtEphInv {
            open spec fn inv(self, v: Link<T>) -> bool {
                link_wf(v)
            }
        }

        #[verifier::external_body] // accept hole
        fn new_rb_link_lock<T: StTInMtT + Ord>(val: Link<T>) -> (lock: RwLock<Link<T>, BSTRBMtEphInv>) {
            RwLock::new(val, Ghost(BSTRBMtEphInv))
        }

        #[verifier::reject_recursive_types(T)]
        #[derive(Clone)]
        pub struct BSTRBMtEph<T: StTInMtT + Ord> {
            root: Arc<RwLock<Link<T>, BSTRBMtEphInv>>,
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBMtEph").finish()
        }
    }

    pub type BSTreeRB<T> = BSTRBMtEph<T>;

    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new()                       -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n) - Parallel construction from sorted slice
        fn from_sorted_slice(values: &[T]) -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn insert(&self, value: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn find(&self, target: &T)     -> Option<T>;
        fn contains(&self, target: &T) -> B;
        fn size(&self)                 -> N;
        fn is_empty(&self)             -> B;
        fn height(&self)               -> N;
        fn minimum(&self)              -> Option<T>;
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

    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {
        fn new() -> Self {
            BSTRBMtEph {
                root: Arc::new(new_rb_link_lock(None)),
            }
        }

        fn insert(&self, value: T) {
            let (mut current, write_handle) = self.root.acquire_write();
            insert_link(&mut current, value);
            if let Some(node) = current.as_mut() {
                node.color = Color::Black;
            }
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
            BSTRBMtEph {
                root: Arc::new(new_rb_link_lock(build_balanced(values))),
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

    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    #[macro_export]
    macro_rules! BSTRBMtEphLit {
        () => { < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> as
                  $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEphTrait<_> >::new() };
        ($($x:expr),* $(,)?) => {{
            let __tree = < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> as
                           $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }
}
