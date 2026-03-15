# Review Against Prose: Chapter 62 — Star Contraction

Date: 2026-03-15
Reviewer: Claude-Opus-4.6 (Agent 4, Round 21)

## Phase 1: Inventory

Source: `src/Chap62/analyses/veracity-review-module-fn-impls.md`

| # | Chap | File | Exec Fns | Spec Fns | Holes |
|---|------|------|----------|----------|-------|
| 1 | 62 | StarPartitionStEph.rs | 1 | 1 | 0 |
| 2 | 62 | StarPartitionMtEph.rs | 1 | 1 | 0 |
| 3 | 62 | StarContractionStEph.rs | 2 (+1 helper) | 1 | 0 |
| 4 | 62 | StarContractionMtEph.rs | 2 (+2 helpers) | 1 | 0 |

Total: 6 trait-level exec fns, 3 helper fns, 4 wf spec fns, 0 holes.

## Phase 2: Prose Inventory

Source: `prompts/Chap62.txt`

| # | Chap | Prose Item | Type |
|---|------|-----------|------|
| 1 | 62 | Defn 62.1: Star Partition | Definition |
| 2 | 62 | Defn 62.2: Isolated Vertices | Definition |
| 3 | 62 | Alg 62.3: Parallel Star Partition | Algorithm |
| 4 | 62 | Defn 62.4: Star Contraction | Definition |
| 5 | 62 | Alg 62.5: Star Contraction (higher-order) | Algorithm |
| 6 | 62 | Thm 62.1: Cost of Star Partition | Theorem |
| 7 | 62 | Lemma 62.2: Number of Satellites | Lemma |
| 8 | 62 | Thm 62.3: Work/Span of Star Contraction | Theorem |
| 9 | 62 | Ex 62.1 (text proof of Thm 62.1) | Exercise |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All exec functions already have APAS + Claude-Opus-4.6 cost annotations.

| # | Chap | File | Function | APAS Cost | Claude Assessment |
|---|------|------|----------|-----------|-------------------|
| 1 | 62 | StarPartitionStEph.rs | sequential_star_partition | W O(n+m), S O(n+m) | Agrees |
| 2 | 62 | StarPartitionMtEph.rs | parallel_star_partition | W O(n+m), S O(lg n) | W O(n+m), S O(n+m) |
| 3 | 62 | StarContractionStEph.rs | star_contract | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 4 | 62 | StarContractionStEph.rs | build_quotient_graph | (no cost stated) | W O(m), S O(m) |
| 5 | 62 | StarContractionStEph.rs | contract_to_vertices | W O((n+m) lg n), S O((n+m) lg n) | Agrees |
| 6 | 62 | StarContractionMtEph.rs | star_contract_mt | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O((n+m) lg n) |
| 7 | 62 | StarContractionMtEph.rs | build_quotient_graph_parallel | (no cost stated) | W O(m), S O(lg m) |
| 8 | 62 | StarContractionMtEph.rs | route_edges_parallel | (no cost stated) | W O(k), S O(lg k) |
| 9 | 62 | StarContractionMtEph.rs | contract_to_vertices_mt | W O((n+m) lg n), S O(lg^2 n) | W O((n+m) lg n), S O((n+m) lg n) |

### 3b. Implementation Fidelity

| # | Chap | File | Function | Deviation |
|---|------|------|----------|-----------|
| 1 | 62 | StarPartitionStEph.rs | sequential_star_partition | Faithful to prose sequential construction (greedy vertex selection) |
| 2 | 62 | StarPartitionMtEph.rs | parallel_star_partition | Matches Alg 62.3 (coin flip, TH edges, inject). Uses HashMap for inject instead of Seq.inject; all loops sequential |
| 3 | 62 | StarContractionStEph.rs | star_contract | Faithful to Alg 62.5 (higher-order base/expand pattern) |
| 4 | 62 | StarContractionMtEph.rs | star_contract_mt | Matches Alg 62.5; quotient edge build uses ParaPair! |

