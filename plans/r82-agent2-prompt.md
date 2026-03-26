# R82 Agent 2 — Fix Chap44: DocumentIndex parse error, STEP 15

## Objective

Fix DocumentIndex.rs and Example44_1.rs in Chap44 so they compile and verify.

## lib.rs Memory Isolation Protocol

Before your first validate, comment out all chapters AFTER Chap44 to save memory.
Use EXACTLY this format:

```
/* R82-ISOLATED: agent 2, working on Chap44
#[cfg(all(not(feature = "experiments_only"), not(feature = "union_find")))]
pub mod Chap45 {
...
R82-ISOLATED */
```

Wrap lines 483-670 (Chap45 through Chap66 + closing) in this block.
Do NOT touch anything before Chap44. Do NOT touch Chap05 or Chap06.

**Before pushing to agent2/ready, REMOVE the isolation wrapper.** Restore lib.rs
to match main except for your Chap44 fixes. Verify with:
`git diff origin/main -- src/lib.rs` — only Chap44 lines should differ.

## What to fix

### 1. Uncomment both files in Chap44

```rust
pub mod Chap44 {
    pub mod DocumentIndex;
    pub mod Example44_1;
}
```

### 2. Debug the parse error in DocumentIndex.rs

The current error is:
```
error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `:`
  --> src/Chap44/DocumentIndex.rs:79:77
```

Line 79 is a trait method with a named return:
```rust
fn to_seq(docs: &DocumentSet) -> (seq: ArraySeqStPerS<DocumentId>)
    ensures
        docs@.finite(),
        seq@.to_set() =~= docs@,
        forall|i: int| 0 <= i < seq@.len() ==> #[trigger] docs@.contains(seq@[i]);
```

This pattern works in other traits. The type aliases are:
```rust
pub type DocumentId = String;
pub type DocumentSet = AVLTreeSetStPer<DocumentId>;
```

These aliases are declared OUTSIDE `verus!` but used inside. Investigate whether:
- The type alias resolution is the problem (try expanding aliases inline)
- The `String` type causes issues in this position
- The ensures on a trait method without a body needs different syntax
- Something else is wrong with the trait declaration

The other trait methods (lines 55-75) have NO ensures and work fine. Only `to_seq`
(which has ensures) fails. The 4 subsequent methods (empty, get_all_words,
word_count — lines 87-95) also fail because the parser aborts at line 79.

### 3. Fix any downstream compilation errors

Once DocumentIndex compiles, Example44_1 imports from it. Fix any issues there too.

## Important

- Read `src/Chap44/DocumentIndex.rs` and `src/Chap44/Example44_1.rs` fully before editing.
- Read the trait-impl pattern in CLAUDE.md and `src/standards/mod_standard.rs`.
- Do NOT add `assume` or `accept`.
- Do NOT add `external_body` on algorithmic functions.

## STEP 15

At most 15 edit/verify iterations. Then stop and report.

## Validation

Run `scripts/validate.sh` (with isolation), then before pushing restore lib.rs
and run full `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.
Push to `agent2/ready`.

## Report

Write `plans/agent2-round82-report.md` with what was wrong and how you fixed it.
