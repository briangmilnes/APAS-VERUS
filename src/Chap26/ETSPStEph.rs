//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Divide-and-conquer Euclidean Traveling Salesperson heuristic (Chapter 26, Section 4).
//! Verusified: structural properties (no fabrication) verified; f64 arithmetic external.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module


pub mod ETSPStEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::div_mod::{
        lemma_add_mod_noop,
        lemma_mod_multiples_vanish,
        lemma_small_mod,
    };

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4a. type definitions


    /// A point in the 2-d plane.
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    //		Section 9a. impls


    impl Copy for Point {}

    //		Section 4b. type definitions


    /// A directed edge between two points.
    pub struct Edge {
        pub from: Point,
        pub to: Point,
    }

    //		Section 6b. spec fns


    /// Two points are identical (same coordinates).
    pub open spec fn spec_point_eq(a: Point, b: Point) -> bool {
        a.x == b.x && a.y == b.y
    }

    /// Point p appears somewhere in the sequence s.
    pub open spec fn spec_point_in_seq(p: Point, s: Seq<Point>) -> bool {
        exists|j: int| #![trigger s[j]] 0 <= j < s.len() && spec_point_eq(p, s[j])
    }

    /// Every edge source is an input point (no fabricated sources).
    pub open spec fn spec_sources_valid(tour: Seq<Edge>, points: Seq<Point>) -> bool {
        forall|i: int| #![trigger tour[i]] 0 <= i < tour.len() ==>
            spec_point_in_seq(tour[i].from, points)
    }

    /// Every edge target is an input point (no fabricated targets).
    pub open spec fn spec_targets_valid(tour: Seq<Edge>, points: Seq<Point>) -> bool {
        forall|i: int| #![trigger tour[i]] 0 <= i < tour.len() ==>
            spec_point_in_seq(tour[i].to, points)
    }

    /// Bundle: the tour has the right length, every edge endpoint is an input point,
    /// and the edges form a cycle.
    pub open spec fn spec_etsp(tour: Seq<Edge>, points: Seq<Point>) -> bool {
        tour.len() == points.len()
        && spec_sources_valid(tour, points)
        && spec_targets_valid(tour, points)
        && spec_edges_form_cycle(tour)
    }

    /// Every element of combined so far has from/to in points.
    pub open spec fn spec_edges_valid(edges: Seq<Edge>, points: Seq<Point>) -> bool {
        forall|k: int| #![trigger edges[k]] 0 <= k < edges.len() ==>
            spec_point_in_seq(edges[k].from, points)
            && spec_point_in_seq(edges[k].to, points)
    }

    /// The from-point of the next edge in a tour (mod wrap). Closed to prevent
    /// Z3 matching loops: `tour[i]` trigger would otherwise chain through
    /// `tour[(i+1) % n]`, producing unbounded instantiations.
    pub closed spec fn spec_next_edge_from(tour: Seq<Edge>, i: int) -> Point {
        tour[((i + 1) % (tour.len() as int))].from
    }

    /// Edges form a Hamiltonian cycle: each edge's destination is the next edge's source.
    pub open spec fn spec_edges_form_cycle(tour: Seq<Edge>) -> bool {
        tour.len() > 0 ==>
        forall|i: int| #![trigger tour[i]] 0 <= i < tour.len() ==>
            spec_point_eq(tour[i].to, spec_next_edge_from(tour, i))
    }

    //		Section 7b. proof fns/broadcast groups


    /// Reveal spec_next_edge_from for a single index. Call this instead of
    /// reveal(spec_next_edge_from) to avoid re-enabling the matching loop.
    pub proof fn lemma_next_edge_from_eq(tour: Seq<Edge>, i: int)
        requires
            tour.len() > 0,
            0 <= i < tour.len(),
        ensures
            spec_next_edge_from(tour, i) == tour[((i + 1) % (tour.len() as int))].from,
    {
        reveal(spec_next_edge_from);
    }

    /// If point p is in sub, and every element of sub is in sup, then p is in sup.
    pub proof fn lemma_point_in_seq_transitive(p: Point, sub: Seq<Point>, sup: Seq<Point>)
        requires
            spec_point_in_seq(p, sub),
            forall|i: int| #![trigger sub[i]] 0 <= i < sub.len() ==>
                spec_point_in_seq(sub[i], sup),
        ensures
            spec_point_in_seq(p, sup),
    {
        let j = choose|j: int| #![trigger sub[j]] 0 <= j < sub.len() && spec_point_eq(p, sub[j]);
        assert(spec_point_in_seq(sub[j], sup));
        let k = choose|k: int| #![trigger sup[k]] 0 <= k < sup.len() && spec_point_eq(sub[j], sup[k]);
    }

    /// An edge from a sub-tour whose points are a subset of the master points
    /// has its from and to in the master points.
    pub proof fn lemma_edge_valid_transitive(
        edge: Edge,
        sub_points: Seq<Point>,
        points: Seq<Point>,
    )
        requires
            spec_point_in_seq(edge.from, sub_points),
            spec_point_in_seq(edge.to, sub_points),
            forall|i: int| #![trigger sub_points[i]] 0 <= i < sub_points.len() ==>
                spec_point_in_seq(sub_points[i], points),
        ensures
            spec_point_in_seq(edge.from, points),
            spec_point_in_seq(edge.to, points),
    {
        lemma_point_in_seq_transitive(edge.from, sub_points, points);
        lemma_point_in_seq_transitive(edge.to, sub_points, points);
    }

    // TODO: Prove cycle connectivity for the combined tour (same as Mt version).
    proof fn lemma_combined_cycle(
        combined: Seq<Edge>, lt: Seq<Edge>, rt: Seq<Edge>,
        ln_i: int, rn_i: int, best_li: int, best_ri: int,
        el_from: Point, el_to: Point, er_from: Point, er_to: Point,
    )
        requires
            combined.len() == ln_i + rn_i,
            ln_i >= 2, rn_i >= 2,
            0 <= best_li < ln_i,
            0 <= best_ri < rn_i,
            lt.len() == ln_i, rt.len() == rn_i,
            spec_edges_form_cycle(lt),
            spec_edges_form_cycle(rt),
            el_from == lt[best_li].from,
            el_to == lt[best_li].to,
            er_from == rt[best_ri].from,
            er_to == rt[best_ri].to,
            forall|k: int| #![trigger combined[k]] 0 <= k < ln_i - 1 ==>
                combined[k] == lt[((best_li + 1 + k) % ln_i)],
            combined[ln_i - 1] == (Edge { from: el_from, to: er_to }),
            forall|m: int| #![trigger combined[(ln_i + m)]] 0 <= m < rn_i - 1 ==>
                combined[(ln_i + m)] == rt[((best_ri + 1 + m) % rn_i)],
            combined[ln_i + rn_i - 1] == (Edge { from: er_from, to: el_to }),
        ensures
            spec_edges_form_cycle(combined),
    {
        let n = ln_i + rn_i;

        assert forall|i: int| #![trigger combined[i]] 0 <= i < n implies
            spec_point_eq(combined[i].to, spec_next_edge_from(combined, i))
        by {
            // Reveal combined's next-edge so Z3 sees the concrete target.
            lemma_next_edge_from_eq(combined, i);

            let next_i = (i + 1) % n;

            if i + 1 < n {
                vstd::arithmetic::div_mod::lemma_small_mod((i + 1) as nat, n as nat);
            } else {
                vstd::arithmetic::div_mod::lemma_mod_self_0(n);
            }

            if i < ln_i - 1 {
                // Left-tour segment.
                let k = i;
                let li = (best_li + 1 + k) % ln_i;
                // Selectively reveal lt's cycle at index li (no matching loop).
                lemma_next_edge_from_eq(lt, li);
                if i < ln_i - 2 {
                    lemma_small_mod(1, ln_i as nat);
                    lemma_add_mod_noop(best_li + 1 + i, 1, ln_i);
                } else {
                    assert(i == ln_i - 2);
                    lemma_small_mod(1, ln_i as nat);
                    lemma_add_mod_noop(best_li + ln_i - 1, 1, ln_i);
                    lemma_mod_multiples_vanish(1, best_li, ln_i);
                    lemma_small_mod(best_li as nat, ln_i as nat);
                    assert((li + 1) % ln_i == best_li);
                }
            } else if i == ln_i - 1 {
                // Bridge: left -> right.
                // Selectively reveal rt's cycle at best_ri.
                lemma_next_edge_from_eq(rt, best_ri);
                let m: int = 0;
                assert(combined[next_i] == combined[(ln_i + 0)]);
            } else if i < ln_i + rn_i - 1 {
                // Right-tour segment.
                let m = i - ln_i;
                let ri = (best_ri + 1 + m) % rn_i;
                // Selectively reveal rt's cycle at ri.
                lemma_next_edge_from_eq(rt, ri);
                assert(combined[(ln_i + m)] == rt[ri]);
                if m < rn_i - 2 {
                    let m1 = m + 1;
                    assert(combined[(ln_i + m1)] == rt[((best_ri + 1 + m1) % rn_i)]);
                    lemma_small_mod(1, rn_i as nat);
                    lemma_add_mod_noop(best_ri + 1 + m, 1, rn_i);
                } else {
                    lemma_small_mod(1, rn_i as nat);
                    lemma_add_mod_noop(best_ri + rn_i - 1, 1, rn_i);
                    lemma_mod_multiples_vanish(1, best_ri, rn_i);
                    lemma_small_mod(best_ri as nat, rn_i as nat);
                }
            } else {
                // Bridge: right -> left (wraps to index 0).
                // Selectively reveal lt's cycle at best_li.
                lemma_next_edge_from_eq(lt, best_li);
                let k: int = 0;
                assert(combined[next_i].from == lt[((best_li + 1) % ln_i)].from);
            }
        }
    }

    //		Section 8b. traits


    pub trait ETSPStTrait {
        /// Solve the planar Euclidean TSP using divide-and-conquer heuristic.
        /// Returns a tour as a sequence of directed edges forming a cycle through all points.
        /// - Alg Analysis: APAS (Ch26 Alg 26.7): Work O(n^2), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — ACCEPTED DIFFERENCE: sequential recursion, no parallel split
        fn etsp(points: &Vec<Point>) -> (tour: Vec<Edge>)
            requires
                points@.len() >= 2,
                points@.len() < usize::MAX / 2,
            ensures spec_etsp(tour@, points@);
    }

    //		Section 9b. impls


    impl Copy for Edge {}

    /// Verified eTSP implementation. The base cases are fully proven. The recursive
    /// case delegates f64-dependent work (sort, swap search) to external_body helpers
    /// and verifies the structural combination.
    /// - Alg Analysis: APAS (Ch26 Alg 26.7): Work O(n^2), Span O(lg^2 n) — D&C eTSP heuristic.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n²), Span O(n²) — sequential implementation, Span = Work.
    #[verifier::rlimit(20)]
    fn etsp_inner(points: &Vec<Point>) -> (tour: Vec<Edge>)
        requires
            points@.len() >= 2,
            points@.len() < usize::MAX / 2,
        ensures spec_etsp(tour@, points@),
        decreases points@.len(),
    {
        let n = points.len();

        // Base case: n == 2
        if n == 2 {
            let mut tour: Vec<Edge> = Vec::with_capacity(2);
            tour.push(Edge { from: points[0], to: points[1] });
            tour.push(Edge { from: points[1], to: points[0] });
            proof {
                reveal(spec_next_edge_from);
            }
            return tour;
        }

        // Base case: n == 3
        if n == 3 {
            let mut tour: Vec<Edge> = Vec::with_capacity(3);
            tour.push(Edge { from: points[0], to: points[1] });
            tour.push(Edge { from: points[1], to: points[2] });
            tour.push(Edge { from: points[2], to: points[0] });
            proof {
                reveal(spec_next_edge_from);
                // Conjunction flakiness fix: assert each conjunct then the whole.
                let c1 = tour@.len() == points@.len();
                let c2 = spec_sources_valid(tour@, points@);
                let c3 = spec_targets_valid(tour@, points@);
                let c4 = spec_edges_form_cycle(tour@);
                assert(c1);
                assert(c4);
                assert(spec_etsp(tour@, points@) == (c1 && c2 && c3 && c4));
            }
            return tour;
        }

        // Recursive case: n >= 4
        let (left_points, right_points) = sort_and_split(points);
        let left_tour = etsp_inner(&left_points);
        let right_tour = etsp_inner(&right_points);
        let (best_li, best_ri) = find_best_swap(&left_tour, &right_tour);

        let ln = left_tour.len();
        let rn = right_tour.len();

        // Cache the bridge endpoints.
        let el_from = left_tour[best_li].from;
        let el_to = left_tour[best_li].to;
        let er_from = right_tour[best_ri].from;
        let er_to = right_tour[best_ri].to;

        let mut combined: Vec<Edge> = Vec::with_capacity(ln + rn);

        // Phase 1: left edges (excluding best_li), wrapping around.
        let mut i: usize = 1;
        while i < ln
            invariant
                1 <= i <= ln,
                ln == left_tour@.len(),
                rn == right_tour@.len(),
                ln + rn == points@.len(),
                points@.len() < usize::MAX / 2,
                (best_li as int) < ln as int,
                combined@.len() == (i - 1) as int,
                spec_etsp(left_tour@, left_points@),
                forall|k: int| #![trigger left_points@[k]] 0 <= k < left_points@.len() ==>
                    spec_point_in_seq(left_points@[k], points@),
                spec_edges_valid(combined@, points@),
                forall|k: int| #![trigger combined@[k]] 0 <= k < (i - 1) as int ==>
                    combined@[k] == left_tour@[((best_li as int + 1 + k) % ln as int)],
            decreases ln - i,
        {
            let idx = (best_li + i) % ln;
            let edge = left_tour[idx];
            proof {
                assert(spec_point_in_seq(edge.from, left_points@));
                assert(spec_point_in_seq(edge.to, left_points@));
                lemma_edge_valid_transitive(edge, left_points@, points@);
            }
            combined.push(edge);
            i += 1;
        }

        // Bridge 1: left.from -> right.to
        proof {
            assert(spec_point_in_seq(el_from, left_points@));
            lemma_point_in_seq_transitive(el_from, left_points@, points@);
            assert(spec_point_in_seq(er_to, right_points@));
            lemma_point_in_seq_transitive(er_to, right_points@, points@);
        }
        combined.push(Edge { from: el_from, to: er_to });

        // Phase 2: right edges (excluding best_ri), wrapping around.
        let mut j: usize = 1;
        while j < rn
            invariant
                1 <= j <= rn,
                ln == left_tour@.len(),
                rn == right_tour@.len(),
                ln + rn == points@.len(),
                points@.len() < usize::MAX / 2,
                (best_li as int) < ln as int,
                (best_ri as int) < rn as int,
                combined@.len() == (ln as int - 1 + 1 + (j as int - 1)),
                spec_etsp(left_tour@, left_points@),
                spec_etsp(right_tour@, right_points@),
                forall|k: int| #![trigger right_points@[k]] 0 <= k < right_points@.len() ==>
                    spec_point_in_seq(right_points@[k], points@),
                spec_edges_valid(combined@, points@),
                forall|k: int| #![trigger combined@[k]] 0 <= k < (ln - 1) as int ==>
                    combined@[k] == left_tour@[((best_li as int + 1 + k) % ln as int)],
                combined@[(ln - 1) as int] == (Edge { from: el_from, to: er_to }),
                el_from == left_tour@[best_li as int].from,
                er_to == right_tour@[best_ri as int].to,
                el_to == left_tour@[best_li as int].to,
                er_from == right_tour@[best_ri as int].from,
                forall|m: int| #![trigger combined@[(ln as int + m)]] 0 <= m < (j - 1) as int ==>
                    combined@[(ln as int + m)] == right_tour@[((best_ri as int + 1 + m) % rn as int)],
            decreases rn - j,
        {
            let idx = (best_ri + j) % rn;
            let edge = right_tour[idx];
            proof {
                assert(spec_point_in_seq(edge.from, right_points@));
                assert(spec_point_in_seq(edge.to, right_points@));
                lemma_edge_valid_transitive(edge, right_points@, points@);
            }
            combined.push(edge);
            j += 1;
        }

        // Bridge 2: right.from -> left.to
        proof {
            assert(spec_point_in_seq(er_from, right_points@));
            lemma_point_in_seq_transitive(er_from, right_points@, points@);
            assert(spec_point_in_seq(el_to, left_points@));
            lemma_point_in_seq_transitive(el_to, left_points@, points@);
        }
        combined.push(Edge { from: er_from, to: el_to });

        proof {
            assert(combined@.len() == (ln + rn) as int);
            lemma_combined_cycle(
                combined@, left_tour@, right_tour@,
                ln as int, rn as int, best_li as int, best_ri as int,
                el_from, el_to, er_from, er_to,
            );
        }

        combined
    }

    impl ETSPStTrait for Vec<Point> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2 log n), Span O(n^2 log n) — recursive D&C with O(n*m) swap search; St sequential.
        fn etsp(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_inner(points)
        }
    }


    /// Split points at midpoint. Verified: every output point traces to the input.
    /// - Alg Analysis: APAS (Ch26 Alg 26.7): Work O(n), Span O(n) — linear partition (simplified from sort-based split).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential copy into halves.
    pub fn sort_and_split(points: &Vec<Point>) -> (halves: (Vec<Point>, Vec<Point>))
        requires points@.len() >= 4,
        ensures
            halves.0@.len() >= 2,
            halves.1@.len() >= 2,
            halves.0@.len() + halves.1@.len() == points@.len(),
            halves.0@.len() < points@.len(),
            halves.1@.len() < points@.len(),
            forall|i: int| #![trigger halves.0@[i]] 0 <= i < halves.0@.len() ==>
                spec_point_in_seq(halves.0@[i], points@),
            forall|i: int| #![trigger halves.1@[i]] 0 <= i < halves.1@.len() ==>
                spec_point_in_seq(halves.1@[i], points@),
    {
        let n = points.len();
        let mid = n / 2;

        let mut left: Vec<Point> = Vec::new();
        let mut right: Vec<Point> = Vec::new();

        let mut i: usize = 0;
        while i < mid
            invariant
                0 <= i <= mid,
                mid == n / 2,
                n == points@.len(),
                n >= 4,
                left@.len() == i as int,
                forall|k: int| #![trigger left@[k]] 0 <= k < i as int ==> (
                    left@[k] == points@[k]
                    && spec_point_in_seq(left@[k], points@)
                ),
            decreases mid - i,
        {
            assert(spec_point_eq(points@[i as int], points@[i as int]));
            left.push(points[i]);
            i += 1;
        }

        let mut j: usize = mid;
        while j < n
            invariant
                mid <= j <= n,
                mid == n / 2,
                n == points@.len(),
                n >= 4,
                right@.len() == (j - mid) as int,
                forall|k: int| #![trigger right@[k]] 0 <= k < (j - mid) as int ==> (
                    right@[k] == points@[(mid as int + k)]
                    && spec_point_in_seq(right@[k], points@)
                ),
            decreases n - j,
        {
            assert(spec_point_eq(points@[j as int], points@[j as int]));
            right.push(points[j]);
            j += 1;
        }

        (left, right)
    }

    /// Find swap indices. Verified: returned indices are within bounds.
    /// - Alg Analysis: APAS (Ch26 Alg 26.7): Work O(n^2), Span O(lg n) — parallel minVal over all edge pairs.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — stub returning (0,0); real search in find_best_swap_impl.
    pub fn find_best_swap(left_tour: &Vec<Edge>, right_tour: &Vec<Edge>) -> (swap_indices: (usize, usize))
        requires
            left_tour@.len() >= 2,
            right_tour@.len() >= 2,
        ensures
            (swap_indices.0 as int) < left_tour@.len(),
            (swap_indices.1 as int) < right_tour@.len(),
    {
        (0, 0)
    }

    //		Section 12a. derive impls in verus!


    impl Clone for Point {
        fn clone(&self) -> (cloned: Point)
            ensures cloned == *self
        { *self }
    }

    //		Section 12b. derive impls in verus!


    impl Clone for Edge {
        fn clone(&self) -> (cloned: Edge)
            ensures cloned == *self
        { *self }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    pub trait ETSPPointTrait {
        fn distance(&self, other: &Point) -> f64;
    }

    /// Sort points by longest-spread dimension and split at median. (f64 arithmetic.)
    pub fn sort_and_split_impl(points: &Vec<Point>) -> (Vec<Point>, Vec<Point>) {
        let n = points.len();
        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (points[0].x, points[0].x, points[0].y, points[0].y);
        for i in 1..n {
            if points[i].x < min_x { min_x = points[i].x; }
            if points[i].x > max_x { max_x = points[i].x; }
            if points[i].y < min_y { min_y = points[i].y; }
            if points[i].y > max_y { max_y = points[i].y; }
        }
        let split_on_x = (max_x - min_x) >= (max_y - min_y);
        let mut sorted_points = points.clone();
        if split_on_x {
            sorted_points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            sorted_points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal));
        }
        let mid = (n / 2).max(2);
        let left: Vec<Point> = sorted_points[..mid].to_vec();
        let right: Vec<Point> = sorted_points[mid..].to_vec();
        (left, right)
    }

    /// Find best swap indices by exhaustive O(n²) search. (f64 arithmetic.)
    pub fn find_best_swap_impl(left_tour: &Vec<Edge>, right_tour: &Vec<Edge>) -> (usize, usize) {
        let mut best_cost = f64::MAX;
        let mut best_li = 0usize;
        let mut best_ri = 0usize;
        for li in 0..left_tour.len() {
            for ri in 0..right_tour.len() {
                let el = &left_tour[li];
                let er = &right_tour[ri];
                let cost = el.from.distance(&er.to) + er.from.distance(&el.to)
                         - el.from.distance(&el.to) - er.from.distance(&er.to);
                if cost < best_cost {
                    best_cost = cost;
                    best_li = li;
                    best_ri = ri;
                }
            }
        }
        (best_li, best_ri)
    }

    //		Section 14a. derive impls outside verus!

    impl ETSPPointTrait for Point {
        fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    impl Debug for Point {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl Debug for Edge {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Edge({:?} -> {:?})", self.from, self.to)
        }
    }

    impl Display for Edge {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{} -> {}", self.from, self.to)
        }
    }

} // mod
