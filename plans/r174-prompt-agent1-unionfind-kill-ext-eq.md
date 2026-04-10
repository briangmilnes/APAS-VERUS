# R174 Prompt — Agent 1: Kill =~= extensional equality in UnionFind wf. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`, `assume`, `admit`, or `accept`.**
6. **NEVER use rlimit above 30.**

## Read all standards first

## The remaining blocker

You closed `spec_elem_at_neq` in R173 — that killed the 9M `spec_elements_distinct`
loop. Good. But `lemma_rank_lt_elements` still fails because of a **secondary
matching loop**: `=~=` from domain equality cross-fires with `contains_key` triggers.

The chain is:
1. Proof needs `spec_elements_distinct` in scope (for the induction argument)
2. Getting `spec_elements_distinct` requires wf sub-predicates in scope
3. wf sub-predicates include `spec_parent_rank_same_dom`
4. `spec_parent_rank_same_dom` contains `parent.dom() =~= rank.dom()`
5. `=~=` on Map domains generates `forall|k| dom1.contains(k) <==> dom2.contains(k)`
6. Every `contains_key` in the recursive proof context matches that quantifier
7. ~400MB, never converges

## The fix: eliminate =~= from the wf predicate

The same pattern that worked for `spec_elements_distinct` works for `=~=`:
**wrap it in a closed predicate so Z3 never sees the extensional axiom.**

### Step 1: Replace domain =~= with directional closed predicates

Find every `=~=` in the UnionFind wf sub-predicates. Replace each with two
directional closed predicates:

```rust
// BEFORE (in spec_parent_rank_same_dom or wherever):
uf.parent@.dom() =~= uf.rank@.dom()

// AFTER: two directional predicates
pub closed spec fn spec_parent_dom_implies_rank<V: StT + Hash>(
    uf: &UnionFindStEph<V>
) -> bool {
    forall|k: V::V| uf.parent@.contains_key(k) ==> uf.rank@.contains_key(k)
}

pub closed spec fn spec_rank_dom_implies_parent<V: StT + Hash>(
    uf: &UnionFindStEph<V>
) -> bool {
    forall|k: V::V| uf.rank@.contains_key(k) ==> uf.parent@.contains_key(k)
}
```

Do the same for `roots.dom() =~= parent.dom()` or any other domain equality.

### Step 2: Update the wf groups

Replace the `=~=` conjuncts in whichever wf group contains them (probably group A,
`spec_wf_type_dom`) with the directional predicates. The wf predicate now says
"parent dom subset of rank dom AND rank dom subset of parent dom" instead of
"parent dom =~= rank dom". Logically identical, but Z3 never sees `=~=`.

### Step 3: Write directional reveal lemmas

```rust
proof fn lemma_parent_implies_rank<V: StT + Hash>(uf: &UnionFindStEph<V>, k: V::V)
    requires spec_parent_dom_implies_rank(uf), uf.parent@.contains_key(k),
    ensures uf.rank@.contains_key(k),
{ reveal(spec_parent_dom_implies_rank); }
```

One per direction. These are the only way to extract the pointwise fact from
the closed predicate. Each reveal opens only ONE direction, never both at once,
and never in the same Z3 context as `contains_key` terms from other sources.

### Step 4: Reprove lemma_rank_lt_elements

The recursive proof needs:
- `spec_elements_distinct` (closed, with closed trigger wrapper — done in R173)
- `rank.contains_key(v)` → need `spec_rank_dom_implies_parent` direction
- `parent.contains_key(v)` → need `spec_parent_dom_implies_rank` direction

Each direction is revealed in an isolated helper call, not in the main proof
context. The main proof context never sees `=~=` or the full bidirectional
quantifier.

### Step 5: Fix lemma_union_merge_wf

This function has 3 `=~=` in its requires (`parent.dom`, `rank.dom`, `roots.dom`).
Replace those requires with the directional predicates too. The function should
then verify at rlimit 30 instead of needing rlimit 80 / 19.8 GB.

### Step 6: Fix fn union

After merge_wf is fixed, tackle the exec `union` function. Its trait ensures
has `self@.parent.dom() =~= old(self)@.parent.dom()`. If needed, change the
trait ensures to use a closed `spec_dom_unchanged` predicate.

## The principle

`=~=` on Map/Set domains in SMT = unbounded universal quantifier. In a cubical
type theory, logical equivalence *is* equality (univalence). But Z3 is first-order:
it encodes `A =~= B` as `forall|k| A.contains(k) <==> B.contains(k)`, which
cross-fires with every `contains_key` in scope.

The fix is always the same: wrap `=~=` in a closed predicate, reveal only the
direction you need, only in the proof context that needs it. This is manually
building what a cubical prover gives you for free.

## Order of attack

1. Replace `=~=` in wf sub-predicates with directional closed predicates
2. Fix `lemma_rank_lt_elements` (the recursive induction proof)
3. Fix `lemma_union_merge_wf` (the 19.8 GB monster)
4. Fix `fn union`
5. Validate after each: `scripts/validate.sh isolate Chap65`

## Validation

```bash
scripts/validate.sh isolate Chap65
```

## Report

Write `plans/agent1-round174-report.md`.

## RCP

`git add -A && git commit -m "R174 Agent 1: eliminate =~= from UnionFind wf, prove remaining holes (−N holes)"`, then `git push`.
