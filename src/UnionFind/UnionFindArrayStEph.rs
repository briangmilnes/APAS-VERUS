//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Union-Find (Disjoint Set Union) — Array-based, Sequential Ephemeral.
//!
//! Reimplementation using Vec<usize> instead of HashMap to avoid =~= extensional
//! equality matching loops in Z3. Based on the AlgoStar/F* CLRS Ch21 implementation.
//!
//! Three operations: make_set, find (root-chasing), union (by rank).
//! Spec uses pure_find (recursive root-chasing on the spec-level parent Seq).

pub mod UnionFindArrayStEph {

    //  1. module

    //  2. imports

    use vstd::prelude::*;

    //  3. broadcast use

    verus! {

    //  4. type definitions

    /// Array-based union-find forest. parent[i] and rank[i] for i in [0, n).
    /// Invariant: parent[i] < n for all i < n.
    pub struct UnionFindArray {
        pub parent: Vec<usize>,
        pub rank: Vec<usize>,
    }

    //  5. view impls

    /// View is a pair of (parent Seq, rank Seq).
    pub struct UnionFindArrayView {
        pub parent: Seq<int>,
        pub rank: Seq<int>,
        pub n: nat,
    }

    impl View for UnionFindArray {
        type V = UnionFindArrayView;
        open spec fn view(&self) -> UnionFindArrayView {
            UnionFindArrayView {
                parent: self.parent@.map(|_i, v: usize| v as int),
                rank: self.rank@.map(|_i, v: usize| v as int),
                n: self.parent@.len(),
            }
        }
    }

    //  6. spec fns

    /// Recursive root-chasing. The spec-level find operation.
    pub open spec fn spec_pure_find(parent: Seq<int>, n: nat, x: int) -> int
        decreases n,
    {
        if x < 0 || x >= n as int {
            x
        } else if parent[x] == x {
            x
        } else {
            spec_pure_find(parent, n, parent[x])
        }
    }

    /// Is x a root in the forest?
    pub open spec fn spec_is_root(parent: Seq<int>, x: int) -> bool {
        0 <= x && x < parent.len() as int && parent[x] == x
    }

    /// Forest well-formedness: lengths match, all parent indices in bounds.
    pub open spec fn spec_wf(uf: UnionFindArrayView) -> bool {
        &&& uf.n > 0
        &&& uf.parent.len() == uf.n
        &&& uf.rank.len() == uf.n
        &&& forall|i: int| 0 <= i < uf.n as int ==>
                0 <= #[trigger] uf.parent[i] && uf.parent[i] < uf.n as int
        &&& forall|i: int| 0 <= i < uf.n as int ==>
                #[trigger] uf.rank[i] >= 0
    }

    /// Rank invariant: a non-root's rank < its parent's rank.
    pub open spec fn spec_rank_invariant(uf: UnionFindArrayView) -> bool {
        forall|i: int| 0 <= i < uf.n as int && uf.parent[i] != i ==>
            #[trigger] uf.rank[i] < uf.rank[uf.parent[i] as int]
    }

    /// Full invariant: wf + rank invariant.
    pub open spec fn spec_unionfindarraysteph_wf(uf: &UnionFindArray) -> bool {
        spec_wf(uf@) && spec_rank_invariant(uf@)
    }

    /// Two elements are in the same set.
    pub open spec fn spec_same_set(uf: UnionFindArrayView, x: int, y: int) -> bool {
        spec_pure_find(uf.parent, uf.n, x) == spec_pure_find(uf.parent, uf.n, y)
    }

    //  7. proof fns

    /// pure_find returns a value in [0, n).
    proof fn lemma_pure_find_in_bounds(parent: Seq<int>, n: nat, x: int)
        requires
            n > 0,
            parent.len() == n,
            forall|i: int| 0 <= i < n as int ==>
                0 <= #[trigger] parent[i] && parent[i] < n as int,
            0 <= x && x < n as int,
        ensures
            0 <= spec_pure_find(parent, n, x) && spec_pure_find(parent, n, x) < n as int,
        decreases n,
    {
        if parent[x] == x {
        } else {
            lemma_pure_find_in_bounds(parent, n, parent[x]);
        }
    }

    /// pure_find returns a root.
    proof fn lemma_pure_find_is_root(parent: Seq<int>, n: nat, x: int)
        requires
            n > 0,
            parent.len() == n,
            forall|i: int| 0 <= i < n as int ==>
                0 <= #[trigger] parent[i] && parent[i] < n as int,
            spec_rank_invariant(UnionFindArrayView { parent, rank: Seq::empty(), n }),
            0 <= x && x < n as int,
        ensures
            spec_is_root(parent, spec_pure_find(parent, n, x)),
        decreases n,
    {
        if parent[x] == x {
        } else {
            lemma_pure_find_is_root(parent, n, parent[x]);
        }
    }

    //  8. traits

    pub trait UnionFindArrayStEphTrait: Sized {
        /// Create an n-element forest where each element is its own set.
        fn new(n: usize) -> (uf: Self)
            requires n > 0, n < usize::MAX,
            ensures spec_unionfindarraysteph_wf(&uf),
                    uf@.n == n as nat,
                    forall|i: int| 0 <= i < n as int ==>
                        spec_is_root(uf@.parent, i);

        /// Find the root of x's set.
        fn find(&mut self, x: usize) -> (root: usize)
            requires
                spec_unionfindarraysteph_wf(old(self)),
                0 <= x && (x as int) < old(self)@.n as int,
            ensures
                spec_unionfindarraysteph_wf(self),
                self@ == old(self)@,
                0 <= root && (root as int) < self@.n as int,
                root as int == spec_pure_find(self@.parent, self@.n, x as int),
                spec_is_root(self@.parent, root as int);

        /// Union the sets containing x and y.
        fn union(&mut self, x: usize, y: usize)
            requires
                spec_unionfindarraysteph_wf(old(self)),
                0 <= x && (x as int) < old(self)@.n as int,
                0 <= y && (y as int) < old(self)@.n as int,
            ensures
                spec_unionfindarraysteph_wf(self),
                self@.n == old(self)@.n,
                // Merge: x and y in the same set after union.
                spec_same_set(self@, x as int, y as int),
                // Stability: elements disjoint from both x and y are unchanged.
                forall|z: int| 0 <= z < self@.n as int
                    && spec_pure_find(old(self)@.parent, old(self)@.n, z)
                        != spec_pure_find(old(self)@.parent, old(self)@.n, x as int)
                    && spec_pure_find(old(self)@.parent, old(self)@.n, z)
                        != spec_pure_find(old(self)@.parent, old(self)@.n, y as int)
                    ==> spec_pure_find(self@.parent, self@.n, z)
                        == spec_pure_find(old(self)@.parent, old(self)@.n, z);

        /// Number of distinct sets.
        fn num_sets(&self) -> (count: usize)
            requires spec_unionfindarraysteph_wf(self);

        /// Number of elements.
        fn size(&self) -> (n: usize)
            requires spec_unionfindarraysteph_wf(self),
            ensures n as nat == self@.n;
    }

    //  9. impls

    impl UnionFindArrayStEphTrait for UnionFindArray {
        /// - Alg Analysis: APAS (Ch65): Work O(n), Span O(n).
        fn new(n: usize) -> (uf: Self) {
            let mut parent: Vec<usize> = Vec::new();
            let mut rank: Vec<usize> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    parent@.len() == i as nat,
                    rank@.len() == i as nat,
                    forall|j: int| 0 <= j < i as int ==>
                        #[trigger] parent@[j] == j,
                    forall|j: int| 0 <= j < i as int ==>
                        #[trigger] rank@[j] == 0,
                decreases n - i,
            {
                parent.push(i);
                rank.push(0usize);
                i = i + 1;
            }
            UnionFindArray { parent, rank }
        }

        /// - Alg Analysis: APAS (Ch65): Work O(log n), Span O(log n).
        fn find(&mut self, x: usize) -> (root: usize) {
            let mut curr = x;
            while self.parent[curr] != curr
                invariant
                    spec_unionfindarraysteph_wf(self),
                    self@ == old(self)@,
                    0 <= curr && (curr as int) < self@.n as int,
                    spec_pure_find(self@.parent, self@.n, curr as int)
                        == spec_pure_find(self@.parent, self@.n, x as int),
                decreases self@.rank[curr as int],
            {
                curr = self.parent[curr];
            }
            curr
        }

        /// - Alg Analysis: APAS (Ch65): Work O(log n), Span O(log n).
        fn union(&mut self, x: usize, y: usize) {
            let root_x = self.find(x);
            let root_y = self.find(y);
            if root_x == root_y {
                return;
            }
            if self.rank[root_x] < self.rank[root_y] {
                self.parent.set(root_x, root_y);
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent.set(root_y, root_x);
            } else {
                self.parent.set(root_y, root_x);
                let new_rank = self.rank[root_x] + 1;
                self.rank.set(root_x, new_rank);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n).
        fn num_sets(&self) -> (count: usize) {
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < self.parent.len()
                invariant
                    spec_unionfindarraysteph_wf(self),
                    0 <= i <= self.parent@.len(),
                decreases self.parent@.len() - i,
            {
                if self.parent[i] == i {
                    count = count + 1;
                }
                i = i + 1;
            }
            count
        }

        fn size(&self) -> (n: usize) {
            self.parent.len()
        }
    }

    } // verus!

} // mod
