# Agent 4 Round 6 — Ordered Sets + Tables (Chap43 + Chap42 + Chap41)

## Mode: AFK — execute relentlessly

Read CLAUDE.md and `src/standards/*.rs` before starting. Do the proof work.
Run `scripts/validate.sh` after each file. Fix errors before moving on.
When done, commit all changes, push to `agent4/ready`, then stop.

## Assignment

**Focus on Mt wrappers and set-semantics assumes. Target: -30.**

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 43 | OrderedSetMtEph.rs | 39 | assume (lock-boundary) |
| 2 | 43 | OrderedTableMtEph.rs | 16 | ext_body (Mt delegation) |
| 3 | 43 | OrderedTableMtPer.rs | 22 | ext_body |
| 4 | 42 | TableMtEph.rs | 15 | 2 assume + 13 ext_body |
| 5 | 41 | AVLTreeSetStEph.rs | 17 | assume (set semantics) |
| 6 | 41 | AVLTreeSetStPer.rs | 12 | assume (set semantics) |
| 7 | 41 | AVLTreeSetMtPer.rs | 13 | assume |
| 8 | 41 | ArraySetStEph.rs | 9 | assume |

Files NOT assigned (leave alone):
- Chap43: OrderedSetStEph (15), OrderedSetStPer (13), OrderedTableStEph (16),
  OrderedTableStPer (10), AugOrderedTable* (12 total).
- Chap41: AVLTreeSetMtEph (24), ArraySetEnumMtEph (5), Example41_3 (4).
- Chap42: TableStEph (1), TableStPer (2).

## Strategy

### Phase 1: OrderedSetMtEph lock-boundary (39 assumes, mechanical)

OrderedSetMtEph.rs wraps OrderedSetStEph via RwLock. All 39 holes are lock-boundary
assumes bridging the ghost shadow to locked values. This is the same pattern as
Chap06 graph wrappers.

1. Read the struct, View impl, lock predicate, ghost shadow.
2. For each assume: reader accept (value from lock matches ghost) or provable bridge.
3. Work through methodically — insert, delete, find, size, union, intersect, etc.

Expected: ~20-30 provable or acceptable.

### Phase 2: Chap41 set-semantics assumes (real proof work)

AVLTreeSetStEph.rs (17 holes) and AVLTreeSetStPer.rs (12 holes) have operation-spec
assumes: insert produces `old@.insert(x@)`, delete produces `old@.remove(x@)`,
union/intersection/difference match set operations.

These are real proof work. The implementations use sorted sequences (AVLTreeSeq)
and the proofs need to connect tree operations to set operations.

Prior work (Round 4, Agent 2): 9 wf-bridge assumes removed from StEph, 2 from StPer.
The remaining assumes are the harder set-semantics ones.

ArraySetStEph.rs (9 holes): simpler flat-array set. May be easier to prove.

### Phase 3: TableMtEph lock-boundary (15 holes)

Similar to OrderedSetMtEph but for key-value tables. 2 assumes + 13 ext_body.

### Phase 4: OrderedTable Mt delegation (if time permits)

OrderedTableMtEph (16) and OrderedTableMtPer (22) delegate to OrderedSet + Table.
These are ext_body holes wrapping combined operations.

## Dependency Note

Chap41 → Chap42 → Chap43. Changes to Chap41 AVLTreeSet specs may cascade into
Chap43 OrderedSet. If you strengthen AVLTreeSetStEph ensures, check that
OrderedSetStEph still verifies.

## Rules

- Do NOT convert `assume` to `accept` wholesale. Read each assume, try to prove first.
- Do NOT add new assumes or external_body.
- Do NOT modify files outside your assignment (Chap41, 42, 43 — assigned files only).
- Run `scripts/validate.sh` after each file change.
- Check cascades: Chap41 changes may affect Chap42/43.

## Prior Work

Agent 4's Round 4 report is in `plans/AGENT4.md`. Key context:
- BSTParaStEph.rs (Chap38): 7 assumes proved via view_ord_consistent (Round 4).
- Arc<RwLock> removed from BSTParaStEph — now plain RwLock + ghost shadow.
- BottomUpDP St loops proved in Chap51 (Round 4).
- Chap43/45/47/49/51 holes documented in "Not Attempted" section.

Agent 2's Chap41 work (in `plans/AGENT2.md`):
- AVLTreeSetStEph: wf-bridge fix removed 9 assumes (Round 4).
- AVLTreeSetStPer: from_vec wf removed 2 assumes (Round 4).
- Remaining: 17 StEph + 12 StPer are operation-spec (set semantics).

## Baseline

3771 verified, 0 errors. Chap43: 143 holes. Chap42: 18 holes. Chap41: 84 holes.

## Deliverable

When done: commit, push to `agent4/ready`, update `plans/AGENT4.md` with results.
