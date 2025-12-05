//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Arc filtering (n_plus, n_minus) and vertex map-reduce (ng_of_vertices, etc.) are parallel.

pub mod DirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct DirGraphMtEph<V: StT + MtT + Hash + 'static> {
        V: SetStEph<V>,
        A: SetStEph<Edge<V>>,
    }

    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash + 'static> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                        -> Self;
        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |A|), Span Θ(1)
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                -> &SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn arcs(&self)                                    -> &SetStEph<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeV(&self)                                   -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeA(&self)                                   -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V)                  -> B;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer in n_plus+n_minus
        fn ng(&self, v: &V)                               -> SetStEph<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|), Parallelism Θ((|u_set| × |A|)/(log |u_set| + log |A|)) - parallel map-reduce
        fn ng_of_vertices(&self, u_set: &SetStEph<V>)       -> SetStEph<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn n_plus(&self, v: &V)                            -> SetStEph<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn n_minus(&self, v: &V)                           -> SetStEph<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|), Parallelism Θ((|u_set| × |A|)/(log |u_set| + log |A|)) - parallel map-reduce
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>)    -> SetStEph<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|), Parallelism Θ((|u_set| × |A|)/(log |u_set| + log |A|)) - parallel map-reduce
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>)   -> SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V)            -> B;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - calls parallel in_degree + out_degree
        fn degree(&self, v: &V)                           -> N;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - calls parallel n_minus
        fn in_degree(&self, v: &V)                         -> N;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - calls parallel n_plus
        fn out_degree(&self, v: &V)                        -> N;
    }

    impl<V: StT + MtT + Hash + 'static> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {
        fn empty() -> DirGraphMtEph<V> {
            DirGraphMtEph {
                V: SetLit![],
                A: SetLit![],
            }
        }
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> DirGraphMtEph<V> { DirGraphMtEph { V, A } }
        fn vertices(&self) -> &SetStEph<V> { &self.V }
        fn arcs(&self) -> &SetStEph<Edge<V>> { &self.A }
        fn sizeV(&self) -> N { self.V.size() }
        fn sizeA(&self) -> N { self.A.size() }

        fn neighbor(&self, u: &V, v: &V) -> B {
            // Adjacent if there is an arc either way
            self.A.mem(&Edge(u.clone_mt(), v.clone_mt()))
        }

        fn ng(&self, v: &V) -> SetStEph<V> { self.n_plus(v).union(&self.n_minus(v)) }

        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices = u_set.iter().cloned().collect::<Vec<V>>();

            // Parallel map-reduce with proper base cases
            fn parallel_ng_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: DirGraphMtEph<V>,
            ) -> SetStEph<V> {
                let n = vertices.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    return graph.ng(&vertices[0]);
                }

                let mid = n / 2;
                let mut right_verts = vertices;
                let left_verts = right_verts.split_off(mid);

                let graph_left = graph.clone();
                let graph_right = graph;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_ng_of_vertices(left_verts, graph_left), move || {
                        parallel_ng_of_vertices(right_verts, graph_right)
                    });

                left_result.union(&right_result)
            }

            parallel_ng_of_vertices(vertices, self.clone())
        }

        fn n_plus(&self, v: &V) -> SetStEph<V> {
            // PARALLEL: filter arcs using divide-and-conquer
            let arcs = self.A.iter().cloned().collect::<Vec<Edge<V>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_nplus<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> SetStEph<V> {
                let n = arcs.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    let Edge(x, y) = &arcs[0];
                    return if x == &v {
                        let mut s = SetLit![];
                        s.insert(y.clone_mt());
                        s
                    } else {
                        SetLit![]
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_nplus(left_arcs, v_left), move || parallel_nplus(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_nplus(arcs, v.clone_mt())
        }

        fn n_minus(&self, v: &V) -> SetStEph<V> {
            // PARALLEL: filter arcs using divide-and-conquer
            let arcs = self.A.iter().cloned().collect::<Vec<Edge<V>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_nminus<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> SetStEph<V> {
                let n = arcs.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    let Edge(x, y) = &arcs[0];
                    return if y == &v {
                        let mut s = SetLit![];
                        s.insert(x.clone_mt());
                        s
                    } else {
                        SetLit![]
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) = ParaPair!(
                    move || parallel_nminus(left_arcs, v_left),
                    move || parallel_nminus(right_arcs, v_right)
                );

                left_result.union(&right_result)
            }

            parallel_nminus(arcs, v.clone_mt())
        }

        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices = u_set.iter().cloned().collect::<Vec<V>>();

            // Parallel map-reduce with proper base cases
            fn parallel_nplus_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: DirGraphMtEph<V>,
            ) -> SetStEph<V> {
                let n = vertices.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    return graph.n_plus(&vertices[0]);
                }

                let mid = n / 2;
                let mut right_verts = vertices;
                let left_verts = right_verts.split_off(mid);

                let graph_left = graph.clone();
                let graph_right = graph;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_nplus_of_vertices(left_verts, graph_left), move || {
                        parallel_nplus_of_vertices(right_verts, graph_right)
                    });

                left_result.union(&right_result)
            }

            parallel_nplus_of_vertices(vertices, self.clone())
        }

        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices = u_set.iter().cloned().collect::<Vec<V>>();

            // Parallel map-reduce with proper base cases
            fn parallel_nminus_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: DirGraphMtEph<V>,
            ) -> SetStEph<V> {
                let n = vertices.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    return graph.n_minus(&vertices[0]);
                }

                let mid = n / 2;
                let mut right_verts = vertices;
                let left_verts = right_verts.split_off(mid);

                let graph_left = graph.clone();
                let graph_right = graph;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_nminus_of_vertices(left_verts, graph_left), move || {
                        parallel_nminus_of_vertices(right_verts, graph_right)
                    });

                left_result.union(&right_result)
            }

            parallel_nminus_of_vertices(vertices, self.clone())
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> B { &e.0 == v || &e.1 == v }

        fn degree(&self, v: &V) -> N { self.in_degree(v) + self.out_degree(v) }
        fn in_degree(&self, v: &V) -> N { self.n_minus(v).size() }
        fn out_degree(&self, v: &V) -> N { self.n_plus(v).size() }
    }

    impl<V: StT + MtT + Hash + 'static> Debug for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DirGraphMtEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash + 'static> Display for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<V: StT + MtT + Hash + 'static> PartialEq for DirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }

    impl<V: StT + MtT + Hash + 'static> Eq for DirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! DirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __A: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::from_sets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::from_sets(__V, __A)
        }}}
}
