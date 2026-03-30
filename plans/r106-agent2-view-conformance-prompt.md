# R106 Agent 2 — Fix ALL View type mismatches. AFK. Go for it.

## Objective

Fix every View type mismatch reported by `veracity-compare-par-mut`. Not triage.
Not report. FIX. This is the project's current state — nothing to hide (PBOGH).

You already triaged these in your first pass. You classified 10 View errors into
Category A (St wrong) and Category B (Mt wrong). Now do the rewrites.

## The errors

### Category B: Mt wrong (standalone, no cascade — do these first)

**Chap37 BSTRBMtEph.rs**: View = Link<T> → BalBinTree<T> to match StEph.
176 Link-based spec refs. Rewrite them. It's one file. Mt is standalone.
`scripts/validate.sh isolate Chap37`

### Category A: St wrong (cascade — work bottom-up)

**Chap05 SetStEph.rs**: View = Seq<T> → Set<T::V>.
~100 internal refs. SetStEph is used by Chap06 and downstream but those files
call through the trait — they use `self@.contains()` etc., not `self@[i]`.
Check how bad the cascade actually is before assuming it's 100+ files.
`scripts/validate.sh isolate Chap05`

**Chap06 graph StEph files** (DirGraphStEph, UnDirGraphStEph, LabDirGraphStEph,
LabUnDirGraphStEph): View = Seq<V> → GraphView<V::V>/LabGraphView<V::V,L::V>.
Check what GraphView looks like in the MtEph files and mirror it.
`scripts/validate.sh isolate Chap06`, then downstream chapters.

**Chap43 OrderedSetStPer**: View = Seq<T> → Set<T::V>.
141 self@ refs. OrderedSet callers are mostly in Chap43 itself + Example43_1.
`scripts/validate.sh isolate Chap43`

**Chap43 OrderedTableStPer**: View = Seq<Pair<K,V>> → Map<K::V,V::V>.
This is the biggest. StPer uses Seq<Pair> everywhere internally. But the Mt
already has Map<K::V,V::V> — use it as your template.
`scripts/validate.sh isolate Chap43`, then Chap44, 52.

## How to do a View rewrite

1. Change the `type V = ...` in the View impl
2. Change the `view()` fn to return the new type
3. `scripts/validate.sh isolate ChapNN` — see what breaks
4. For each error: the old code did `self@[i]` or `self@.len()` on a Seq.
   The new code does `self@.contains(x)` or `self@.len()` on a Set/Map.
   Rewrite the spec to use the abstract type's operations.
5. Repeat until clean
6. **Commit after each chapter is clean** — save progress

## Work order

1. Chap37 BSTRBMtEph (Mt standalone — safest, no cascade)
2. Chap05 SetStEph (check cascade before panicking)
3. Chap06 graph files (4 files, check cascade)
4. Chap43 OrderedSetStPer
5. Chap43 OrderedTableStPer (biggest, save for last)

If a cascade is truly massive (>50 files), do the chapter's internal rewrites
and commit that progress, then report what downstream files still need fixing.

## Rules

- Do NOT add assume or accept.
- Do NOT skip a View fix because it's "too many refs." Rewrite the refs.
- Commit after each chapter. `git add -A && git commit`.
- Mt files are standalone — Mt MUST NOT import from St.
- Read `src/standards/view_standard.rs` before starting.
- Read the MtEph/MtPer variant's View impl as your template for what the
  correct View type and view() function should look like.
- Use `scripts/validate.sh isolate ChapNN` for iteration. Full validate
  after all chapters are done.
- Read logs instead of re-running. `ls -t logs/validate.*.log | head -1 | xargs cat`
- If an approach fails twice, try a different approach. Don't loop.

## No step limit. No subagents.

Do NOT use the Agent tool. Work until done or stuck.

## Report

Write `plans/agent2-r106-view-conformance-report.md`.
