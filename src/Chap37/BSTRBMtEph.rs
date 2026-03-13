//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Ephemeral Red-Black balanced binary search tree with coarse RwLock for multi-threaded access.
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

pub mod BSTRBMtEph {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 2. imports

    use crate::vstdplus::accept::accept;

    // (Arc kept for filter_parallel/reduce_parallel closure sharing.)

    // 4. type definitions

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

    // 6. spec fns

    /// Structural node count for RB tree links.
    pub open spec fn link_spec_size<T: StTInMtT + Ord>(link: Link<T>) -> nat
        decreases link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + link_spec_size(node.left) + link_spec_size(node.right),
        }
    }

    /// Spec-level containment for RB tree links.
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

    /// Spec-level height for RB tree links.
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

    // Verified RB tree algorithms (Layer 1).

    fn new_node<T: StTInMtT + Ord>(key: T) -> (node: Node<T>)

        ensures
            node.key == key,
            node.size == 1,
            node.left is None,
            node.right is None,
    {
        Node {
            key,
            color: Color::Red,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn is_red<T: StTInMtT + Ord>(link: &Link<T>) -> (red: bool)

        ensures
            (link is None) ==> !red,
    {
        match link {
            Some(node) => node.color == Color::Red,
            None => false,
        }
    }

    fn size_link<T: StTInMtT + Ord>(link: &Link<T>) -> (size: N)

        ensures
            (link is None) ==> size == 0,
    {
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
            node.color == old(node).color,
    {
        let ls = size_link(&node.left);
        let rs = size_link(&node.right);
        if ls < usize::MAX && rs <= usize::MAX - 1 - ls {
            node.size = 1 + ls + rs;
        }
    }

    fn rotate_left<T: StTInMtT + Ord>(link: &mut Link<T>)

        ensures link_spec_size(*link) == link_spec_size(*old(link)),
    {
        let ghost old_link = *link;
        if let Some(mut h) = link.take() {
            let ghost old_h_left = h.left;
            if let Some(mut x) = h.right.take() {
                let ghost old_x_left = x.left;
                let ghost old_x_right = x.right;
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

    fn rotate_right<T: StTInMtT + Ord>(link: &mut Link<T>)

        ensures link_spec_size(*link) == link_spec_size(*old(link)),
    {
        let ghost old_link = *link;
        if let Some(mut h) = link.take() {
            let ghost old_h_right = h.right;
            if let Some(mut x) = h.left.take() {
                let ghost old_x_left = x.left;
                let ghost old_x_right = x.right;
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

    fn flip_colors<T: StTInMtT + Ord>(link: &mut Link<T>)

        ensures link_spec_size(*link) == link_spec_size(*old(link)),
    {
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

    fn fix_up<T: StTInMtT + Ord>(link: &mut Link<T>)

        ensures link_spec_size(*link) == link_spec_size(*old(link)),
    {
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

    fn insert_link<T: StTInMtT + Ord>(link: &mut Link<T>, value: T)

        ensures link_spec_size(*link) <= link_spec_size(*old(link)) + 1,
        decreases old(link),
    {
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

    fn find_link<'a, T: StTInMtT + Ord>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)

        ensures
            (link is None) ==> found is None,
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

    fn min_link<T: StTInMtT + Ord>(link: &Link<T>) -> (min: Option<&T>)

        ensures
            (link is None) ==> min is None,
            (link is Some) ==> min is Some,
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

    fn max_link<T: StTInMtT + Ord>(link: &Link<T>) -> (max: Option<&T>)

        ensures
            (link is None) ==> max is None,
            (link is Some) ==> max is Some,
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

        ensures true,
        decreases *link,
    {
        if let Some(node) = link {
            in_order_collect(&node.left, out);
            out.push(node.key.clone());
            in_order_collect(&node.right, out);
        }
    }

    fn pre_order_collect<T: StTInMtT + Ord>(link: &Link<T>, out: &mut Vec<T>)

        ensures true,
        decreases *link,
    {
        if let Some(node) = link {
            out.push(node.key.clone());
            pre_order_collect(&node.left, out);
            pre_order_collect(&node.right, out);
        }
    }

    fn in_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> (result: Vec<T>)

        ensures true,
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

    fn pre_order_parallel<T: StTInMtT + Ord>(link: &Link<T>) -> (result: Vec<T>)

        ensures true,
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
        node.color = Color::Black;
        update(&mut node);
        Some(node)
    }

    fn filter_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, predicate: &Arc<F>) -> (result: Vec<T>)
        where
            F: Fn(&T) -> bool + Send + Sync,

        ensures true,
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

    fn reduce_parallel<T: StTInMtT + Ord, F>(link: &Link<T>, op: &Arc<F>, identity: T) -> (result: T)
        where
            F: Fn(T, T) -> T + Send + Sync,

        ensures true,
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

        ensures
            (link is None) ==> h == 0,
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
    pub struct BSTRBMtEphInv;

    impl<T: StTInMtT + Ord> RwLockPredicate<Link<T>> for BSTRBMtEphInv {
        open spec fn inv(self, v: Link<T>) -> bool {
            link_spec_size(v) <= usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {
        pub(crate) root: RwLock<Link<T>, BSTRBMtEphInv>,
        pub(crate) ghost_root: Ghost<Link<T>>,
    }

    pub type BSTreeRB<T> = BSTRBMtEph<T>;

    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            link_spec_size(self.ghost_root@) <= usize::MAX
        }

        pub closed spec fn spec_ghost_root(self) -> Link<T> {
            self.ghost_root@
        }
    }

    impl<T: StTInMtT + Ord> View for BSTRBMtEph<T> {
        type V = Link<T>;
        open spec fn view(&self) -> Link<T> { self.spec_ghost_root() }
    }

    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized + View<V = Link<T>> {
        spec fn spec_bstrbmteph_wf(&self) -> bool;

        fn new() -> (tree: Self)
            ensures tree.spec_bstrbmteph_wf(),
                    tree@ is None;

        fn from_sorted_slice(values: &[T]) -> (tree: Self)
            ensures tree.spec_bstrbmteph_wf();

        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstrbmteph_wf(),
            ensures self.spec_bstrbmteph_wf(),
                    match r {
                        Ok(_) => link_spec_size(self@) <= link_spec_size(old(self)@) + 1,
                        Err(_) => self@ == old(self)@,
                    };

        fn contains(&self, target: &T) -> (found: B)
            requires self.spec_bstrbmteph_wf(),
            ensures found == link_contains(self@, *target);

        fn size(&self) -> (n: N)
            requires self.spec_bstrbmteph_wf(),
            ensures n as nat == link_spec_size(self@);

        fn is_empty(&self) -> (b: B)
            requires self.spec_bstrbmteph_wf(),
            ensures b == (self@ is None);

        fn height(&self) -> (h: N)
            requires self.spec_bstrbmteph_wf(),
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

    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {
        open spec fn spec_bstrbmteph_wf(&self) -> bool {
            link_spec_size(self@) <= usize::MAX
        }

        fn new() -> Self {
            BSTRBMtEph {
                root: RwLock::new(None, Ghost(BSTRBMtEphInv)),
                ghost_root: Ghost(None),
            }
        }

        fn from_sorted_slice(values: &[T]) -> Self {
            let link = build_balanced(values);
            let ghost ghost_link = link;
            BSTRBMtEph {
                root: RwLock::new(link, Ghost(BSTRBMtEphInv)),
                ghost_root: Ghost(ghost_link),
            }
        }

        // Writer: assume ghost == inner, exec-check precondition, mutate or bail.
        fn insert(&mut self, value: T) -> (r: Result<(), ()>) {
            let (mut current, write_handle) = self.root.acquire_write();
            proof { accept(self.ghost_root@ == current); }
            let sz = compute_link_spec_size(&current);
            if sz < usize::MAX {
                insert_link(&mut current, value);
                if let Some(node) = current.as_mut() {
                    node.color = Color::Black;
                }
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
            proof { accept(found == link_contains(self@, *target)); }
            handle.release_read();
            found
        }

        // Reader: assume return value matches ghost.
        fn size(&self) -> (n: N) {
            let handle = self.root.acquire_read();
            let n = size_link(handle.borrow());
            proof { accept(n as nat == link_spec_size(self@)); }
            handle.release_read();
            n
        }

        // Predicate: assume return predicate matches spec predicate.
        fn is_empty(&self) -> (b: B) {
            let handle = self.root.acquire_read();
            let b = handle.borrow().is_none();
            proof { accept(b == (self@ is None)); }
            handle.release_read();
            b
        }

        // Reader: assume return value matches ghost.
        fn height(&self) -> (h: N) {
            let handle = self.root.acquire_read();
            let h = height_rec(handle.borrow());
            proof { accept(h as nat == link_height(self@)); }
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

    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    } // verus!

    // 13. macros

    #[macro_export]
    macro_rules! BSTRBMtEphLit {
        () => {
            < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> >::new()
        };
        ($($x:expr),* $(,)?) => {{
            let mut __tree = < $crate::Chap37::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> >::new();
            $( let _ = __tree.insert($x); )*
            __tree
        }};
    }

    // 14. derive impls outside verus!

    impl std::fmt::Debug for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Color::Red => write!(f, "Red"),
                Color::Black => write!(f, "Black"),
            }
        }
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(self, f)
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("color", &self.color)
                .field("size", &self.size)
                .finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.key)
        }
    }

    impl std::fmt::Debug for BSTRBMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBMtEphInv").finish()
        }
    }

    impl std::fmt::Display for BSTRBMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBMtEphInv")
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Debug for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTRBMtEph").finish()
        }
    }

    impl<T: StTInMtT + Ord> std::fmt::Display for BSTRBMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTRBMtEph(size={})", self.size())
        }
    }
}
