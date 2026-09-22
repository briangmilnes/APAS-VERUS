// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r211): the manual `loop { match it.next() { .. } }` of
//! prophetic_manual_loop_next.rs with the two index-wise sequence invariants
//! replaced by one `skip` clause,
//!     IteratorSpec::remaining(&it).unref() == orig.skip(pos),
//! and an extensional `=~=` re-establishment in the `Some` arm.
//!
//! RESULT: FAILS — 4 errors as written (verus 0.2026.09.13.671956e): the
//! invariant `0 <= pos <= orig.len()` is not re-established, the `Some`-arm
//! asserts `orig[old_pos] == *x` and `remaining(&it).unref() =~= orig.skip(pos)`
//! fail, and the `None`-arm `pos == orig.len()` fails. Z3 does not relate
//! `orig.skip(pos).len()` and `orig.skip(pos)[i]` to `orig` without
//! `lemma_seq_skip_len`/`lemma_seq_skip_index` (vstd/seq_lib.rs:3706, :3732),
//! which are not broadcast by default. The index-wise form needs no seq
//! lemmas and is the one adopted.
//! DATE: 2026-09-21
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-prophetic_manual_loop_next.20260921-145824.log
//!      (as Form 2 of the first run of prophetic_manual_loop_next.rs);
//!      LOG: logs/validate-standard-prophetic_manual_loop_next_skip.20260921-145939.log
//!      (2 verified, 4 errors, from this path)

pub mod prophetic_manual_loop_next_skip {

    use vstd::prelude::*;
    use vstd::std_specs::iter::*;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct ExampleS<T> {
        pub seq: Vec<T>,
    }

    impl<T> ExampleS<T> {
        pub fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.seq.iter()
        }
    }

    // Form 2: the two sequence invariants as one `skip` clause.
    fn test_loop_borrow_next_skip(a: &ExampleS<u64>) {
        let ghost orig: Seq<u64> = a.seq@;
        let mut collected: Vec<u64> = Vec::new();
        let mut it = a.iter();
        let ghost mut pos: int = 0;
        loop
            invariant
                IteratorSpec::obeys_prophetic_iter_laws(&it),
                IteratorSpec::decrease(&it) is Some,
                0 <= pos <= orig.len(),
                IteratorSpec::remaining(&it).unref() == orig.skip(pos),
                collected.len() == pos,
                forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == orig[i],
            decreases IteratorSpec::decrease(&it)->0,
        {
            let ghost old_pos = pos;
            match it.next() {
                Some(x) => {
                    proof {
                        pos = pos + 1;
                        assert(orig[old_pos] == *x);
                        assert(IteratorSpec::remaining(&it).unref() =~= orig.skip(pos));
                    }
                    collected.push(*x);
                },
                None => {
                    assert(pos == orig.len());
                    assert(collected@ =~= orig);
                    break;
                },
            }
        }
    }

    } // verus!
} // pub mod prophetic_manual_loop_next_skip
