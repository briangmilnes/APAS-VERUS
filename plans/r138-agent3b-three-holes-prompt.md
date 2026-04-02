# R138b Agent 3 — Fix 3 holes: Chap43 capacity, Chap52 element wf, Chap37/39 assumes→accepts. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `toplevel_coarse_rwlocks_for_mt_modules.rs` — capacity exec-time guards and accept patterns
- `capacity_bounds_standard.rs`
- `spec_wf_standard.rs`

Report file: `plans/r138-agent3b-three-holes-report.md`

## Task 1: Chap43 OrderedSetMtEph capacity (1 hole)

`src/Chap43/OrderedSetMtEph.rs:547`: `assume(inner@.len() + 1 < usize::MAX)`

This is inside a lock — the standard pattern. After acquire_write, check capacity
in exec code, return Err if full.

```rust
// After acquire_write:
if locked_val.size() >= usize::MAX - 1 {
    write_handle.release_write(locked_val);
    return Err(());
}
// ... proceed with operation ...
```

The trait method should return `Result`. Update callers.

## Task 2: Chap52 AdjTableGraphMtPer element wf (1 hole)

`src/Chap52/AdjTableGraphMtPer.rs:450`: `assume(neighbors.spec_avltreesetmtper_wf())`

The graph stores adjacency sets (`AVLTreeSetMtPer<V>`) but the graph's wf predicate
doesn't guarantee the stored sets are wf.

Fix: add to the graph's wf predicate:
```rust
&& forall|v: V::V| self.spec_adj().dom().contains(v)
    ==> self.spec_adj()[v].spec_avltreesetmtper_wf()  // or however the set wf is expressed
```

Then prove the wf is maintained through graph operations (add_vertex, add_edge,
delete_vertex, delete_edge). The assume becomes provable from the graph's own wf.

Check what `spec_adj()` returns and how the set values are accessed in the spec
to get the trigger right.

## Task 3: Chap37/39 convert assumes to accepts (6 holes → 6 accepted)

Six files have bare `assume()` for the find value correspondence:

```
Chap37/BSTAVLMtEph.rs:1019:    assume(found.is_some() ==> found.unwrap() == *target)
Chap37/BSTBBAlphaMtEph.rs:754:  assume(found.is_some() ==> found.unwrap() == *target)
Chap37/BSTPlainMtEph.rs:754:    assume(found.is_some() ==> found.unwrap() == *target)
Chap37/BSTRBMtEph.rs:1277:      assume(found.is_some() ==> found.unwrap() == *target)
Chap37/BSTSplayMtEph.rs:2030:   assume(found.is_some() ==> found.unwrap() == *target)
Chap39/BSTTreapMtEph.rs:1373:   assume(result.is_some() ==> result.unwrap()@ == target@)
```

These are the standard lock-boundary reader accept pattern from
`toplevel_coarse_rwlocks_for_mt_modules.rs`. Convert each from bare `assume()`
to `accept()` using `crate::vstdplus::accept::accept`.

Change:
```rust
proof { assume(found.is_some() ==> found.unwrap() == *target); }
```
To:
```rust
proof { accept(found.is_some() ==> found.unwrap() == *target); }
```

This doesn't change the proof strength — `accept` has the same semantics as
`assume`. It changes the classification in veracity from `[algorithmic]` to
`[accepted]`, correctly identifying these as reviewed lock-boundary trust points
rather than unreviewed proof gaps.

## Validation

Run `scripts/validate.sh isolate Chap43` then `isolate Chap52` then `isolate Chap37`.
Then `scripts/rtt.sh`.

## Rules

- Tasks 1-2: Do NOT add assumes or accepts. Prove from wf.
- Task 3: Convert assume to accept ONLY for these 6 specific lines. Do not touch
  any other assumes.
- Do NOT weaken ensures.

## When done

RCP.
