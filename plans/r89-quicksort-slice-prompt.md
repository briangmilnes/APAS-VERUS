# R89 — Fix QuickSortMtEphSlice, STEP 20

## Objective

Get QuickSortMtEphSlice.rs (Chap36) compiling and verifying. The file is currently
commented out in lib.rs. It needs a `to_vec` method on ArraySeqMtEphSliceS and then
13 verification errors fixed.

## Background

QuickSortMtEphSlice implements parallel quicksort over `ArraySeqMtEphSliceS<T>`.
Three variants: first-element pivot, median-of-three, and random pivot. All use
`ParaPair!` for parallel recursion on left/right partitions.

The file was previously labeled "waiting for -V new-mut-ref" but that was wrong.
The real blocker is that `ArraySeqMtEphSliceS` has no `to_vec` method, and the
sort functions need to extract sorted elements back into a Vec for `concat_three_vecs`.

## What to fix

### Step 1: Add `to_vec` to ArraySeqMtEphSliceS (Chap19)

Read `src/Chap19/ArraySeqMtEphSlice.rs`. Add a `to_vec` method to the trait and impl:

```rust
fn to_vec(&self) -> (v: Vec<T>)
    requires self.spec_arrayseqmtephslice_wf(),
    ensures
        v@.len() == self.spec_len(),
        forall|i: int| #![trigger v@[i]]
            0 <= i < self.spec_len() ==> v@[i] == self.spec_index(i),
;
```

The impl can clone elements from `self.data[self.start..self.start + self.len]`.
The struct is `{ data: Arc<Vec<T>>, start: usize, len: usize }`.

### Step 2: Uncomment QuickSortMtEphSlice in lib.rs

Remove the `//` prefix from `pub mod QuickSortMtEphSlice;` in Chap36 block.

### Step 3: Fix QuickSortMtEphSlice.rs

The file currently has `(*la.data).clone()` and `(*ra.data).clone()` as a workaround
for the missing `to_vec`. Replace these with `la.to_vec()` / `ra.to_vec()` using the
new method.

The impl bound needs `Sync` added:
```rust
impl<T: TotalOrder + Eq + Clone + Send + Sync + 'static> QuickSortMtEphSliceTrait<T>
```

After adding `to_vec`, the 13 verification errors should mostly resolve because the
`ensures v@ =~= sorted_elems` assertions will connect through the `to_vec` spec.

Fix any remaining verification errors. The proof lemmas (`lemma_partition_sort_concat`,
`lemma_total_ordering`, `lemma_elements_from_vec`) are already proved — they should
not need changes.

## CRITICAL: Read existing working QuickSort files first

- `src/Chap36/QuickSortStEph.rs` — sequential quicksort (working)
- `src/Chap36/QuickSortMtEph.rs` — parallel quicksort over ArraySeqMtEph (working)
- `src/Chap19/ArraySeqMtEphSlice.rs` — the slice module you'll add `to_vec` to

## Isolation

```bash
scripts/validate.sh isolate Chap36
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify QuickSortStEph.rs or QuickSortMtEph.rs.
- Do NOT add assume or accept.
- Use external_body only as last resort on functions too hard to prove.
- The `Sync` bound addition on the impl is already done — just keep it.

## STEP 20

## Report

Write `plans/agent-r89-quicksort-slice-report.md`.
