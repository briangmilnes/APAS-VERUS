# R138 Agent 4 — Merge TotalOrderBridge into TotalOrder. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/vstdplus/total_order.rs` — the existing TotalOrder trait.

Report file: `plans/r138-agent4-totalorder-bridge-report.md`

## Problem

You created `TotalOrderBridge` in R137 as a separate trait to bridge `Ord::cmp_spec`
to `TotalOrder::le`. This works but adds a separate trait that rippled across 132
call sites. The bridge should be part of `TotalOrder` itself.

## What to do

1. Read the current `TotalOrder` trait in `src/vstdplus/total_order.rs`.

2. Add the bridge lemmas directly to `TotalOrder`:
   ```rust
   proof fn cmp_spec_less_implies_le(a: Self, b: Self)
       requires a.cmp_spec(&b) == Ordering::Less
       ensures TotalOrder::le(a, b);
   proof fn cmp_spec_greater_implies_le(a: Self, b: Self)
       requires a.cmp_spec(&b) == Ordering::Greater
       ensures TotalOrder::le(b, a);
   ```

3. Add `Ord` as a supertrait of `TotalOrder` if it isn't already. TotalOrder
   types should always be Ord — they represent the same concept.

4. Remove the `TotalOrderBridge` trait entirely.

5. Replace all 132 `TotalOrderBridge` references with `TotalOrder`.

6. Verify all existing TotalOrder impls still work with the new bridge lemmas.
   The impls for u8..u128, i8..i128, usize, isize should have empty proof bodies
   (Z3 proves from concrete definitions). String may need assumes — check.

## Validation

Run `scripts/validate.sh`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes in the bridge lemma impls for numeric types — Z3 should
  prove these automatically.
- If String's impl needs an assume, document it.
- The final state should have zero references to `TotalOrderBridge` anywhere.

## When done

RCP.
