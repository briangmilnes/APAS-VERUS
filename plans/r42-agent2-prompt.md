# R42 Agent 2: Fix Trait WF Requires + Prove Remaining StPer Methods

## Baseline
- Main at `e83db19f`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4320 verified, 153 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R41, you proved 11 methods in OrderedTableStPer.rs. 9 remain. Your report says
methods 1-7 (domain, collect, first_key, last_key, previous_key, next_key, difference)
are blocked by missing `self.spec_orderedtablestper_wf()` in the trait requires.

## Assignment

### Part A: Add WF Requires to Trait Methods (Unblocks 7 methods)

The trait `OrderedTableStPerTrait` defines signatures for these methods without
`requires self.spec_orderedtablestper_wf()`. Add the wf requires to the trait
signature for each blocked method.

**Exact changes needed** (you started this before disk filled — re-apply these):

In `OrderedTableStPer.rs` trait definition:
- `domain`: add `requires self.spec_orderedtablestper_wf()`
- `collect`: add `requires self.spec_orderedtablestper_wf()`
- `first_key`: add `requires self.spec_orderedtablestper_wf()`
- `last_key`: add `requires self.spec_orderedtablestper_wf()`
- `previous_key`: add `requires self.spec_orderedtablestper_wf()`
- `next_key`: add `requires self.spec_orderedtablestper_wf()`
- `difference`: add `other.spec_orderedtablestper_wf()` to existing requires
- `split_key` ensures: add `parts.0.spec_orderedtablestper_wf(), parts.2.spec_orderedtablestper_wf()`

In `AugOrderedTableStPer.rs` trait definition:
- `calculate_reduction`: add `requires base.spec_orderedtablestper_wf()` (already has reducer requires)
- `domain`: add `requires self.spec_augorderedtablestper_wf()`
- `collect`: add `requires self.spec_augorderedtablestper_wf()`
- `first_key`: add `requires self.spec_augorderedtablestper_wf()`
- `last_key`: add `requires self.spec_augorderedtablestper_wf()`
- `previous_key`: add `requires self.spec_augorderedtablestper_wf()`
- `next_key`: add `requires self.spec_augorderedtablestper_wf()`
- `difference`: add `other.spec_augorderedtablestper_wf()` to existing requires

**Steps:**
1. Apply the above trait signature changes
2. Update all impls to match (requires must match trait exactly)
3. For `difference`, also add `requires other.spec_orderedtablestper_wf()`
4. Update the impl to match (requires must match trait exactly)
5. Check all callers — RTT tests, other files that call these methods. They may need
   to prove wf at the call site. If tests construct an OrderedTableStPer via `new()` or
   `singleton()`, and those already ensure wf, callers should be fine.

**IMPORTANT**: When adding requires to a trait method, you MUST check:
- All implementations match the new signature
- All call sites can satisfy the new requires
- RTT tests still pass (they may need wf proof at call sites)

Also add wf requires to the corresponding methods in `OrderedTableStEphTrait` if they're
similarly missing. Agent 1 may also benefit from this.

### Part B: Prove Unblocked Methods (7 methods)

Once wf is in the requires, prove:

| # | Method | Notes |
|---|--------|-------|
| 1 | domain | Iterate pairs, collect keys into ArraySetStEph |
| 2 | collect | Return entries as sorted AVLTreeSeqStPerS sequence |
| 3 | first_key | Delegate to base_set.first(), extract key from Pair |
| 4 | last_key | Delegate to base_set.last(), extract key from Pair |
| 5 | previous_key | Find predecessor pair, extract key |
| 6 | next_key | Find successor pair, extract key |
| 7 | difference | Filter self, keeping entries whose keys are not in other |

Methods 3-6 (first_key, last_key, previous_key, next_key) should be straightforward
delegations once wf is available — the base AVLTreeSetStPer already has these methods
with strong ensures.

### Part C: rank_key and select_key (if time)

These need TotalOrder reasoning with Set::filter length proofs. Hard but try if you
finish Part B quickly.

### Expected Results

Conservative: Add wf requires + prove 5-7 methods.
Optimistic: Add wf requires + prove 7-9 methods (including rank/select).

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent2-r42-report.md`.
