# Agent 4 — Round 50 Prompt

## Primary Objective: ClonePreservesView Trait

Create a `ClonePreservesView` marker trait in `src/vstdplus/` that captures the property
`clone()@ == self@` for generic code, then wire it into the type hierarchy so the
`assume(spec_graphview_wf(quotient@))` in Chap62 StarContraction can be proved.

### Background

In Verus, generic `Clone::clone` carries no postcondition. Our `partial_eq_eq_clone_standard.rs`
puts `ensures result@ == self@` on concrete Clone impls (with an assume inside the body),
but when code is generic over `V: HashOrd` (which includes `Clone + View`), callers can't
use that postcondition. This blocks proving that cloned vertices maintain their view, which
blocks proving quotient graph well-formedness in `build_quotient_graph`.

### Steps

1. **Read the standard**: `src/standards/partial_eq_eq_clone_standard.rs` — understand
   the existing Clone patterns, especially the `obeys_feq_clone` broadcast pattern.

2. **Create the trait** in `src/vstdplus/clone_view.rs` (or a suitable location):
   ```rust
   // Inside verus!
   pub trait ClonePreservesView: Clone + View {
       fn clone_view(&self) -> (result: Self)
           ensures result@ == self@;
   }
   ```
   The trait provides a `clone_view` method with the view-preserving postcondition.
   Concrete impls delegate to `self.clone()` with the standard assume bridge inside.

3. **Implement for key types**: At minimum, implement for `usize`, `u64`, `i64`, and
   any concrete types used as graph vertices. These impls contain the assume bridge
   (approved pattern per `partial_eq_eq_clone_standard.rs`).

4. **Add `ClonePreservesView` as a supertrait of `HashOrd`** in `src/Types.rs`:
   ```rust
   pub trait HashOrd: StT + Hash + Ord + ClonePreservesView {}
   impl<T> HashOrd for T where T: StT + Hash + Ord + ClonePreservesView {}
   ```

5. **Use `clone_view()` in `build_quotient_graph`** (both StEph and MtEph in Chap62)
   instead of `clone()`. Since `V: HashOrd` now implies `ClonePreservesView`, the
   postcondition `result@ == self@` flows through, and you can prove
   `spec_graphview_wf(quotient@)` — removing the 2 assumes.

6. **Audit and fix cascading effects**: Adding `ClonePreservesView` to `HashOrd` may
   require updating concrete type impls that currently satisfy `HashOrd` via the blanket
   impl. Check that `usize`, `u64`, `i64`, `Edge<V>`, and other graph vertex types
   get `ClonePreservesView` impls.

7. **Alternative approach**: If adding to `HashOrd` causes too many cascading changes,
   add `ClonePreservesView` as a direct bound on `build_quotient_graph` and
   `star_contract` instead of modifying `HashOrd`. The goal is removing the 2 assumes.

### Validation

- `scripts/validate.sh` — 0 errors, no trigger warnings
- `scripts/rtt.sh` — all pass
- `scripts/ptt.sh` — all pass
- The 2 `assume(spec_graphview_wf(quotient@))` in Chap62 must be gone

### Files to Read First

- `src/standards/partial_eq_eq_clone_standard.rs`
- `src/Types.rs` (HashOrd definition, line ~78)
- `src/Chap62/StarContractionStEph.rs` (build_quotient_graph + assume)
- `src/Chap62/StarContractionMtEph.rs` (build_quotient_graph_parallel + assume)
- `src/vstdplus/` directory listing (find the right home for the new trait)

### Current State

- Main at post-merge of agent4 R49
- 4450 verified, 2613 RTT, 147 PTT
- Agent4 R49 replaced 2 external_body with 2 focused assumes on quotient graph wf
- Those 2 assumes are your targets
