# R68 Agent 5: BSTParaStEph Strengthen + Chap53 Repair

## Goal

Two tasks, in priority order:

1. **Primary**: Strengthen `collect_in_order` and `in_order` ensures in BSTParaStEph.rs
   to include membership/set-equality postconditions. This directly unblocks hole closures
   in downstream files (OrderedSetStEph, OrderedTable*).

2. **Secondary**: Repair PQMinStPer.rs and GraphSearchStPer.rs (Chap53) — fix
   `.elements` → `.tree` references, uncomment in lib.rs, re-enable RTT.

## Task 1: BSTParaStEph collect_in_order Strengthening

**File**: `src/Chap38/BSTParaStEph.rs`

### Current state

`collect_in_order` currently ensures only `out@.len()` (or similar). Downstream code
(OrderedSetStEph `to_seq`, line 575) needs:
- `result@.to_set() =~= self@` — the collected sequence contains exactly the tree's elements
- `forall|i| 0 <= i < result@.len() ==> self@.contains(result@[i])` — each element is in the tree

### What to strengthen

Read BSTParaStEph.rs and find:
1. `collect_in_order` — what does it currently ensure? Add membership ensures.
2. `in_order` — what does it currently ensure? If it already has membership, good.
   If not, strengthen it too.

The proof strategy for `collect_in_order`:
- It's a recursive in-order traversal that pushes elements onto a Vec.
- The loop invariant (or recursive postcondition) should track that pushed elements
  are exactly the left subtree's elements, then the node's key, then the right subtree's.
- ParamBST's `expose` ensures give you `left@ ∪ {key} ∪ right@ == self@` and disjointness.
- Use that to prove the collected sequence's set equals the tree's set.

### Also check

- Do any ParamBST methods need `ensures result@.finite()`? The type invariant guarantees
  it but it may not be exposed in ensures. If `split`, `union`, `intersection`, `difference`
  don't ensure `result@.finite()`, add it — it's trivially true from the Set being
  representable in a finite tree.

- If `spec_bstparasteph_wf()` implies `self@.finite()`, make that connection explicit
  (proof lemma or direct ensures).

### Validation

After strengthening, run:
- `scripts/validate.sh` — must still be 0 errors
- `scripts/rtt.sh` — all pass
- `scripts/ptt.sh` — all pass
- Check that NO existing ensures are weakened

## Task 2: Chap53 Repair (PQMinStPer + GraphSearchStPer)

**Context**: Agent 2 (R67) rewired AVLTreeSetStPer from `elements: AVLTreeSeqStPerS<T>`
to `tree: ParamBST<T>`. Two Chap53 files reference `.elements` on AVLTreeSetStPer and
were commented out.

### Files to fix

1. `src/Chap53/PQMinStPer.rs` — references `frontier.elements` or similar
2. `src/Chap53/GraphSearchStPer.rs` — references `.elements` on AVLTreeSetStPer

### Steps

1. **Read** each file, find all `.elements` references
2. **Replace** `.elements` with `.tree` (or use AVLTreeSetStPer's trait methods instead
   of reaching into the struct)
3. **Uncomment** in `src/lib.rs`:
   - `pub mod PQMinStPer;` (Chap53)
   - `pub mod GraphSearchStPer;` (Chap53)
4. **Uncomment** test entries in `Cargo.toml`:
   - `TestPQMinStPer`
   - `TestGraphSearchStPer`
5. **Validate** — 0 errors
6. **RTT** — all pass including the re-enabled tests
7. **PTT** — all pass

### What NOT to change

- Do NOT modify AVLTreeSetStPer.rs (already rewired by Agent 2 R67)
- Do NOT modify AVLTreeSetStEph.rs or any Chap41 files
- Do NOT modify OrderedTable* or OrderedSet* files (other agents' territory)
- If PQMinStPer or GraphSearchStPer have OTHER issues beyond `.elements`, fix them
  if straightforward, otherwise leave and report

## Constraints

- Do NOT add `assume`, `accept`, or `external_body` on algorithmic logic.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially.
- Task 1 (BSTParaStEph strengthening) takes priority over Task 2.
