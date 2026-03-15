# Agent 3 — Round 17: Spec Audit Chap37 + Chap38 + Chap39

## Project State

103 holes, 4150 verified, 38 clean chapters, 8 holed.

## Mission: Fix Weak/Missing requires/ensures Against APAS Prose

This round is about **spec correctness**, not hole closure. Audit every trait
function's `requires`/`ensures` against the textbook definitions in `prompts/`.
Where specs are weak or missing, write the correct spec. If the corrected spec
breaks an existing proof body, add `#[verifier::external_body]`.

**Read `prompts/Chap37.txt`, `prompts/Chap38.txt`, `prompts/Chap39.txt` FIRST.**

## Chap37: AVL Trees + Splay Trees

Files: `src/Chap37/AVLTreeSeqStEph.rs`, `src/Chap37/AVLTreeSeqStPer.rs`,
`src/Chap37/BSTSplayStEph.rs`

### AVLTreeSeqStEph.rs — Mostly correct

Most functions have correct specs (empty, singleton, nth, from_vec, push_back,
contains_value, etc.). Issues found:

| # | Function | Problem | Action |
|---|----------|---------|--------|
| 1 | `set(&mut self, i, item)` | Weak: missing ensures on result sequence | Add: `self@ == old(self)@.update(i as int, item@)` |
| 2 | `update(&mut self, i, item)` | Weak: no postcondition | Same as set — specify updated sequence |

### AVLTreeSeqStPer.rs — Weaker than StEph

Several functions have weaker specs than their StEph counterpart:

| # | Function | Problem | Action |
|---|----------|---------|--------|
| 1 | `subseq_copy` | No spec postcondition | Add: result spec_seq matches source subrange |
| 2 | `to_arrayseq` | No spec postcondition | Add: length + indexing match |
| 3 | `values_in_order` | No spec | Add: result matches spec_seq |

### BSTSplayStEph.rs — CRITICAL: Many empty postconditions

| # | Function | Problem | Correct ensures |
|---|----------|---------|-----------------|
| 1 | `contains` | `ensures true` (empty!) | `ensures result == self.spec_contains(target@)` |
| 2 | `find` | `found.is_some() ==> *rv == *target` only | Add: `found.is_none() ==> !self.spec_contains(target@)` |
| 3 | `in_order` | `ensures true` (empty!) | `ensures result@ == self.spec_in_order()` (match spec fn) |
| 4 | `pre_order` | `ensures true` (empty!) | `ensures result@ == self.spec_pre_order()` (match spec fn) |
| 5 | `insert` | Weak: only wf preserved | Add: `self.spec_contains(value@)`, `self.spec_size() >= old(self).spec_size()` |
| 6 | `minimum` | Partial: "in tree" but no ordering | Add: `forall|x| self.spec_contains(x) ==> min@ <= x` |
| 7 | `maximum` | Partial: "in tree" but no ordering | Add: `forall|x| self.spec_contains(x) ==> max@ >= x` |

These spec functions (`spec_contains`, `spec_in_order`, `spec_pre_order`, `spec_min`,
`spec_max`) already exist in the trait. The ensures just need to reference them.

## Chap38: Parametric BSTs — Mostly Strong

File: `src/Chap38/BSTParaStEph.rs`

This file has **excellent specs** on core operations (expose, join_mid, split, union,
intersect, difference, insert, delete, find). Only 2 issues:

| # | Function | Problem | Action |
|---|----------|---------|--------|
| 1 | `filter` | WEAK: `result.subset_of(self@)` only | If closure ensures available, add predicate semantics. Otherwise note as "partial — closure limitation". |
| 2 | `reduce` | MISSING: no postcondition | Add at minimum: `result` relates to the tree's elements. If the reduction function's ensures is available, specify `result == fold(self@, base, op)`. |

## Chap39: Treaps — Weak on find/traversal

Files: `src/Chap39/BSTTreapStEph.rs`, `src/Chap39/BSTSetTreapStEph.rs` (if exists)
or `src/Chap39/BSTSetTreapMtEph.rs`

### BSTTreapStEph.rs

| # | Function | Problem | Correct ensures |
|---|----------|---------|-----------------|
| 1 | `find` | Weak: `found implies contains` only | Add: `found.is_none() ==> !self.spec_contains(target@)` (iff semantics) |
| 2 | `contains` | Weak: similar | `result == self.spec_contains(target@)` |
| 3 | `in_order` | Weak: length only | `result@ == self.spec_in_order()` |
| 4 | `pre_order` | Weak: length only | `result@ == self.spec_pre_order()` |

Note: The treap's heap-priority property is a structural invariant, not an operation
postcondition. It should be part of `spec_bsttreapsteph_wf()`. Check if it is — if
not, strengthening wf is a separate (harder) task. For this round, focus on the
operation specs.

### BSTSetTreapStEph.rs (or wrapper file)

Check this file for additional trait functions. If it wraps BSTTreapStEph, its specs
should delegate correctly. Audit the wrapper ensures match the inner type's ensures.

## Deliverables

1. **`src/Chap37/analyses/spec-audit.md`** — per-function table
2. **`src/Chap38/analyses/spec-audit.md`** — per-function table
3. **`src/Chap39/analyses/spec-audit.md`** — per-function table
4. Corrected trait ensures in all files
5. `external_body` on impl fns that can't prove the strengthened spec
6. Clean validation (0 errors)

## DO NOT TOUCH

- Chap41, Chap42 (Agent 1)
- Chap43 (Agent 2)
- Chap45, Chap47 (Agent 4)
- Any Example files
- Mt/MtPer wrapper files

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **The prose is the source of truth.** Not what's easy to prove.
- **NO accept().** NO assume→accept.
- **Add `external_body` if you can't prove the correct spec.** Never weaken ensures.
- **DO NOT delete existing ensures.** Only add to them.
- Push to `agent3/ready`. Write `plans/agent3-round17-report.md`.

## Target

Audit all trait fns in Chap37 (3 files) + Chap38 (1 file) + Chap39 (1-2 files).
Fix ~15 weak/missing specs. BSTSplayStEph has the most critical gaps (7 functions
with empty or partial postconditions).
