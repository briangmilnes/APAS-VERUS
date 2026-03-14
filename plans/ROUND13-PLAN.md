# Round 13 Plan — 4 Agents, 272 holes → target < 200

## Context

3999 verified, 272 holes, 38 clean, 8 holed. Previous rounds averaged 6 holes
per agent per round. That's unacceptable. This round demands 18+ per agent.

Stop reporting blockers. Start proving. If you can't prove it one way, try
another way. If you've exhausted your ideas, move to the next hole — don't
write a paragraph about why it's hard.

## Agent Assignments

### Agent 1: Chap43 Mt files — ALL 61 Mt assumes+ext_body, target -40

- OrderedSetMtEph.rs (36 assume, 3 ext_body = 39)
- OrderedTableMtPer.rs (14 assume, 8 ext_body = 22)

Strengthen RwLock invariants. Replace ALL assumes with asserts. Prove the
ext_body wrappers. Read AVLTreeSetMtEph.rs for the pattern. This is mechanical
— do it fast, do it all.

### Agent 2: Chap43 St/StPer bodies — ALL 53 ext_body+assume, target -20

- OrderedTableStEph.rs (14 ext_body)
- OrderedSetStEph.rs (14 ext_body, 1 assume)
- OrderedSetStPer.rs (12 ext_body)
- OrderedTableStPer.rs (10 ext_body)
- AugOrderedTableStPer.rs (2 assume)

These are deterministic AVL tree wrappers. Many delegate to one inner call.
Remove external_body, write the body, chain the inner ensures. Start with
the simplest (find, size, first, last, singleton) and work outward.

### Agent 3: feq Unlock — Chap41 + Chap38 — 32 holes, target -15

- ArraySetStEph.rs (3 feq assumes) — UNBLOCKS CHAP42
- AVLTreeSetStEph.rs (14 assumes)
- AVLTreeSetStPer.rs (10 assumes)
- BSTParaStEph.rs (15 assumes) — continue from R10

Apply your broadcast proof trick from R11. Add requires + cascade. Don't stop
at "cascade is too broad" — DO the cascade in your files, document it for others.

### Agent 4: Chap41 Mt + Chap42 + Chap39 + Chap47 — 51 holes, target -15

- AVLTreeSetMtEph.rs (10 holes)
- AVLTreeSetMtPer.rs (12 holes)
- TableMtEph.rs (11 ext_body)
- BSTTreapMtEph.rs (8 assume)
- Chap47 all files (10 holes)

Wide coverage. Prove the easiest hole in each file first, then go deeper.
Chap47 probe arithmetic is simple modular math — just do it.

## File Partition

| Agent | Chapters | Files |
|-------|----------|-------|
| 1 | Chap43 Mt | OrderedSetMtEph, OrderedTableMtPer, OrderedTableMtEph, AugOrderedTableMtEph |
| 2 | Chap43 St | OrderedTableStEph, OrderedSetStEph, OrderedSetStPer, OrderedTableStPer, AugOrderedTableStEph, AugOrderedTableStPer |
| 3 | Chap41 St, Chap38 | ArraySetStEph, AVLTreeSetStEph, AVLTreeSetStPer, BSTParaStEph |
| 4 | Chap41 Mt, Chap42, Chap39, Chap47 | AVLTreeSetMtEph, AVLTreeSetMtPer, TableMtEph, BSTTreapMtEph, all Chap47 |

## Combined Target: 272 → < 200 (-72)

| Agent | Holes | Target reduction |
|-------|-------|-----------------|
| 1 | 61 | -40 |
| 2 | 53 | -20 |
| 3 | 32 | -15 |
| 4 | 51 | -15 |
| **Total** | | **-90 (stretch) / -72 (minimum)** |

## Attitude

Prove or move on. Don't write 500 words about why something is blocked.
Try it, and if it fails, try the next hole. Volume matters this round.
