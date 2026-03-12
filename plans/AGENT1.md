# Agent 1 — Round 5: Graphs + Foundation Near-Clean

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

Chap05 (9 holes), Chap06 (51 holes), Chap18 (6 holes), Chap19 (3 holes).
Total: 69 holes across 36 files.

## Priority

1. **Chap18** (6 holes) — all trivial_spec_wf `{ true }`. Vec-backed types where
   `true` IS the correct wf. Add `// accept hole` comment to silence the checker.
   This gets Chap18 to 0 holes.

2. **Chap19** (3 holes) — same pattern as Chap18. Trivial wf on Vec-backed seqs.
   Gets Chap19 to 0 holes.

3. **Chap06** (51 holes) — all assumes in MtEph graph files. These are coarse RwLock
   lock-boundary assumes (`inner@ == self@`, result forwarding). The pattern is
   `acquire_read → borrow → delegate to St method → assume result matches`.
   DO NOT convert these to accept. Leave them as assume. Focus on:
   - Proving `inner@ == self@` from the RwLock invariant if possible.
   - If not provable, leave the assume and move on.

4. **Chap05/SetMtEph** (9 holes) — same lock-boundary pattern. `type_invariant`
   proof fn has 2 assumes for finiteness + valid_key_type. Leave these unless you
   can prove them from the RwLock invariant.

## Rules

- Run `scripts/validate.sh` after each file or small batch.
- Show full output in response text.
- DO NOT add assume, accept, admit, or external_body.
- DO NOT convert existing assume to accept.
- If a hole can't be closed, leave it and move on.
- Commit to agent branch when done. Push.

## Success Criteria

- Chap18 and Chap19 reach 0 holes (trivial_wf accept-hole comments).
- Chap06 hole count reduced (lock-boundary proves).
- Zero new assumes or accepts introduced.
- validate.sh: 0 errors. rtt.sh: all pass.
