# Agent 1 — Round 26: Add Real Specs to Chap37 MtEph Helpers + AVLTreeSeq Holes

## R25 Feedback

You removed 53 `requires true` — but you just **deleted** them. The prompt said
"replace every `requires true` with the **real** well-formedness predicate." You were
given a table listing which functions need `spec_is_bst_link`. You ignored it.

The MtEph files (BSTSplayMtEph.rs, BSTRBMtEph.rs) now have NO preconditions at all on
BST operations like `splay`, `bst_insert`, `insert_link`, `find_link`, `min_link`,
`max_link`. The StEph counterparts have full specs with `spec_is_bst_link`. You were
told to read BSTSplayStEph.rs and copy the corresponding requires/ensures. You didn't.

Additionally, the MtEph files don't even HAVE `spec_is_bst_link` defined. It exists
in BSTSplayStEph.rs (line 114) but was never duplicated into the Mt files. Mt files
must not import from St counterparts — you need to duplicate the spec.

This round: do the actual work. Copy specs from StEph to MtEph.

## Mission

1. Add real requires/ensures to all Chap37 MtEph helper functions
2. Prove 3 AVLTreeSeq holes (unblocks Chap41 → Chap43 → Chap52)

## Part 1: BSTSplayMtEph.rs — Add Real Specs (Priority 1)

### Step 1: Duplicate `spec_is_bst_link` into BSTSplayMtEph.rs

Read `src/Chap37/BSTSplayStEph.rs` line 114. Copy `spec_is_bst_link` (the open spec fn)
into BSTSplayMtEph.rs section 6 (spec fns). Adapt the type bounds if needed (Mt uses
`StTInMtT + Ord` instead of `TotalOrder + Clone`).

### Step 2: Add requires/ensures to each function

Read the StEph version of each function and copy its spec. Here is what BSTSplayStEph.rs
currently has:

| # | Function | StEph requires | StEph ensures |
|---|----------|---------------|---------------|
| 1 | `new_node(key)` | (none) | `node.key == key, node.size == 1, node.left is None, node.right is None` |
| 2 | `size_link(link)` | (none) | `size as nat == spec_size_link(link)` |
| 3 | `height_link(link)` | `spec_height_link(link) < usize::MAX` | `height as nat == spec_height_link(link)` |
| 4 | `update(node)` | (none) | `node.key == old(node).key, node.left == old(node).left, node.right == old(node).right` |
| 5 | `splay(root, target)` | `spec_is_bst_link(&Some(root))` | `spec_is_bst_link(&Some(result))` + content/size preservation |
| 6 | `bst_insert(link, value)` | (size bound) | `link_spec_size <= old + 1` |
| 7 | `insert_link(link, value)` | (size bound) | `link_spec_size <= old + 1` |
| 8 | `find_link(link, target)` | (none in StEph) | `None when None` + containment |
| 9 | `min_link(link)` | (none in StEph) | `None↔None, Some↔Some` + min value correctness |
| 10 | `max_link(link)` | (none in StEph) | `None↔None, Some↔Some` + max value correctness |
| 11 | `in_order_collect(link)` | (none) | length matches size |
| 12 | `pre_order_collect(link)` | (none) | length matches size |
| 13 | `in_order_parallel(link)` | (none) | length matches size |
| 14 | `pre_order_parallel(link)` | (none) | length matches size |
| 15 | `build_balanced(sorted)` | `sorted.len() > 0` or similar | result is BST with same content |
| 16 | `filter_parallel(link, f)` | closure requires per standard | result subset |
| 17 | `reduce_parallel(link, f, id)` | closure requires per standard | reduction result |
| 18 | `height_rec(link)` | `spec_height_link(link) < usize::MAX` | `result == spec_height_link(link)` |

For functions 1-4, 11-14, 18: these are structural operations that genuinely don't need
BST ordering. Add the ensures (copying from StEph) but omit requires (don't write
`requires true`).

For functions 5-10: these DO need BST ordering. Add `spec_is_bst_link` as requires.

For functions 15-17: add the appropriate preconditions per the closure/build pattern.

### Step 3: Validate

After adding specs, run `scripts/validate.sh`. You are ADDING specs (requires/ensures),
not proving new bodies. The existing `external_body` functions will accept any spec. The
non-external functions must still verify with the new specs — if one fails, the ensures
you added is too strong and needs adjustment.

## Part 2: BSTRBMtEph.rs — Add Real Specs (Priority 2)

Same process. Read `src/Chap37/BSTRBStEph.rs` for the real specs. BSTRBStEph.rs uses
`spec_is_bst_link` (or equivalent) for ordering. Duplicate the spec definition into
BSTRBMtEph.rs and add requires/ensures to all helper functions.

RB-specific functions:
- `is_red(link)` — no requires (pure check), add ensures about result
- `rotate_left(root)` / `rotate_right(root)` — requires BST ordering, ensures ordering preserved
- `flip_colors(root)` — no ordering change, add ensures about color changes
- `fix_up(root)` — requires BST ordering, ensures ordering preserved

## Part 3: AVLTreeSeq Holes — 3 external_body (Priority 3)

These 3 holes block the Chap37 → Chap41 → Chap43 → Chap52 dependency chain.

| # | File | Function | Holes |
|---|------|----------|-------|
| 1 | AVLTreeSeq.rs | iterator `next` | 1 external_body |
| 2 | AVLTreeSeqMtPer.rs | `build_balanced_from_slice` | 1 external_body |
| 3 | AVLTreeSeqMtPer.rs | `subseq_copy` | 1 external_body |

Read each file and the corresponding StEph/StPer versions. The StEph versions may already
have these proved — if so, the proof technique transfers.

For the iterator `next`: read `src/standards/iterators_standard.rs` for the iterator
proof pattern. The invariant maintenance through `next()` is the key proof obligation.

## Part 4: AVLTreeSeq `requires true` — 12 instances (Priority 4)

Your R25 report noted 12 `requires true` remaining in AVLTreeSeq files:
- AVLTreeSeq.rs: 1
- AVLTreeSeqStEph.rs: 3
- AVLTreeSeqStPer.rs: 5
- AVLTreeSeqMtPer.rs: 3

Replace with real specs. Read the functions, determine the actual precondition (wf
predicate, bounds check, etc.), and add it. If truly no precondition, omit requires.

## Priority Order

1. BSTSplayMtEph.rs — duplicate `spec_is_bst_link`, add real requires/ensures
2. BSTRBMtEph.rs — same treatment
3. AVLTreeSeq holes — prove 3 external_body (unblocks dependency chain)
4. AVLTreeSeq `requires true` — replace 12 instances

## Important

- **Read BSTSplayStEph.rs and BSTRBStEph.rs** before touching the Mt files. The specs are
  already proved there — you're copying them.
- Mt files must NOT import from St counterparts. Duplicate `spec_is_bst_link`.
- Do NOT add `requires true`, `assume`, `accept`, or `external_body`.
- If a function genuinely needs no precondition, omit requires entirely.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- `spec_is_bst_link` duplicated into BSTSplayMtEph.rs and BSTRBMtEph.rs
- Real requires/ensures on all helper functions in both Mt files
- 3 AVLTreeSeq holes proved (or assessed with specific blockers)
- 12 `requires true` replaced in AVLTreeSeq files
- `plans/agent1-round26-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
