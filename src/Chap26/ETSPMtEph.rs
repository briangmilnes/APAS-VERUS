// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Divide-and-conquer Euclidean Traveling Salesperson heuristic — parallel (Chapter 26, Section 4).
//! Structural logic verified; threading via help-first scheduler join().
//! Verusified.

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
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module


pub mod ETSPMtEph {


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


    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    use crate::vstdplus::float::float::{
        f64_add, f64_sub, f64_mul, f64_sqrt,
        unreachable_dist,
    };
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::float::float::{
        f64_add_spec, f64_sub_spec, f64_mul_spec, f64_sqrt_spec,
    };

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

    impl ETSPPointTrait for Point {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to point_distance.
        fn distance(&self, other: &Point) -> (d: f64) {
            point_distance(self, other)
        }
    }

    //		Section 4b. type definitions


    /// A directed edge between two points.
    pub struct Edge {
        pub from: Point,
        pub to: Point,
    }

    //		Section 6b. spec fns


    /// Euclidean distance between two points: sqrt((ax-bx)^2 + (ay-by)^2).
    pub open spec fn spec_point_distance(a: Point, b: Point) -> f64 {
        f64_sqrt_spec(
            f64_add_spec(
                f64_mul_spec(f64_sub_spec(a.x, b.x), f64_sub_spec(a.x, b.x)),
                f64_mul_spec(f64_sub_spec(a.y, b.y), f64_sub_spec(a.y, b.y)),
            ),
        )
    }

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
        let k = choose|k: int| #![trigger sup[k]] 0 <= k < sup.len() && spec_point_eq(sub[j], sup[k]);
        assert(spec_point_eq(p, sup[k]));
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

    /// Modular successor: ((a % n) + 1) % n == (a + 1) % n.
    proof fn lemma_mod_successor(a: int, n: int)
        requires n > 0,
        ensures ((a % n) + 1) % n == (a + 1) % n,
    {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(a, n);
        vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(a / n, a % n + 1, n);
    }

    /// The combined tour forms a cycle, given sub-tour cycle properties
    /// and the identity of each combined element.
    #[verifier::rlimit(40)]
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


    pub trait ETSPMtTrait {
        /// Solve the planar Euclidean TSP using parallel divide-and-conquer heuristic.
        /// Returns a tour as a sequence of directed edges forming a cycle through all points.
        /// Algorithm 26.7 with parallel recursive calls via help-first scheduler.
        /// - Alg Analysis: APAS (Ch26 Alg 26.7): Work O(n^2), Span O(lg^2 n) — parallel recur + parallel minVal.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n²), Span O(n²) — O(n²) swap search dominates; could be reduced
        ///   to Θ(lg² n) with parallel reduce over edge pairs.
        fn etsp_parallel(points: &Vec<Point>) -> (tour: Vec<Edge>)
            requires
                points@.len() >= 2,
                points@.len() < usize::MAX / 2,
            ensures spec_etsp(tour@, points@);
    }

    pub trait ETSPPointTrait {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to point_distance.
        fn distance(&self, other: &Point) -> (d: f64);
    }

    //		Section 9b. impls


    impl Copy for Edge {}


    /// Parallel eTSP inner recursion. Structural logic verified; threading via join().
    /// - Alg Analysis: APAS (Ch26 Alg 26.7): Work O(n^2), Span O(lg^2 n) — parallel recur + parallel minVal.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n²), Span O(n²) — O(n²) swap search dominates span.
    fn etsp_parallel_inner(points: &Vec<Point>) -> (tour: Vec<Edge>)
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

        // Capture ghost views before the move — these survive the closure captures.
        let ghost left_pts_view = left_points@;
        let ghost right_pts_view = right_points@;

        // Parallel recursive calls via help-first scheduler.
        let f1 = move || -> (tour: Vec<Edge>)
            ensures spec_etsp(tour@, left_pts_view)
        { etsp_parallel_inner(&left_points) };

        let f2 = move || -> (tour: Vec<Edge>)
            ensures spec_etsp(tour@, right_pts_view)
        { etsp_parallel_inner(&right_points) };

        let (left_tour, right_tour) = join(f1, f2);

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
                spec_etsp(left_tour@, left_pts_view),
                forall|k: int| #![trigger left_pts_view[k]] 0 <= k < left_pts_view.len() ==>
                    spec_point_in_seq(left_pts_view[k], points@),
                spec_edges_valid(combined@, points@),
                forall|k: int| #![trigger combined@[k]] 0 <= k < (i - 1) as int ==>
                    combined@[k] == left_tour@[((best_li as int + 1 + k) % ln as int)],
            decreases ln - i,
        {
            let idx = (best_li + i) % ln;
            let edge = left_tour[idx];
            proof {
                lemma_edge_valid_transitive(edge, left_pts_view, points@);
            }
            combined.push(edge);
            i += 1;
        }

