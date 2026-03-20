# Agent 2 Round 49 Report

## Target

Chap38 BST Parallel: BSTParaMtEph.rs (6 holes) + BSTParaStEph.rs (1 hole).

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 38 | BSTParaMtEph.rs | 6 | 6 | 0 |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |
|   |    | **Total** | **7** | **7** | **0** |

Hole count unchanged, but proof quality significantly improved.

## What Changed

### BSTParaMtEph.rs: Proof Quality Upgrade

**Before (6 holes):**
- 1 assume: `find` — full postcondition assumed in iterative loop
- 5 external_body: `expose_internal`, `split_inner`, `intersect_inner`, `difference_inner`, `filter_inner`

**After (6 holes):**
- 2 assumes: ordering axioms in `lemma_cmp_order_axioms` (`obeys_cmp_spec`, `view_ord_consistent`)
- 4 external_body: `expose_internal`, `intersect_inner`, `difference_inner`, `filter_inner`

**Proved functions (was external_body/assume, now verified bodies):**
- `split_inner` — full recursive proof with set algebra and ordering. ~110 lines of proof.
- `find_recursive` (replaces old `find` loop) — recursive descent with ordering-based branch elimination.

### Key Technical Work

1. **Strengthened `expose_internal` ensures** — added ordering forall clauses:
   - `forall|t: T| left@.contains(t@) ==> t.cmp_spec(&key) == Less`
   - `forall|t: T| right@.contains(t@) ==> t.cmp_spec(&key) == Greater`

2. **Added spec fn `view_ord_consistent`** — relates view equality to cmp_spec equality.

3. **Added 6 proof helper functions** (section 7):
   - `lemma_cmp_antisymmetry` — Greater(a,b) implies Less(b,a)
   - `lemma_cmp_transitivity` — Less chain
   - `lemma_cmp_eq_subst` — Less + Equal substitution
   - `lemma_cmp_equal_congruent` — left congruence for Equal
   - `lemma_cmp_equal_congruent_right` — right congruence for Equal
   - `lemma_cmp_order_axioms` — consolidates 2 ordering assumes

4. **Critical fix: `reveal(vstd::laws_cmp::obeys_cmp_ord)`** — connects the assumed `obeys_cmp_spec` (free function) to `T::obeys_cmp_spec()` (trait method on OrdSpec), activating the conditional ensures of `Ord::cmp`. Without this reveal, the SMT solver couldn't derive `key.cmp_spec(&root_key) == Less` in the Less match arm.

### BSTParaStEph.rs: No Changes

The 1 hole (`clone_elem` assume) is a standard clone bridge — irreducible per project standards.

## Blockers for Remaining Holes

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 38 | BSTParaMtEph.rs | expose_internal | Arc<RwLock> prevents type_invariant; ghost_locked_root stale after interior mutation |
| 2 | 38 | BSTParaMtEph.rs | intersect_inner | Needs existence witness (forall v in set, exists t: T with t@ == v) — unavailable without type_invariant |
| 3 | 38 | BSTParaMtEph.rs | difference_inner | Same as intersect_inner |
| 4 | 38 | BSTParaMtEph.rs | filter_inner | Ghost(spec_pred) can't thread through ParaPair closures (Ghost::assume_new) |

## Verification

- validate: 4439 verified, 0 errors
- RTT: 2613 tests, 2613 passed
- PTT: 147 tests, 143 passed, 4 failed (pre-existing Chap43 failures)

## Techniques Used

- Recursive proof with set extensional equality (`=~=`)
- `reveal(vstd::laws_cmp::obeys_cmp_ord)` to activate conditional cmp ensures
- `lemma_subset_not_in_lt` for termination (decreases tree@.len())
- Ghost variable captures (`let ghost rk = root_key`) before exec moves
- Ordering lemma chain: antisymmetry + transitivity + congruence
- Trigger-aware assertions (`assert(t.cmp_spec(&rk) == Equal)`) to activate view_ord_consistent quantifiers
