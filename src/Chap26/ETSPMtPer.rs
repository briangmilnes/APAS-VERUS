//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer Euclidean Traveling Salesperson heuristic — parallel (Chapter 26, Section 4).
//! Uses the help-first scheduler for fork-join parallelism on recursive calls.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod ETSPMtPer {

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap26::ETSPStPer::ETSPStPer::*;

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
            requires points@.len() >= 2,
            ensures tour@.len() == points@.len();
    }

    //		9. impls

    impl ETSPMtTrait for Vec<Point> {
        #[verifier::external_body]
        fn etsp_parallel(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_parallel_inner(points)
        }
    }

    } // verus!

    //		Parallel recursive eTSP implementation (outside verus! — uses f64 arithmetic + ParaPair!).

    use crate::Chap26::ETSPStPer::ETSPStPer::*;
    use crate::Types::Types::Pair;

    /// Recursive parallel eTSP implementation.
    /// - APAS: Algorithm 26.7 — parallel recursive calls, parallel minVal over edge pairs.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — fork-join on recursive calls,
    ///   but O(n²) swap search is sequential. Span would be Θ(lg² n) with parallel reduce.
    fn etsp_parallel_inner(points: &Vec<Point>) -> Vec<Edge> {
        let n = points.len();
        if n == 2 {
            return vec![
                Edge { from: points[0].clone(), to: points[1].clone() },
                Edge { from: points[1].clone(), to: points[0].clone() },
            ];
        }
        if n == 3 {
            let (a, b, c) = (&points[0], &points[1], &points[2]);
            return vec![
                Edge { from: a.clone(), to: b.clone() },
                Edge { from: b.clone(), to: c.clone() },
                Edge { from: c.clone(), to: a.clone() },
            ];
        }

        // Find the dimension with the largest spread
        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (points[0].x, points[0].x, points[0].y, points[0].y);
        for i in 1..n {
            if points[i].x < min_x { min_x = points[i].x; }
            if points[i].x > max_x { max_x = points[i].x; }
            if points[i].y < min_y { min_y = points[i].y; }
            if points[i].y > max_y { max_y = points[i].y; }
        }
        let split_on_x = (max_x - min_x) >= (max_y - min_y);

        // Sort points by the chosen dimension and split at median
        let mut sorted_points = points.clone();
        if split_on_x {
            sorted_points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            sorted_points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal));
        }

        // Split at median, ensuring both halves have at least 2 points
        let mid = (n / 2).max(2);
        let left_points: Vec<Point> = sorted_points[..mid].to_vec();
        let right_points: Vec<Point> = sorted_points[mid..].to_vec();

        // Parallel recursive calls via help-first scheduler
        let left_arc = Arc::new(left_points);
        let right_arc = Arc::new(right_points);

        let Pair(left_tour, right_tour) = crate::ParaPair!(
            {
                let lp = left_arc.clone();
                move || etsp_parallel_inner(&*lp)
            },
            {
                let rp = right_arc.clone();
                move || etsp_parallel_inner(&*rp)
            }
        );

        // Find best swap: try all pairs of edges (one from each tour)
        let mut best_cost = f64::MAX;
        let mut best_li = 0usize;
        let mut best_ri = 0usize;

        for li in 0..left_tour.len() {
            for ri in 0..right_tour.len() {
                let el = &left_tour[li];
                let er = &right_tour[ri];
                // swapCost = d(u_l, v_r) + d(u_r, v_l) - d(u_l, v_l) - d(u_r, v_r)
                let cost = el.from.distance(&er.to) + er.from.distance(&el.to)
                         - el.from.distance(&el.to) - er.from.distance(&er.to);
                if cost < best_cost {
                    best_cost = cost;
                    best_li = li;
                    best_ri = ri;
                }
            }
        }

        // Combine tours by swapping the best pair of edges
        let el = &left_tour[best_li];
        let er = &right_tour[best_ri];
        let ln = left_tour.len();
        let rn = right_tour.len();

        let mut combined: Vec<Edge> = Vec::with_capacity(ln + rn);

        // Left side: edges after best_li (wrapping), up to but not including best_li
        for i in 1..ln {
            combined.push(left_tour[(best_li + i) % ln].clone());
        }

        // Bridge from left to right: el.from -> er.to
        combined.push(Edge { from: el.from.clone(), to: er.to.clone() });

        // Right side: edges after best_ri (wrapping)
        for i in 1..rn {
            combined.push(right_tour[(best_ri + i) % rn].clone());
        }

        // Bridge from right to left: er.from -> el.to
        combined.push(Edge { from: er.from.clone(), to: el.to.clone() });

        combined
    }

} // mod
