# R38 Agent Prompts

## Baseline
- Main at `485299d3`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4332 verified, 204 holes, 29 clean chapters

---

## Agent 1: Chap47 Flat Hash Tables + ParaHashTable Warnings

### MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.

Read CLAUDE.md and `src/standards/partial_eq_eq_clone_standard.rs` before starting.

### Assignment

You are Agent 1 for R38. Your scope is **Chap47 flat hash tables + ParaHashTable warnings**.

### Task 1: ParaHashTableStEph.rs warnings (8 warnings + 1 external_body)

File: `src/Chap47/ParaHashTableStEph.rs`

Fix these fn_missing_wf warnings by adding real `requires`/`ensures`:
- Line 572: `createTable` — add `ensures table.spec_hashtable_wf()`
- Line 623: `insert` — add `requires table.spec_hashtable_wf()`
- Line 637: `lookup` — add `requires table.spec_hashtable_wf()`
- Line 647: `delete` — add `requires table.spec_hashtable_wf()`
- Line 660: `metrics` — add `requires table.spec_hashtable_wf()`
- Line 669: `loadAndSize` — add `requires table.spec_hashtable_wf()`
- Line 686: `resize` — add `requires table.spec_hashtable_wf()` AND `ensures resized.spec_hashtable_wf()`

The `call_hash_fn` external_body (line 463) is a stretch goal — prove if time permits.

### Task 2: Prove eq/clone bridge assumes in flat hash tables

These files all have the same two assume patterns:

**Pattern A — Eq bridge:** `assume(eq == spec_flat_has_key(table.table@[slot], key))`
After calling `k == key` (PartialEq::eq), the result `eq` should equal the spec.
The proof: `PartialEq::eq` ensures `r == (self@ == other@)`. Assert that
`(k@ == key@) == spec_flat_has_key(...)` from the spec definition.

**Pattern B — Clone bridge:** `assume(key == pairs@[j].0)` / `assume(value == pairs@[j].1)`
After cloning key/value, the clone equals the original at the View level.
The proof: `Clone::clone` ensures `cloned@ == self@`. Use that directly.

**Pattern C — assume(false) table full:** `assume(false); // Table full`
Prove that with load factor < 1, the table always has an empty slot. This requires
showing that `count < m` implies at least one slot is Empty. Use pigeonhole: if all
m slots are Occupied, count >= m, contradicting count < m.

Files and holes:

1. `src/Chap47/LinProbFlatHashTableStEph.rs` — 6 holes
   - Line 132: eq bridge in insert
   - Line 348: assume(false) table full
   - Line 391: eq bridge in lookup
   - Line 497: eq bridge in delete
   - Lines 715-716: clone bridges in resize

2. `src/Chap47/QuadProbFlatHashTableStEph.rs` — 6 holes
   - Line 110: eq bridge in insert
   - Line 366: assume(false) table full
   - Line 404: eq bridge in lookup
   - Line 563: eq bridge in delete
   - Lines 835-836: clone bridges in resize

3. `src/Chap47/DoubleHashFlatHashTableStEph.rs` — 9 holes
   - Line 96: wf bridge in insert (forall quantifier)
   - Line 149: eq bridge in insert
   - Line 362: assume(false) table full
   - Line 375: wf bridge in lookup
   - Line 425: eq bridge in lookup
   - Line 521: wf bridge in delete
   - Line 573: eq bridge in delete
   - Lines 807-808: clone bridges in resize

4. `src/Chap47/StructChainedHashTable.rs` — 4 holes + 1 external_body
   - Line 124: eq bridge in insert
   - Line 163: eq bridge in lookup
   - Line 167: clone bridge in lookup
   - Line 205: eq bridge in delete
   - Line 398: external_body resize (stretch goal)

### Strategy

Start with LinProbFlatHashTableStEph.rs — it's the simplest (linear probing).
Crack the eq bridge and clone bridge patterns there, then replicate to QuadProb
and DoubleHash. Do StructChainedHashTable last (slightly different structure).

Read `src/standards/partial_eq_eq_clone_standard.rs` to understand the eq/clone
ensures pattern.

### Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Do NOT run validate concurrently with other agents.
Write your report to `plans/agent1-r38-report.md`.

---

## Agent 2: Chap43 OrderedSet/OrderedTable Proofs

### MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.
**DO NOT convert assume() to accept().** Leave assumes as assumes.

Read CLAUDE.md and the relevant standards before starting.

### Assignment

You are Agent 2 for R38. Your scope is **all holed files in Chap43**.

### Task 1: Fix fn_missing_requires/ensures warnings (5 warnings)

1. `src/Chap43/AugOrderedTableMtEph.rs:72` — `recalculate_reduction`: add `requires table.spec_augorderedtablemteph_wf()`
2. `src/Chap43/AugOrderedTableMtEph.rs:85` — `calculate_reduction`: add `requires base.spec_orderedtablemteph_wf()`
3. `src/Chap43/OrderedSetStEph.rs` — `from_sorted_elements`: add `requires` with `spec_orderedsetsteph_wf` on output
4. `src/Chap43/OrderedSetStPer.rs` — `from_sorted_elements`: same pattern
5. `src/Chap43/OrderedTableMtEph.rs` — fn_missing_wf_ensures: add wf ensures

Read each function, understand what it needs, add the REAL precondition.

### Task 2: Prove external_body delegation wrappers

These are functions that delegate to a base type method. Remove `#[verifier::external_body]`,
call the base method, and let its ensures satisfy ours.

1. `src/Chap43/OrderedSetMtEph.rs:344` — `to_seq` external_body
   - Delegates through RwLock to AVLTreeSetStEph::to_seq
   - Pattern: acquire lock, call inner.to_seq(), return result

2. `src/Chap43/OrderedTableStEph.rs` — `collect` external_body
   - Delegates to base table's collect

3. `src/Chap43/OrderedTableStPer.rs` — `collect` external_body
   - Same pattern as StEph

4. `src/Chap43/OrderedTableMtEph.rs` — `rank_key` and `select_key` external_body (2 holes)
   - Delegate through RwLock to OrderedTableStEph methods

### Task 3: Prove algorithmic assumes

1. `src/Chap43/AugOrderedTableMtEph.rs:92` — closure requires assume
   - Need to propagate closure requires from caller. Read `using_closures_standard.rs`.

2. `src/Chap43/AugOrderedTableStPer.rs:124` — lemma_reducer_clone_total assume
   - Clone bridge for closures. Hard — may need to leave if stuck.

3. `src/Chap43/OrderedSetStEph.rs` — 2 assumes (to_seq clone bridge, select filter cardinality)
4. `src/Chap43/OrderedSetStPer.rs` — 1 assume (select filter cardinality)
5. `src/Chap43/OrderedTableStEph.rs` — rank_key assume + select_key assume(false)
6. `src/Chap43/OrderedTableStPer.rs` — rank_key assume + select_key assume(false)

### Strategy

Start with Task 1 (warnings — quick wins). Then Task 2 (delegation wrappers — mechanical).
Then Task 3 (algorithmic assumes — hardest). If a proof is genuinely stuck after real
effort, leave the assume and report what you tried.

### Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent2-r38-report.md`.

---

## Agent 3: Chap41 AVL + Chap57 Dijkstra + Misc Warnings

### MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.
**DO NOT convert assume() to accept().** Leave assumes as assumes.

Read CLAUDE.md before starting.

### Assignment

You are Agent 3 for R38. Your scope is **Chap41 StEph, Chap57, Chap59 StEph, Chap39 warnings**.

### Task 1: Fix requires_true warnings in BSTTreapMtEph.rs (2 warnings)

File: `src/Chap39/BSTTreapMtEph.rs`
- Line 353: `requires true` — replace with real requires (likely `self.spec_bstparatreapmteph_wf()`)
- Line 389: `requires true` — same pattern

Read the functions, understand what they need, add real preconditions.

### Task 2: Fix fn_missing_requires warnings (4 warnings)

1. `src/Chap41/AVLTreeSetMtEph.rs:311` — `parallel_filter`: add real requires
2. `src/Chap41/AVLTreeSetMtEph.rs:372` — `parallel_intersect`: add real requires
3. `src/Chap41/AVLTreeSetMtPer.rs` — `parallel_sort`: add real requires
4. `src/Chap57/DijkstraStEphU64.rs:93` — `pq_entry_new`: add real requires

### Task 3: Fix bare_impl warning in AVLTreeSetStEph.rs

