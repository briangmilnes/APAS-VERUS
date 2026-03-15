# Agent 2 — Round 17: Spec Audit Chap43

## Project State

103 holes, 4150 verified, 38 clean chapters, 8 holed.

## Mission: Fix Weak/Missing requires/ensures Against APAS Prose

This round is about **spec correctness**, not hole closure. You are auditing every
trait function's `requires`/`ensures` against the textbook definitions in `prompts/`.
Where specs are weak or missing, write the correct spec from the prose. If the
corrected spec breaks an existing proof body, add `#[verifier::external_body]` to
preserve the strong spec.

**Read `prompts/Chap43.txt` FIRST.** It contains ADT 43.1 (Ordered Sets) and the
ordered tables extension. These are the source of truth.

## Chap43: Ordered Sets & Tables (ADT 43.1)

Files to audit:
- `src/Chap43/OrderedSetStEph.rs`
- `src/Chap43/OrderedSetStPer.rs`
- `src/Chap43/OrderedTableStEph.rs`
- `src/Chap43/OrderedTableStPer.rs`
- `src/Chap43/AugOrderedTableStEph.rs`
- `src/Chap43/AugOrderedTableStPer.rs`

### The Problem

Nearly ALL ordering functions have **weak** ensures. They verify membership
(`v ∈ A` / `k ∈ dom(T)`) but lack the **extremality** property that makes ordered
operations meaningful. `first` returning "something in the set" is useless — it must
return "the minimum."

### Prose Definitions → Correct Ensures

**Ordered Sets (ADT 43.1):**

| Fn | Prose | Correct ensures (beyond current) |
|----|-------|----------------------------------|
| `first` | `first(A) = min[|A|]` | `Some(v) ==> forall|k| self@.contains(k) ==> v@ <= k` |
| `last` | `last(A) = max[|A|]` | `Some(v) ==> forall|k| self@.contains(k) ==> v@ >= k` |
| `previous` | `previous(A,k) = max{k'∈A | k'<k}` | `Some(v) ==> v@ < k@ && forall|k'| self@.contains(k') && k' < k@ ==> k' <= v@` |
| `next` | `next(A,k) = min{k'∈A | k'>k}` | `Some(v) ==> v@ > k@ && forall|k'| self@.contains(k') && k' > k@ ==> k' >= v@` |
| `split` | `split(A,k) = ({k'<k}, k∈A?, {k'>k})` | `forall|x| left@.contains(x) ==> x < k@`, `forall|x| right@.contains(x) ==> x > k@`, `left@.union(right@).union(if mid { Set::empty().insert(k@) } else { Set::empty() }) =~= self@` |
| `join` | `join(A1,A2) = A1∪A2` | `self@ =~= left@.union(right@)` (requires `max(left) < min(right)`) |
| `get_range` | `getRange(A,k1,k2) = {k∈A | k1≤k≤k2}` | `forall|x| range@.contains(x) ==> x >= k1@ && x <= k2@ && self@.contains(x)`, `forall|x| self@.contains(x) && x >= k1@ && x <= k2@ ==> range@.contains(x)` |
| `rank` | `rank(A,k) = |{k'∈A | k'<k}|` | Needs a spec fn. At minimum: `rank <= self@.len()` AND `forall|k'| self@.contains(k') && k' < k@ ==> (counted)` — this is hard to express with vstd Set. Consider defining a spec fn `spec_rank` and ensuring `rank == spec_rank(self@, k@)`. |
| `select` | `select(A,i) = k s.t. rank(A,k)=i` | `Some(v) ==> self@.contains(v@)` + `spec_rank(self@, v@) == i` |
| `split_rank` | `splitRank(A,i) = ({k | rank<i}, {k | rank≥i})` | Both parts subset, partition complete, cardinality of left == i |

**Ordered Tables:** Same patterns but on `dom()` (the key set). `first_key` = min of
dom, `previous_key` = max of {k' ∈ dom | k' < k}, etc. Value preservation:
`forall|k| left@.dom().contains(k) ==> left@[k] == self@[k]`.

**AugOrderedTable:** Same as OrderedTable + `reduce_val` and `reduce_range`. The
reduction specs depend on the reduction function's semantics.

### What to Do for Each Function

1. Read the current trait ensures
2. Compare against the prose definition above
3. If weak/missing, add the extremality/universality clause to the **trait** ensures
4. Check if the **impl** body still proves the strengthened spec
5. If not, add `#[verifier::external_body]` to the impl fn
6. Validate

### The `spec_rank` Helper

For `rank`, `select`, `split_rank`, you'll need a spec function. Define it in the
trait as abstract, implement as open in the impl:

```rust
spec fn spec_rank(&self, k: T::V) -> nat;

// In impl:
open spec fn spec_rank(&self, k: T::V) -> nat {
    // Count elements less than k — may need a recursive or vstd-based definition
}
```

If defining spec_rank is too complex, at minimum ensure:
- `rank` result is exact for `rank == 0` (no elements less than k)
- `rank` result satisfies `rank <= self@.len()`
- `select(i)` result satisfies `self@.contains(v@)`

### AugOrderedTableStEph: Special Case

Most ensures are just `dom().finite()` — these are **missing**, not weak. They need
the same ensures as OrderedTableStEph plus reduction specs. For `reduce_val` and
`reduce_range`, the spec depends on the reduction function. At minimum ensure
`self@.dom().finite()` plus `result` relates to the values in the table/range.

### Functions That Are Already Correct

- `join` in StPer variants: `joined@ == left@.union(right@)` — correct.
- `split_key` None/Some case in StPer: value extraction is correct.
- `map`, `filter`, `reduce` in OrderedTableStEph: real bodies from R16, keep them.

## Deliverables

1. **`src/Chap43/analyses/spec-audit.md`** — per-function table for ALL 6 files
2. Corrected trait ensures in all 6 files
3. `external_body` on impl fns that can't prove the strengthened spec
4. Clean validation (0 errors)

## DO NOT TOUCH

- Chap41, Chap42 (Agent 1)
- Chap37, Chap38, Chap39 (Agent 3)
- Chap45, Chap47 (Agent 4)
- Any Example files
- Mt/MtPer wrapper files (OrderedSetMtEph, OrderedTableMtPer, etc.)

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **The prose is the source of truth.** ADT 43.1 defines the postconditions.
- **NO accept().** NO assume→accept.
- **Add `external_body` if you can't prove the correct spec.** Never weaken ensures.
- **DO NOT delete existing ensures.** Only add to them.
- Read `src/standards/using_closures_standard.rs` before writing closure specs.
- Push to `agent2/ready`. Write `plans/agent2-round17-report.md`.

## Target

Audit all trait fns in 6 Chap43 files (~60 ordering functions). Fix ~40 weak specs.
Produce spec-audit.md. Hole count WILL increase — that's correct. Every new hole
has a spec that matches the textbook.
