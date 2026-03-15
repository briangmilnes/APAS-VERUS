# Agent 4 — Round 15

## Status: 149 holes, 4078 verified, 38 clean chapters.

Your mission: close 3 chapters (Chap37, Chap47, Chap45), then attack Chap41.

Closing Chap37 is the **highest priority in the entire project** — it currently blocks
Chap41, 44, 45, and transitively Chap43, 52, 53, 55, 57, 65. Five holes stand between
us and unlocking all that downstream work.

## Your files

### Priority 1: Close Chap37 (5 holes → 0)

**AVLTreeSeq.rs (1 external_body)**
- Line ~1117: `external_body` on iterator `next()`.
- Standard iterator pattern. See `src/Chap18/ArraySeqStEph.rs` for reference.
- Remove `external_body`, write the real body advancing the cursor.

**AVLTreeSeqMtPer.rs (2 external_body)**
- Line ~508: `build_balanced` — constructs balanced tree from sorted slice.
- Line ~623: `subseq_copy` — extracts subsequence.
- Remove `external_body`, write real implementations using the inner StPer methods.

**AVLTreeSeqStPer.rs (1 assume)**
- Line ~506: `assume(val@ == a@[mid as int]@)` — slice indexing assume.
- Need to prove that the element retrieved from the slice views to the spec sequence
  element at that index. This may need a seq_to_slice or slice_index lemma.

**BSTSplayStEph.rs (1 trivial_wf)**
- `spec_bstsplaysteph_wf() -> bool { true }` — trivial well-formedness.
- Add the real BST invariant: sorted ordering, valid tree structure.
- Look at how BSTParaStEph or other BST files define their wf spec.

### Priority 2: Close Chap47 (2 holes → 0)

**ParaHashTableStEph.rs (2 external_body)**
- These are hash probe functions with `ensures result < table_size`.
- Remove `external_body`, write the modular arithmetic body.
- `assert(x % n < n)` is all Verus needs for the proof.

### Priority 3: Close Chap45 (2 effective holes → 0)

**BalancedTreePQ.rs (1 external)**
- Line ~557: `#[verifier::external]` — check what function this is on.
- If it's a display/debug helper, leave it. If algorithmic, remove and prove.

**BinaryHeapPQ.rs (1 assume)**
- Line ~947: `assume(Self::spec_sorted(result.seq@))` — sorted postcondition.
- Need to prove the heap-to-sorted extraction actually produces sorted output.
- This is the heap sort correctness proof.

*Skip Example45_2.rs (1 external) per project rules.*

### Priority 4: Chap41 (12 holes, excluding Example41_3)

**AVLTreeSetMtEph.rs (9 holes: 2 assume + 2 unsafe_impl + 5 external_body)**
- You worked on this file in R10. You know the ghost field sync pattern.
- 5 ext_body: Mt wrapper stubs — lock, call inner, bridge ghost state.
- 2 assume: `size == self@.len()` and `found == self@.contains(x@)` — same ghost bridge.
- 2 unsafe_impl (Send+Sync): these are correct for RwLock types. Leave them.

**AVLTreeSetStEph.rs (2 assume)**
- Both are likely feq/wf assumes. Check what they assume and whether the inner
  AVLTreeSeqStEph ensures can prove them.

**ArraySetEnumMtEph.rs (1 external_body)**
- Mt wrapper stub. Same lock pattern.

## DO NOT

- Touch Chap43 (Agent 2)
- Touch Chap42 (Agent 1)
- Touch Chap41/AVLTreeSetMtPer (Agent 1)
- Touch Chap39 (Agent 3)
- Touch Chap38 (Agent 3)
- Touch Example files

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversion.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.
- **Close Chap37 FIRST** before anything else. It unblocks the most downstream work.
- Push to `agent4/ready`. Write `plans/agent4-round15-report.md`.

## Target: -12 (stretch -19). Close Chap37 + Chap47 + Chap45 = 3 new clean chapters.
