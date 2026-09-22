# r207 agent 2 — review `src/vstdplus/` against vstd 0.2026.09.13

Read `plans/r207-round-plan.md` first and obey its common rules. This is a
review. You edit nothing under `src/`. Your only output file is
`docs/VstdplusReview.md`.

## Question

`src/vstdplus/` holds 26 modules (7,000 lines) written to fill gaps in older
vstd releases. vstd has changed in 80 commits since May. For each module: which
definitions does vstd 0.2026.09.13 now provide, which remain needed, and which
are unused by the codebase?

## Inputs

- `src/vstdplus/**/*.rs`
- `~/projects/verus/source/vstd/` at tag `release/0.2026.09.13.671956e`
- User counts per module (files under `src/` outside `vstdplus` that import
  it): `accept` 88, `feq` 206, `clone_plus` 67, `total_order` 66,
  `hash_map_with_view_plus` 38, `seq_set` 30, `clone_view` 29,
  `hash_set_with_view_plus` 26, `monoid` 22, `smart_ptrs` 19, `arc_rwlock` 18,
  `float` 16, `multiset` 12, `rand` 10, `checked_nat` 9, `checked_int` 7,
  `seq` 4, `pervasives_plus` 2, `hashed_checked_u32` 1, `strings` 1,
  `threads_plus` 1, `VecQueue` 0, `hash_set_specs` 0, `partial_order` 0,
  `sqrt` 0. Re-derive these; the numbers are a starting point.
- Upstream changes to check against: `Set`/`Map` finite by type (`#2486`);
  `ISet`/`IMap`; `group_set_lemmas`, `group_map_lemmas`, `group_seq_lemmas`;
  `Set::to_seq` lemmas (`#1797`); `laws_eq`/`laws_cmp` renames; `BTreeMap`
  specs (`#2831`); string specs (`#2738`, `#2854`); `NonZero` (`#2471`);
  `checked_next_multiple_of` (`#2696`); `saturating_mul` (`#2809`);
  `no_unwind` on integer arithmetic (`#2893`); `Structural` for tuples and
  arrays (`#2830`); the prophetic iterator model.

## Procedure

For each module, in this order: read the module; list every `pub` item; for
each item search vstd for an item with the same statement (not the same name);
count the module's users under `src/`; classify.

Classification of each `pub` item, one of:

| # | Class | Meaning |
|---|-------|---------|
| 1 | provided | vstd 09.13 has an equivalent; name it with file and line |
| 2 | needed | no vstd equivalent, and used under `src/` |
| 3 | unused | no user under `src/` (state whether an RTT or PTT uses it) |
| 4 | broken | references a vstd item that no longer exists, or a statement that is now vacuous (for example `finite()`) |

Special cases:
- `seq_set.rs`: 51 of the 167 name-resolution errors are here. Agent 1 is
  editing it concurrently; read it, do not edit it, and review the version on
  disk at the time you read it. Say which version you reviewed (the `git diff`
  state).
- `accept.rs`: needed by policy; review only whether the cargo stub still builds
  under Rust 1.98.1.
- `float.rs`: compare with `vstd/float.rs` (3,682 bytes at 09.13).
- `multiset.rs`: compare with `vstd/multiset.rs` and `multiset_lib.rs`.
- `partial_order.rs`, `total_order.rs`: compare with `vstd/relations.rs`,
  `laws_cmp.rs`, and the `OrdSpec` traits in `std_specs/cmp.rs`.

## Deliverable: `docs/VstdplusReview.md`

1. Summary table: one row per module with lines, users, and counts of items
   in each class.
2. Per module: a table of `pub` items with class, the vstd equivalent where one
   exists, and the recommended action (delete, replace users with the vstd
   item, keep, or repair). A recommendation names the concrete edit.
3. A ranked list of the recommendations by number of source lines removed.
4. Items you could not classify and why.

Recommend only. Do not delete, rename, or edit any module.
