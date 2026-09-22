// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Iterator PTT Standard: required proof time tests for collection iterators.
//!
//! Every APAS-VERUS collection that implements `iter()` must have a corresponding
//! PTT file in `rust_verify_test/tests/ChapNN/` that proves all 6 iterator loop
//! patterns verify correctly. This standard defines the required patterns, naming
//! conventions, and the coverage inventory.
//!
//! ## Why PTTs for iterators?
//!
//! Under the verus 0.2026.09.13 prophetic iterator model (see
//! `iterators_standard.rs` and `prophetic_iterators_standard.rs`) a collection's
//! `iter()` carries three postconditions (`IteratorSpec::remaining`, the
//! non-prophetic contents `peek` reads, `IteratorSpec::decrease is Some`), and a
//! custom iterator type implements the five `IteratorSpecImpl` spec fns. A PTT
//! confirms that callers can actually use the iterator — that those
//! postconditions and vstd's `Iterator::next` contract compose across every loop
//! form. Without PTTs, an iterator can verify internally but be unusable in
//! practice (a missing `decrease is Some`, a `remaining` not tied to the
//! collection's view, a constructor `ensures` shape that blocks `next`).
//!
//! ## The 6 required patterns
//!
//! Every collection with `iter()` must have PTTs for these patterns:
//!
//! | # | Pattern | Syntax | What it tests |
//! |---|---------|--------|---------------|
//! | 1 | loop-borrow-iter | `let mut it = a.iter();` + `loop { match it.next() }` | Manual loop with `iter()` + `next()` |
//! | 2 | loop-borrow-into | `let mut it = (&a).into_iter();` + `loop` | Manual loop via `IntoIterator` for `&Self` |
//! | 3 | for-borrow-iter | `for x in it: a.iter()` | Verus `for` loop with `iter()` |
//! | 4 | for-borrow-into | `for x in it: (&a).into_iter()` | Verus `for` loop via `IntoIterator` |
//! | 5 | loop-consume | `let mut it = a.into_iter();` + `loop` | Manual consuming iteration |
//! | 6 | for-consume | `for x in it: a.into_iter()` | Verus `for` consuming iteration |
//!
//! Patterns 5-6 (consuming) are only required if the collection implements
//! `IntoIterator for Self` (not just `IntoIterator for &Self`).
//!
//! ## PTT file naming
//!
//! PTT files live in `rust_verify_test/tests/ChapNN/` and follow this naming:
//!
//! ```text
//! rust_verify_test/tests/ChapNN/Prove<ModuleName>.rs
//! ```
//!
//! For example:
//! - `rust_verify_test/tests/Chap18/ProveArraySeqStEph.rs`
//! - `rust_verify_test/tests/Chap37/ProveAVLTreeSeqStPer.rs`
//!
//! If a single PTT file already exists for the module (testing other things), add
//! the iterator patterns to it. Do not create a separate iterator-only PTT file.
//!
//! ## Test naming convention
//!
//! Each test function is named `<module>_<pattern>`:
//!
//! ```text
//! #[test] arrayseqsteph_loop_borrow_iter
//! #[test] arrayseqsteph_loop_borrow_into
//! #[test] arrayseqsteph_for_borrow_iter
//! #[test] arrayseqsteph_for_borrow_into
//! #[test] arrayseqsteph_loop_consume
//! #[test] arrayseqsteph_for_consume
//! ```
//!
//! ## Template: loop-borrow-iter
//!
//! The manual loop runs on the iterator's own `next()` (its contract is the
//! `IteratorSpec` one in vstd/std_specs/iter.rs), not on `VerusForLoopWrapper`,
//! which vstd declares only under `verus_keep_ghost` and which does not
//! compile under `cargo` (src/experiments/prophetic_manual_loop_next.rs). A
//! ghost counter `pos` holds the number of items consumed; the two clauses
//! over `IteratorSpec::remaining(&it)` are the wrapper's `wf_inner`, written
//! out. The loop measures termination with the iterator's non-prophetic
//! `IteratorSpec::decrease(&it)->0`, and draws its conclusion before `break`,
//! because the prophetic `remaining()` equality does not survive the break
//! and may not appear in `decreases`.
//!
//! ```rust
//! test_verify_one_file! {
//!     #[test] modulename_loop_borrow_iter verus_code! {
//!         use vstd::prelude::*;
//!         use vstd::std_specs::iter::*;
//!         use apas_verus::ChapNN::ModuleName::ModuleName::*;
//!
//!         fn test_loop_borrow_iter() {
//!             let a: ModuleS<u64> = ModuleS::new(/* constructor args */);
//!             let ghost orig: Seq<u64> = a@;
//!             let mut collected: Vec<u64> = Vec::new();
//!             let mut it: std::slice::Iter<'_, u64> = a.iter();
//!             let ghost mut pos: int = 0;
//!             loop
//!                 invariant
//!                     IteratorSpec::obeys_prophetic_iter_laws(&it),
//!                     IteratorSpec::decrease(&it) is Some,
//!                     0 <= pos <= orig.len(),
//!                     IteratorSpec::remaining(&it).len() == orig.len() - pos,
//!                     forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
//!                         ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
//!                     collected.len() == pos,
//!                     forall|i: int| 0 <= i < collected.len()
//!                         ==> #[trigger] collected@[i] == orig[i],
//!                 decreases IteratorSpec::decrease(&it)->0,
//!             {
//!                 let ghost old_pos = pos;
//!                 match it.next() {
//!                     Some(x) => {
//!                         proof {
//!                             pos = pos + 1;
//!                             assert(orig[old_pos] == *x);
//!                         }
//!                         collected.push(*x);
//!                     },
//!                     None => {
//!                         assert(pos == orig.len());
//!                         assert(collected@ =~= orig);
//!                         break;
//!                     },
//!                 }
//!             }
//!         }
//!     } => Ok(())
//! }
//! ```
//!
//! ## Template: for-borrow-iter
//!
//! The `for` loop names its wrapper (`it`) and reasons through `it.index()`
//! and the prophetic `it.seq()`. `it` is out of scope after the loop; the
//! post-loop assertion follows from `it.index() == it.seq().len()`.
//!
//! ```rust
//! test_verify_one_file! {
//!     #[test] modulename_for_borrow_iter verus_code! {
//!         use vstd::prelude::*;
//!         use vstd::std_specs::iter::*;
//!         use apas_verus::ChapNN::ModuleName::ModuleName::*;
//!
//!         fn test_for_borrow_iter() {
//!             let a: ModuleS<u64> = ModuleS::new(/* constructor args */);
//!             let ghost orig: Seq<u64> = a@;
//!             let mut collected: Vec<u64> = Vec::new();
//!             for x in it: a.iter()
//!                 invariant
//!                     it.seq() == orig.as_ref(),
//!                     collected.len() == it.index(),
//!                     forall|i: int| 0 <= i < collected.len()
//!                         ==> #[trigger] collected@[i] == *it.seq()[i],
//!             {
//!                 collected.push(*x);
//!             }
//!             assert(collected@ =~= orig);
//!         }
//!     } => Ok(())
//! }
//! ```
//!
//! Consuming variants (`loop-consume`, `for-consume`) use `a.into_iter()`,
//! `std::vec::IntoIter<u64>`, `it.seq() == orig` (no `.as_ref()`), and no
//! deref: `it.seq()[i]` in a `for` loop, `IteratorSpec::remaining(&it)[i] ==
//! orig[pos + i]` and `orig[old_pos] == x` in a manual loop; they push `x`
//! rather than `*x`.
//!
//! ## Adapting for Mt variants
//!
//! Mt iterators operate on a locked snapshot. The PTT constructs the Mt struct,
//! then calls `iter()` which returns an iterator over the locked inner data.
//! The loop invariants are the same — the Mt wrapper is transparent to the
//! iterator contract. The only differences:
//! - Import path includes the Mt module.
//! - Constructor may require `Arc`/lock setup.
//! - The returned std iterator type (`std::slice::Iter` over a snapshot, or
//!   `std::vec::IntoIter` over a flattened copy) is the type of `it` in the
//!   manual loop's `let mut it: ... = a.iter();`.
//!
//! ## Adapting for tree/set collections
//!
//! Tree-backed collections (AVLTreeSeq, BSTSet*, OrderedSet, OrderedTable) iterate
//! over an in-order traversal. `orig` is the tree's linearized sequence, not a
//! Vec backing store. The patterns are identical — only the constructor and
//! types change. A custom iterator type (the three lazy AVLTreeSeq iterators)
//! implements `IteratorSpecImpl` per `prophetic_iterators_standard.rs`; its
//! PTT names the custom type as the type of `it` in the manual loop.
//!
//! ## Coverage inventory
//!
//! Collections with `iter()` that HAVE iterator PTTs (6 patterns each):
//!
//! | # | Chap | Module | PTT file | Patterns |
//! |---|------|--------|----------|----------|
//! | 1 | 05 | MappingStEph | ProveMappingStEph.rs | 8 |
//! | 2 | 05 | RelationStEph | ProveRelationStEph.rs | 8 |
//! | 3 | 05 | SetMtEph | ProveSetMtEph.rs | 6 |
//! | 4 | 06 | DirGraphStEph | ProveDirGraphStEph.rs | 8 |
//! | 5 | 06 | DirGraphMtEph | ProveDirGraphMtEph.rs | 8 |
//! | 6 | 06 | LabDirGraphStEph | ProveLabDirGraphStEph.rs | 8 |
//! | 7 | 06 | LabDirGraphMtEph | ProveLabDirGraphMtEph.rs | 8 |
//! | 8 | 06 | LabUnDirGraphStEph | ProveLabUnDirGraphStEph.rs | 8 |
//! | 9 | 06 | LabUnDirGraphMtEph | ProveLabUnDirGraphMtEph.rs | 8 |
//! | 10 | 06 | UnDirGraphStEph | ProveUnDirGraphStEph.rs | 8 |
//! | 11 | 06 | UnDirGraphMtEph | ProveUnDirGraphMtEph.rs | 8 |
//! | 12 | 17 | MathSeq | ProveMathSeq.rs | 12 |
//! | 13 | 18 | ArraySeqStEph | ProveArraySeqStEph.rs | 12 |
//! | 14 | 19 | ArraySeqMtEph | ProveArraySeqMtEph.rs | 12 |
//! | 15 | 19 | ArraySeqStEph | ProveArraySeqStEph.rs | 12 |
//! | 16 | 19 | ArraySeqStPer | ProveArraySeqStPer.rs | 12 |
//! | 17 | 23 | PrimTreeSeqStPer | ProvePrimTreeSeqStPer.rs | 12 |
//! | 18 | 37 | AVLTreeSeq | ProveAVLTreeSeq.rs | 4 |
//! | 19 | 41 | AVLTreeSetMtEph | ProveAVLTreeSetMtEph.rs | 6 |
//! | 20 | 43 | OrderedSetStEph | ProveOrderedSetStEph.rs | 8 |
//! | 21 | 43 | OrderedTableStEph | ProveOrderedTableStEph.rs | 8 |
//! | 22 | 43 | OrderedTableStPer | ProveOrderedTableStPer.rs | 8 |
//!
//! Collections with `iter()` that are MISSING iterator PTTs:
//!
//! | # | Chap | Module | Status |
//! |---|------|--------|--------|
//! | 1 | 05 | SetStEph | No PTT file |
//! | 2 | 18 | ArraySeq (unparameterized) | PTT exists, no iter patterns |
//! | 3 | 18 | ArraySeqMtEph | PTT exists, no iter patterns |
//! | 4 | 18 | ArraySeqMtPer | PTT exists, no iter patterns |
//! | 5 | 18 | ArraySeqStPer | PTT exists, no iter patterns |
//! | 6 | 18 | LinkedListStEph | PTT exists, no iter patterns |
//! | 7 | 18 | LinkedListStPer | PTT exists, no iter patterns |
//! | 8 | 19 | ArraySeqMtEphSlice | PTT exists, partial (6 patterns) |
//! | 9 | 37 | AVLTreeSeqMtPer | No PTT file |
//! | 10 | 37 | AVLTreeSeqStEph | No PTT file |
//! | 11 | 37 | AVLTreeSeqStPer | No PTT file |
//! | 12 | 37 | BSTSetAVLMtEph | No PTT file |
//! | 13 | 37 | BSTSetBBAlphaMtEph | No PTT file |
//! | 14 | 37 | BSTSetPlainMtEph | No PTT file |
//! | 15 | 37 | BSTSetRBMtEph | No PTT file |
//! | 16 | 37 | BSTSetSplayMtEph | No PTT file |
//! | 17 | 41 | AVLTreeSetMtEph | PTT exists, partial (6 patterns) |
//! | 18 | 43 | AugOrderedTableMtEph | No PTT file |
//! | 19 | 43 | AugOrderedTableStEph | No PTT file |
//! | 20 | 43 | AugOrderedTableStPer | No PTT file |
//! | 21 | 43 | OrderedSetStPer | No PTT file |
//! | 22 | 43 | OrderedTableMtEph | No PTT file |

// This file does not compile — it is a standard reference document only.
// The compilable iterator examples are in iterators_standard.rs (delegated),
// prophetic_iterators_standard.rs (custom) and wrapping_iterators_standard.rs.
// The compilable PTT examples are in rust_verify_test/tests/standards/
// Proveiterators_standard.rs and Proveprophetic_iterators_standard.rs.