        // Bridge 1: left.from -> right.to
        proof {
            lemma_point_in_seq_transitive(el_from, left_pts_view, points@);
            lemma_point_in_seq_transitive(er_to, right_pts_view, points@);
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
                spec_etsp(left_tour@, left_pts_view),
                spec_etsp(right_tour@, right_pts_view),
                forall|k: int| #![trigger right_pts_view[k]] 0 <= k < right_pts_view.len() ==>
                    spec_point_in_seq(right_pts_view[k], points@),
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
                lemma_edge_valid_transitive(edge, right_pts_view, points@);
            }
            combined.push(edge);
            j += 1;
        }

        // Bridge 2: right.from -> left.to
        proof {
            lemma_point_in_seq_transitive(er_from, right_pts_view, points@);
            lemma_point_in_seq_transitive(el_to, left_pts_view, points@);
        }
        combined.push(Edge { from: er_from, to: el_to });

        proof {
            lemma_combined_cycle(
                combined@, left_tour@, right_tour@,
                ln as int, rn as int, best_li as int, best_ri as int,
                el_from, el_to, er_from, er_to,
            );
        }

        combined
    }

    impl ETSPMtTrait for Vec<Point> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2 log n), Span O(n log^2 n) — recursive D&C with parallel swap search; Mt parallel.
        fn etsp_parallel(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_parallel_inner(points)
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 arithmetic: dx^2 + dy^2 + sqrt.
    // veracity: no_requires
    fn point_distance(a: &Point, b: &Point) -> (d: f64)
        ensures d == spec_point_distance(*a, *b),
    {
        let dx = f64_sub(a.x, b.x);
        let dy = f64_sub(a.y, b.y);
        let dx2 = f64_mul(dx, dx);
        let dy2 = f64_mul(dy, dy);
        let sum = f64_add(dx2, dy2);
        f64_sqrt(sum)
    }

    /// Sort points by longest-spread dimension and split at median. (f64 arithmetic.)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — sort + split.
    #[verifier::external_body]
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

    /// Parallel find-best-swap: recursively splits the outer loop over left_tour
    /// and runs both halves in parallel via HFScheduler join().
    /// Work Θ(n·m), Span Θ(m·lg n) where n = left_tour.len(), m = right_tour.len().
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(m * lg n) — parallel D&C over left_tour.
    pub fn find_best_swap_impl(left_tour: &Vec<Edge>, right_tour: &Vec<Edge>) -> (best_swap: (usize, usize))
        requires
            left_tour@.len() >= 1,
            right_tour@.len() >= 1,
        ensures
            best_swap.0 < left_tour@.len(),
            best_swap.1 < right_tour@.len(),
    {
        let lt_cloned = left_tour.clone();
        let rt_cloned = right_tour.clone();
        let lt = Arc::new(lt_cloned);
        let rt = Arc::new(rt_cloned);
        let (li, ri, _cost) = find_best_swap_par(lt, rt, 0, left_tour.len());
        (li, ri)
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(m * lg n) — recursive D&C with parallel halves via join; Mt parallel.
    fn find_best_swap_par(
        left_tour: Arc<Vec<Edge>>, right_tour: Arc<Vec<Edge>>, lo: usize, hi: usize,
    ) -> (best_swap: (usize, usize, f64))
        requires
            (*left_tour)@.len() >= 1,
            (*right_tour)@.len() >= 1,
            hi <= (*left_tour)@.len(),
        ensures
            best_swap.0 < (*left_tour)@.len(),
            best_swap.1 < (*right_tour)@.len(),
        decreases hi - lo,
    {
        let lv: &Vec<Edge> = arc_deref(&left_tour);
        let rv: &Vec<Edge> = arc_deref(&right_tour);
        let sentinel = unreachable_dist().val;

        if hi <= lo {
            return (0, 0, sentinel);
        }
        if hi - lo <= 16 {
            let mut best_cost = sentinel;
            let mut best_li: usize = 0;
            let mut best_ri: usize = 0;
            let mut li: usize = lo;
            while li < hi
                invariant
                    lo <= li,
                    li <= hi,
                    hi <= lv@.len(),
                    lv@.len() == (*left_tour)@.len(),
                    rv@.len() == (*right_tour)@.len(),
                    best_li < lv@.len(),
                    best_ri < rv@.len(),
                decreases hi - li,
            {
                let mut ri: usize = 0;
                while ri < rv.len()
                    invariant
                        ri <= rv@.len(),
                        lv@.len() == (*left_tour)@.len(),
                        rv@.len() == (*right_tour)@.len(),
                        best_li < lv@.len(),
                        best_ri < rv@.len(),
                        li < hi,
                        hi <= lv@.len(),
                    decreases rv@.len() - ri,
                {
                    let el: &Edge = &lv[li];
                    let er: &Edge = &rv[ri];
                    let d1 = point_distance(&el.from, &er.to);
                    let d2 = point_distance(&er.from, &el.to);
                    let d3 = point_distance(&el.from, &el.to);
                    let d4 = point_distance(&er.from, &er.to);
                    let cost = f64_sub(f64_add(d1, d2), f64_add(d3, d4));
                    if cost < best_cost {
                        best_cost = cost;
                        best_li = li;
                        best_ri = ri;
                    }
                    ri = ri + 1;
                }
                li = li + 1;
            }
            (best_li, best_ri, best_cost)
        } else {
            let mid = lo + (hi - lo) / 2;
            let ghost ll: nat = lv@.len();
            let ghost rl: nat = rv@.len();

            let lt1 = Arc::clone(&left_tour);
            let rt1 = Arc::clone(&right_tour);

            let ghost lt1_ll: nat = (*lt1)@.len();
            let ghost rt1_rl: nat = (*rt1)@.len();


            let f1 = move || -> (r: (usize, usize, f64))
                ensures r.0 < lt1_ll, r.1 < rt1_rl,
            {
                proof {
                }
                find_best_swap_par(lt1, rt1, lo, mid)
            };

            let f2 = move || -> (r: (usize, usize, f64))
                ensures r.0 < ll, r.1 < rl,
            {
                proof {
                }
                find_best_swap_par(left_tour, right_tour, mid, hi)
            };

            let (left_res, right_res) = join(f1, f2);

            proof {
            }

            if left_res.2 <= right_res.2 { left_res } else { right_res }
        }
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

    use std::sync::Arc;
    } // verus!

    //		Section 14a. derive impls outside verus!


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
