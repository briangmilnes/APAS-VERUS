<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Bogus Views Review and 4-Way Work Split

## What Is a Bogus View?

A **bogus view** is a `View` implementation where:

1. **`view()` (or `spec_set_view`) has `#[verifier::external_body]`** — the spec is trusted, not verified.
2. **Returns a trivial value** — `Set::empty()` or `Map::empty()` — when the structure may have content.

Callers that reason about `self@` get `Set::empty()` or `Map::empty()` regardless of the actual data. The `ensures` clauses on trait methods reference `self@`, so the specs are consistent, but the view itself is a placeholder.

## Bogus Views Inventory

| # | Chapter | File | Type | View | Returns | Blocker |
|---|---------|------|------|------|---------|---------|
| 1 | 41 | ArraySetEnumMtEph.rs | ArraySetEnumMtEph | external_body | Set::empty() | bitvec crate (Verus can't link) |
| 2 | 41 | AVLTreeSetMtPer.rs | AVLTreeSetMtPer | spec_set_view external_body | Set::empty() | MtPer parallel tree ops |
| 3 | 43 | OrderedSetMtEph.rs | OrderedSetMtEph | external_body | Set::empty() | ParamTreap MtEph |
| 4 | 43 | OrderedTableMtPer.rs | OrderedTableMtPer | external_body | Map::empty() | ParamTreap MtPer |

## Identity Views (Lower Priority)

These use `view() -> Self` (no abstraction). The Chap52 review notes they "will need revision when verified":

| # | Chapter | File | View | Notes |
|---|---------|------|------|-------|
| 5 | 52 | EdgeSetGraphMtPer.rs | view() -> Self | Identity; adequate for external_body |
| 6 | 52 | AdjTableGraphMtPer.rs | view() -> Self | Identity; adequate for external_body |

## Verified Views (Not Bogus)

For comparison, these have verified `view()` (no external_body on the view fn):

- **OrderedSetStEph, OrderedSetStPer**: `view() -> self.base_set@`
- **AugOrderedTable***: `view() -> self.base_table@`
- **TableStEph, TableMtEph, TableStPer**: `view() -> spec_entries_to_map(self.entries@)`
- **AVLTreeSetStEph, AVLTreeSetStPer, AVLTreeSetMtEph**: delegate to verified `spec_set_view` from AVLTreeSeq

## 4-Way Work Split

| # | Bucket | Agent | Scope | Work | Est. |
|---|--------|-------|-------|------|------|
| 1 | Chap41 Set Views | agent1 | ArraySetEnumMtEph, AVLTreeSetMtPer | Replace external_body view with verified spec. ArraySetEnum: blocked by bitvec; document or switch repr. AVLTreeSetMtPer: verify spec_set_view from tree inorder. | Hard / Blocked |
| 2 | Chap43 OrderedSetMtEph | agent2 | OrderedSetMtEph.rs | Replace external_body view() with verified spec. Delegate to ParamTreap/AVLTreeSeq view or define spec_inorder for treap. | Medium |
| 3 | Chap43 OrderedTableMtPer | agent3 | OrderedTableMtPer.rs | Replace external_body view() with verified spec. Delegate to underlying treap/table view. | Medium |
| 4 | Chap52 Identity Views | main or backlog | EdgeSetGraphMtPer, AdjTableGraphMtPer | Add proper Set/Map views when verifying these modules. Low priority; identity is adequate for current external_body. | Low |

## Assignment Rationale

- **agent1**: Chap41 (agent1 scope per chap02-41-proof-hole-fixes). ArraySetEnum is likely blocked; AVLTreeSetMtPer is the realistic target.
- **agent2**: Chap43 OrderedSetMtEph — single file, clear boundary.
- **agent3**: Chap43 OrderedTableMtPer — single file, mirrors OrderedSet.
- **Bucket 4**: Chap52 identity views are lower priority; can be done by main or deferred.

## Success Criteria

For each bogus view:

1. Remove `#[verifier::external_body]` from `view()` or `spec_set_view()`.
2. Implement a verified `open spec fn view(&self) -> V` that correctly reflects the structure.
3. All `ensures` clauses that reference `self@` remain valid.
4. Verus verification passes.

## Blockers

| Module | Blocker |
|--------|---------|
| ArraySetEnumMtEph | `bitvec` crate — Verus cannot link. Would need alternative bit-vector repr or keep external_body. |
| AVLTreeSetMtPer | Parallel tree operations; need to connect AVLTreeSeqMtPer view to set abstraction. |
| OrderedSetMtEph | ParamTreap; need inorder spec for treap. |
| OrderedTableMtPer | ParamTreap key-value; need map-from-treap spec. |
