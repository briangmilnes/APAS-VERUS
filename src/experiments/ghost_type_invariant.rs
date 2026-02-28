//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Experiment: Type invariants on ghost/spec graph structures.
//!
//! RESULT: FAILS. Type invariants require private fields, but then:
//! - `pub open spec fn` cannot use constructors or field access
//! - The struct becomes "opaque" outside the module
//! - Even though Verus controls ghost struct creation, it won't allow
//!   `pub open` functions to expose the constructor pattern
//!
//! Error: "disallowed: constructor for an opaque datatype"
//!
//! The conflict: type_invariant needs private fields, but `pub open spec fn`
//! needs to be well-formed "everywhere" which is wider than the module scope.
//!
//! Conclusion: Use explicit `wf_graph` predicates instead of type_invariant
//! for ghost structures that need `pub open spec fn` constructors/accessors.

pub mod ghost_type_invariant {

    use vstd::prelude::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

        /// A ghost directed graph with a type invariant enforcing well-formedness.
        #[verifier::reject_recursive_types(V)]
        pub ghost struct GhostDirGraph<V> {
            vertices: Set<V>,  // Must be private for type_invariant
            arcs: Set<(V, V)>, // Must be private for type_invariant
        }

        impl<V> GhostDirGraph<V> {
            /// Well-formedness invariant.
            #[verifier::type_invariant]
            spec fn wf(&self) -> bool {
                &&& self.vertices.finite()
                &&& self.arcs.finite()
                &&& forall|u: V, v: V| self.arcs.contains((u, v)) ==> 
                        self.vertices.contains(u) && self.vertices.contains(v)
            }

            /// Access the vertex set (closed - can't be open with private fields).
            pub open spec fn V(self) -> Set<V> {
                self.vertices
            }

            /// Access the arc set (closed - can't be open with private fields).
            pub open spec fn A(self) -> Set<(V, V)> {
                self.arcs
            }

            /// FAILS: pub open spec fn cannot use constructor for opaque datatype.
            /// Even though Verus controls ghost creation, this is disallowed.
            pub open spec fn empty() -> Self {
                GhostDirGraph { vertices: Set::empty(), arcs: Set::empty() }
            }

            /// FAILS: Same issue - constructor in pub open spec fn.
            pub open spec fn add_vertex(self, v: V) -> Self {
                GhostDirGraph { vertices: self.vertices.insert(v), ..self }
            }

            /// FAILS: Same issue.
            pub open spec fn add_arc(self, u: V, v: V) -> Self
                recommends self.vertices.contains(u), self.vertices.contains(v)
            {
                GhostDirGraph { arcs: self.arcs.insert((u, v)), ..self }
            }

            /// FAILS: Uses private field in pub open spec fn.
            pub open spec fn n_plus(self, v: V) -> Set<V> {
                Set::new(|w: V| self.arcs.contains((v, w)))
            }

            /// FAILS: Uses private field in pub open spec fn.
            pub open spec fn n_minus(self, v: V) -> Set<V> {
                Set::new(|u: V| self.arcs.contains((u, v)))
            }
        }

        proof fn test_basic() {
            let g: GhostDirGraph<int> = GhostDirGraph::empty();
            let g = g.add_vertex(1).add_vertex(2).add_arc(1, 2);
            assert(g.V().contains(1));
        }

    } // verus!

} // mod
