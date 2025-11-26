//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled arc filtering (out_neighbors, in_neighbors) are parallel.

pub mod LabDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabDirGraphMtEph<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static> {
        vertices: SetStEph<V>,
        labeled_arcs: SetStEph<LabEdge<V, L>>,
    }

    pub trait LabDirGraphMtEphTrait<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                                                                      -> Self;
        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |A|), Span Θ(|V| + |A|), Parallelism Θ(1) - sequential
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn vertices(&self)                                                                              -> &SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn labeled_arcs(&self)                                                                          -> &SetStEph<LabEdge<V, L>>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn arcs(&self)                                                                                  -> SetStEph<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_vertex(&mut self, v: V);
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_arc_label(&self, from: &V, to: &V)                                                       -> Option<&L>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn has_arc(&self, from: &V, to: &V)                                                             -> bool;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn out_neighbors(&self, v: &V)                                                                  -> SetStEph<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn in_neighbors(&self, v: &V)                                                                   -> SetStEph<V>;
    }

    impl<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static> LabDirGraphMtEphTrait<V, L>
        for LabDirGraphMtEph<V, L>
    {
        fn empty() -> Self {
            LabDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_arcs: SetStEph::empty(),
            }
        }

        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> Self {
            LabDirGraphMtEph { vertices, labeled_arcs }
        }

        fn vertices(&self) -> &SetStEph<V> { &self.vertices }

        fn labeled_arcs(&self) -> &SetStEph<LabEdge<V, L>> { &self.labeled_arcs }

        fn arcs(&self) -> SetStEph<Edge<V>> {
            let mut arcs = SetStEph::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                arcs.insert(Edge(labeled_arc.0.clone_mt(), labeled_arc.1.clone_mt()));
            }
            arcs
        }

        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            self.vertices.insert(from.clone_mt());
            self.vertices.insert(to.clone_mt());
            self.labeled_arcs.insert(LabEdge(from, to, label));
        }

        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *from && labeled_arc.1 == *to {
                    return Some(&labeled_arc.2);
                }
            }
            None
        }

        fn has_arc(&self, from: &V, to: &V) -> bool {
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *from && labeled_arc.1 == *to {
                    return true;
                }
            }
            false
        }

        fn out_neighbors(&self, v: &V) -> SetStEph<V> {
            // PARALLEL: filter labeled arcs using divide-and-conquer
            let arcs = self.labeled_arcs.iter().cloned().collect::<Vec<LabEdge<V, L>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_out<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
                arcs: Vec<LabEdge<V, L>>,
                v: V,
            ) -> SetStEph<V> {
                let n = arcs.len();
                if n == 0 {
                    return SetStEph::empty();
                }
                if n == 1 {
                    return if arcs[0].0 == v {
                        let mut s = SetStEph::empty();
                        s.insert(arcs[0].1.clone_mt());
                        s
                    } else {
                        SetStEph::empty()
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_out(left_arcs, v_left), move || parallel_out(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_out(arcs, v.clone_mt())
        }

        fn in_neighbors(&self, v: &V) -> SetStEph<V> {
            // PARALLEL: filter labeled arcs using divide-and-conquer
            let arcs = self.labeled_arcs.iter().cloned().collect::<Vec<LabEdge<V, L>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_in<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
                arcs: Vec<LabEdge<V, L>>,
                v: V,
            ) -> SetStEph<V> {
                let n = arcs.len();
                if n == 0 {
                    return SetStEph::empty();
                }
                if n == 1 {
                    return if arcs[0].1 == v {
                        let mut s = SetStEph::empty();
                        s.insert(arcs[0].0.clone_mt());
                        s
                    } else {
                        SetStEph::empty()
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_in(left_arcs, v_left), move || parallel_in(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_in(arcs, v.clone_mt())
        }
    }

    impl<V: StT + MtT + Hash, L: StTInMtT + Hash> Display for LabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V: StT + MtT + Hash, L: StTInMtT + Hash> Debug for LabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabDirGraph {{ vertices: {:?}, labeled_arcs: {:?} }}",
                self.vertices, self.labeled_arcs
            )
        }
    }

    #[macro_export]
    macro_rules! LabDirGraphMtEphLit {
        () => {{
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ($from:expr, $to:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_arcs = $crate::SetLit![ $( $crate::Types::Types::LabEdge($from, $to, $label) ),* ];
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }};
    }
}
