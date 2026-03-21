<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 56 Prompt

## Branch

Work on `agent2/ready`. Base: `fd7f38bb4`.

## DO NOT TOUCH

- Chap47 (any file)
- Chap41 (any file — Agent 3 is working there)
- Chap45 (any file — Agent 4 is working there)
- Chap43 files other than OrderedTableStPer.rs

## Assignment: Close 12 from_vec capacity assumes in OrderedTableStPer.rs

**File:** `src/Chap43/OrderedTableStPer.rs`

There are 12 `assume(result_vec@.len() < usize::MAX)` holes, all at `from_vec` call sites.
These were exposed by R55's capacity chain (adding `values@.len() < usize::MAX` to
`AVLTreeSeqStPerS::from_vec` requires).

**Pattern:** Every hole follows the same structure:

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
proof { assume(result_vec@.len() < usize::MAX); }  // <-- THIS IS THE HOLE
let seq = AVLTreeSeqStPerS::from_vec(result_vec);
```

**Fix:** For each hole:

1. Add `result_vec@.len() <= i as nat` to the loop invariant. This is trivially maintained:
   the loop body does at most one `push` per iteration, and `i` increments by 1.

2. After the loop, `result_vec@.len() <= n`. The wf chain now guarantees
   `self.base_set.elements@.len() < usize::MAX` (from the broadcast group
   `group_avltreeseqstper_len_bound`). So `n < usize::MAX` and therefore
   `result_vec@.len() <= n < usize::MAX`.

3. Replace the `assume` with a proof block:
   ```rust
   proof {
       // result_vec@.len() <= n (from loop invariant)
       // n == self.base_set.elements@.len() < usize::MAX (from wf + broadcast)
       assert(result_vec@.len() < usize::MAX);
   }
   ```

4. The broadcast group `group_avltreeseqstper_len_bound` is already in the file's
   `broadcast use`. It fires on `s.spec_avltreeseqstper_wf()` and establishes
   `s@.len() < usize::MAX`. Use this to get `n < usize::MAX`.

**Special cases:**
- `split_key` (line ~2662): builds TWO vecs (`left_vec`, `right_vec`). Both need the same
  treatment — their combined length equals the source length.
- `split_rank_key` (line ~3067-3068): same pattern, two vecs.
- `union` (line ~1690): result_vec grows from both self and other. Need
  `result_vec@.len() <= self@.dom().len() + other@.dom().len()` and the sum capacity bound.

**Approach:** Start with the simplest cases (tabulate, map, filter — single source) and
work through systematically. Validate after every 3-4 fixes to catch issues early.

## Validation

Run `scripts/validate.sh` after each batch of fixes. Show full output. Fix all warnings.

## Report

Write `plans/agent2-round56-report.md` with holes before/after table including Chap column.
