# R37 Agent 1: OrderedTableMtEph Ordering Ops + AVLTreeSetStEph Assumes

## Goal

Prove the 6 ordering operations in OrderedTableMtEph.rs (which agent4 left
as external_body in R36). Then tackle the 2 algorithmic assumes in
AVLTreeSetStEph.rs if time.

## Context

OrderedTableMtEph wraps `TableMtEph<K, V>` directly — no RwLock.
The 6 ordering functions iterate over `self.base_table.entries()` to find
min/max/predecessor/successor/rank/select by key.

Agent2 proved the **identical algorithms** in OrderedTableStEph.rs in R36
using TotalOrder bridging (cmp → match on Ordering → reflexive/transitive).
Your job: apply the same proof technique to the MtEph versions.

## Tier 1: OrderedTableMtEph.rs (6 external_body → proved)

Read the **proved versions** in `src/Chap43/OrderedTableStEph.rs` first:
- `first_key` (lines ~576-631)
- `last_key` (lines ~656-731)
- `previous_key` (lines ~735-839)
- `next_key` (lines ~839-943)
- `rank_key` (lines ~1095-1121)
- `select_key` (lines ~1113-1141)

Then apply the same proof approach to `src/Chap43/OrderedTableMtEph.rs`:
- `first_key` (line 505) — external_body
- `last_key` (line 523) — external_body
- `previous_key` (line 541) — external_body
- `next_key` (line 560) — external_body
- `rank_key` (line 651) — external_body
- `select_key` (line 667) — external_body

### Technique (from StEph R36 proofs)

1. **TotalOrder bridging**: Use `K::cmp(a, b)` which returns
   `core::cmp::Ordering`. Match on `Less`/`Equal`/`Greater` and invoke
   `K::reflexive()`, `K::transitive()`, `K::antisymmetric()`, `K::total()`
   as needed.

2. **Direct entry iteration**: Access entries via `self.base_table.entries`
   (ArraySeqMtEphS). Iterate with index, not collect().

3. **Ghost src_idx witness**: For rank/select, track source indices as
   ghost sequence to eliminate exists quantifiers.

4. **Loop invariant pattern** (first_key example):
   ```rust
   invariant
     forall|j: int| #![trigger entries[j]]
       0 <= j < i ==> TotalOrder::le(min_val, entries[j].spec_key()),
   ```

### Key difference from StEph

The entry access path differs:
- StEph: `self.base_table.entries` is `ArraySeqStEphS<Pair<K,V>>`
- MtEph: `self.base_table.entries` is `ArraySeqMtEphS<Pair<K,V>>`

The view (`entries@`) should be the same `Seq<Pair<K,V>>`. Check if
`spec_index` and `@` behave identically.

**Expected: -6 holes.**

## Tier 2: AVLTreeSetStEph.rs (2 assumes)

Two identical assumes at lines 1059 and 1334:
```rust
assume(new_vec@.len() < usize::MAX);
```

Root cause: `spec_avltreesetsteph_wf()` bounds tree size to `< usize::MAX`,
but insert creates `new_vec` with `len + 1` elements. Need to prove
`len + 1 < usize::MAX`.

Options:
1. Add `requires self@.len() + 1 < usize::MAX` to the insert functions
2. Strengthen wf to bound size to `< usize::MAX - 1`
3. Prove from existing wf that the arithmetic works

Read the surrounding context. Check what callers exist (Chap43 files) and
whether adding requires cascades badly.

**Expected: -1 to -2 holes.**

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Read the proved StEph versions FIRST — they are your proof template.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent1-round37-report.md`.
- Commit, push to `agent1/ready`.
