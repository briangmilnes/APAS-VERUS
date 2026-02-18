//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer Euclidean Traveling Salesperson heuristic — parallel (Chapter 26, Section 4).
//! Structural logic verified; threading via help-first scheduler join().
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod ETSPMtEph {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap26::ETSPStEph::ETSPStEph::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		4. spec functions

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
                (best_ri as int) < rn as int,
                combined@.len() == (ln as int - 1 + 1 + (j as int - 1)),
                spec_etsp(right_tour@, right_pts_view),
                forall|k: int| #![trigger right_pts_view[k]] 0 <= k < right_pts_view.len() ==>
                    spec_point_in_seq(right_pts_view[k], points@),
                spec_edges_valid(combined@, points@),
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
        }

        combined
    }

    impl ETSPMtTrait for Vec<Point> {
        fn etsp_parallel(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_parallel_inner(points)
        }
    }

    } // verus!

} // mod