`src/Chap41/AVLTreeSetStEph.rs` — has 1 bare_impl warning. Move the method into the trait.

### Task 4: Prove assumes in AVLTreeSetStEph.rs (2 assumes)

`src/Chap41/AVLTreeSetStEph.rs` — 2 assumes about Vec length bounds after insert.
These are `assume(new_vec.len() <= ...)` style. The proof needs to show that
inserting into a sorted vec doesn't exceed the length bound. Read the insert
function carefully and trace the length arithmetic.

### Task 5: Prove assumes in DijkstraStEphU64.rs (3 assumes)

File: `src/Chap57/DijkstraStEphU64.rs`
- Line 166: `assume(obeys_feq_clone::<PQEntry>())` — feq clone bridge for PQEntry
- Line 201: `assume(BinaryHeapPQ::spec_is_exec_heap(...))` — heap invariant maintenance
- Line 242: `assume(remaining_budget > 0)` — Dijkstra PQ insert budget

For the budget proof: total PQ inserts <= |E| (each edge relaxed at most once).
Track edges processed vs budget consumed.

### Task 6: Fix fn_missing_requires in JohnsonStEphI64.rs (3 warnings)

File: `src/Chap59/JohnsonStEphI64.rs`
- Line 72: `adjust_distance` — add real requires
- Line 88: `reweight_edge` — add real requires
- Line 329: `create_negative_cycle_result` — add real requires

The assume at line 437 (graph size bound) is a stretch goal.

### Strategy

Start with Tasks 1-3 (warnings — fast). Then Task 4 (AVL assumes — medium).
Then Task 5 (Dijkstra — medium-hard). Task 6 last (Johnson warnings — fast).

### Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent3-r38-report.md`.

---

## Agent 4: Chap47 Chained Hash Tables + BSTParaStEph

### MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.
**DO NOT convert assume() to accept().** Leave assumes as assumes.

Read CLAUDE.md and `src/standards/partial_eq_eq_clone_standard.rs` before starting.

### Assignment

You are Agent 4 for R38. Your scope is **Chap47 chained hash tables + Chap38 BSTParaStEph**.

### Task 1: Prove eq/clone bridge assumes in LinkedListChainedHashTableStEph.rs (12 holes)

File: `src/Chap47/LinkedListChainedHashTableStEph.rs`

All 12 holes are eq bridge or clone bridge assumes:
- Lines 55-56: clone bridges (chain_to_vec)
- Line 190: eq bridge (insert)
- Line 283: eq bridge (lookup)
- Line 286: clone bridge (lookup)
- Line 344: eq bridge (delete)
- Lines 357-358: clone bridges (delete)
- Lines 469-470: clone bridges (resize helper)
- Lines 566-567: clone bridges (resize)

**Eq bridge pattern:** After `k == key` (PartialEq::eq), the result equals the spec
because PartialEq::eq ensures `r == (self@ == other@)`. Assert that
`(k@ == key@) == (bucket_seq@[i].0 == key)` using the View equivalence.

**Clone bridge pattern:** After `k = entry.clone()`, the clone's View equals the
original's View because Clone::clone ensures `cloned@ == self@`.

Read `src/standards/partial_eq_eq_clone_standard.rs` for the ensures patterns.

### Task 2: Prove eq/clone bridge assumes in VecChainedHashTableStEph.rs (12 holes)

File: `src/Chap47/VecChainedHashTableStEph.rs`

Identical pattern to LinkedList version:
- Lines 53-54: clone bridges
- Line 191: eq bridge
- Line 285: eq bridge
- Line 288: clone bridge
- Line 345: eq bridge
- Lines 358-359: clone bridges
- Lines 472-473: clone bridges
- Lines 568-569: clone bridges

### Task 3: Prove expose assume in BSTParaStEph.rs (1 hole)

File: `src/Chap38/BSTParaStEph.rs`

One assume in the `expose` function — clone/view bridge for the BST node type.
Read the function, understand the clone ensures, and prove the bridge.

### Strategy

Start with LinkedListChainedHashTableStEph.rs — crack all 12 eq/clone bridges.
The pattern will be identical for VecChainedHashTableStEph.rs (copy-paste proofs).
Then do BSTParaStEph.rs (1 hole, different pattern).

### Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent4-r38-report.md`.
