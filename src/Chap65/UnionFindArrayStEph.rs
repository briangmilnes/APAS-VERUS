// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Union-Find (Disjoint Set Union) — Array-based, Sequential Ephemeral.
//!
//! Array-based union-find with union by rank (no path compression).
//! Elements are indices [0, n). Spec uses pure_find with rank-based termination.
//! Size-rank invariant: each root's subtree has >= rank+1 elements.
//
//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions — struct UnionFindArray
//	Section 5. view impls — struct UnionFindArray
//	Section 6. spec fns — struct UnionFindArray
//	Section 7. proof fns/broadcast groups — struct UnionFindArray
//	Section 8. traits — struct UnionFindArray
//	Section 9. impls — struct UnionFindArray

pub mod UnionFindArrayStEph {

	//		Section 2. imports

    use vstd::prelude::*;

    verus! {

	//		Section 4. type definitions — struct UnionFindArray

    pub struct UnionFindArray { pub parent: Vec<usize>, pub rank: Vec<usize> }

	//		Section 5. view impls — struct UnionFindArray

    pub struct UnionFindArrayView { pub parent: Seq<int>, pub rank: Seq<int>, pub n: nat }

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

	//		Section 6. spec fns — struct UnionFindArray

    pub open spec fn spec_pure_find(parent: Seq<int>, rank: Seq<int>, n: nat, x: int) -> int
        recommends 0 <= x < n as int, parent.len() == n, rank.len() == n,
        decreases n as int - rank[x],
    {
        decreases_when(
            0 <= x && x < n as int && rank.len() == n && parent.len() == n
            && (forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] parent[i] && parent[i] < n as int)
            && (forall|i: int| 0 <= i < n as int && parent[i] != i ==> #[trigger] rank[i] < rank[parent[i] as int])
            && (forall|i: int| 0 <= i < n as int ==> rank[i] < n as int)
        );
        if x < 0 || x >= n as int { x }
        else if parent[x] == x { x }
        else { spec_pure_find(parent, rank, n, parent[x]) }
    }

    pub open spec fn spec_is_root(parent: Seq<int>, x: int) -> bool {
        0 <= x && x < parent.len() as int && parent[x] == x
    }

    pub open spec fn spec_wf(uf: UnionFindArrayView) -> bool {
        &&& uf.n > 0 &&& uf.parent.len() == uf.n &&& uf.rank.len() == uf.n
        &&& forall|i: int| 0 <= i < uf.n as int ==> 0 <= #[trigger] uf.parent[i] && uf.parent[i] < uf.n as int
        &&& forall|i: int| 0 <= i < uf.n as int ==> #[trigger] uf.rank[i] >= 0
    }

    pub open spec fn spec_rank_invariant(uf: UnionFindArrayView) -> bool {
        forall|i: int| #![trigger uf.parent[i], uf.rank[i]]
            0 <= i < uf.n as int && uf.parent[i] != i ==>
            uf.rank[i] < uf.rank[uf.parent[i] as int]
    }

    pub open spec fn spec_rank_bounded(uf: UnionFindArrayView) -> bool {
        forall|i: int| 0 <= i < uf.n as int ==> #[trigger] uf.rank[i] < uf.n as int
    }

    pub open spec fn spec_same_set(uf: UnionFindArrayView, x: int, y: int) -> bool {
        spec_pure_find(uf.parent, uf.rank, uf.n, x) == spec_pure_find(uf.parent, uf.rank, uf.n, y)
    }

    pub open spec fn spec_count_with_root(
        parent: Seq<int>, rank: Seq<int>, n: nat, root: int, k: nat,
    ) -> nat
        decreases n - k,
    {
        if k >= n { 0nat }
        else {
            (if spec_pure_find(parent, rank, n, k as int) == root { 1nat } else { 0nat })
                + spec_count_with_root(parent, rank, n, root, k + 1)
        }
    }

    pub open spec fn spec_size_rank_inv(uf: UnionFindArrayView) -> bool {
        forall|r: int| 0 <= r < uf.n as int && uf.parent[r] == r ==>
            spec_count_with_root(uf.parent, uf.rank, uf.n, r, 0) >= #[trigger] uf.rank[r] + 1
    }

    pub open spec fn spec_count_above(rank: Seq<int>, threshold: int, k: nat, n: nat) -> nat
        recommends rank.len() == n,
        decreases n - k,
    {
        if k >= n { 0nat }
        else { (if rank[k as int] > threshold { 1nat } else { 0nat }) + spec_count_above(rank, threshold, k + 1, n) }
    }

	//		Section 7. proof fns/broadcast groups — struct UnionFindArray

    proof fn lemma_count_above_mono(rank: Seq<int>, r1: int, r2: int, k: nat, n: nat)
        requires rank.len() == n, r1 <= r2,
        ensures spec_count_above(rank, r2, k, n) <= spec_count_above(rank, r1, k, n),
        decreases n - k,
    { if k < n { lemma_count_above_mono(rank, r1, r2, k + 1, n); } }

    proof fn lemma_count_above_strict(rank: Seq<int>, r: int, px: int, k: nat, n: nat)
        requires rank.len() == n, 0 <= px < n as int, rank[px] > r, k as int <= px,
        ensures spec_count_above(rank, r, k, n) > spec_count_above(rank, rank[px], k, n),
        decreases n - k,
    {
        if k as int == px { lemma_count_above_mono(rank, r, rank[px], k + 1, n); }
        else { lemma_count_above_strict(rank, r, px, k + 1, n); }
    }

    proof fn lemma_pure_find_in_bounds(parent: Seq<int>, rank: Seq<int>, n: nat, x: int)
        requires
            n > 0, parent.len() == n, rank.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] parent[i] && parent[i] < n as int,
            forall|i: int| 0 <= i < n as int && parent[i] != i ==> #[trigger] rank[i] < rank[parent[i] as int],
            forall|i: int| 0 <= i < n as int ==> rank[i] < n as int,
            0 <= x < n as int,
        ensures 0 <= spec_pure_find(parent, rank, n, x) < n as int,
        decreases spec_count_above(rank, rank[x], 0, n),
    {
        if parent[x] != x {
            lemma_count_above_strict(rank, rank[x], parent[x], 0, n);
            lemma_pure_find_in_bounds(parent, rank, n, parent[x]);
        }
    }

