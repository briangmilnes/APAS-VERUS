# R109 Agent 2 — Rename spec_impl_wf → spec_parahashtablesteph_wf. DOT.

## Objective

Rename `spec_impl_wf` to `spec_parahashtablesteph_wf` across all 7 Chap47 files.
This follows the project naming standard: `spec_<module>_wf`.

## Files (all in src/Chap47/)

| # | File | Role |
|---|------|------|
| 1 | ParaHashTableStEph.rs | Trait declaration + default |
| 2 | VecChainedHashTableStEph.rs | Impl (uses default) |
| 3 | LinkedListChainedHashTableStEph.rs | Impl (uses default) |
| 4 | StructChainedHashTable.rs | Impl (overrides) |
| 5 | DoubleHashFlatHashTableStEph.rs | Impl (overrides) |
| 6 | LinProbFlatHashTableStEph.rs | Impl (overrides) |
| 7 | QuadProbFlatHashTableStEph.rs | Impl (overrides) |

## How

Global find-and-replace `spec_impl_wf` → `spec_parahashtablesteph_wf` in all 7 files.
That's it. No logic changes.

`scripts/validate.sh isolate Chap47`

Commit when clean.

## Rules

- DOT. Rename only. No other changes.
- Do NOT touch files outside Chap47.
- Do NOT add assumes, accepts, or weaken specs.
- No subagents.

## Report

Write `plans/agent2-r109-rename-report.md`.
