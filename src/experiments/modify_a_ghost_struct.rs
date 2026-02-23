//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Experiment: Can we modify a ghost struct?
//!
//! RESULT: YES. Ghost structs can be:
//! - Created in proof fns
//! - Updated via functional update (`GhostGraph { field: new_val, ..old }`)
//! - Reassigned to mutable ghost variables (`let mut g; g = ...`)
//! - Modified via direct field assignment (`g.field = new_val`)
//! - Wrapped in `Ghost<T>` for use in exec fns
//! - Modified inside `proof { }` blocks when wrapped in `Ghost<T>`

pub mod modify_a_ghost_struct {

    use vstd::prelude::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

        /// A simple ghost struct with public fields.
        #[verifier::reject_recursive_types(V)]
        pub ghost struct GhostGraph<V> {
            pub vertices: Set<V>,
            pub arcs: Set<(V, V)>,
        }

        /// Test: Can we create a ghost struct in a proof fn?
        proof fn test_create_ghost() {
            let g: GhostGraph<int> = GhostGraph { 
                vertices: Set::empty(), 
                arcs: Set::empty() 
            };
            assert(g.vertices.len() == 0);
        }

        /// Test: Can we modify a ghost struct via functional update?
        proof fn test_functional_update() {
            let g1: GhostGraph<int> = GhostGraph { 
                vertices: Set::empty(), 
                arcs: Set::empty() 
            };
            let g2 = GhostGraph { vertices: g1.vertices.insert(1), ..g1 };
            assert(g2.vertices.contains(1));
            assert(!g1.vertices.contains(1)); // g1 unchanged
        }

        /// Test: Can we use a mutable ghost variable?
        proof fn test_mutable_ghost_var() {
            let mut g: GhostGraph<int> = GhostGraph { 
                vertices: Set::empty(), 
                arcs: Set::empty() 
            };
            // Try to reassign
            g = GhostGraph { vertices: g.vertices.insert(1), ..g };
            assert(g.vertices.contains(1));
            
            // Try again
            g = GhostGraph { vertices: g.vertices.insert(2), ..g };
            assert(g.vertices.contains(1));
            assert(g.vertices.contains(2));
        }

        /// Test: Can we modify fields directly?
        proof fn test_direct_field_modify() {
            let mut g: GhostGraph<int> = GhostGraph { 
                vertices: Set::empty(), 
                arcs: Set::empty() 
            };
            // Try direct field assignment
            g.vertices = g.vertices.insert(1);
            assert(g.vertices.contains(1));
        }

        /// Test: Ghost struct in exec function via Ghost<T>.
        fn test_ghost_in_exec() {
            let g: Ghost<GhostGraph<int>> = Ghost(GhostGraph { 
                vertices: Set::empty(), 
                arcs: Set::empty() 
            });
            proof {
                assert(g@.vertices.len() == 0);
            }
        }

        /// Test: Modify Ghost<T> in exec function.
        fn test_modify_ghost_in_exec() {
            let mut g: Ghost<GhostGraph<int>> = Ghost(GhostGraph { 
                vertices: Set::empty(), 
                arcs: Set::empty() 
            });
            proof {
                g@ = GhostGraph { vertices: g@.vertices.insert(1), ..g@ };
            }
            proof {
                assert(g@.vertices.contains(1));
            }
        }

    } // verus!

} // mod
