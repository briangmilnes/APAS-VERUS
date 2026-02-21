<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Proof Holes by Directory (Veracity Run)

Run: `veracity-review-proof-holes -d src/<dir>` on each directory.

## Summary by Directory

| # | Dir | Clean | Holed | Holes | Notes |
|---|-----|-------|-------|-------|------|
| 1 | Chap02 | 1 | 1 | 8 | HFSchedulerMtEph: external_body, assume(false) |
| 2 | Chap03 | 1 | 0 | 0 | All clean |
| 3 | Chap05 | 5 | 0 | 0 | assume_eq_clone workarounds (info) |
| 4 | Chap06 | 20 | 0 | 0 | All clean |
| 5 | Chap11 | 2 | 3 | 6 | assume(false) in thread join (accepted) |
| 6 | Chap12 | 1 | 2 | 18 | Exercise12_1, Exercise12_5 |
| 7 | Chap17 | 1 | 0 | 0 | assume_eq_clone workarounds |
| 8 | Chap18 | 0 | 7 | 2 | ArraySeq external, assume_eq_clone |
| 9 | Chap19 | 4 | 0 | 0 | assume_eq_clone workarounds |
| 10 | Chap21 | 12 | 0 | 0 | All clean |
| 11 | Chap23 | 0 | 2 | 9 | assume_eq_clone workarounds |
| 12 | Chap26 | 6 | 2 | 8 | ETSP: assume, external_body |
| 13 | Chap27 | 4 | 0 | 0 | All clean |
| 14 | Chap28 | 11 | 0 | 0 | All clean |
| 15 | Chap35 | 2 | 2 | 2 | external_body partition |
| 16 | Chap36 | 2 | 1 | 1 | QuickSortMtEphSlice external_body |
| 17 | Chap37 | 0 | 1 | 15+ | AVLTreeSeq: assume, external_body |
| 18 | Chap38 | 2 | 0 | 3 | BSTPara: RwLock, not_verusified |
| 19 | Chap39 | 0 | 4 | 14+ | BSTSetTreap, BSTTreap external_body |
| 20 | Chap40+ | (see full log) | | | |

## Hole Types

- **assume(false)** in thread join — accepted idiom (assume-false-diverge rule)
- **assume_eq_clone_workaround** — Verus limitation, info only
- **external_body** — threading, FFI, or unverified helpers
- **assume()** — proof gaps to close (e.g. ETSP, AVLTreeSeq)