    proof fn lemma_pure_find_is_root(parent: Seq<int>, rank: Seq<int>, n: nat, x: int)
        requires
            n > 0, parent.len() == n, rank.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] parent[i] && parent[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] rank[i] >= 0,
            forall|i: int| 0 <= i < n as int && parent[i] != i ==> #[trigger] rank[i] < rank[parent[i] as int],
            forall|i: int| 0 <= i < n as int ==> rank[i] < n as int,
            0 <= x < n as int,
        ensures spec_is_root(parent, spec_pure_find(parent, rank, n, x)),
        decreases spec_count_above(rank, rank[x], 0, n),
    {
        if parent[x] != x {
            lemma_count_above_strict(rank, rank[x], parent[x], 0, n);
            lemma_pure_find_is_root(parent, rank, n, parent[x]);
        }
    }

    proof fn lemma_find_after_link(
        po: Seq<int>, ro: Seq<int>, pn: Seq<int>, rn: Seq<int>,
        n: nat, ra: int, rb: int, z: int,
    )
        requires
            n > 0, po.len() == n, ro.len() == n, pn.len() == n, rn.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] po[i] && po[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] ro[i] >= 0,
            forall|i: int| 0 <= i < n as int && po[i] != i ==> #[trigger] ro[i] < ro[po[i] as int],
            forall|i: int| 0 <= i < n as int ==> ro[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] pn[i] && pn[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] rn[i] >= 0,
            forall|i: int| 0 <= i < n as int && pn[i] != i ==> #[trigger] rn[i] < rn[pn[i] as int],
            forall|i: int| 0 <= i < n as int ==> rn[i] < n as int,
            0 <= ra < n as int, 0 <= rb < n as int, ra != rb,
            po[ra] == ra, po[rb] == rb, pn == po.update(ra, rb),
            forall|i: int| 0 <= i < n as int ==> rn[i] >= ro[i],
            0 <= z < n as int,
        ensures ({
            let fo = spec_pure_find(po, ro, n, z);
            let fn_ = spec_pure_find(pn, rn, n, z);
            if fo == ra { fn_ == rb } else if fo == rb { fn_ == rb } else { fn_ == fo }
        }),
        decreases spec_count_above(ro, ro[z], 0, n),
    {
        if po[z] == z {
            if z == ra {
                assert(pn[ra] == rb); assert(pn[rb] == rb);
                assert(spec_pure_find(pn, rn, n, rb) == rb);
            } else if z == rb { assert(pn[rb] == rb); }
            else { assert(pn[z] == z); }
        } else {
            assert(z != ra); assert(pn[z] == po[z]);
            lemma_count_above_strict(ro, ro[z], po[z], 0, n);
            lemma_find_after_link(po, ro, pn, rn, n, ra, rb, po[z]);
        }
    }

    proof fn lemma_map_update(s: Seq<usize>, idx: int, val: usize)
        requires 0 <= idx < s.len(),
        ensures s.update(idx, val).map(|_i, v: usize| v as int) =~=
                s.map(|_i, v: usize| v as int).update(idx, val as int),
    {
        let left = s.update(idx, val).map(|_i, v: usize| v as int);
        let right = s.map(|_i, v: usize| v as int).update(idx, val as int);
        assert forall|j: int| 0 <= j < s.len() implies #[trigger] left[j] == right[j] by {}
    }

    proof fn lemma_root_counted_at(
        parent: Seq<int>, rank: Seq<int>, n: nat, root: int, k: nat,
    )
        requires
            n > 0, parent.len() == n, rank.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] parent[i] && parent[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] rank[i] >= 0,
            forall|i: int| 0 <= i < n as int && parent[i] != i ==> #[trigger] rank[i] < rank[parent[i] as int],
            forall|i: int| 0 <= i < n as int ==> rank[i] < n as int,
            0 <= root < n as int, parent[root] == root, k as int <= root,
        ensures spec_count_with_root(parent, rank, n, root, k) >= 1,
        decreases n - k,
    { if k as int != root { lemma_root_counted_at(parent, rank, n, root, k + 1); } }

    proof fn lemma_count_disjoint(
        parent: Seq<int>, rank: Seq<int>, n: nat, rx: int, ry: int, k: nat,
    )
        requires
            n > 0, parent.len() == n, rank.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] parent[i] && parent[i] < n as int,
            forall|i: int| 0 <= i < n as int && parent[i] != i ==> #[trigger] rank[i] < rank[parent[i] as int],
            forall|i: int| 0 <= i < n as int ==> rank[i] < n as int,
            0 <= rx < n as int, 0 <= ry < n as int, rx != ry,
            parent[rx] == rx, parent[ry] == ry, k <= n,
        ensures spec_count_with_root(parent, rank, n, rx, k) + spec_count_with_root(parent, rank, n, ry, k) <= (n - k) as nat,
        decreases n - k,
    { if k < n { lemma_count_disjoint(parent, rank, n, rx, ry, k + 1); } }

    /// After linking ra → rb, winner's count = sum of both old counts.
    proof fn lemma_count_additive(
        po: Seq<int>, ro: Seq<int>, pn: Seq<int>, rn: Seq<int>,
        n: nat, ra: int, rb: int, k: nat,
    )
        requires
            n > 0, po.len() == n, ro.len() == n, pn.len() == n, rn.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] po[i] && po[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] ro[i] >= 0,
            forall|i: int| 0 <= i < n as int && po[i] != i ==> #[trigger] ro[i] < ro[po[i] as int],
            forall|i: int| 0 <= i < n as int ==> ro[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] pn[i] && pn[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] rn[i] >= 0,
            forall|i: int| 0 <= i < n as int && pn[i] != i ==> #[trigger] rn[i] < rn[pn[i] as int],
            forall|i: int| 0 <= i < n as int ==> rn[i] < n as int,
            0 <= ra < n as int, 0 <= rb < n as int, ra != rb,
            po[ra] == ra, po[rb] == rb, pn == po.update(ra, rb),
            forall|i: int| 0 <= i < n as int ==> rn[i] >= ro[i],
            k <= n,
        ensures spec_count_with_root(pn, rn, n, rb, k) == spec_count_with_root(po, ro, n, ra, k) + spec_count_with_root(po, ro, n, rb, k),
        decreases n - k,
    {
        if k < n {
            lemma_count_additive(po, ro, pn, rn, n, ra, rb, k + 1);
            lemma_find_after_link(po, ro, pn, rn, n, ra, rb, k as int);
        }
    }

    /// After linking ra → rb, other roots' counts are unchanged.
    proof fn lemma_count_other(
        po: Seq<int>, ro: Seq<int>, pn: Seq<int>, rn: Seq<int>,
        n: nat, ra: int, rb: int, other: int, k: nat,
    )
        requires
            n > 0, po.len() == n, ro.len() == n, pn.len() == n, rn.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] po[i] && po[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] ro[i] >= 0,
            forall|i: int| 0 <= i < n as int && po[i] != i ==> #[trigger] ro[i] < ro[po[i] as int],
            forall|i: int| 0 <= i < n as int ==> ro[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] pn[i] && pn[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] rn[i] >= 0,
            forall|i: int| 0 <= i < n as int && pn[i] != i ==> #[trigger] rn[i] < rn[pn[i] as int],
            forall|i: int| 0 <= i < n as int ==> rn[i] < n as int,
            0 <= ra < n as int, 0 <= rb < n as int, ra != rb,
            po[ra] == ra, po[rb] == rb, pn == po.update(ra, rb),
            forall|i: int| 0 <= i < n as int ==> rn[i] >= ro[i],
            0 <= other < n as int, other != ra, other != rb, k <= n,
        ensures spec_count_with_root(pn, rn, n, other, k) == spec_count_with_root(po, ro, n, other, k),
        decreases n - k,
    {
        if k < n {
            lemma_count_other(po, ro, pn, rn, n, ra, rb, other, k + 1);
            lemma_find_after_link(po, ro, pn, rn, n, ra, rb, k as int);
        }
    }

    /// Two distinct roots of equal rank r: r + 1 < n.
    proof fn lemma_rank_lt_n_minus_1(
        parent: Seq<int>, rank: Seq<int>, n: nat, rx: int, ry: int,
    )
        requires
            n > 0, parent.len() == n, rank.len() == n,
            forall|i: int| 0 <= i < n as int ==> 0 <= #[trigger] parent[i] && parent[i] < n as int,
            forall|i: int| 0 <= i < n as int ==> #[trigger] rank[i] >= 0,
            forall|i: int| 0 <= i < n as int && parent[i] != i ==> #[trigger] rank[i] < rank[parent[i] as int],
            forall|i: int| 0 <= i < n as int ==> rank[i] < n as int,
            spec_size_rank_inv(UnionFindArrayView { parent, rank, n }),
            0 <= rx < n as int, 0 <= ry < n as int, rx != ry,
            parent[rx] == rx, parent[ry] == ry, rank[rx] == rank[ry],
        ensures rank[rx] + 1 < n as int,
    {
        let r = rank[rx];
        let uf = UnionFindArrayView { parent, rank, n };
        // Trigger size_rank_inv on rx and ry.
        assert(uf.parent[rx] == rx);
        assert(uf.rank[rx] >= 0);
        assert(uf.parent[ry] == ry);
        assert(uf.rank[ry] >= 0);
        lemma_count_disjoint(parent, rank, n, rx, ry, 0);
        // 2*(r+1) <= n, and n >= 2*(r+1) >= 2. So r+1 <= n/2 < n.
    }

	//		Section 8. traits — struct UnionFindArray

    pub trait UnionFindArrayStEphTrait: Sized + View<V = UnionFindArrayView> {
        spec fn spec_unionfindarraysteph_wf(&self) -> bool;

        fn new(n: usize) -> (uf: Self)
            requires n > 0, n < usize::MAX,
            ensures uf.spec_unionfindarraysteph_wf(), uf@.n == n as nat,
                forall|i: int| 0 <= i < n as int ==> #[trigger] spec_is_root(uf@.parent, i);

        fn find(&mut self, x: usize) -> (root: usize)
            requires old(self).spec_unionfindarraysteph_wf(), (x as int) < old(self)@.n as int,
            ensures self.spec_unionfindarraysteph_wf(), self@ == old(self)@,
                (root as int) < self@.n as int,
                root as int == spec_pure_find(self@.parent, self@.rank, self@.n, x as int),
                spec_is_root(self@.parent, root as int);

        fn union(&mut self, x: usize, y: usize)
            requires old(self).spec_unionfindarraysteph_wf(),
                (x as int) < old(self)@.n as int, (y as int) < old(self)@.n as int,
            ensures self.spec_unionfindarraysteph_wf(), self@.n == old(self)@.n,
                spec_same_set(self@, x as int, y as int),
                forall|z: int| 0 <= z < self@.n as int
                    && spec_pure_find(old(self)@.parent, old(self)@.rank, old(self)@.n, z)
                        != spec_pure_find(old(self)@.parent, old(self)@.rank, old(self)@.n, x as int)
                    && spec_pure_find(old(self)@.parent, old(self)@.rank, old(self)@.n, z)
                        != spec_pure_find(old(self)@.parent, old(self)@.rank, old(self)@.n, y as int)
                    ==> #[trigger] spec_pure_find(self@.parent, self@.rank, self@.n, z)
                        == spec_pure_find(old(self)@.parent, old(self)@.rank, old(self)@.n, z);

        fn num_sets(&self) -> (count: usize) requires self.spec_unionfindarraysteph_wf();
        fn size(&self) -> (n: usize)
            requires self.spec_unionfindarraysteph_wf(), ensures n as nat == self@.n;
    }

	//		Section 9. impls — struct UnionFindArray

    impl UnionFindArrayStEphTrait for UnionFindArray {
        open spec fn spec_unionfindarraysteph_wf(&self) -> bool {
            spec_wf(self@) && spec_rank_invariant(self@) && spec_rank_bounded(self@) && spec_size_rank_inv(self@)
        }

        fn new(n: usize) -> (uf: Self) {
            let mut parent: Vec<usize> = Vec::new();
            let mut rank: Vec<usize> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant 0 <= i <= n, parent@.len() == i as nat, rank@.len() == i as nat,
                    forall|j: int| 0 <= j < i as int ==> #[trigger] parent@[j] == j,
                    forall|j: int| 0 <= j < i as int ==> #[trigger] rank@[j] == 0,
                decreases n - i,
            { parent.push(i); rank.push(0usize); i = i + 1; }
            let uf = UnionFindArray { parent, rank };
            proof {
                // spec_size_rank_inv: for each root r, count(r) >= rank[r]+1 = 1.
                // Every element is a root (parent[i]=i). find(i)=i for all i.
                // So count(r, 0) includes exactly r (the element itself). count(r, 0) >= 1.
                assert forall|r: int| 0 <= r < n as int && uf@.parent[r] == r implies
                    spec_count_with_root(uf@.parent, uf@.rank, uf@.n, r, 0) >= #[trigger] uf@.rank[r] + 1
                by {
                    lemma_root_counted_at(uf@.parent, uf@.rank, uf@.n, r, 0);
                }
            }
            uf
        }

        fn find(&mut self, x: usize) -> (root: usize) {
            let mut curr = x;
            while self.parent[curr] != curr
                invariant self.spec_unionfindarraysteph_wf(), self@ == old(self)@,
                    (curr as int) < self@.n as int,
                    spec_pure_find(self@.parent, self@.rank, self@.n, curr as int)
                        == spec_pure_find(self@.parent, self@.rank, self@.n, x as int),
                decreases self@.n as int - self@.rank[curr as int],
            { curr = self.parent[curr]; }
            proof {
                lemma_pure_find_is_root(self@.parent, self@.rank, self@.n, x as int);
                lemma_pure_find_in_bounds(self@.parent, self@.rank, self@.n, x as int);
            }
            curr
        }

        fn union(&mut self, x: usize, y: usize) {
            let root_x = self.find(x);
            let root_y = self.find(y);
            if root_x == root_y { return; }
            let ghost po = self@.parent;
            let ghost ro = self@.rank;
            let ghost n = self@.n;
            if self.rank[root_x] < self.rank[root_y] {
                self.parent.set(root_x, root_y);
                proof {
                    lemma_map_update(old(self).parent@, root_x as int, root_y);
                    let pn = self@.parent; let rn = self@.rank;
                    assert(pn =~= po.update(root_x as int, root_y as int));
                    assert(rn =~= ro);
                    assert forall|i: int| 0 <= i < n as int implies 0 <= #[trigger] pn[i] && pn[i] < n as int by {}
                    assert forall|i: int| 0 <= i < n as int && pn[i] != i implies #[trigger] rn[i] < rn[pn[i] as int]
                    by { if i == root_x as int {} else {} }
                    lemma_count_additive(po, ro, pn, rn, n, root_x as int, root_y as int, 0);
                    assert forall|r: int| 0 <= r < n as int && pn[r] == r implies
                        spec_count_with_root(pn, rn, n, r, 0) >= #[trigger] rn[r] + 1
                    by {
                        if r == root_y as int {} else {
                            assert(r != root_x as int);
                            lemma_count_other(po, ro, pn, rn, n, root_x as int, root_y as int, r, 0);
                        }
                    }
                    lemma_find_after_link(po, ro, pn, rn, n, root_x as int, root_y as int, x as int);
                    lemma_find_after_link(po, ro, pn, rn, n, root_x as int, root_y as int, y as int);
                    assert forall|z: int| #![trigger spec_pure_find(pn, rn, n, z)]
                        0 <= z < n as int && spec_pure_find(po, ro, n, z) != root_x as int
                        && spec_pure_find(po, ro, n, z) != root_y as int
                        implies spec_pure_find(pn, rn, n, z) == spec_pure_find(po, ro, n, z)
                    by { lemma_find_after_link(po, ro, pn, rn, n, root_x as int, root_y as int, z); }
                }
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent.set(root_y, root_x);
                proof {
                    lemma_map_update(old(self).parent@, root_y as int, root_x);
                    let pn = self@.parent; let rn = self@.rank;
                    assert(pn =~= po.update(root_y as int, root_x as int));
                    assert(rn =~= ro);
                    assert forall|i: int| 0 <= i < n as int implies 0 <= #[trigger] pn[i] && pn[i] < n as int by {}
                    assert forall|i: int| 0 <= i < n as int && pn[i] != i implies #[trigger] rn[i] < rn[pn[i] as int]
                    by { if i == root_y as int {} else {} }
                    lemma_count_additive(po, ro, pn, rn, n, root_y as int, root_x as int, 0);
                    assert forall|r: int| 0 <= r < n as int && pn[r] == r implies
                        spec_count_with_root(pn, rn, n, r, 0) >= #[trigger] rn[r] + 1
                    by {
                        if r == root_x as int {} else {
                            assert(r != root_y as int);
                            lemma_count_other(po, ro, pn, rn, n, root_y as int, root_x as int, r, 0);
                        }
                    }
                    lemma_find_after_link(po, ro, pn, rn, n, root_y as int, root_x as int, x as int);
                    lemma_find_after_link(po, ro, pn, rn, n, root_y as int, root_x as int, y as int);
                    assert forall|z: int| #![trigger spec_pure_find(pn, rn, n, z)]
                        0 <= z < n as int && spec_pure_find(po, ro, n, z) != root_x as int
                        && spec_pure_find(po, ro, n, z) != root_y as int
                        implies spec_pure_find(pn, rn, n, z) == spec_pure_find(po, ro, n, z)
                    by { lemma_find_after_link(po, ro, pn, rn, n, root_y as int, root_x as int, z); }
                }
            } else {
                self.parent.set(root_y, root_x);
                let n_len = self.parent.len();
                proof {
                    assert(self@.rank[root_x as int] < n as int);
                    lemma_rank_lt_n_minus_1(po, ro, n, root_x as int, root_y as int);
                    // rank[root_x] + 1 < n, connecting view to exec level.
                    assert(ro[root_x as int] + 1 < n as int);
                    assert(self.rank@[root_x as int] < n_len);
                }
                let new_rank = self.rank[root_x] + 1;
                self.rank.set(root_x, new_rank);
                proof {
                    lemma_map_update(old(self).parent@, root_y as int, root_x);
                    lemma_map_update(old(self).rank@, root_x as int, new_rank);
                    let pn = self@.parent; let rn = self@.rank;
                    assert(pn =~= po.update(root_y as int, root_x as int));
                    assert(rn =~= ro.update(root_x as int, new_rank as int));
                    assert forall|i: int| 0 <= i < n as int implies 0 <= #[trigger] pn[i] && pn[i] < n as int by {}
                    assert forall|i: int| 0 <= i < n as int implies #[trigger] rn[i] >= 0 by {}
                    assert forall|i: int| 0 <= i < n as int && pn[i] != i implies #[trigger] rn[i] < rn[pn[i] as int]
                    by { if i == root_y as int { assert(pn[i] == root_x as int); } else { assert(pn[i] == po[i]); } }
                    assert forall|i: int| 0 <= i < n as int implies #[trigger] rn[i] < n as int by {}
                    assert forall|i: int| 0 <= i < n as int implies rn[i] >= ro[i] by {}
                    lemma_count_additive(po, ro, pn, rn, n, root_y as int, root_x as int, 0);
                    assert forall|r: int| 0 <= r < n as int && pn[r] == r implies
                        spec_count_with_root(pn, rn, n, r, 0) >= #[trigger] rn[r] + 1
                    by {
                        if r == root_x as int {
                            assert(po[root_x as int] == root_x as int);
                            assert(po[root_y as int] == root_y as int);
                            assert(spec_count_with_root(po, ro, n, root_x as int, 0) >= ro[root_x as int] + 1);
                            assert(spec_count_with_root(po, ro, n, root_y as int, 0) >= ro[root_y as int] + 1);
                        } else {
                            assert(r != root_y as int);
                            lemma_count_other(po, ro, pn, rn, n, root_y as int, root_x as int, r, 0);
                        }
                    }
                    lemma_find_after_link(po, ro, pn, rn, n, root_y as int, root_x as int, x as int);
                    lemma_find_after_link(po, ro, pn, rn, n, root_y as int, root_x as int, y as int);
                    assert forall|z: int| #![trigger spec_pure_find(pn, rn, n, z)]
                        0 <= z < n as int && spec_pure_find(po, ro, n, z) != root_x as int
                        && spec_pure_find(po, ro, n, z) != root_y as int
                        implies spec_pure_find(pn, rn, n, z) == spec_pure_find(po, ro, n, z)
                    by { lemma_find_after_link(po, ro, pn, rn, n, root_y as int, root_x as int, z); }
                }
            }
        }

        fn num_sets(&self) -> (count: usize) {
            let mut count: usize = 0; let mut i: usize = 0;
            while i < self.parent.len()
                invariant self.spec_unionfindarraysteph_wf(), 0 <= i <= self.parent@.len(), count <= i,
                decreases self.parent@.len() - i,
            { if self.parent[i] == i { count = count + 1; } i = i + 1; }
            count
        }

        fn size(&self) -> (n: usize) { self.parent.len() }
    }

    } // verus!

} // mod
