# Agent 4 — Round 5: DP + Remaining Near-Clean

## READ FIRST

Read CLAUDE.md sections on assume/accept BEFORE touching any code.
**DO NOT convert assume() to accept(). DO NOT add accept() anywhere.**
**DO NOT assume or accept closure requires/ensures in algorithmic code.**
If you add a single unauthorized accept(), the human loses 30 minutes
cleaning up after you. Don't.

Read these standards files before starting:
- `src/standards/using_closures_standard.rs`
- `src/standards/partial_eq_eq_clone_standard.rs`
- `src/standards/spec_wf_standard.rs`

## Assignment

Chap38 (33 holes), Chap50 (22 holes), Chap53 (23 holes),
Chap12 (1 hole), Chap26 (4 holes), Chap65 (1 hole), Chap66 (3 holes).
Total: 87 holes across 30 files.

## Priority

1. **Near-clean chapters first** — quick wins to get chapters to 0:
   - Chap12 (1 hole) — Exercise12_5 trivial_wf on lock-free stack. Add
     `// accept hole` if `{ true }` is correct.
   - Chap65 (1 hole) — UnionFindStEph. Assess and close if possible.
   - Chap66 (3 holes) — BoruvkaStEph. StdRng + raw HashMap holes. Assess.
   - Chap26 (4 holes) — ETSPStEph/MtEph. f64 sort/swap holes. Likely permanent.

2. **Chap50** (22 holes) — MatrixChainMtEph (8), OptBinSearchTreeMtEph (6),
   OptBinSearchTreeMtPer (3), plus St files with 1 each. All have clean deps.
   These are the DP memoization modules. Lock-boundary assumes on readers,
   external_body on parallel recursion. Prove readers where possible.

3. **Chap53** (23 holes) — GraphSearchStEph (4 assume), rest external_body on
   Mt search implementations. Assess what's provable.

4. **Chap38** (33 holes) — BSTParaStEph/MtEph. Per-node locking, genuinely
   hard parallel BST. Assess but don't expect big reductions.

## Rules

- Run `scripts/validate.sh` after each file or small batch.
- Show full output in response text.
- DO NOT add assume, accept, admit, or external_body.
- DO NOT convert existing assume to accept.
- If a hole can't be closed, leave it and move on.
- Commit to agent branch when done. Push.

## Success Criteria

- Chap12 reaches 0 holes (trivial_wf accept-hole comment).
- Chap65, Chap66 assessed, reduced where possible.
- Chap50 reader lock-boundary holes proved where possible.
- Zero new assumes or accepts introduced.
- validate.sh: 0 errors. rtt.sh: all pass.
