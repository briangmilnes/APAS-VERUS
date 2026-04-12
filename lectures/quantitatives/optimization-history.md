# Lecture Data — Section 8: Optimization History

Generated: 2026-04-11. Source: keyword grep across `plans/agent*-round*-report.md`.

## Round-report keyword frequency

| # | Keyword | Reports mentioning | Notes |
|---|---------|--------------------|-------|
| 1 | any optimization keyword | 90 / 281 reports | one or more of the below |
| 2 | "opaque" | 32 | both `pub closed spec fn` and `#[verifier::opaque]` |
| 3 | "matching loop" | 11 | Z3 quantifier matching loops |
| 4 | "broadcast" / "cross-fire" | 15 | broadcast group quantifier interactions |
| 5 | "rlimit" | 20 | rlimit overrides or budget exhaustion |
| 6 | "decompose" / "micro-split" | included in agg | wf decomposition pattern |

## Patterns captured in `docs/VerusOptimizationsApplied.md`

(See that file for full Problem / What-didn't-work / What-worked
detail per pattern. List below for cross-reference.)

| # | Pattern | Origin round | Where applied |
|---|---------|--------------|---------------|
| 1 | `#[verifier::opaque]` vs `pub closed spec fn` | R195 | UnionFindPCStEph |
| 2 | Z3 conjunction flakiness workaround | R28 | many chapters |
| 3 | Broadcast group cross-fire | R175 | Kruskal (now removed in R196) |
| 4 | Decompose wf into per-clause lemmas | R172, R192 | UnionFindPCStEph |
| 5 | Closure clone workaround (`clone_fn`, `clone_pred`) | early | vstdplus + D&C helpers |
| 6 | Ghost-capture-before-call | R195 | UnionFindPCStEph::find |
| 7 | Iterator `assume(iter_invariant)` policy | various | every iterator-bearing collection |
| 8 | HFScheduler instead of raw thread::spawn | foundational | every Mt module |
| 9 | Trait-impl pattern + RwLockPredicate naming | foundational | codebase-wide |
| 10 | Eq/Clone bridge: in-impl-body `assume` | foundational | every PartialEq/Clone impl |

## Big optimization landings (round-by-round narrative)

Drawn from round-end commit messages and report headers.

| # | Round | Landing | Impact |
|---|-------|---------|--------|
| 1 | R28 | Conjunction flakiness workaround pattern documented | unblocked many chapters |
| 2 | R130 | Major holes-closure round (R130 closes 30→…) | mid-project sweep |
| 3 | R162 | 173 vacuous APAS annotations removed | quality cleanup |
| 4 | R170 | Big minimize-proofs landing (18 chapters, 5 agents) | tooling milestone |
| 5 | R172 | wf decomposition into 5 opaque groups (Chap65 UF) | killed 9M-instantiation matching loop |
| 6 | R175 | Broadcast group cross-fire wrapper pattern | unblocked Kruskal |
| 7 | R175 | Eq/clone assume → accept promotion (160 sites, 66 files) | trust-base cleanup |
| 8 | R176 | Chap43 OrderedTableMtEph minimization (z3 −57%) | big single-file win |
| 9 | R192 | UnionFindPCStEph compress_step_wf split (3 lemmas) | rlimit relief |
| 10 | R195 | `#[verifier::opaque]` recognized as fix for find() loop invariant | killed 139K instantiation matching loop |
| 11 | R196 | UnionFind relocated to Chap65; old UF (3 external_body) retired | strict upgrade; deleted obsolete uf_opaque_wrappers nested module |

## Hot-spot files (most rounds of work per file)

| # | Chap | File | Work concentration |
|---|------|------|--------------------|
| 1 | 65 | UnionFindPCStEph.rs | R190–R196 (path compression saga) |
| 2 | 65 | KruskalStEph.rs | R130, R175, R196 (UF integration) |
| 3 | 43 | OrderedTableMtEph.rs | R175–R176 (minimization) |
| 4 | 36 | (Z3 trait-axiom flakiness fixes) | R195 Verus upgrade |
| 5 | 35 | (Z3 trait-axiom flakiness fixes) | R195 Verus upgrade |
| 6 | 41 | OrdKeyMap.rs (Send/Sync impls) | awaiting upstream |

## Net Z3 reduction examples

- Chap43 OrderedTableMtEph: z3 RSS −57% after R176 minimize.
- Chap59 Johnson files: z3 RSS −31% (756→520 MB) after R175 merge.
- Chap65 UnionFindPCStEph::find: 139,490 quantifier instantiations
  killed entirely by R195 opaque pattern (matching loop disappeared).

## Caveats

- The keyword frequency is a lower bound — many reports describe
  optimizations without using the canonical keyword.
- "Big landings" list is curated; full audit would require reading
  all 281 reports.
- Z3 reduction percentages are from individual commit messages — a
  systematic before/after profile sweep would be needed for a
  publication-grade plot.
