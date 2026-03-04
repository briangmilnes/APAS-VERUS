// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0
//! Module Standard: how to structure an APAS-VERUS module file.
//!
//! Every APAS-VERUS source file wraps its content in a `pub mod ModuleName` block.
//! The module name matches the file name (without extension). Inside the module,
//! imports come first, then `verus!` wraps sections 2-11. Sections 12-13 go
//! outside `verus!` but inside the module.
//!
//! Table of Contents sections:
//!  1. module          — `pub mod Name {`
//!  2. imports         — `use` statements inside verus!
//!  3. broadcast use   — broadcast group imports
//!  4. type definitions
//!  5. view impls
//!  6. spec fns
//!  7. proof fns / broadcast groups
//!  8. traits
//!  9. impls
//! 10. iterators
//! 11. derive impls in verus!
//! 12. macros          — outside verus!, inside module
//! 13. derive impls outside verus!
//!
//! Reference: src/Chap18/ArraySeqStEph.rs
// 1. module
pub mod mod_standard {

    use vstd::prelude::*;

    verus! {

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    // 5. view impls
    /// View maps Vec<T> to Seq<T> (the abstract sequence).
    impl<T> View for ExampleS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    // 6. spec fns
    /// Well-formedness predicate (named spec_<module>_wf per convention).
    pub open spec fn spec_modstandard_wf<T>(s: &ExampleS<T>) -> bool {
        s@.len() >= 0
    }

    // 8. traits
    pub trait ExampleTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;

        fn length(&self) -> (len: usize)
            ensures
                len as nat == self.spec_len(),
        ;
    }

    // 9. impls
    impl<T> ExampleTrait<T> for ExampleS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
        }

        fn new(length: usize, init: T) -> (s: Self) where T: Copy {
            let mut v: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    v@.len() == i as int,
                decreases length - i,
            {
                v.push(init);
                i = i + 1;
            }
            ExampleS { seq: v }
        }

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }
    }

    } // verus!
      // 13. derive impls outside verus!
      //
      // Debug, Display, macro_rules!, unsafe marker traits go here.
      // They live inside the pub mod but outside verus!.
} // pub mod mod_standard
