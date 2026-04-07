# R153 Agent 2 — Add next/prev/rank/select to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/ordered-bst-refactor.md` — the design doc.
Read `src/Chap38/OrdKeyMap.rs` — the module you're extending.
Read the `bst_next_by_key` (305 lines) and `bst_prev_by_key` (286 lines)
functions in `src/Chap43/OrderedTableStEph.rs` — these are the reference
implementations you're adapting.

Report file: `plans/r153-agent2-ordkeymap-next-prev-report.md`

## Problem

OrdKeyMap needs ordering operations: next (successor), prev (predecessor),
rank, and select. These are the operations that make an ordered map "ordered."
Currently they're 300+ line functions duplicated in OrderedTableStEph and
OrderedTableStPer.

## What to add

### next (successor)

```rust
fn next(&self, key: &K) -> (succ: Option<Pair<K, V>>)
    requires self.spec_ordkeymap_wf(),
    ensures
        succ matches Some(p) ==> self@.contains_key(p.fst@),
        succ matches Some(p) ==> TotalOrder::le(key@, p.fst@) && p.fst@ != key@,
        succ matches Some(p) ==> forall|k: K::V|
            self@.contains_key(k) && TotalOrder::le(key@, k) && k != key@
            ==> #[trigger] TotalOrder::le(p.fst@, k),
        succ is None ==> forall|k: K::V|
            self@.contains_key(k) ==> TotalOrder::le(k, key@) || k == key@;
```

Implementation: BST descent on `self.inner`. At each node, compare key with
node's key. If key < node.key, node is a candidate — recurse left, take the
closer result. If key >= node.key, recurse right. Standard BST successor.

### prev (predecessor)

Mirror of next with reversed ordering.

### rank

```rust
fn rank(&self, key: &K) -> (r: usize)
    requires self.spec_ordkeymap_wf(),
    ensures r as int == ...;  // count of keys strictly less than key
```

BST descent counting left subtree sizes.

### select

```rust
fn select(&self, i: usize) -> (entry: Option<Pair<K, V>>)
    requires self.spec_ordkeymap_wf(),
    ensures ...;  // i-th smallest key-value pair
```

BST descent using size-based indexing.

## Approach

Copy `bst_next_by_key` from OrderedTableStEph. Adapt:
1. Change parameter from `tree: &ParamBST<Pair<K,V>>` to use `self.inner`
2. The internal recursion stays on `Link` / BST nodes — same tree traversal
3. The ensures speak in Map terms via OrdKeyMap's View
4. Bridge proof at the top level: connect BST-level result to Map postcondition

The proof logic (TotalOrder transitivity chains, BST ordering exclusion) stays
the same. What shrinks is the bridge overhead — OrdKeyMap's wf already bundles
key uniqueness and pair_set_to_map properties.

## Priority

next and prev are the most important — they're the hardest proofs and the
biggest payoff. If time runs short, do next and prev only. Rank and select
can be Phase 2.

## Validation

`scripts/validate.sh isolate Chap38` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrderedTableStEph or any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
