# Session restart — slidesCMUCS.md

Active deliverable: `lectures/slidesCMUCS.md` (CMU CS talk).

## Status

| # | Status | Item |
|---|---|---|
| 1 | done | `lectures/build.sh` produces both `slidesCMUCS.pdf` and `slidesCMUCS.pptx` (79 slides) on every run |
| 2 | done | pptx XML now has `lang="en-US"` on every `<a:rPr>` (LibreOffice search regression — turned out to be a stale open file in Impress, not the build) |
| 3 | open | Add 3 code slides showing Verus `fn` w/ requires/ensures, traits, impls. Insertion point: between slide 16 "Verus" (line 170) and slide 17 "Views and the Libraries" (line 185) |
| 4 | open | Add 4 slides after slide 66 — content not yet drafted; user has not specified what they cover |
| 5 | open | Cut ~7 slides from main talk (slides 1–69) to fit. Candidates queued below. After-Extras (70–79) are out of scope — leave alone |
| 6 | open | Rewrite the two "Outline" slides as one — proposed bullet list below (11 bullets, or 8 if paired sections fold) |

## Cuts queued for slides 1–69

Pick any 7. Each row is a same-titled consecutive pair/triple that merges cleanly.

| # | Lines | Title | Merge | Saves |
|---|---|---|---|---|
| 1 | 11, 21 | Outline of the talk | 2→1 | 1 |
| 2 | 111, 124 | APAS-AI | 2→1 | 1 |
| 3 | 205, 217 | Wrapping Rust — external fn/method | 2→1 | 1 |
| 4 | 241, 250, 258 | Tokenized State Machines | 3→2 | 1 |
| 5 | 297, 304, 314, 325 | Quantitatives | 4→2 | 2 |
| 6 | 343, 353 | Pain Points | 2→1 | 1 |
| 7 | 362, 372 | AutoCLRS | 2→1 | 1 |
| 8 | 379, 392, 401 | Veracity — SE AIPP | 3→2 | 1 |
| 9 | 592, 601, 613 | SE in AIPP | 3→2 | 1 |
| 10 | 638, 650, 662 | SE in AIPP — Problems | 3→2 | 1 |
| 11 | 714, 723 | Internet Apocalypse | 2→1 | 1 |

Priority picks: #5 Quantitatives (saves 2) and #4 Tokenized State Machines (Hance's work, three slides is a lot for a citation).

## Proposed single-slide outline (replaces slides at lines 11 + 21)

11-bullet form:

```markdown
# Outline of the talk

- Background
- Algorithms Parallel and Sequential (APAS)
- Rust — The Good, The Bad, The Ugly
- APAS-AI — AI Paired Programming APAS in Rust
- Rusticate — sending Python back to the family estate
- Verus — Proving Rust
- APAS-VERUS — AI Paired Proving APAS in Verus
- Veracity — Software Engineering AI Paired Proving
- Software Engineering in AI Paired Proving
- The AI Paired Programming Interfaces
- The Internet Apocalypse
```

8-bullet form (folds pairs that travel together):

```markdown
# Outline of the talk

- Background
- APAS and APAS-AI — Algorithms in Rust, AI-paired
- Rust — The Good, The Bad, The Ugly
- Rusticate — sending Python back to the family estate
- Verus and APAS-VERUS — Proving Rust, at scale
- Veracity and Software Engineering in AI Paired Proving
- The AI Paired Programming Interfaces
- The Internet Apocalypse
```

## Side notes (no action needed)

- Verus `ResourceAlgebra` trait: new, on `bsdinis/*` MIT branches, NOT in pinned tag `release/rolling/0.2026.04.20.8dcd677`. Splits old PCM into RA (no required unit) + PCM (RA + unit). `Option<RA>` is the lift back to PCM. Combinators added: `Auth`, `Exclusive`, `Agreement`, `Sum`, `Product`. Not relevant for APAS-VERUS today; revisit if/when verified concurrent data structures are on the table.
- Project baseline (per memory): 5765 verified, 4209 rtt, 289 ptt, 0 holes, 46 clean chapters. Confirm before any new round.

## First things on resume

1. `ls -la lectures/slidesCMUCS.{md,pdf,pptx}` — confirm artifacts
2. Decide what the 4 new post-#66 slides cover (user input required)
3. Pick the 7 cuts from the table above
4. Specify which Verus features to feature in the 3 code slides (then ask Claude to sketch)
