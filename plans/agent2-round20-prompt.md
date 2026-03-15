# Agent 2 — Round 20: Prove Chap05 (3 holes) + Chap38 (17 holes)

## Mission

Two chapters with clean dependencies (no external blockers). Fix 20 holes total.

## Part A: Chap05 MappingStEph (3 holes)

Three `external_body` on simple delegation functions in `MappingStEph.rs`.

### Holes

| # | Line | Function | Current Body |
|---|------|----------|-------------|
| 1 | 386 | from_vec | `MappingStEph { mapping: RelationStEph::from_vec(v) }` |
| 2 | 392 | from_relation | `MappingStEph { mapping: r.clone() }` |
| 3 | 397 | size | `self.mapping.size()` |

### Required Reading

- `src/Chap05/MappingStEph.rs` — read the trait ensures and the View impl.
- `src/Chap05/RelationStEph.rs` — MappingStEph delegates to RelationStEph.

### Fix Strategy

These delegate to RelationStEph methods whose ensures should propagate. Read the
trait ensures clauses (added in R19), understand what they promise, then:

1. Remove `#[verifier::external_body]`.
2. Add proof assertions to bridge MappingStEph's View to RelationStEph's ensures.
3. If RelationStEph's ensures are too weak to prove MappingStEph's ensures,
   strengthen RelationStEph (it's in the same chapter, clean deps).

## Part B: Chap38 BSTParaStEph (8 holes)

### Holes

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 446 | expose | assume() | T::clone has no verified ensures |
| 2 | 530 | insert | assume() | `left@.len() + right@.len() < usize::MAX` |
| 3 | 541 | delete | assume() | Same overflow bound |
| 4 | 892 | union | assume() | Same overflow bound |
| 5 | 1275 | reduce | external_body | Fold over tree |
| 6 | 1299 | in_order | external_body | Tree to sorted sequence |
| 7 | 1312 | filter_inner | external_body | Filter over tree |
| 8 | 1428 | clone | external_body | Clone impl |

### Fix Strategy

**expose (assume #1)**: The assume is about `T::clone` not having verified ensures.
This is the eq/clone workaround pattern — the assume is INSIDE clone, which is
allowed per CLAUDE.md. Read the code to confirm it's the standard pattern. If so,
this assume is expected and not a target. Leave it.

**insert/delete/union overflow assumes (#2-4)**: These assume
`left@.len() + right@.len() < usize::MAX`. This is a practical bound — BSTs with
2^64 elements don't exist. Options:
- Add `requires self@.len() < usize::MAX as nat` to the trait (propagates the bound).
- Or leave the assume with a comment — it's a practical axiom, not algorithmic.
Read the trait ensures to decide which approach fits better.

**reduce (#5)**: Tree fold. Needs recursive traversal proof. Check if there's a
`to_seq` or `in_order` function you can use to convert to sequence first, then
fold. Or write a direct inductive proof on the tree structure.

**in_order (#6)**: Tree to sorted sequence. Needs inductive proof that in-order
traversal produces a sorted sequence matching the set view. This is a classic BST
proof — left subtree keys < root < right subtree keys.

**filter_inner (#7)**: Filter over tree. Needs proof that filtered tree maintains
BST invariant and contains exactly the filtered elements.

**clone (#8)**: Clone impl. This is the eq/clone workaround pattern — use the
standard `assume` inside the body. Read `src/standards/partial_eq_eq_clone_standard.rs`.

### Priority

Do #5-7 (reduce, in_order, filter_inner) first — those are the real algorithmic
proofs. #1 and #8 are eq/clone workarounds (expected). #2-4 are overflow bounds
(practical axioms).

## Part C: Chap38 BSTParaMtEph (9 holes)

### Holes

| # | Function | Type |
|---|----------|------|
| 1 | View impl | external_body |
| 2 | split_inner | assume_specification |
| 3-9 | join_pair, union, intersect, difference, filter, reduce, in_order | external_body |

### Fix Strategy

BSTParaMtEph wraps BSTParaStEph with Arc/RwLock for thread safety. The Mt holes are
delegation wrappers. If you prove the StEph versions, the MtEph versions may become
provable by delegating through the lock.

Read the MtEph code to understand the wrapping pattern. The `assume_specification`
on `split_inner` is a Verus interop issue — check if it can be replaced with a
normal function call.

For Mt external_body functions that just delegate: try removing external_body and
letting the ensures propagate from St. For ones with actual threading (`join_pair`),
the external_body may be necessary (thread spawn boundary).

## Procedure

1. Read `src/Chap05/MappingStEph.rs` and fix the 3 holes.
2. `scripts/validate.sh` — 0 errors.
3. Read `src/Chap38/BSTParaStEph.rs` — fix holes #5-7 first (algorithmic proofs).
4. `scripts/validate.sh` — iterate.
5. Fix remaining BSTParaStEph holes.
6. Read `src/Chap38/BSTParaMtEph.rs` — fix what you can.
7. Final `scripts/validate.sh` — 0 errors.

## Important

- Do NOT modify files outside Chap05 and Chap38.
- Do NOT add `assume` or `accept` (except the existing eq/clone pattern in clone bodies).
- Overflow assumes in insert/delete/union: add `requires` to propagate the bound,
  or leave with a comment — ask yourself which is cleaner.
- For BST proofs, search vstd for `sorted`, `set`, `fold_left` lemmas before writing new ones.

## Deliverables

- Holes reduced in Chap05 and Chap38.
- `plans/agent2-round20-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
