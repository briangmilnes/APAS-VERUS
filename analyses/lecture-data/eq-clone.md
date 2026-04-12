# Lecture Data — Section 7: Eq / Clone / View

Generated: 2026-04-11 from main at `7ec8eb2bf`. Counts cover
`src/Chap*/` and `src/vstdplus/`.

## Implementation counts

| # | Construct | Count | Notes |
|---|-----------|-------|-------|
| 1 | `impl Clone for ...` | 156 | |
| 2 | `impl PartialEq for ...` | 94 | |
| 3 | `impl Eq for ...` | 181 | many marker-trait `impl Eq for X {}` |
| 4 | `impl View for ...` | 291 | |
| 5 | `impl DeepView for ...` | 1 | rare; most types use View only |

## Spec-bridge usage

| # | Construct | Count | Purpose |
|---|-----------|-------|---------|
| 6 | `PartialEqSpecImpl` | 162 | trait providing `eq_spec` |
| 7 | `obeys_eq_spec` | 73 | gates eq spec activation |
| 8 | `ClonePreservesView` | 173 | bridge: clone preserves View output |

## Observations

- **156 Clone impls + 173 ClonePreservesView mentions**: nearly 1:1.
  Most Clones in the codebase are paired with a `ClonePreservesView`
  bound to make `x.clone()@ == x@` a usable fact.

- **94 PartialEq impls vs 162 PartialEqSpecImpl mentions**: each
  PartialEq impl typically references the spec impl multiple times
  (in trait bound, in `obeys_eq_spec()`, in `eq_spec()`). The 1:1.7
  ratio is consistent with the eq/clone pattern documented in
  `src/standards/partial_eq_eq_clone_standard.rs`.

- **291 View impls**: roughly one per public type. View is the
  abstraction underlying all proof reasoning about collection state.

- **DeepView is essentially unused** (1 impl) — `View` is sufficient
  for the project's needs. Worth noting if the lecture talks about
  the View vs DeepView design tradeoff.

## Trust-base impact

The eq/clone bridge pattern (per CLAUDE.md, the only `assume` permitted
outside thread-spawn boundaries) accounts for a large share of the
production `assume(...)` count. Each PartialEq impl typically contains
exactly one `assume(r == (self@ == other@))` inside the eq() body, and
each Clone impl typically contains exactly one
`assume(cloned@ == self@)`.

Lower bound on eq/clone-derived assumes: **~250** (94 PartialEq + 156
Clone). The total line-leading `assume` count in `src/Chap*/` is 38 —
substantially smaller, suggesting either:
  - many Clone/PartialEq impls don't carry the assume (use a different
    pattern), OR
  - the assumes live in `vstdplus/` modules, not in `Chap*/`.

Spot-check needed if this is a lecture point.
