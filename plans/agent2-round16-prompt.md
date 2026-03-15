# Agent 2 — Round 16

## Project State

136 holes, 4097 verified, 38 clean chapters, 8 holed.

## Your Assignment: Chap43 St files (53 holes total)

Chap43 implements ordered sets and ordered tables — ADTs where keys are sorted and
operations like `first(A) = min[|A|]`, `previous(A,k) = max{k' | k' < k}` have
ordering semantics. The specs reflect this: `first_key` ensures
`first matches Some(k) ==> self@.dom().contains(k@)` and
`self@.dom().len() == 0 <==> first matches None`.

**CRITICAL: DO NOT weaken ensures.** The postconditions encode the ordering semantics
from the APAS textbook. If you cannot prove a postcondition, leave `external_body` in
place and report what you tried. Never delete ensures to make proofs pass.

### Priority 1: OrderedSetStEph.rs (12 holes: 1 assume + 11 external_body)

The 11 `external_body` functions need real bodies that PROVE the existing ensures.
The inner type is `AVLTreeSetStEph<T>` (from Chap37).

**Pattern for these functions:**
1. Remove `#[verifier::external_body]`
2. Call `self.base_set.entries()` or similar to get the sorted sequence
3. Iterate with `while i < size` (NOT `for`) with loop invariants
4. Use `elem.cmp(k)` with `std::cmp::Ordering` match (Verus can't use `<`/`>` on refs)
5. For clone: `Pair(pair.0.clone(), pair.1.clone())` (NOT `pair.clone()`)
6. **Keep the full ensures clause.** Add invariants that track the ensures properties
   through the loop (e.g., for `first`: if entries is sorted and non-empty, `nth(0)`
   is the minimum, so `self@.contains(nth(0)@)`)

The key challenge: proving `self@.contains(result@)` after cloning an element from the
sorted sequence. You need to connect the sequence element to set membership. Look for
lemmas that relate `AVLTreeSetStEph.entries()` to the set view.

### Priority 2: OrderedTableStEph.rs (12 holes: 12 external_body)

Same pattern but for key-value tables. The 3 functions `map`, `filter`, `reduce` are
already proved (from last round). The remaining 12 are ordering operations.

Note: `collect` is `external_body` because it uses `sort_by` with closures which Verus
can't verify inside `verus!`. Leave `collect` as `external_body`.

### Priority 3: OrderedSetStPer.rs (5 ext_body) + OrderedTableStPer.rs (9 ext_body)

Persistent versions. Same functions, persistent semantics (return new value, don't mutate).

### Stretch: AugOrderedTable files (7 holes)

These wrap OrderedTable and add reduction. Lower priority.

## DO NOT TOUCH

- Chap42 or Chap41/MtPer (Agent 1)
- Chap39 or Chap38 (Agent 3)
- Chap37, Chap47, Chap45, Chap41/MtEph, Chap41/StEph (Agent 4)

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **NO accept().** NO assume→accept conversion.
- **DO NOT weaken ensures.** If the trait says
  `first matches Some(k) ==> self@.dom().contains(k@)`, you must PROVE it.
  Replacing it with `self@.dom().finite()` is a spec regression, not a proof.
  Leave `external_body` and report what you tried rather than gutting the spec.
- Read `src/standards/using_closures_standard.rs` before writing closure code.
- Read `src/standards/partial_eq_eq_clone_standard.rs` for feq/clone patterns.
- Push to `agent2/ready`. Write `plans/agent2-round16-report.md`.

## Target: -10 (stretch -20). Strong specs only.
