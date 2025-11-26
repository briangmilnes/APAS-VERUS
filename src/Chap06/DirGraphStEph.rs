//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

pub mod DirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;

    verus! {

    // Broadcast groups for hash collections, sets, and our custom axioms
    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
    };

    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphStEph<V: StT + Hash> {
        V: SetStEph<V>,
        A: SetStEph<Edge<V>>,
    }

    impl<V: StT + Hash> DirGraphStEph<V> {
        /// Returns an iterator over the vertices
        pub fn iter_vertices(&self) -> (it: SetStEphIter<V>)
            requires valid_key_type_Edge::<V>()
        {
            self.V.iter()
        }

        /// Returns an iterator over the arcs
        pub fn iter_arcs(&self) -> (it: SetStEphIter<Edge<V>>)
            requires valid_key_type_Edge::<V>()
        {
            self.A.iter()
        }
    }

    // Ghost view for graph vertex iterator: (visited, current, remaining) sets
    #[verifier::reject_recursive_types(V)]
    pub ghost struct DirGraphVertexIterView<V> {
        pub visited: Set<V>,
        pub current: Option<V>,
        pub remaining: Set<V>,
    }

    // Ghost view for graph arc iterator: (visited, current, remaining) sets  
    #[verifier::reject_recursive_types(V)]
    pub ghost struct DirGraphArcIterView<V> {
        pub visited: Set<(V, V)>,
        pub current: Option<(V, V)>,
        pub remaining: Set<(V, V)>,
    }

    // Helper: convert Option<V> to Set<V>
    pub open spec fn option_to_set<V>(opt: Option<V>) -> Set<V> {
        match opt {
            None => Set::empty(),
            Some(v) => Set::empty().insert(v),
        }
    }

    // View type: (vertices, arcs) as spec sets
    pub trait DirGraphStEphTrait<V: StT + Hash>: 
        View<V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V)>)> + Sized {
        
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: DirGraphStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures g@.0 =~= Set::<<V as View>::V>::empty(),
                    g@.1 =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        fn FromSets(vertices: SetStEph<V>, arcs: SetStEph<Edge<V>>) -> (g: DirGraphStEph<V>)
            ensures g@.0 =~= vertices@,
                    g@.1 =~= arcs@.map(|e: (V::V, V::V)| e);

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.0;

        /// APAS: Work Θ(1), Span Θ(1)
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>)
            ensures a@.map(|e: (V::V, V::V)| e) =~= self@.1;

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.0.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeA(&self) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.1.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn Neighbor(&self, u: &V, v: &V) -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == self@.1.contains((u@, v@));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn NG(&self, v: &V) -> SetStEph<V>
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NGOfVertices(&self, u_set: &SetStEph<V>) -> SetStEph<V>
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn NPlus(&self, v: &V) -> SetStEph<V>
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn NMinus(&self, v: &V) -> SetStEph<V>
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NPlusOfVertices(&self, u_set: &SetStEph<V>) -> SetStEph<V>
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NMinusOfVertices(&self, u_set: &SetStEph<V>) -> SetStEph<V>
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(1), Span Θ(1)
        fn Incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn Degree(&self, v: &V) -> N
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn InDegree(&self, v: &V) -> (n: N)
            requires valid_key_type_Edge::<V>();

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn OutDegree(&self, v: &V) -> (n: N)
            requires valid_key_type_Edge::<V>();
    }

    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {
        closed spec fn spec_vertices(&self) -> Set<V::V> {
            self.V@
        }

        closed spec fn spec_arcs(&self) -> Set<(V::V, V::V)> {
            self.A@
        }

        fn empty() -> (g: DirGraphStEph<V>) {
            DirGraphStEph {
                V: SetStEph::empty(),
                A: SetStEph::empty(),
            }
        }

        fn FromSets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }

        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self.spec_vertices()
        { &self.V }

        fn arcs(&self) -> (a: &SetStEph<Edge<V>>)
            ensures a@ == self.spec_arcs()
        { &self.A }

        fn sizeV(&self) -> (n: N)
            ensures n == self.spec_vertices().len()
        { self.V.size() }

        fn sizeA(&self) -> (n: N)
            ensures n == self.spec_arcs().len()
        { self.A.size() }

        fn Neighbor(&self, u: &V, v: &V) -> (b: B)
            ensures b == self.spec_neighbor(u@, v@)
        {
            // Adjacent if there is an arc either way
            self.A.mem(&Edge(u.clone_plus(), v.clone_plus()))
        }

        fn NG(&self, v: &V) -> (result: SetStEph<V>)
            ensures result@ == self.spec_ng(v@)
        { self.NPlus(v).union(&self.NMinus(v)) }

        #[verifier::external_body]
        fn NGOfVertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            let mut result: SetStEph<V> = SetStEph::empty();
            for u in u_set.iter() {
                let ng_u = self.NG(u);
                result = result.union(&ng_u);
            }
            result
        }

        #[verifier::external_body]
        fn NPlus(&self, v: &V) -> (result: SetStEph<V>)
            ensures result@ == self.spec_nplus(v@)
        {
            let mut out: SetStEph<V> = SetStEph::empty();
            for edge in self.A.iter() {
                let x = edge.0.clone_plus();
                let y = edge.1.clone_plus();
                if feq(&x, v) {
                    let _ = out.insert(y);
                }
            }
            out
        }

        #[verifier::external_body]
        fn NMinus(&self, v: &V) -> (result: SetStEph<V>)
            ensures result@ == self.spec_nminus(v@)
        {
            let mut inn: SetStEph<V> = SetStEph::empty();
            for edge in self.A.iter() {
                let x = edge.0.clone_plus();
                let y = edge.1.clone_plus();
                if feq(&y, v) {
                    let _ = inn.insert(x);
                }
            }
            inn
        }

        #[verifier::external_body]
        fn NPlusOfVertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            let mut result: SetStEph<V> = SetStEph::empty();
            for u in u_set.iter() {
                let plus_u = self.NPlus(u);
                result = result.union(&plus_u);
            }
            result
        }

        #[verifier::external_body]
        fn NMinusOfVertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            let mut result: SetStEph<V> = SetStEph::empty();
            for u in u_set.iter() {
                let minus_u = self.NMinus(u);
                result = result.union(&minus_u);
            }
            result
        }

        fn Incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            ensures b == self.spec_incident(e@, v@)
        { feq(&e.0, v) || feq(&e.1, v) }

        #[verifier::external_body]  // overflow check on addition
        fn Degree(&self, v: &V) -> N { self.InDegree(v) + self.OutDegree(v) }

        fn InDegree(&self, v: &V) -> (n: N)
            ensures n == self.spec_nminus(v@).len()
        { self.NMinus(v).size() }

        fn OutDegree(&self, v: &V) -> (n: N)
            ensures n == self.spec_nplus(v@).len()
        { self.NPlus(v).size() }
    }

    } // verus!

    // Clone implementation (outside verus! block)
    impl<V: StT + Hash> Clone for DirGraphStEph<V> {
        fn clone(&self) -> Self {
            DirGraphStEph { V: self.V.clone(), A: self.A.clone() }
        }
    }

    impl<V: StT + Hash> Debug for DirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DirGraphStEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StT + Hash> Display for DirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }

    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}

    // Macro defined outside verus! block
    #[macro_export]
    macro_rules! DirGraphStEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __A: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::FromSets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::FromSets(__V, __A)
        }}
    }
}
