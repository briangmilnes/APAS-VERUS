# Agent 3 — Round 5: Tables + Priority Queues

## READ FIRST

Read CLAUDE.md sections on assume/accept BEFORE touching any code.
**DO NOT convert assume() to accept(). DO NOT add accept() anywhere.**
**DO NOT assume or accept closure requires/ensures in algorithmic code.**
If you add a single unauthorized accept(), the human loses 30 minutes
cleaning up after you. You did this last round. Don't do it again.

Read these standards files before starting:
- `src/standards/using_closures_standard.rs`
- `src/standards/partial_eq_eq_clone_standard.rs`
- `src/standards/spec_wf_standard.rs`

## Assignment

Chap43 (132 holes), Chap45 (26 holes), Chap47 (39 holes).
Total: 197 holes across 27 files.

## Priority

1. **Chap45** (26 holes) — BinaryHeapPQ (8 holes, clean deps). Also SortedListPQ,
   UnsortedListPQ, LeftistHeapPQ. Focus on BinaryHeapPQ first — assess which
   assumes are lock-boundary vs algorithmic. Leave what you can't prove.

2. **Chap47** (39 holes) — Hash table implementations. ParaHashTableStEph has
   1 hole with clean deps — start there. The rest are internal-dep blocked.
   Assess and reduce where possible.

3. **Chap43** (132 holes) — OrderedSet/OrderedTable, the heaviest chapter.
   Many are Mt coarse lock assumes + eq/clone assumes. Leave assumes as-is.
   Focus on any trivial_wf or fn_missing_spec that can be fixed mechanically.

## Rules

- Run `scripts/validate.sh` after each file or small batch.
- Show full output in response text.
- DO NOT add assume, accept, admit, or external_body.
- DO NOT convert existing assume to accept.
- Seriously. You mass-converted assumes to accepts in round 4. That cost
  the human 30+ minutes of manual cleanup across 26 files. If you do it
  again you will be reset to main and your work discarded.
- If a hole can't be closed, leave it and move on.
- Commit to agent branch when done. Push.

## Success Criteria

- Chap45 BinaryHeapPQ holes assessed and reduced where provable.
- Chap47 ParaHashTableStEph (1 hole) closed if possible.
- Chap43 hole count assessed, trivial fixes applied.
- Zero new assumes or accepts introduced.
- validate.sh: 0 errors. rtt.sh: all pass.
