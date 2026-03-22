<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 57 Prompt

## Branch

Work on `agent1/ready`. Push when done.

## DO NOT TOUCH

- Chap47 (any file)
- Chap41 (any file — Agent 2)
- Chap45 (any file — Agent 2)
- Chap05 (any file — Agent 3)
- Chap42 (any file — Agent 3)
- Chap62, Chap63, Chap64 (any file — Agent 4)
- Any file in any other chapter not listed in your assignment

## Assignment: Close 12 capacity assumes in OrderedTableStPer.rs

**File:** `src/Chap43/OrderedTableStPer.rs`

There are 12 `assume(result_vec@.len() < usize::MAX)` (and 2 `left_vec`/`right_vec` variants)
at `from_vec` call sites. These were exposed by R55's capacity chain.

### The Pattern

Every hole follows this structure:

```rust
while i < n
    invariant
        i <= n,
        ...
    decreases n - i,
{
    result_vec.push(...);
    i += 1;
}
proof { assume(result_vec@.len() < usize::MAX); }  // THE HOLE
let seq = AVLTreeSeqStPerS::from_vec(result_vec);
```

### The Fix

For each hole:

1. Add `result_vec@.len() <= i as nat` (or equivalent) to the loop invariant.
   This is trivially maintained: the loop body does at most one `push` per
   iteration, and `i` increments by 1.

2. After the loop, `result_vec@.len() <= n`. The wf chain guarantees
   `self.base_set.elements@.len() < usize::MAX` via the broadcast group
   `group_avltreeseqstper_len_bound`. So `n < usize::MAX` and therefore
   `result_vec@.len() <= n < usize::MAX`.

3. Delete the `assume` line. The `assert` should follow automatically from
   the loop invariant and the wf bound. If Verus needs help, add:
   ```rust
   assert(result_vec@.len() < usize::MAX);
   ```

4. The broadcast group `group_avltreeseqstper_len_bound` is already in the
   file's `broadcast use`.

### Special Cases

- `split_key` (line ~2662): Builds TWO vecs (`left_vec`, `right_vec`). Both
  need the treatment. Their combined length equals the source length, so each
  is bounded.
- `split_rank_key` (line ~3067-3068): Same — two vecs, combined ≤ source.
- `union` (line ~1690): result_vec grows from BOTH self and other. Need
  `result_vec@.len() <= self@.dom().len() + other@.dom().len()` and both
  sources are bounded by wf.

### Approach

Start with the simplest cases (tabulate, map, filter — single source).
Validate after every 3-4 fixes. Work through all 12 systematically.

## Validation

Run `scripts/validate.sh` after each batch. Show full output. Fix all warnings and errors.

## Report

Write `plans/agent1-round57-report.md` with holes before/after table including Chap column.
