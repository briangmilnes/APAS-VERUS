# Verus Optimizations Applied in APAS-VERUS

A running catalog of the SMT/proof-engineering optimizations the
APAS-VERUS project has discovered and applied. Each entry records
**what the problem was**, **what we tried first that didn't work**,
**what worked**, and **where to see it in the codebase**. The point
is not to claim novelty — most of these are folklore in the Verus /
Z3 community — but to give a concrete reference for future agents
and for talks/papers.

---

## 1. Opaque vs `pub closed spec fn`

**Problem.** `find()` in `src/Chap65/UnionFindPCStEph.rs` had a 35-line
loop invariant containing 15 `forall` quantifiers (well-formedness
restated for two parent maps + find-preservation predicate + same-domain
predicate). Z3 cross-fired every quantifier at every iteration,
producing 139,490 instantiations of the parent-in-domain quantifier
alone, blowing rlimit even at 500.

**What didn't work.** Bundling the quantifiers into `pub closed spec fn`
predicates. `pub closed` only hides the body **across modules** — within
the defining module Verus still unfolds the body, so Z3 saw all the
inner quantifiers.

**What worked.** `#[verifier::opaque]` paired with `pub open spec fn`.
This hides the body inside the defining module too. Z3 sees only the
function symbol until an explicit `reveal(...)` in an isolated
`assert ... by { reveal(...); }` block. The matching loop disappears.

**Pattern.**

```rust
#[verifier::opaque]
pub open spec fn spec_light_wf<V: Bounds>(
    parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat,
) -> bool {
    // forall quantifiers — invisible to Z3 unless revealed
}

while ... invariant
    spec_light_wf::<V>(parent, rank, n),  // opaque — Z3 sees a boolean
    ...
{
    proof {
        assert(... fact you need ...) by {
            reveal(spec_light_wf);   // local reveal, isolated context
        }
    }
}
```

**Where.** `src/Chap65/UnionFindPCStEph.rs` (R195 — `spec_light_wf`,
`spec_find_preserved`, `spec_same_domain`).

**Note on veracity hole counting.** `veracity-review-verus-proof-holes`
appears to count `#[verifier::opaque]` annotations as holes. They are
not — they are proof bundling. Future audits should subtract them
from the trust-base count or veracity should be patched to recognize
them. Tracked separately; not blocking.

---

## 2. Z3 conjunction flakiness (R28 workaround)

**Problem.** Z3 verifies sub-assertions individually
(`assert(c1); assert(c2); assert(c3); assert(c4)`) but fails the
conjunction `assert(c1 && c2 && c3 && c4)`, even though each conjunct
is already proved.

**What worked.** Build the conjunction incrementally and assert the
equivalence to the wf predicate explicitly:

```rust
proof {
    reveal_with_fuel(spec_my_wf, 1);
    assert(c1);
    assert(c2);
    assert(c3);
    assert(c4);
    assert(spec_my_wf(x) == (c1 && c2 && c3 && c4));
}
```

The trailing equivalence assertion forces Z3 to construct the
conjunction in its own context.

**Where.** Pattern documented across many chapters; first encountered
in R28. Memory: `Z3 Conjunction Flakiness Workaround (R28)`.

---

## 3. Broadcast group cross-fire (R175)

