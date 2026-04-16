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
//! Iterator verification involves 10 interlocking components (see `iterators_standard.rs`).
//! A PTT confirms that callers can actually use the iterator — that the `ensures` from
//! `iter()`, `next()`, `ForLoopGhostIteratorNew`, and `ForLoopGhostIterator` compose
//! correctly across all loop forms. Without PTTs, an iterator can verify internally but
//! be unusable in practice (e.g., missing ghost iterator fields, wrong `iter_invariant`
//! shape, `next()` ensures that don't connect to `for` loop ghost state).
//!
//! ## The 6 required patterns
//!
//! Every collection with `iter()` must have PTTs for these patterns:
//!
//! | # | Pattern | Syntax | What it tests |
//! |---|---------|--------|---------------|
//! | 1 | loop-borrow-iter | `loop { ... a.iter() ... }` | Manual loop with `iter()` + `next()` |
//! | 2 | loop-borrow-into | `loop { ... (&a).into_iter() ... }` | Manual loop via `IntoIterator` for `&Self` |
//! | 3 | for-borrow-iter | `for x in iter: a.iter()` | Verus `for` loop with `iter()` |
//! | 4 | for-borrow-into | `for x in iter: (&a).into_iter()` | Verus `for` loop via `IntoIterator` |
//! | 5 | loop-consume | `loop { ... a.into_iter() ... }` | Manual consuming iteration |
//! | 6 | for-consume | `for x in iter: a.into_iter()` | Verus `for` consuming iteration |
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
//! ```rust
//! test_verify_one_file! {
//!     #[test] modulename_loop_borrow_iter verus_code! {
//!         use vstd::prelude::*;
//!         use apas_verus::ChapNN::ModuleName::ModuleName::*;
//!
//!         fn test_loop_borrow_iter() {
//!             let a: ModuleS<u64> = ModuleS::new(/* constructor args */);
//!
//!             let mut it: ModuleIter<u64> = a.iter();
//!             let ghost iter_seq: Seq<u64> = it@.1;
//!             let ghost mut items: Seq<u64> = Seq::empty();
//!
//!             #[verifier::loop_isolation(false)]
//!             loop
//!                 invariant
//!                     items =~= iter_seq.take(it@.0 as int),
//!                     iter_invariant(&it),
//!                     iter_seq == it@.1,
//!                     it@.0 <= iter_seq.len(),
//!                 decreases iter_seq.len() - it@.0,
//!             {
//!                 if let Some(x) = it.next() {
//!                     proof { items = items.push(*x); }
//!                 } else {
//!                     break;
//!                 }
//!             }
//!
//!             assert(it@.0 == iter_seq.len());
//!             assert(items =~= iter_seq);
//!         }
//!     } => Ok(())
//! }
//! ```
//!
//! ## Template: for-borrow-iter
//!
//! ```rust
//! test_verify_one_file! {
//!     #[test] modulename_for_borrow_iter verus_code! {
//!         use vstd::prelude::*;
//!         use apas_verus::ChapNN::ModuleName::ModuleName::*;
//!
//!         fn test_for_borrow_iter() {
//!             let a: ModuleS<u64> = ModuleS::new(/* constructor args */);
//!
//!             let it: ModuleIter<u64> = a.iter();
//!             let ghost iter_seq: Seq<u64> = it@.1;
//!             let ghost mut items: Seq<u64> = Seq::empty();
//!
//!             for x in iter: it
//!                 invariant
//!                     iter.elements == iter_seq,
//!                     items =~= iter_seq.take(iter.pos),
//!                     iter.pos <= iter_seq.len(),
//!             {
//!                 proof { items = items.push(*x); }
//!             }
//!
//!             assert(items =~= iter_seq);
//!         }
//!     } => Ok(())
//! }
//! ```
//!
//! ## Adapting for Mt variants
//!
//! Mt iterators operate on a locked snapshot. The PTT constructs the Mt struct,
//! then calls `iter()` which returns an iterator over the locked inner data.
//! The ghost state and loop invariants are the same — the Mt wrapper is transparent
//! to the iterator protocol. The only differences:
//! - Import path includes the Mt module.
//! - Constructor may require `Arc`/lock setup.
//! - Iterator type name includes Mt variant suffix.
//!
//! ## Adapting for tree/set collections
//!
//! Tree-backed collections (AVLTreeSeq, BSTSet*, OrderedSet, OrderedTable) iterate
//! over an in-order traversal. The `iter_seq` is the tree's linearized sequence,
//! not a Vec backing store. The patterns are identical — only the constructor and
//! types change.
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
// The compilable iterator examples are in iterators_standard.rs.
// The compilable PTT examples are in rust_verify_test/tests/standards/Proveiterators_standard.rs.
