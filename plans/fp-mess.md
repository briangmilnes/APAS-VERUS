# Floating Point in APAS-VERUS: Current State and Options

## What vstd Provides (verus-lang latest)

### `vstd/float.rs`
- `FloatBitsProperties` trait: `is_finite_spec()`, `is_nan_spec()`, `is_infinite_spec()`,
  `to_bits_spec()` for f32/f64.
- Clone specs for f32/f64.
- Float casting (`ieee_float_cast`, `float_cast_spec`).
- No `View` impl for f32/f64. No View impl for any primitive — View is for
  types where abstract != concrete (Vec→Seq, HashMap→Map).

### `vstd/std_specs/cmp.rs`
- Uninterpreted functions for comparison results:
  `le_ensures<A>(x, y, o)`, `eq_ensures<A>(x, y, o)`, `partial_cmp_ensures<A>(x, y, o)`,
  `lt_ensures`, `gt_ensures`, `ge_ensures`.
- `assume_specification` for `<f64 as PartialOrd>::le`, etc. — each ensures
  the corresponding uninterpreted function.
- Deliberately does NOT set `obeys_partial_cmp_spec() == true` for floats because
  Rust float operations are not guaranteed deterministic (RFC 3514).

### `vstd/std_specs/ops.rs`
- Uninterpreted functions for arithmetic results:
  `add_ensures<A>(x, y, o)`, `sub_ensures<A>(x, y, o)`, `mul_ensures<A>(x, y, o)`,
  `div_ensures<A>(x, y, o)`, `neg_ensures<A>(x, o)`.
- `assume_specification` for `<f64 as Add>::add`, etc. — each ensures
  the corresponding uninterpreted function.
- Again, no `obeys_add_spec() == true` for floats.

### `vstd/laws_cmp.rs`
- Proves `obeys_cmp_spec` for all integer types. No floats.

### Summary
vstd gives us uninterpreted hooks (`add_ensures`, `le_ensures`, etc.) that fire
when native operators (`+`, `<=`) are used on f64 inside verus. It provides no
axioms about those hooks — that's deliberately left to projects.

## What We Built (`vstdplus/float.rs`)

### `FloatTotalOrder` trait
- Reflexive, antisymmetric, transitive, totality axioms for finite f64/f32.
- Built on `le_ensures` from vstd.
- Broadcast axiom group `group_float_finite_total_order`.

### `WrappedF64` newtype
- Exists solely to give f64 a `View` impl (`View::V = f64`).
- Our containers (AVLTreeSeqStEphS, etc.) require `V: View`, so raw f64 can't
  be stored directly.
- Provides exec methods: `dist_le`, `dist_lt`, `dist_add`, `dist_sub`,
  `approx_eq` — all `#[verifier::external_body]`.

### Uninterpreted arithmetic specs
- `f64_add_spec(a, b) -> f64` (2-argument)
- `f64_sub_spec(a, b) -> f64` (2-argument)
- `f64_approx_eq_spec(a, b) -> bool`

### Arithmetic axioms
- `axiom_f64_add_zero_right`
- `axiom_f64_add_commutative`
- `axiom_f64_add_finite_preserves`
- `axiom_f64_add_monotone_left`
- Broadcast group `group_float_arithmetic`.

### Sentinels
- `UNREACHABLE_SPEC()` — uninterpreted, axiomatized as not finite (runtime: infinity).
- `zero_dist()`, `finite_dist(v)`, `unreachable_dist()` — constructors.

## The Problems

### 1. Two Disconnected Worlds

vstd's `assume_specification` for `<f64 as Add>::add` ensures `add_ensures::<f64>(x, y, o)`
— a 3-argument uninterpreted function (inputs + output).

Our `dist_add` ensures `r@ == f64_add_spec(self@, other@)` — a 2-argument
uninterpreted function (inputs only, deterministic output).

These are different uninterpreted functions. Z3 knows nothing connects them.
If code uses `+` on f64 inside verus, it gets `add_ensures`. If code calls
`dist_add` on WrappedF64, it gets `f64_add_spec`. Our axioms (commutativity,
monotonicity, etc.) are stated over `f64_add_spec`, so they don't apply to
native `+`. Native `+` has no axioms at all.

Same disconnect for comparison: our `FloatTotalOrder::le` is defined as
`le_ensures::<f64>(self, other, true)`, which does connect to vstd's `<=`.
But arithmetic doesn't connect.

### 2. WrappedF64 Exists Only for View

The only reason WrappedF64 exists is that our containers require `V: View`.
It's a newtype wrapper that adds no semantic value — `View::V = f64`, so
`w@ == w.val`. All its methods are external_body wrappers around raw f64
operations. Every float operation in algorithm code goes through WrappedF64
methods instead of native operators, which means algorithm code never triggers
vstd's operator specs.

