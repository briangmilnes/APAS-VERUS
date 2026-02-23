//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Divide-and-conquer Euclidean Traveling Salesperson heuristic — parallel (Chapter 26, Section 4).
//! Structural logic verified; threading via help-first scheduler join().
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls
//	11. derive impls in verus!

//		1. module




pub mod ETSPMtEph {

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::div_mod::{
        lemma_add_mod_noop,
        lemma_mod_multiples_vanish,
        lemma_small_mod,
    };

    verus! {

    //		2. imports

    //		2. imports

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;


    //		3. broadcast use

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };


    //		4. type definitions

    //		4. type definitions

    /// A point in the 2-d plane.
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    /// A directed edge between two points.
    pub struct Edge {
        pub from: Point,
        pub to: Point,
    }


    //		6. spec fns

    //		5. spec fns

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

    /// Edges form a Hamiltonian cycle: each edge's destination is the next edge's source.
    pub open spec fn spec_edges_form_cycle(tour: Seq<Edge>) -> bool {
        tour.len() > 0 ==>
        forall|i: int| #![trigger tour[i]] 0 <= i < tour.len() ==>
            spec_point_eq(tour[i].to, tour[((i + 1) % (tour.len() as int))].from)
    }


    //		7. proof fns/broadcast groups

    //		7. proof fns

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
            spec_point_eq(combined[i].to, combined[((i + 1) % n)].from)
        by {
            let next_i = (i + 1) % n;

            if i + 1 < n {
                vstd::arithmetic::div_mod::lemma_small_mod((i + 1) as nat, n as nat);
                assert(next_i == i + 1);
            } else {
                vstd::arithmetic::div_mod::lemma_mod_self_0(n);
                assert(next_i == 0);
            }

            if i < ln_i - 1 {
                let k = i;
                assert(combined[k] == lt[((best_li + 1 + k) % ln_i)]);
                let li = (best_li + 1 + k) % ln_i;
                assert(spec_point_eq(lt[li].to, lt[((li + 1) % ln_i)].from));
                assert(combined[i] == lt[li]);
                if i < ln_i - 2 {
                    lemma_small_mod(1, ln_i as nat);
                    lemma_add_mod_noop(best_li + 1 + i, 1, ln_i);
                    assert(combined[(i + 1)] == lt[((best_li + 1 + (i + 1)) % ln_i)]);
                    assert((best_li + 1 + (i + 1)) % ln_i == (li + 1) % ln_i);
                    assert(combined[next_i] == lt[((li + 1) % ln_i)]);
                } else {
                    assert(i == ln_i - 2);
                    assert(next_i == ln_i - 1);
                    assert(combined[next_i] == (Edge { from: el_from, to: er_to }));
                    assert(el_from == lt[best_li].from);
                    lemma_small_mod(1, ln_i as nat);
                    lemma_add_mod_noop(best_li + ln_i - 1, 1, ln_i);
                    lemma_mod_multiples_vanish(1, best_li, ln_i);
                    lemma_small_mod(best_li as nat, ln_i as nat);
                    assert((li + 1) % ln_i == best_li);
                    assert(spec_point_eq(lt[li].to, lt[best_li].from));
                }
                assert(combined[i].to == lt[li].to);
                assert(combined[next_i].from == lt[((li + 1) % ln_i)].from);
                assert(spec_point_eq(combined[i].to, combined[next_i].from));
            } else if i == ln_i - 1 {
                assert(combined[i].to == er_to);
                assert(er_to == rt[best_ri].to);
                assert(spec_point_eq(rt[best_ri].to, rt[((best_ri + 1) % rn_i)].from));
                assert(next_i == ln_i);
                let m: int = 0;
                assert(combined[(ln_i + m)] == rt[((best_ri + 1 + m) % rn_i)]);
                assert(combined[next_i] == combined[(ln_i + 0)]);
                assert(combined[next_i].from == rt[((best_ri + 1) % rn_i)].from);
                assert(spec_point_eq(combined[i].to, combined[next_i].from));
            } else if i < ln_i + rn_i - 1 {
                let m = i - ln_i;
                assert(combined[(ln_i + m)] == rt[((best_ri + 1 + m) % rn_i)]);
                let ri = (best_ri + 1 + m) % rn_i;
                assert(spec_point_eq(rt[ri].to, rt[((ri + 1) % rn_i)].from));
                assert(combined[(ln_i + m)] == rt[ri]);
                if m < rn_i - 2 {
                    let m1 = m + 1;
                    assert(0 <= m1 && m1 < rn_i - 1);
                    assert(combined[(ln_i + m1)] == rt[((best_ri + 1 + m1) % rn_i)]);
                    lemma_small_mod(1, rn_i as nat);
                    lemma_add_mod_noop(best_ri + 1 + m, 1, rn_i);
                    assert((best_ri + 1 + m1) % rn_i == (ri + 1) % rn_i);
                    assert(combined[next_i] == rt[((ri + 1) % rn_i)]);
                } else {
                    assert(m == rn_i - 2);
                    assert(next_i == ln_i + rn_i - 1);
                    assert(combined[next_i] == (Edge { from: er_from, to: el_to }));
                    assert(er_from == rt[best_ri].from);
                    lemma_small_mod(1, rn_i as nat);
                    lemma_add_mod_noop(best_ri + rn_i - 1, 1, rn_i);
                    lemma_mod_multiples_vanish(1, best_ri, rn_i);
                    lemma_small_mod(best_ri as nat, rn_i as nat);
                    assert((ri + 1) % rn_i == best_ri);
                    assert(spec_point_eq(rt[ri].to, rt[best_ri].from));
                }
                assert(combined[i].to == rt[ri].to);
                assert(combined[next_i].from == rt[((ri + 1) % rn_i)].from);
                assert(spec_point_eq(combined[i].to, combined[next_i].from));
            } else {
                assert(i == ln_i + rn_i - 1);
                assert(combined[i].to == el_to);
                assert(el_to == lt[best_li].to);
                assert(spec_point_eq(lt[best_li].to, lt[((best_li + 1) % ln_i)].from));
                assert(next_i == 0);
                let k: int = 0;
                assert(combined[k] == lt[((best_li + 1 + k) % ln_i)]);
                assert(combined[next_i] == combined[0]);
                assert(combined[next_i].from == lt[((best_li + 1) % ln_i)].from);
                assert(spec_point_eq(combined[i].to, combined[next_i].from));
            }
        }
    }


    //		8. traits

    //		8. traits

    pub trait ETSPMtTrait {
        /// Solve the planar Euclidean TSP using parallel divide-and-conquer heuristic.
        /// Returns a tour as a sequence of directed edges forming a cycle through all points.
        /// Algorithm 26.7 with parallel recursive calls via help-first scheduler.
        /// - APAS: Work Θ(n²), Span Θ(lg² n) — Algorithm 26.7 with parallel recur + parallel minVal.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — O(n²) swap search dominates; could be reduced
        ///   to Θ(lg² n) with parallel reduce over edge pairs.
        fn etsp_parallel(points: &Vec<Point>) -> (tour: Vec<Edge>)
            requires
                points@.len() >= 2,
                points@.len() < usize::MAX / 2,
            ensures spec_etsp(tour@, points@);
    }


    //		9. impls

    impl Copy for Point {}

    impl Copy for Edge {}

    //		9. impls

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
                assert(spec_point_eq(tour@[0].from, points@[0]));
                assert(spec_point_eq(tour@[0].to, points@[1]));
                assert(spec_point_eq(tour@[1].from, points@[1]));
                assert(spec_point_eq(tour@[1].to, points@[0]));
                assert(spec_point_eq(tour@[0].to, tour@[1].from));
                assert(spec_point_eq(tour@[1].to, tour@[0].from));
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
                assert(spec_point_eq(tour@[0].from, points@[0]));
                assert(spec_point_eq(tour@[0].to, points@[1]));
                assert(spec_point_eq(tour@[1].from, points@[1]));
                assert(spec_point_eq(tour@[1].to, points@[2]));
                assert(spec_point_eq(tour@[2].from, points@[2]));
                assert(spec_point_eq(tour@[2].to, points@[0]));
                assert(spec_point_eq(tour@[0].to, tour@[1].from));
                assert(spec_point_eq(tour@[1].to, tour@[2].from));
                assert(spec_point_eq(tour@[2].to, tour@[0].from));
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
                assert(spec_point_in_seq(edge.from, left_pts_view));
                assert(spec_point_in_seq(edge.to, left_pts_view));
                lemma_edge_valid_transitive(edge, left_pts_view, points@);
            }
            combined.push(edge);
            i += 1;
        }

        // Bridge 1: left.from -> right.to
        proof {
            assert(spec_point_in_seq(el_from, left_pts_view));
            lemma_point_in_seq_transitive(el_from, left_pts_view, points@);
            assert(spec_point_in_seq(er_to, right_pts_view));
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
                assert(spec_point_in_seq(edge.from, right_pts_view));
                assert(spec_point_in_seq(edge.to, right_pts_view));
                lemma_edge_valid_transitive(edge, right_pts_view, points@);
            }
            combined.push(edge);
            j += 1;
        }

        // Bridge 2: right.from -> left.to
        proof {
            assert(spec_point_in_seq(er_from, right_pts_view));
            lemma_point_in_seq_transitive(er_from, right_pts_view, points@);
            assert(spec_point_in_seq(el_to, left_pts_view));
            lemma_point_in_seq_transitive(el_to, left_pts_view, points@);
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

    impl ETSPMtTrait for Vec<Point> {
        fn etsp_parallel(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_parallel_inner(points)
        }
    }

    //		10. external helpers (f64-dependent)

    /// Sort points by longest-spread dimension and split at median.
    /// Structural ensures: both halves are non-trivial and every point traces to the input.
    #[verifier::external_body]
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
        sort_and_split_impl(points)
    }

    /// Find the pair of edges (one from each tour) with minimum swap cost.
    #[verifier::external_body]
    pub fn find_best_swap(left_tour: &Vec<Edge>, right_tour: &Vec<Edge>) -> (swap_indices: (usize, usize))
        requires
            left_tour@.len() >= 2,
            right_tour@.len() >= 2,
        ensures
            (swap_indices.0 as int) < left_tour@.len(),
            (swap_indices.1 as int) < right_tour@.len(),
    {
        find_best_swap_impl(left_tour, right_tour)
    }


    //		11. derive impls in verus!

    impl Clone for Point {
        fn clone(&self) -> (cloned: Point)
            ensures cloned == *self
        { *self }
    }

    impl Clone for Edge {
        fn clone(&self) -> (cloned: Edge)
            ensures cloned == *self
        { *self }
    }

    } // verus!


    pub trait ETSPPointTrait {
        fn distance(&self, other: &Point) -> f64;
    }

    impl ETSPPointTrait for Point {
        fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
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

    /// Parallel find-best-swap: recursively splits the outer loop over left_tour
    /// and runs both halves in parallel via std::thread::scope.
    /// Work Θ(n·m), Span Θ(m·lg n) where n = left_tour.len(), m = right_tour.len().
    pub fn find_best_swap_impl(left_tour: &Vec<Edge>, right_tour: &Vec<Edge>) -> (usize, usize) {
        let (li, ri, _) = find_best_swap_par(left_tour, right_tour, 0, left_tour.len());
        (li, ri)
    }

    fn find_best_swap_par(
        left_tour: &Vec<Edge>, right_tour: &Vec<Edge>, lo: usize, hi: usize,
    ) -> (usize, usize, f64) {
        const THRESHOLD: usize = 16;
        if hi <= lo {
            return (0, 0, f64::MAX);
        }
        if hi - lo <= THRESHOLD {
            let mut best_cost = f64::MAX;
            let mut best_li = 0usize;
            let mut best_ri = 0usize;
            for li in lo..hi {
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
            (best_li, best_ri, best_cost)
        } else {
            let mid = lo + (hi - lo) / 2;
            std::thread::scope(|s| {
                let left_half = s.spawn(|| find_best_swap_par(left_tour, right_tour, lo, mid));
                let right_res = find_best_swap_par(left_tour, right_tour, mid, hi);
                let left_res = left_half.join().unwrap();
                if left_res.2 <= right_res.2 { left_res } else { right_res }
            })
        }
    }

} // mod
