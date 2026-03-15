# Agent 4 — Round 16

## Project State

136 holes, 4097 verified, 38 clean chapters, 8 holed.

## Your Assignment: Close Chap37 + Chap47 + Chap45, then Chap41

### Priority 1: Close Chap37 (4 real holes → 0)

Chap37 blocks Chap41, 44, 45, and transitively Chap43, 52, 53, 55, 57, 65.
Closing it is the **highest priority in the entire project**.

Run `scripts/holes.sh src/Chap37/` to see exact holes. Expected:

**AVLTreeSeq.rs (1 external_body)**
- Iterator `next()` function. Reference implementation:
  `src/Chap18/ArraySeqStEph.rs` — look at how `IteratorImpl::next` is proved there.
- Remove `external_body`, write body that advances the cursor index and returns
  the element. Add ensures matching the iterator protocol.

**AVLTreeSeqMtPer.rs (2 external_body)**
- `build_balanced_from_slice` and `subseq_copy` — these use parallel code
  (nested fns, spawn/wait). If the body uses `ParaPair!` macro or `spawn`/`wait`,
  you may need to keep `external_body` on the parallel part but factor out the
  sequential logic into a proved helper.

**BSTSplayStEph.rs (1 trivial_wf)**
- `spec_bstsplaysteph_wf() -> bool { true }` — trivial well-formedness returns true.
- Strengthen to a real BST invariant: sorted ordering, tree structure valid.
- Look at `src/Chap38/BSTParaStEph.rs` `spec_bstparasteph_wf` for reference on
  how BST well-formedness specs are defined in this project.

### Priority 2: Close Chap47 (2 real holes → 0)

Run `scripts/holes.sh src/Chap47/` to see exact holes.

**ParaHashTableStEph.rs (2 external_body)**
- Check what functions these are on. If they wrap opaque `Fn` trait closures or
  `std::hash::Hash`, they may need to stay as `external_body`.
- If they're probe arithmetic (`(hash + attempt) % size`), remove `external_body`
  and add `assert(x % n < n)` for the modulus bound.

### Priority 3: Close Chap45 (3 holes, 2 effective)

**BalancedTreePQ.rs (1 external)**
- Check what function has `#[verifier::external]`. If it's a Display/Debug impl
  or a non-algorithmic helper, it may be acceptable to leave.

**BinaryHeapPQ.rs (1 assume)**
- `assume(Self::spec_sorted(result.seq@))` — sorted postcondition of extract_all.
- This requires proving the heap-to-sorted extraction produces sorted output.
  Needs connecting `delete_min` minimality through the extraction loop.

*Skip Example45_2.rs per project rules.*

### Priority 4: Chap41 remaining (12 effective holes, excluding MtPer and Example)

**AVLTreeSetMtEph.rs (9 holes: 2 assume + 2 unsafe_impl + 5 external_body)**
- 5 ext_body: Mt wrapper stubs — lock, call inner, bridge ghost state
- 2 assume: ghost field ↔ locked value bridge (size, find)
- 2 unsafe_impl (Send+Sync): correct for RwLock types, leave as-is

**AVLTreeSetStEph.rs (1 assume)**
- Check what the assume is. If it's a vec-length or wf bound, look for inductive
  lemmas or tree structure invariants that can prove it.

**ArraySetEnumMtEph.rs (1 external_body)**
- Mt wrapper stub. Same lock pattern.

## DO NOT TOUCH

- Chap43 (Agent 2)
- Chap42 or Chap41/AVLTreeSetMtPer (Agent 1)
- Chap39 or Chap38 (Agent 3)
- Any Example files

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **NO accept().** NO assume→accept conversion.
- **DO NOT weaken ensures.** Prove the existing postconditions or leave `external_body`.
- **Close Chap37 FIRST** before anything else. It unblocks the most downstream work.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.
- Push to `agent4/ready`. Write `plans/agent4-round16-report.md`.
- **Do the proof work.** If something is hard, try harder — search vstd, write
  intermediate lemmas, decompose the obligation. Report what you tried if stuck.

## Target: -8 (stretch -15). Close Chap37 + Chap47 + Chap45 = 3 new clean chapters.