If we could use raw f64 in containers, WrappedF64 would be unnecessary.
But relaxing the View bound on containers is a deep structural change.

### 3. Ad-Hoc Arithmetic Axioms

We have four arithmetic axioms (zero identity, commutativity, finite preservation,
left monotonicity). Missing: associativity, right monotonicity, sub properties,
connection to ordering (`a + positive >= a`), triangle inequality, etc.

These axioms are stated over `f64_add_spec` which is our own uninterpreted function,
not vstd's `add_ensures`. They work only through WrappedF64's `dist_add`.

### 4. Scale of the Problem

21 files use WrappedF64 or float-related code:
- Chap56: PathWeightUtils, SSSPResult, AllPairsResult (St/Per variants)
- Chap57: DijkstraStEphF64
- Chap58: BellmanFordStEphF64
- Chap64: TSPApproxStEph
- Chap65: PrimStEph, KruskalStEph
- Chap66: BoruvkaStEph, BoruvkaMtEph
- Chap30: Probability
- vstdplus: float.rs, partial_order.rs
- experiments: f64_float_cmp_sort, f64_sort, f64_bits_sort

## Options

### Option A: Bridge the Two Worlds

Add axioms connecting `f64_add_spec` to `add_ensures`:
```
axiom fn bridge_add(a: f64, b: f64)
    ensures add_ensures::<f64>(a, b, f64_add_spec(a, b));
```

This lets native `+` produce results equal to `f64_add_spec`, so our existing
axioms apply. WrappedF64 stays. Minimal code changes — just add bridge axioms
and the existing 21 files keep working.

**Pro**: Smallest change. Unblocks proofs that use native `+`.
**Con**: Still two parallel systems. WrappedF64 stays. Doesn't simplify anything.

### Option B: Restate Axioms Over vstd's Functions

Replace `f64_add_spec` with axioms directly over `add_ensures`:
```
axiom fn add_commutative(a: f64, b: f64, o1: f64, o2: f64)
    requires add_ensures::<f64>(a, b, o1), add_ensures::<f64>(b, a, o2),
    ensures o1 == o2;
```

WrappedF64's methods become thin wrappers that call native operators, inheriting
vstd's specs automatically. Or WrappedF64 methods could be eliminated entirely
if algorithm code uses native operators.

**Pro**: Single system. Aligns with vstd's design philosophy.
**Con**: 3-argument ensures is awkward (need to existentially quantify the output
in many places). Significant rewrite of float.rs and all 21 consumer files.

### Option C: Make f64_add_spec Deterministic and Connect to add_ensures

Keep our 2-argument `f64_add_spec(a, b) -> f64` as the "deterministic model"
of float addition (assuming IEEE 754 determinism for finite values on a single
platform). Add a bridge axiom that says `add_ensures` always agrees with our
deterministic model for finite values:
```
axiom fn f64_add_deterministic(a: f64, b: f64, o: f64)
    requires a.is_finite_spec(), b.is_finite_spec(),
             add_ensures::<f64>(a, b, o),
    ensures o == f64_add_spec(a, b);
```

This says: for finite inputs, f64 addition is deterministic and equals our
spec function. Then native `+` on finite f64 produces `f64_add_spec(a, b)`,
and all our axioms apply.

**Pro**: Best of both worlds. Clean 2-arg spec for reasoning, connected to
vstd's operator specs. Minimal changes to consumer files.
**Con**: The determinism axiom is a platform assumption (true on all modern
hardware but not guaranteed by Rust's semantics — RFC 3514).

### Option D: Ditch WrappedF64

If containers could accept types without View (using the type itself as its
own spec representation), WrappedF64 would be unnecessary. This would require
either:
- Adding `impl View for f64` somewhere (View::V = f64), or
- Relaxing View bounds on containers to accept raw primitives.

**Pro**: Eliminates the wrapper entirely. Algorithm code uses native f64.
**Con**: Deep container trait changes. View is pervasive. May not be feasible
without Verus changes.

## What We Don't Know

1. Does the latest Verus allow `impl View for f64`? (Verus may reject it for
   primitives.) Need to test.
2. Is the determinism assumption (Option C) sound enough for the project?
   APAS is a textbook — platform-specific behavior matters less than
   mathematical correctness.
3. How much of the graph chapter code (Chap59-66) actually needs float
   arithmetic vs. just float comparison? Comparison is already connected
   through `le_ensures`. Only arithmetic is disconnected.
4. Can vstd's `add_ensures` be extended upstream to provide the axioms we
   need? (Probably not — vstd explicitly chose to leave floats unaxiomatized.)

## Recommendation

No recommendation yet. This needs discussion. The cleanest path is probably
Option C (deterministic bridge) combined with keeping WrappedF64 as the View
wrapper but making its methods use native operators internally. But the scope
of changes and the soundness of the determinism assumption need to be evaluated.