### 3c. Spec Fidelity

All trait fns have `requires spec_wf(graph)` (graph well-formedness). No functional ensures (no postcondition about partition validity, contraction correctness, or satellite count). The prose provides cost bounds and the satellite count lemma (Lemma 62.2: at least n/4 satellites in expectation), but these are probabilistic statements that cannot be directly encoded as Verus ensures.

Spec strength: **weak** -- wf preconditions only.

## Phase 4: Parallelism Review

| # | Chap | File | Function | Parallel? | Mechanism |
|---|------|------|----------|-----------|-----------|
| 1 | 62 | StarPartitionMtEph.rs | parallel_star_partition | No | All loops are sequential; no ParaPair! or join() |
| 2 | 62 | StarContractionMtEph.rs | star_contract_mt | Partial | Quotient edge build uses ParaPair! via route_edges_parallel |
| 3 | 62 | StarContractionMtEph.rs | route_edges_parallel | Yes | ParaPair! divide-and-conquer |
| 4 | 62 | StarContractionMtEph.rs | build_quotient_graph_parallel | Yes | Delegates to route_edges_parallel |

Parallelism assessment: `parallel_star_partition` is **not actually parallel** -- all four loops (vertex indexing, coin flips, TH edge construction, partition assembly) are sequential. This is a significant deviation from the prose (Alg 62.3), which achieves O(lg n) span via parallel coin flips and Seq.inject. The star contraction Mt file does achieve parallelism in quotient edge routing via ParaPair!. The sequential partition dominates overall span, making the claimed O(lg^2 n) unreachable.

## Phase 5: Runtime Test Review

| # | Chap | Test File | Coverage |
|---|------|-----------|----------|
| 1 | 62 | TestStarPartitionStEph.rs | sequential_star_partition |
| 2 | 62 | TestStarPartitionMtEph.rs | parallel_star_partition |
| 3 | 62 | TestStarContractionStEph.rs | star_contract, contract_to_vertices |
| 4 | 62 | TestStarContractionMtEph.rs | star_contract_mt, contract_to_vertices_mt |

All 4 source files have corresponding RTTs. Coverage is complete for trait-level functions.

## Phase 6: PTT Review

No PTTs exist for Chap62. None are needed.

## Phase 7: Gap Analysis

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 62 | parallel_star_partition is sequential | High | All loops sequential; O(lg n) span from prose not achieved |
| 2 | 62 | No functional ensures | Medium | No postcondition on partition validity or contraction ratio |
| 3 | 62 | StarPartitionMtEph uses HashMap not Seq.inject | Low | Prose uses Seq.inject for parallel updates; HashMap is sequential |
| 4 | 62 | route_edges_parallel merge is sequential | Low | After ParaPair! split, merging right into left is O(k/2) sequential |

## Phase 8: TOC Review

All files follow the standard TOC structure. Trait definitions inside verus!; implementations outside (cfg(not(verus_keep_ghost))). StarPartitionStEph and StarContractionStEph have clean separation. The `use std::collections::HashMap` import appears unconditionally in StarContractionStEph.rs and StarPartitionMtEph.rs (line 15-16 area) rather than under cfg(not(verus_keep_ghost)); this is acceptable since HashMap is used in type signatures that Verus needs to parse.

## Summary

Chapter 62 implements the star partition (Alg 62.3) and star contraction (Alg 62.5) in both St and Mt variants. All 4 modules are **clean** (0 holes). The star contraction Mt variant achieves genuine parallelism in quotient edge routing via ParaPair!. However, the star partition Mt variant (`parallel_star_partition`) is entirely sequential despite its name -- all loops iterate sequentially. This is the most significant gap: the O(lg n) span from the prose is not achieved. Cost annotations are present on all exec functions. Specs are structural (wf-only), matching the prose's informal treatment. RTT coverage is complete.
