//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer Euclidean Traveling Salesperson heuristic (Chapter 26, Section 4).
//! Verusified: structural properties (no fabrication) verified; f64 arithmetic external.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. spec functions
//	7. proof fns
//	8. traits
//	9. impls
//	10. external helpers (f64-dependent)
//	13. derive impls outside verus!

//		1. module

pub mod ETSPStEph {

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

    impl Copy for Point {}
    impl Clone for Point {
        fn clone(&self) -> (r: Point)
            ensures r == *self
        { *self }
    }

    impl Copy for Edge {}
    impl Clone for Edge {
        fn clone(&self) -> (r: Edge)
            ensures r == *self
        { *self }
    }

    //		5. spec functions

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

    /// Bundle: the tour has the right length and every edge endpoint is an input point.
    pub open spec fn spec_etsp(tour: Seq<Edge>, points: Seq<Point>) -> bool {
        tour.len() == points.len()
        && spec_sources_valid(tour, points)
        && spec_targets_valid(tour, points)
    }

    /// Every element of combined so far has from/to in points.
    pub open spec fn spec_edges_valid(edges: Seq<Edge>, points: Seq<Point>) -> bool {
        forall|k: int| #![trigger edges[k]] 0 <= k < edges.len() ==>
            spec_point_in_seq(edges[k].from, points)
            && spec_point_in_seq(edges[k].to, points)
    }

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

    //		8. traits

    pub trait ETSPStTrait {
        /// Solve the planar Euclidean TSP using divide-and-conquer heuristic.
        /// Returns a tour as a sequence of directed edges forming a cycle through all points.
        /// - APAS: Work Θ(n²), Span Θ(lg² n) — Algorithm 26.7, D&C eTSP heuristic.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential implementation, Span = Work.
        fn etsp(points: &Vec<Point>) -> (tour: Vec<Edge>)
            requires
                points@.len() >= 2,
                points@.len() < usize::MAX / 2,
            ensures spec_etsp(tour@, points@);
    }

    //		9. impls

    /// Verified eTSP implementation. The base cases are fully proven. The recursive
    /// case delegates f64-dependent work (sort, swap search) to external_body helpers
    /// and verifies the structural combination.
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
                (best_ri as int) < rn as int,
                combined@.len() == (ln as int - 1 + 1 + (j as int - 1)),
                spec_etsp(right_tour@, right_points@),
                forall|k: int| #![trigger right_points@[k]] 0 <= k < right_points@.len() ==>
                    spec_point_in_seq(right_points@[k], points@),
                spec_edges_valid(combined@, points@),
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
        }

        combined
    }

    impl ETSPStTrait for Vec<Point> {
        fn etsp(points: &Vec<Point>) -> (tour: Vec<Edge>) {
            etsp_inner(points)
        }
    }

    //		10. external helpers (f64-dependent)

    /// Sort points by longest-spread dimension and split at median.
    /// Structural ensures: both halves are non-trivial and every point traces to the input.
    #[verifier::external_body]
    pub fn sort_and_split(points: &Vec<Point>) -> (result: (Vec<Point>, Vec<Point>))
        requires points@.len() >= 4,
        ensures
            result.0@.len() >= 2,
            result.1@.len() >= 2,
            result.0@.len() + result.1@.len() == points@.len(),
            result.0@.len() < points@.len(),
            result.1@.len() < points@.len(),
            forall|i: int| #![trigger result.0@[i]] 0 <= i < result.0@.len() ==>
                spec_point_in_seq(result.0@[i], points@),
            forall|i: int| #![trigger result.1@[i]] 0 <= i < result.1@.len() ==>
                spec_point_in_seq(result.1@[i], points@),
    {
        sort_and_split_impl(points)
    }

    /// Find the pair of edges (one from each tour) with minimum swap cost.
    #[verifier::external_body]
    pub fn find_best_swap(left_tour: &Vec<Edge>, right_tour: &Vec<Edge>) -> (result: (usize, usize))
        requires
            left_tour@.len() >= 2,
            right_tour@.len() >= 2,
        ensures
            (result.0 as int) < left_tour@.len(),
            (result.1 as int) < right_tour@.len(),
    {
        find_best_swap_impl(left_tour, right_tour)
    }

    } // verus!

    //		13. derive impls outside verus!

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

} // mod
