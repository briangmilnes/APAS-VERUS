//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.6: Cost analysis of all contiguous subsequences.

pub mod Exercise21_6 {

    //! Exercise 21.6: Cost analysis of all contiguous subsequences.
    //!
    //! Work: Theta(n^2) — each of the n^2 subsequences requires constant work to create.
    //! Span: Theta(lg n) — the tabulate operations can be parallelized with logarithmic depth.
    //! Optimal for generating all contiguous subsequences since there are Theta(n^2) of them.

    use vstd::prelude::*;

    verus! {
    // Theoretical analysis exercise — no implementation code needed.
    } // verus!
}
