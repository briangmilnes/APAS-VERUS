# R103 Agent 2 — Test new Verus string specs on DocumentIndex, STEP 20

## Objective

Chap44 DocumentIndex has 4 external_body holes on functions that use String
operations (to_lowercase, chars, is_alphabetic, sort_unstable_by). Verus
recently added string specs (PR #2238, Natalie Neamtu). Test if the String
ops are now verifiable.

## The 4 holes

| # | Function | String ops used |
|---|----------|----------------|
| 1 | make_index | String::to_lowercase, sort_unstable_by, chars iteration |
| 2 | tokens | String::to_lowercase, chars(), char::is_alphabetic() |
| 3 | to_seq | No String ops — just AVLTreeSeq iteration + ArraySeq append |
| 4 | get_all_words | No String ops — just Table.collect iteration + ArraySeq append |

Wait — holes 3 and 4 don't use Strings. They're external_body because of
loop invariant complexity, not String ops. Those might be provable without
new string specs.

## Strategy

### Start with to_seq and get_all_words (no String deps)

These are loops building ArraySeqStPerS from iteration. Similar to
num_edges/vertices patterns in AdjTableGraph. Try removing external_body
and adding loop invariants.

### Then test tokens and make_index

Check what Verus string specs are available:
```bash
grep -r "spec.*str\|spec.*String\|spec.*char" ~/projects/verus/source/vstd/
```

If `to_lowercase`, `chars`, `is_alphabetic` have specs, try proving tokens.
If not, leave as external_body.

## Read first

- `src/Chap44/DocumentIndex.rs` — the 4 external_body functions
- `~/projects/verus/source/vstd/string.rs` — new string specs
- `~/projects/verus/source/vstd/std_specs/str.rs` — str specs if they exist

## Isolation

```bash
scripts/validate.sh isolate Chap44
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add assume or accept.
- Prioritize to_seq and get_all_words (no String deps, likely provable).
- For String functions, check what specs exist before attempting. If no
  specs for to_lowercase/chars, leave external_body.

## STEP 20

## Report

Write `plans/agent2-r103-documentindex-strings-report.md`.
