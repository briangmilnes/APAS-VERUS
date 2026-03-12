# Agent 2 — Round 5: BST + Collections Spec Work

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
- `src/standards/multi_struct_standard.rs`

## Assignment

Chap37 (66 holes), Chap39 (38 holes), Chap42 (18 holes).
Total: 122 holes across 27 files.

## Priority

1. **Chap42** (18 holes) — TableMtEph (6 external_body algorithmic parallel with
   join()), TableStEph (4), TableStPer (4). Smallest chapter, quickest wins.
   The external_body holes on parallel join() code are permanent — leave them.
   Focus on any assume holes that can be proved.

2. **Chap37** (66 holes) — 7 assume (eq/clone in AVLTreeSeq*), 47 external_body
   (Mt parallel code + coarse lock), 12 trivial_wf. The assumes in eq/clone
   bodies are the approved pattern — leave them. The external_body on parallel
   Mt code is permanent. Focus on trivial_wf (add `// accept hole` comments
   where `{ true }` is correct for Vec-backed types).

3. **Chap39** (38 holes) — 38 external_body in BSTTreapMtEph, BSTSetTreapMtEph,
   BSTParaTreapMtEph. These are fine-grained concurrent BST — genuinely hard.
   Assess what's provable vs permanent. Leave what you can't prove.

## Rules

- Run `scripts/validate.sh` after each file or small batch.
- Show full output in response text.
- DO NOT add assume, accept, admit, or external_body.
- DO NOT convert existing assume to accept.
- If a hole can't be closed, leave it and move on.
- Commit to agent branch when done. Push.

## Success Criteria

- Chap37 trivial_wf holes closed with accept-hole comments.
- Chap42 hole count assessed, provable holes reduced.
- Zero new assumes or accepts introduced.
- validate.sh: 0 errors. rtt.sh: all pass.
