# R144 Agent 2 — Fix join_pair to require disjoint T1 < T2 (Chap38). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/BSTParaMtEph.rs` — join_pair and join_pair_inner.
Read `src/Chap39/BSTTreapStEph.rs` — example of disjoint union usage pattern.
  Search for `join_pair_inner` and `disjoint` to see how callers prove the
  precondition and use the result.
Read `prompts/Chap38.txt` — APAS Algorithm 38.4 (joinPair).

Report file: `plans/r144-agent2-joinpair-disjoint-report.md`

## Problem

1 DIFFERS:
```
BSTParaMtEph.rs: join_pair — should add requires T1 < T2 and delegate to join_pair_inner
```

APAS Algorithm 38.4 (joinPair) assumes T1 < T2: all keys in the left tree are
less than all keys in the right tree. Cost: O(lg n) work, O(lg n) span.

Our `join_pair` has ensures `self@.union(other@)` without the disjoint precondition.
It delegates to `union_inner` (general parallel union), giving O(m·lg(n/m)) work.

`join_pair_inner` already exists with the correct disjoint precondition and O(lg n)
cost. The fix: make `join_pair` match APAS by adding the disjoint requires and
delegating to `join_pair_inner`.

## What to do

1. Read `join_pair_inner`'s requires and ensures in both the trait and impl.

2. Update `join_pair`'s trait declaration:
   - Add requires: all keys in self are less than all keys in other (same
     precondition as join_pair_inner)
   - Keep ensures: `joined@ == self@.union(other@)` (still correct for disjoint sets)

3. Update `join_pair`'s impl body:
   - Replace `union_inner` call with `join_pair_inner` call
   - The proof should be simpler since join_pair_inner already proves the result

4. Update ALL callers of `join_pair` in the codebase:
   - Search for `.join_pair(` across all files
   - Each caller must now prove the disjoint precondition
   - Reference `src/Chap39/BSTTreapStEph.rs` for how callers establish disjointness
     (typically after a split, the left and right halves are disjoint by construction)

5. Update the annotation from DIFFERS to matches APAS.

## Callers to check

```bash
grep -rn "\.join_pair(" src/ --include="*.rs" | grep -v analyses
```

Most callers will be after split operations, where disjointness is guaranteed.
If any caller can't prove disjointness, that caller should use `union` instead
of `join_pair`.

## Validation

Run `scripts/validate.sh isolate Chap38`. Check callers' chapters too. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Every caller must prove the new requires — no assume workarounds.

## When done

RCP.
