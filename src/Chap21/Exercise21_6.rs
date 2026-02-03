//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Exercise 21.6: Cost analysis of all contiguous subsequences.

pub mod Exercise21_6 {

    //! Exercise 21.6: Cost analysis of all contiguous subsequences
    //!
    //! This exercise analyzes the work and span complexity of the all_contiguous_subseqs function.
    //! The analysis shows that the nested tabulate + flatten approach achieves:
    //!
    //! Work: Θ(n²) - Each of the n² subsequences requires constant work to create
    //! Span: Θ(lg n) - The tabulate operations can be parallelized with logarithmic depth
    //!
    //! This is optimal for generating all contiguous subsequences since there are Θ(n²) of them.

    // This is a theoretical analysis exercise - no implementation code needed.
    // The cost analysis is documented in the module-level documentation above.
}
