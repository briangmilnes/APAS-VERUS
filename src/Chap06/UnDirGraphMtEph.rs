//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Edge filtering (ng) and vertex map-reduce (ng_of_vertices) are parallel.

pub mod UnDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash + 'static> {
        V: SetStEph<V>,
        E: SetStEph<Edge<V>>,
    }

    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash + 'static> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                        -> Self;
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(1)
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                -> &SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self)                                   -> &SetStEph<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeV(&self)                                   -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeE(&self)                                   -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V)                  -> B;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|) - parallel divide-and-conquer filter
        fn ng(&self, v: &V)                               -> SetStEph<V>;
        /// APAS: Work Θ(|u_set| × |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |E|), Span Θ(log |u_set| + log |E|), Parallelism Θ((|u_set| × |E|)/(log |u_set| + log |E|)) - parallel map-reduce
        fn ng_of_vertices(&self, u_set: &SetStEph<V>)       -> SetStEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V)            -> B;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|) - calls parallel ng
        fn degree(&self, v: &V)                           -> N;
    }

    impl<V: StT + MtT + Hash + 'static> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {
        fn empty() -> UnDirGraphMtEph<V> {
            UnDirGraphMtEph {
                V: SetLit![],
                E: SetLit![],
            }
        }
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> UnDirGraphMtEph<V> { UnDirGraphMtEph { V, E } }
        fn vertices(&self) -> &SetStEph<V> { &self.V }
        fn edges(&self) -> &SetStEph<Edge<V>> { &self.E }
        fn sizeV(&self) -> N { self.V.size() }
        fn sizeE(&self) -> N { self.E.size() }

        fn neighbor(&self, u: &V, v: &V) -> B {
            // Treat edges as unordered: {u,v}
            self.E.mem(&Edge(u.clone_plus(), v.clone_plus())) || self.E.mem(&Edge(v.clone_plus(), u.clone_plus()))
        }

        fn ng(&self, v: &V) -> SetStEph<V> {
            // PARALLEL: filter edges using divide-and-conquer
            let edges = self.E.iter().cloned().collect::<Vec<Edge<V>>>();

            // Parallel divide-and-conquer with proper base cases
            fn parallel_ng<V: StT + MtT + Hash + 'static>(edges: Vec<Edge<V>>, v: V) -> SetStEph<V> {
                let n = edges.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    let Edge(a, b) = &edges[0];
                    if a == &v {
                        let mut s = SetLit![];
                        s.insert(b.clone_plus());
                        return s;
                    } else if b == &v {
                        let mut s = SetLit![];
                        s.insert(a.clone_plus());
                        return s;
                    }
                    return SetLit![];
                }

                let mid = n / 2;
                let mut right_edges = edges;
                let left_edges = right_edges.split_off(mid);

                let v_left = v.clone_plus();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_ng(left_edges, v_left), move || parallel_ng(
                        right_edges,
                        v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_ng(edges, v.clone_plus())
        }

        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices = u_set.iter().cloned().collect::<Vec<V>>();

            // Parallel map-reduce with proper base cases
            fn parallel_ng_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: UnDirGraphMtEph<V>,
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

        fn incident(&self, e: &Edge<V>, v: &V) -> B { &e.0 == v || &e.1 == v }

        fn degree(&self, v: &V) -> N { self.ng(v).size() }
    }

    impl<V: StT + MtT + Hash + 'static> Debug for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("UnDirGraphMtEph")
                .field("V", &self.V)
                .field("E", &self.E)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash + 'static> Display for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.E) }
    }

    impl<V: StT + MtT + Hash + 'static> PartialEq for UnDirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }
    }
    impl<V: StT + MtT + Hash + 'static> Eq for UnDirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! UnDirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::from_sets(__V, __E)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::from_sets(__V, __E)
        }};
    }
}
