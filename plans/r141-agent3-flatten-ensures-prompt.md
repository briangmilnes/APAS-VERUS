# R141 Agent 3 — Fix 2 holes in ArraySeqMtEphSlice: bare_impl + flatten ensures. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — both holes are in this file.

Report file: `plans/r141-agent3-flatten-ensures-report.md`

## Hole 1: bare_impl (line ~506)

Veracity reports: `impl<T: Eq + Clone> ArraySeqMtEphSliceS<T> { — impl without trait`

This bare impl block holds D&C helpers and proof functions: `lemma_monoid_fold_left`,
`lemma_prefix_fold_matching`, `lemma_prefix_fold_split`, `lemma_prefix_fold_eq_fold_left`,
`reduce_dc`, `map_dc_vec`, `filter_dc_vec`, `tabulate_dc_vec`, `scan_dc_vec`.

**Fix:** Convert the exec functions to `pub(crate)` free functions per standard 19
(helper function placement). Move the type parameter `T: Eq + Clone` to each
function's generic list. Change all `Self::fn_name(...)` call sites to `fn_name(...)`.

The proof functions (`lemma_*`) can stay in the bare impl — veracity only flags
exec functions in bare impls. But if it's cleaner to move everything, do so.

**Important:** `scan_dc_vec` and the scan closures use `Self::lemma_prefix_fold_*`
and `Self::scan_dc_vec` — update ALL call sites, including inside the scan impl
and inside named closures within scan_dc_vec.

Count the `Self::` references before and after. There should be zero `Self::` calls
to these functions after the conversion.

## Hole 2: fn_missing_ensures on flatten_dc_vec (line ~1157)

`flatten_dc_vec` returns a `Vec<T>` from a nested sequence but has no ensures.
Callers can't prove anything about the result. This is a real spec gap.

**Fix:** Add ensures. At minimum:

```rust
ensures
    result@.len() == spec_sum_inner_lens(a),
```

Define `spec_sum_inner_lens`:
```rust
pub open spec fn spec_sum_inner_lens<T>(
    a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>,
) -> nat
    decreases a.len,
{
    if a.len == 0 { 0 }
    else {
        let inner_len = (*a.data)@[a.start as int].len as nat;
        let rest = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
            data: a.data, start: a.start + 1, len: (a.len - 1) as usize,
        };
        inner_len + spec_sum_inner_lens(&rest)
    }
}
```

Also strengthen `flatten`'s ensures to include the length:
```rust
ensures
    flattened.spec_arrayseqmtephslice_wf(),
    flattened.spec_len() == spec_sum_inner_lens(a),
```

Prove from the D&C structure:
- Base empty: 0 == 0
- Base singleton: inner.len == spec_sum_inner_lens(singleton)
- Recursive: spec_sum_inner_lens(a) == spec_sum_inner_lens(left) + spec_sum_inner_lens(right)

Write a split lemma for spec_sum_inner_lens if needed.

If full element-level ensures are too hard, the length ensures is sufficient.
Do NOT add assumes to make it work.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken existing ensures on other functions.
- Both holes are in the same file. Make both fixes in one commit.

## When done

RCP.
