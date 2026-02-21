# Derive Experiments: Verus + RTT Results

Each experiment tests `#[derive(TR)]` on struct and enum inside `verus!`.
RTT = runtime tests in `tests/experiments/TestDerive*.rs` that **actually call** the derived method.
Hypothesis/result are in each experiment file header; this doc summarizes.

## Works Table

| # | Trait | Struct | Enum | RTT calls |
|---|-------|:-----:|:----:|-----------|
| 1 | Clone | ✅ | ✅ | `.clone()` |
| 2 | Copy | ✅ | ✅ | assignment (copy semantics) |
| 3 | Debug | ✅ | ✅ | `format!("{:?}", x)` |
| 4 | Default | ✅ | ✅ | `T::default()` |
| 5 | Eq | ✅ | ✅ | `==`, `!=` |
| 6 | Hash | ✅ | ✅ | `HashSet::insert`, `contains` |
| 7 | Ord | ✅ | ✅ | `.cmp()`, `<`, `>` |
| 8 | PartialEq | ✅ | ✅ | `==`, `!=` |
| 9 | PartialOrd | ✅ | ✅ | `.partial_cmp()`, `<`, `>` |

- **Verus:** 615 verified, 0 errors (experiments_only)
- **RTT:** 18 tests, 18 passed

## Test Files

| Trait | Test file | What it calls |
|-------|-----------|---------------|
| Clone | `TestDeriveClone.rs` | `s.clone()`, `e.clone()` |
| Copy | `TestDeriveCopy.rs` | `let t = s` (copy), `let f = e` |
| Debug | `TestDeriveDebug.rs` | `format!("{:?}", s)`, `format!("{:?}", e)` |
| Default | `TestDeriveDefault.rs` | `SStruct::default()`, `EEnum::default()` |
| Eq | `TestDeriveEq.rs` | `a == b`, `a != c` |
| Hash | `TestDeriveHash.rs` | `HashSet::insert`, `set.contains()` |
| PartialEq | `TestDerivePartialEq.rs` | `a == b`, `a != c` |
| PartialOrd | `TestDerivePartialOrd.rs` | `a < b`, `a.partial_cmp(&b)` |
| Ord | `TestDeriveOrd.rs` | `a < b`, `a.cmp(&b)` |

## mut_refs_and_mut_returns Experiment

| Case | Hypothesis | Result |
|------|------------|--------|
| &mut self with old() | Spec mutation via old(self) in requires/ensures | Verifies |
| fn -> &mut T | Return mutable ref with ensures | Fails (use new-mut-ref) |
| mut return (owned) | Return owned T | Works normally |

Useful proof: Counter.inc() and Counter.add(n) with ensures let callers prove
exact state changes (val increases by 1 or n).

## Run Commands

```bash
# Verus (experiments_only)
~/projects/verus/source/target-verus/release/verus --crate-type=lib src/lib.rs \
  --cfg 'feature="experiments_only"'

# RTT (derive tests only)
cargo nextest run --test TestDeriveClone --test TestDeriveCopy --test TestDeriveDebug \
  --test TestDeriveDefault --test TestDeriveEq --test TestDeriveHash \
  --test TestDerivePartialEq --test TestDerivePartialOrd --test TestDeriveOrd
```
