# R92 Agent 3 — Prove Clone preserves wf (eliminate 2 Chap41 assumes), STEP 20

## Objective

The 2 ClonePreservesWf assumes in Chap41 (AVLTreeSetStEph, AVLTreeSetStPer)
exist because `#[derive(Clone)]` has no Verus spec. Write manual Clone impls
with real ensures all the way down the type chain, eliminating the assumes.

## The type chain

```
AVLTreeSetStEph<T>          (Chap41)
  └── tree: ParamBST<T>    (Chap38/BSTParaStEph.rs)
        └── locked_root: RwLock<Option<Box<NodeInner<T>>>, Inv>
              └── NodeInner<T>
                    ├── key: T          (Copy for primitives, Clone for generic)
                    ├── left: ParamBST<T>   (recursive)
                    └── right: ParamBST<T>  (recursive)
```

## Strategy: Bottom up

### 1. NodeInner<T> — manual Clone inside verus!

Currently uses `#[derive(Clone)]` (outside verus! or no spec). Write:
```rust
impl<T: StT + Ord> Clone for NodeInner<T> {
    fn clone(&self) -> (result: Self)
        ensures result.key@ == self.key@,
                result.left@ == self.left@,
                result.right@ == self.right@,
    {
        NodeInner {
            key: self.key.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}
```

The recursive `self.left.clone()` calls ParamBST::clone (step 2).

### 2. ParamBST<T> — manual Clone inside verus!

ParamBST contains `RwLock<Option<Box<NodeInner<T>>>>`. Clone needs to:
- Acquire lock (read)
- Clone the `Option<Box<NodeInner<T>>>` inside
- Build new RwLock with same invariant
- Release lock

```rust
fn clone(&self) -> (result: Self)
    ensures result@ == self@
{
    // Use the existing acquire/release pattern from BSTParaStEph
    // The cloned tree has identical view
}
```

The key ensures: `result@ == self@`. The view of ParamBST is `Set<T::V>`.

### 3. AVLTreeSetStEph<T> — replace assume with proof

Once ParamBST clone ensures `result@ == self@`, AVLTreeSetStEph clone is:
```rust
fn clone_wf(&self) -> (result: Self) {
    let r = AVLTreeSetStEph { tree: self.tree.clone() };
    // r.tree@ == self.tree@ (from ParamBST clone ensures)
    // spec_avltreesetsteph_wf depends on tree@ — same view → same wf
    r
}
```

The assume disappears because wf is a function of the view, and the view
is preserved by clone.

**WAIT** — is wf actually a function of just the view? Check
`spec_avltreesetsteph_wf`. If it depends on structural properties beyond
the view (like tree balance), you need ParamBST clone to preserve those too.

### 4. Same for AVLTreeSetStPer

Repeat for the StPer variant in BSTParaStPer.rs → AVLTreeSetStPer.rs.

## Read first

- `src/Chap41/AVLTreeSetStEph.rs` — ClonePreservesWf impl at line 788
- `src/Chap38/BSTParaStEph.rs` — ParamBST struct, NodeInner, existing Clone
- `src/Chap41/AVLTreeSetStPer.rs` — StPer variant
- `src/Chap38/BSTParaStPer.rs` — StPer ParamBST (may be different from StEph)

Check where Clone is currently defined (inside or outside verus!):
```bash
grep -n "impl.*Clone.*for.*NodeInner\|impl.*Clone.*for.*ParamBST" src/Chap38/BSTParaStEph.rs src/Chap38/BSTParaStPer.rs
```

## Isolation

```bash
scripts/validate.sh isolate Chap41
```

Then check Chap52 (uses AVLTreeSetStEph):
```bash
scripts/validate.sh isolate Chap52
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify any file outside Chap38 and Chap41.
- Do NOT add assume or accept — the whole point is eliminating assumes.
- If `spec_avltreesetsteph_wf` depends on structural properties that View
  doesn't capture, document what's needed and leave the assume with a
  comment explaining the gap. That's still useful information.
- If the RwLock clone is too complex, focus on NodeInner first and see
  how far the proof chain reaches.

## STEP 20

## Report

Write `plans/agent3-r92-clone-chain-report.md`.
