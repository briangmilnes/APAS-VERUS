# Agent 1 — Round 14

## You crushed Round 13: -57 holes across two sessions. Best agent this round.

Keep the momentum. You know these files better than anyone now.

## Your files

**Chap43 Mt — remaining 24 holes across 4 files:**

### OrderedTableMtEph.rs (11 external_body)
- map, filter, reduce, collect, previous_key, next_key, split_key,
  get_key_range, rank_key, split_rank_key, from_sorted_entries

You already proved `collect` and `from_sorted_entries` patterns for OrderedTableMtPer
using the **collect+while loop** technique (length/nth instead of for-loop iteration).
Apply the same pattern here:
1. Remove `external_body`
2. Replace `for pair in ...` with `while i < seq.length() { let pair = seq.nth(i); ... }`
3. Add loop invariants for the accumulated result
4. For `map`/`filter`: build result Vec in while loop, call `from_sorted_entries`
5. For `reduce`: fold through while loop
6. For `split_key`/`split_rank_key`: acquire_write, call inner StEph method, release
7. For `previous_key`/`next_key`/`rank_key`/`get_key_range`: acquire_read, call inner, release

### OrderedSetMtEph.rs (9 holes — 5 depend on StEph wf)
- 5× assume(wf) after split/get_range/split_rank — blocked until StEph ensures wf
- 2× reader ghost!=locked gap (size, find)
- 1× filter (Pred trait lacks f.requires)
- 1× to_seq external_body

**Do the to_seq first** — same collect+while pattern. The 5 wf assumes depend on Agent 2
fixing StEph. Leave those for now. The 2 reader gaps and filter are structural.

### AugOrderedTableMtEph.rs (2 external_body)
- calculate_reduction: closure requires — if reducer.requires can't be proved, leave it
- reduce_range_parallel: ParaPair thread boundary — leave if truly blocked

### AugOrderedTableStPer.rs (2 assume)
- calculate_reduction: `assume(reducer.requires((...)))` — needs f.requires in fn requires
- join_key: `assume(left.reducer.requires((...)))` — same pattern

For these 2: add `requires reducer.requires(...)` or `requires f.requires(...)` to the
function signature and propagate to callers. Read `src/standards/using_closures_standard.rs`.

## DO NOT

- Touch OrderedSetStEph/StPer, OrderedTableStEph/StPer (Agent 2)
- Touch Chap41 Mt files (Agent 4)
- Touch Chap38, Chap39, Chap42, Chap47

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent1/ready`. Write `plans/agent1-round14-report.md`.
- Use the collect+while loop pattern you already proved. It works. Replicate it.

## Target: OrderedTableMtEph 11 → ≤ 5. Total -8.
