//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral splay-style binary search tree with coarse RwLock for multi-threaded access.
//! Layer 1 (verified algorithms on Link/Node) in sections 6/9.
//! Layer 2 (locked wrapper with ghost shadow) in section 11.

//  Table of Contents
//  1. module
//  2. imports
//  4. type definitions
//  6. spec fns
//  9. impls
//  11. top level coarse locking
//  13. macros
//  14. derive impls outside verus!

// 1. module

pub mod BSTSplayMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 2. imports

    // (Arc kept for filter_parallel/reduce_parallel closure sharing.)

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    #[derive(Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        size: N,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    // 6. spec fns

    /// Structural node count for splay tree links.
    pub open spec fn link_spec_size<T: StTInMtT + Ord>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_spec_size(node.left) + link_spec_size(node.right),
        }
    }

    /// Spec-level containment for splay tree links.
    pub open spec fn link_contains<T: StTInMtT + Ord>(link: Link<T>, target: T) -> bool
        decreases link,
    {
        match link {
            None => false,
            Some(node) => node.key == target
                || link_contains(node.left, target)
                || link_contains(node.right, target),
        }
    }

    /// Spec-level height for splay tree links.
    pub open spec fn link_height<T: StTInMtT + Ord>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => {
                let lh = link_height(node.left);
                let rh = link_height(node.right);
                1 + if lh > rh { lh } else { rh }
            }
        }
    }

    // 9. impls

    // Verified splay tree algorithms (Layer 1).

    fn new_node<T: StTInMtT + Ord>(key: T) -> Node<T> {
        Node {
            key,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> N {
        match link.as_ref() {
            None => 0,
            Some(n) => n.size,
        }
    }

    fn update<T: StTInMtT + Ord>(node: &mut Node<T>)
        ensures
            node.left == old(node).left,
            node.right == old(node).right,
            node.key == old(node).key,
    {
        let ls = size_link(&node.left);
        let rs = size_link(&node.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }

    // Bottom-up splay: bring target (or nearest key) toward the root using
    // zig, zig-zig, and zig-zag rotations (Sleator & Tarjan).
    fn splay<T: StTInMtT + Ord>(root: Box<Node<T>>, target: &T) -> (result: Box<Node<T>>)
        ensures link_spec_size(Some(result)) == link_spec_size(Some(root)),
        decreases root,
    {
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

    fn bst_insert<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) -> (inserted: bool)
        ensures link_spec_size(*link) <= link_spec_size(*old(link)) + 1,
        decreases old(link),
    {
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

    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T) -> (inserted: bool)
        ensures link_spec_size(*link) <= link_spec_size(*old(link)) + 1,
    {
        let v = value.clone();
        let inserted = bst_insert(link, value);
        if inserted {
            if let Some(root) = link.take() {
                *link = Some(splay(root, &v));
            }
        }
        inserted
    }

    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> Option<&'a T>
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

    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T>
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

    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> Option<&T>
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

    fn in_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    fn in_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> Vec<T>
        decreases *link,
    {
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

    fn pre_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> Vec<T>
        decreases *link,
    {
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

    fn build_balanced<T: StTInMtT + Ord>(values: &[T]) -> (link: Link<T>)
        ensures link_spec_size(link) <= values@.len(),
        decreases values.len(),
    {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let left_slice = &values[..mid];
        let right_slice = &values[mid + 1..];

        use crate::Types::Types::Pair;
        let f1 = move || -> (l: Link<T>)
            ensures link_spec_size(l) <= left_slice@.len()
        { build_balanced(left_slice) };
        let f2 = move || -> (r: Link<T>)
            ensures link_spec_size(r) <= right_slice@.len()
        { build_balanced(right_slice) };
        let Pair(left, right) = crate::ParaPair!(f1, f2);

        let mut node = Box::new(new_node(values[mid].clone()));
        node.left = left;
        node.right = right;
        update(&mut node);
        Some(node)
    }

    fn filter_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, predicate: &Arc<F>) -> Vec<T>
        where
            F: Fn(&T) -> bool + Send + Sync,
        decreases *link,
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

    fn reduce_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, op: &Arc<F>, identity: T) -> T
        where
            F: Fn(T, T) -> T + Send + Sync,
        decreases *link,
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

    fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> (h: N)
        decreases *link,
    {
        match link {
            | None => 0,
            | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
        }
    }

    /// Exec mirror of link_spec_size for runtime size guards.
    fn compute_link_spec_size<T: StTInMtT + Ord>(link: &Link<T>) -> (n: usize)
        requires link_spec_size(*link) <= usize::MAX,
        ensures n as nat == link_spec_size(*link),
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let l = compute_link_spec_size(&node.left);
                let r = compute_link_spec_size(&node.right);
                1 + l + r
            }
        }
    }

    // 11. top level coarse locking

    /// Lock predicate: link size fits in usize.
    pub struct BSTSplayMtEphInv;

    impl<T: StTInMtT + Ord> RwLockPredicate<Link<T>> for BSTSplayMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            link_spec_size(v) <= usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSplayMtEph<T: StTInMtT + Ord> {
        pub(crate) root: RwLock<Link<T>, BSTSplayMtEphInv>,
        pub(crate) ghost_root: Ghost<Link<T>>,
    }

    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;

    impl<T: StTInMtT + Ord> BSTSplayMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            link_spec_size(self.ghost_root@) <= usize::MAX
        }

        pub closed spec fn spec_ghost_root(self) -> Link<T> {
            self.ghost_root@
        }
    }

    impl<T: StTInMtT + Ord> View for BSTSplayMtEph<T> {
        type V = Link<T>;
        open spec fn view(&self) -> Link<T> { self.spec_ghost_root() }
    }

    pub trait BSTSplayMtEphTrait<T: StTInMtT + Ord>: Sized + View<V = Link<T>> {
        spec fn spec_bstsplaymteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures tree.spec_bstsplaymteph_wf(),
                    tree@ is None;

        fn from_sorted_slice(values: &[T]) -> (tree: Self)
            ensures tree.spec_bstsplaymteph_wf();

        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsplaymteph_wf(),
            ensures self.spec_bstsplaymteph_wf(),
                    match r {
                        Ok(_) => link_spec_size(self@) <= link_spec_size(old(self)@) + 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn contains(&self, target: &T) -> (found: B)
            requires self.spec_bstsplaymteph_wf(),
            ensures found == link_contains(self@, *target);

        fn size(&self) -> (n: N)
            requires self.spec_bstsplaymteph_wf(),
            ensures n as nat == link_spec_size(self@);

        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsplaymteph_wf(),
            ensures b == (self@ is None);

        fn height(&self) -> (h: N)
            requires self.spec_bstsplaymteph_wf(),
            ensures h as nat == link_height(self@);

        fn find(&self, target: &T) -> (found: Option<T>)
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            ensures true;
        fn in_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        fn pre_order(&self) -> (seq: ArraySeqStPerS<T>)
            ensures true;
        fn filter<F>(&self, predicate: F) -> (seq: ArraySeqStPerS<T>)
        where
            F: Fn(&T) -> bool + Send + Sync
            ensures true;
        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync
            ensures true;
    }

    impl<T: StTInMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {
        open spec fn spec_bstsplaymteph_wf(&self) -> bool {
            link_spec_size(self@) <= usize::MAX
        }

        fn new() -> Self {
            BSTSplayMtEph {
                root: RwLock::new(None, Ghost(BSTSplayMtEphInv)),
                ghost_root: Ghost(None),
            }
        }

        fn from_sorted_slice(values: &[T]) -> Self {
            let link = build_balanced(values);
            let ghost ghost_link = link;
            BSTSplayMtEph {
                root: RwLock::new(link, Ghost(BSTSplayMtEphInv)),
                ghost_root: Ghost(ghost_link),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        fn insert(&mut self, value: T) -> (r: Result<(), ()>) {
            let (mut current, write_handle) = self.root.acquire_write();
            proof { assume(self.ghost_root@ == current); }
            let sz = compute_link_spec_size(&current);
            if sz < usize::MAX {
                insert_link(&mut current, value);
                let ghost new_root = current;
                self.ghost_root = Ghost(new_root);
                write_handle.release_write(current);
                Ok(())
            } else {
                write_handle.release_write(current);
                Err(())
            }
        }

        // Reader: assume return value matches ghost.
        fn contains(&self, target: &T) -> (found: B) {
            let handle = self.root.acquire_read();
            let found = find_link(handle.borrow(), target).is_some();
            proof { assume(found == link_contains(self@, *target)); }
            handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        fn size(&self) -> (n: N) {
            let handle = self.root.acquire_read();
            let n = size_link(handle.borrow());
            proof { assume(n as nat == link_spec_size(self@)); }
            handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        fn is_empty(&self) -> (b: B) {
            let handle = self.root.acquire_read();
            let b = handle.borrow().is_none();
            proof { assume(b == (self@ is None)); }
            handle.release_read();
            b
        }

        // Reader: assume return value matches ghost.
        fn height(&self) -> (h: N) {
            let handle = self.root.acquire_read();
            let h = height_rec(handle.borrow());
            proof { assume(h as nat == link_height(self@)); }
            handle.release_read();
            h
        }

        fn find(&self, target: &T) -> Option<T> {
            let handle = self.root.acquire_read();
            let found = find_link(handle.borrow(), target).cloned();
            handle.release_read();
            found
        }

        fn minimum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let min = min_link(handle.borrow()).cloned();
            handle.release_read();
            min
        }

        fn maximum(&self) -> Option<T> {
            let handle = self.root.acquire_read();
            let max = max_link(handle.borrow()).cloned();
            handle.release_read();
            max
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

        fn reduce<F>(&self, op: F, identity: T) -> (accumulated: T)
        where
            F: Fn(T, T) -> T + Send + Sync,
        {
            let handle = self.root.acquire_read();
            let op = Arc::new(op);
            let accumulated = reduce_parallel(handle.borrow(), &op, identity);
            handle.release_read();
            accumulated
        }
    }

    impl<T: StTInMtT + Ord> Default for BSTSplayMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    } // verus!

    // 13. macros

    #[macro_export]
    macro_rules! BSTSplayMtEphLit {
        () => {
            < $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }

    // 14. derive impls outside verus!

    impl<T: StTInMtT + Ord> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    impl std::fmt::Debug for BSTSplayMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSplayMtEphInv").finish()
        }
    }

    impl std::fmt::Display for BSTSplayMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSplayMtEphInv")
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSplayMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTSplayMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSplayMtEph(size={})", self.size())
        }
    }
}
