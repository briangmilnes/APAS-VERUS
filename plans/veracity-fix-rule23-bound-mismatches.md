# Veracity Rule [23] Fix: Free Function Type Parameter Bound Mismatches

## Problem

Rule [23] fires 230 times. It flags free functions whose type parameter bounds differ from the module's trait. For example:

```
free fn iter_invariant param T has bounds `(none)` but trait MathSeqSTrait has `StT`
free fn valid_key_type param T has bounds `View + Clone + Eq` but trait SetStEphTrait has `StT + Hash`
free fn spec_inject param T has bounds `(none)` but trait FooTrait has `StT + Ord`
```

## What the rule should do

The rule is checking the right thing — bound mismatches — but it should be **split into two subcategories** with different severity:

**[23a] Spec/proof fns with looser bounds than the trait: INFO, not WARNING.**

This is *intentional and correct* per our standards. `table_of_contents_standard.rs` section 6b explicitly says: "Spec functions can have LOOSER bounds than the trait (section 8). Here: T: View (spec-only). The trait requires T: View + Copy + PartialEq." Spec and proof functions operate at spec level where exec bounds like `Clone`, `Hash`, `Send`, `Sync`, `'static`, `Ord`, `PartialEq`, `Eq`, `Display`, `Debug` are irrelevant. A spec fn that takes `T: View` when the trait requires `T: StT + Hash` is correct — it uses the minimum bounds needed for spec reasoning.

These 230 warnings break down as:
- 81 have bounds `(none)` — pure spec/proof fns on unconstrained types
- 17 have bounds `View` — spec fns that only need the view
- The rest have various subsets of the trait bounds

Demote all of these to info when the free function's bounds are a **subset** of the trait's bounds (i.e., the free fn is looser). The free fn is callable from the trait context because the trait provides the superset bounds.

**[23b] Free fns with STRICTER or INCOMPATIBLE bounds: keep as WARNING.**

If a free fn requires bounds the trait does NOT have, that's a real problem — callers through the trait can't satisfy the free fn's requirements. Example: free fn requires `Ord` but trait only has `Eq`. This should remain a warning.

## Detection logic

For each free fn with type parameter `T` that has bounds `B_fn`, find the enclosing module's trait with the same type parameter `T` with bounds `B_trait`:

- If every bound in `B_fn` is present in `B_trait` (or implied by a bound in `B_trait` — e.g., `StT` implies `View + Eq + Clone + Display + Debug + Sized`): **[23a] info** — fn is looser, intentional.
- If `B_fn` contains bounds NOT in `B_trait`: **[23b] warning** — fn is stricter or incompatible.

## Trait alias expansion

The bound comparison must expand trait aliases:
- `StT` = `Eq + PartialEq + Clone + Display + Debug + Sized + View`
- `StTInMtT` = `StT + Send + Sync + 'static`
- `MtKey` = `StTInMtT + Ord + 'static`
- `MtVal` = `StTInMtT + 'static`

So `B_fn = View` vs `B_trait = StT` -> fn is looser (View is a subset of StT) -> [23a] info.
And `B_fn = (none)` vs `B_trait = StT + Ord` -> fn is looser -> [23a] info.
But `B_fn = Ord` vs `B_trait = StT` -> fn has `Ord` which trait lacks -> [23b] warning.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust source. All edits must be token-aware or AST-aware. Parse type parameter bounds with brace/comma/semicolon awareness. A string-hacking detector will flag and kill tools that corrupt source syntax.
