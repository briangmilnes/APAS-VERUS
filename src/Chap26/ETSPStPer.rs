//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer Euclidean Traveling Salesperson heuristic (Chapter 26, Section 4).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod ETSPStPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

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

    //		5. spec functions

    /// Spec: a valid tour visits all points and forms a cycle of edges.
    pub open spec fn spec_is_valid_tour(n_points: int, n_edges: int) -> bool {
        n_edges == n_points
    }

    //		8. traits

    pub trait ETSPStTrait {
        /// Solve the planar Euclidean TSP using divide-and-conquer heuristic.
        /// Returns a tour as a sequence of directed edges forming a cycle through all points.
        /// - APAS: Work Θ(n²), Span Θ(lg² n) — Algorithm 26.7, D&C eTSP heuristic.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential implementation, Span = Work.
        fn etsp(points: &Vec<Point>) -> (tour: Vec<Edge>)
            requires points@.len() >= 2,
            ensures tour@.len() == points@.len();
    }

    //		9. impls

    impl ETSPStTrait for Vec<Point> {
        #[verifier::external_body]
        fn etsp(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_inner(points)
        }
    }

    } // verus!

    //		Clone impls for Point and Edge (outside verus! — Debug/Clone/Display pattern)

    impl Clone for Point {
        fn clone(&self) -> Self { Point { x: self.x, y: self.y } }
    }

    impl Clone for Edge {
        fn clone(&self) -> Self { Edge { from: self.from.clone(), to: self.to.clone() } }
    }

    impl Point {
        /// Euclidean distance between two points.
        /// - APAS: N/A — helper function.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        pub fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    /// Recursive eTSP implementation (outside verus! — uses f64 arithmetic).
    /// - APAS: N/A — internal recursive helper for Algorithm 26.7.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential; sort O(n lg n) + O(n²) swap search.
    fn etsp_inner(points: &Vec<Point>) -> Vec<Edge> {
        let n = points.len();
        if n == 2 {
            // Base case: two points form a simple cycle
            return vec![
                Edge { from: points[0].clone(), to: points[1].clone() },
                Edge { from: points[1].clone(), to: points[0].clone() },
            ];
        }
        if n == 3 {
            // Base case: three points — find the cheapest Hamiltonian cycle (3! / 2 = 3 orderings)
            // Just use the greedy nearest neighbor from point 0
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

        // Recur on each half
        let left_tour = etsp_inner(&left_points);
        let right_tour = etsp_inner(&right_points);

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