**Problem.** `Chap65/KruskalStEph.rs` defined a wf predicate as an
open conjunction of 13 quantifiers. Other broadcast groups in the
file (notably `LabEdge`'s) caused Z3 to instantiate the wf
quantifiers against irrelevant terms, exploding instantiation counts.

**What worked.** Wrap the cross-firing call sites in a **nested
module** that does not import the conflicting broadcast groups:

```rust
pub mod uf_opaque_wrappers {
    use vstd::prelude::*;
    use crate::Chap65::UnionFindStEph::UnionFindStEph::*;
    // No `broadcast use` of the conflicting groups.

    pub proof fn lemma_uf_wf_after_op(...) ensures ... {
        // Z3 sees only the imports needed here.
    }
}
```

When the underlying wf is itself bundled opaquely (per #1), the
wrapper module becomes unnecessary and can be deleted — which is what
happened in R196 once `UnionFindPCStEph` adopted `#[verifier::opaque]`.

**Where.** Removed in R196 (`src/Chap65/KruskalStEph.rs`); historical
form preserved in git history at the commits prior to `63dac5569`.

---

## 4. Decompose wf into independently-proved lemmas (R172, R192)

**Problem.** Monolithic wf-preservation lemmas
(`lemma_compress_step_wf`) consumed too much rlimit, with every
branch of every quantifier being checked together.

**What worked.** Micro-split: produce three (or more) independent
lemmas — `lemma_compress_parent_in_dom`, `lemma_compress_rank_inv`,
`lemma_compress_basic` — each proving one wf clause, then a top-level
lemma that combines them with the conjunction-flakiness workaround
above.

**Where.** `src/Chap65/UnionFindPCStEph.rs` (R192).

---

## 5. Closure first-class workarounds (`clone_fn`, `clone_pred`)

**Problem.** Verus does not (as of `ff454ab0f`) recognize `Clone` on
closures or named function items. Generic D&C helpers that take
`f: F` and want to use `f` in two recursive arms cannot prove
`f.requires` survives the call.

**What worked.** The `clone_fn` family in `src/vstdplus/`:

- `clone_fn`, `clone_fn2` — clone closures for join arms while
  preserving `requires`/`ensures`.
- `clone_pred` — same for predicates.

Verus 3390e9af0 (first-class `Fn`/`Copy`) recognizes `Copy` but not
`Clone`, and our trait signatures need `Clone` — so the workaround
remains. Tracked: when Verus accepts `Clone` on closures, these
helpers can be removed.

**Where.** `src/vstdplus/clone_fn.rs`, `src/vstdplus/clone_pred.rs`.

---

## 6. Ghost-capture-before-call (R195)

**Problem.** Inside a method like `find()`, the loop invariant needs
to talk about the original `self` state from before the inner
`find_root(v)` call, but `&self` calls don't reliably preserve
identity through trait dispatch in Verus's loop context.

**What worked.** Pull `let ghost orig_parent = self.parent@; let ghost
orig_rank = self.rank@; let ghost orig_n = self.spec_n();` **before**
the inner call, and add `orig_parent == old(self).parent@` etc. to
the loop invariant.

**Where.** `src/Chap65/UnionFindPCStEph.rs::find` (R195).

---

## 7. Iterator `assume(iter_invariant)` policy

**Problem.** Verus does not allow `requires` on external trait impls
(`std::iter::Iterator::next`). Hand-rolled iterators that don't wrap
`std::slice::Iter` need `assume(iter_invariant(self))` in `next()` to
let the body assume the invariant.

**What works.** Allow exactly one `assume(iter_invariant(self))` at
the top of the iterator's `next()`, then **prove everything after**.
The assume is non-negotiable until Verus supports requires on external
impls; the policy is to flag every such assume in a per-round table
for user review, never convert silently to `accept()`.

**Where.** Confirmed by `src/experiments/iter_requires_on_external_trait.rs`.
Pattern visible across iterator-bearing collections in every chapter.

---

## 8. Help-First Scheduler instead of raw thread::spawn

**Problem.** Naive `std::thread::spawn` can't be wrapped tightly with
Verus specs without large `external_body` regions.

**What worked.** All fork-join goes through `HFScheduler` (Help-First,
work-stealing) in `src/Chap02/HFSchedulerMtEph.rs`. The scheduler
exposes a verified `join` API; only its tiny thread-spawn primitives
are `external_body`.

**Where.** `src/Chap02/HFSchedulerMtEph.rs`. Used by every Mt module.

---

## 9. Per-module trait impl pattern + RwLockPredicate naming

**Problem.** Bare `impl Type` blocks don't carry specs — each method
has to repeat its `requires`/`ensures` per call site.

**What worked.** Every module defines a public trait containing all
public functions with specs in the trait. The struct impl is purely
mechanical. RwLockPredicates follow `XInv` naming for module `X`.

**Where.** Codebase-wide. See `.cursor/rules/apas-verus/trait-impl-pattern.mdc`
and `.cursor/rules/apas-verus/rwlock-predicate-naming.mdc`.

---

## 10. Eq/Clone bridge: assume inside the impl body

**Problem.** Verus cannot prove `r == (self@ == other@)` for the
return value of a `PartialEq::eq` impl that delegates to `inner`'s
`==`, because `==` on the inner type goes through the same trait Verus
can't introspect.

**What worked.** Allow exactly one `assume(r == (self@ == other@))`
inside the `eq()` body, paired with `obeys_eq_spec() == true` and
`eq_spec` declarations on `PartialEqSpecImpl`. The assume is bounded
to the impl body; algorithmic code obtains the property through the
`ensures` clause of `eq()`.

**Where.** Pattern documented in
`src/standards/partial_eq_eq_clone_standard.rs`. Used in every
collection that implements PartialEq.

---

## Future entries (placeholder)

Append new entries here as we discover them. Each entry should follow
the `Problem / What didn't work / What worked / Where` structure so
it's actionable for the next agent.
