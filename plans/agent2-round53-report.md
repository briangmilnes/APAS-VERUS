<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 53 Report

**Branch:** `agent2/ready`
**Date:** 2026-03-21
**Verified:** 4476 (unchanged)

## Summary

No code changes this round. The session was lost to a wrong-prompt incident.

## What Happened

The round 53 Cursor session for agent2 received the agent4 round 52 prompt
by mistake. The session executed agent4's work (closing the Chap39
`filter_parallel` hole) in the agent4 worktree at
`/home/milnes/projects/APAS-VERUS-agent4/`, then committed and pushed to
`agent4/ready`. That work is complete and correct.

The correct agent2 round 53 prompt arrived mid-session. Investigation of
the target files was performed (read `using_closures_standard.rs`, read
`AVLTreeSetStEph.rs` union loop, read callers in Chap43 and Chap53) but no
code changes were made before the session ended.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Change |
|---|:----:|---|:----:|:----:|:----:|
| 1 | 41 | AVLTreeSetStEph.rs | 1 | 1 | 0 |
| 2 | 43 | AugOrderedTableMtEph.rs | 1 | 1 | 0 |
| 3 | 43 | AugOrderedTableStPer.rs | 2 | 2 | 0 |
| 4 | 43 | OrderedSetStEph.rs | 1 | 1 | 0 |
| 5 | 43 | OrderedSetStPer.rs | 2 | 2 | 0 |
| 6 | 43 | OrderedTableMtPer.rs | 1 | 1 | 0 |
| **Total** | | | **8** | **8** | **0** |

## Investigation Findings (for next round)

**Chap41 union capacity hole** (`AVLTreeSetStEph.rs:722`):

The fix is straightforward. The second loop in `union` has invariant
`combined@.len() <= self_len as nat + j as nat`. Adding
`requires self@.len() + other@.len() < usize::MAX as nat` to the trait's
`union` method (and the loop invariant) allows replacing the `assume` with
an `assert`. Cascade: `OrderedSetStEph.union` needs the same requires
propagated. Chap53 callers that union with singletons need
`self@.len() + 1 < usize::MAX`, which is one step weaker than wf
(`self@.len() < usize::MAX`); those callers may need a separate requires.

**Chap43 `from_sorted_seq` assumes** (`OrderedSetStEph.rs:1134`,
`OrderedSetStPer.rs:1031`): Both assume that `self@.filter(pred)` equals
the set view — this is a direct consequence of the filter semantics once
the predicate is known. Investigation pending.

**AugOrderedTableStPer closure assumes** (`AugOrderedTableStPer.rs:117+124`):
The `lemma_reducer_clone_total` proof function assumes closure requires/ensures.
Per `using_closures_standard.rs`, the fix is to lift those obligations into
the lemma's own `requires` clause.

## Blockers

None intrinsic — session time was the only blocker this round.
