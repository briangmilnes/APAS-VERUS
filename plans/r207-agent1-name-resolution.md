# r207 agent 1 — restore name resolution on Verus 0.2026.09.13

Read `plans/r207-round-plan.md` first and obey its common rules.

## Goal

`scripts/validate.sh` currently stops in rustc name resolution with 167 errors
across 90 files (`logs/validate.20260920-145749.log`). Make every one of those
names resolve so that Verus proceeds to type checking. Stop when the newest
validate log contains no `E0425`, `E0432`, or `E0407`. Do not fix the type or
verification errors that appear next; inventory them instead.

## Inventory (from the log)

| # | Unresolved name | Count | What to do at 09.13 |
|---|-----------------|------:|---------------------|
| 1 | `vstd::set::group_set_axioms` | 73 | rename to `vstd::set::group_set_lemmas` |
| 2 | `vstd::map::group_map_axioms` | 33 | rename to `vstd::map::group_map_lemmas` |
| 3 | `vstd::seq_lib::seq_to_set_is_finite` | 22 | removed; `Set` is finite by type. Delete the call statement. If the enclosing `proof` block becomes empty, delete the block |
| 4 | `vstd::set::fold::lemma_fold_insert`, `lemma_fold_empty` | 26 | `fold` now lives in `vstd::iset::fold` over `ISet`; `Set::fold` is a wrapper (`~/projects/verus/source/vstd/set.rs:291-307`). Read `seq_set.rs` and choose the smallest change that resolves and keeps the lemma statements true; report if a statement no longer types |
| 5 | `vstd::std_specs::slice::group_slice_axioms` | 2 | path is now `vstd::slice::group_slice_axioms` |
| 6 | `vstd::relations::injective_on` | 2 | now a method `Set::injective_on(self, r)` in `set_lib.rs:188`; rewrite the two call sites |
| 7 | `vstd::set::axiom_set_insert_len` | 2 | rename to `lemma_set_insert_len` |
| 8 | `vstd::set::axiom_set_new` | 1 | `lemma_set_new(f, a)` now requires `Set::new(f) is Some`; read the site and report if a proof obligation appears |
| 9 | `vstd::set::axiom_set_insert_finite` | 1 | removed; delete the call |
| 10 | `vstd::set_lib::lemma_set_union_finite_iff` | 1 | removed; delete the call |
| 11 | `vstd::std_specs::slice::axiom_spec_slice_iter`, `vstd::std_specs::vec::axiom_spec_into_iter` | 2 | old iterator model; delete the `broadcast use` lines |
| 12 | `initial_value_relation` in `src/standards/prophetic_iterators_standard.rs:174` (E0407) | 1 | the trait no longer has this method; delete the method and its doc comment |

The current definitions are in `~/projects/verus/source/vstd/`. Read the
definition before every rename; confirm the replacement has the same arity
and role.

## Procedure

1. Run `scripts/validate.sh` once; read the log in full.
2. Group the errors by unresolved name. For each group, open each file, read
   the lines around every site, and apply the edit with the Edit tool. Use
   `replace_all` only for an exact identifier token within one file.
3. After each group, run `scripts/validate.sh` and read the new log. Record the
   error count in the doc.
4. When no `E0425`, `E0432`, or `E0407` remains, classify the next error set by
   rustc code and by message with backticked names replaced by `_`, and by file.
   Do not fix them.

## Deliverable: `docs/NameResolutionUpgrade.md`

Sections: (1) starting measurement; (2) a table of every rename or deletion
made, with Chap, file, line, old, new; (3) the validate error count after each
group; (4) the inventory of the errors that appear once name resolution passes,
by kind and by file, with the log name; (5) anything you judged unsafe to edit
and why. Use the vocabulary rules.
