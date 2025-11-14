<style>
body {
    max-width: 1600px;
    margin: 0 auto;
    padding: 20px;
}
</style>

# APAS-VERUS Custom Iterator Traits

## Overview

APAS-VERUS has developed custom traits (`GhostIteratorTrait` and `ExecIteratorTrait`) specifically for manual `while` and `loop` patterns where explicit control over the iterator is needed. This approach provides more flexibility for complex verification scenarios but requires manual invariant management.

---

## Traits

| Name | File | Line | Used in Loop Types | Purpose |
|------|------|------|---------------------------------|---------|
| `GhostIteratorTrait` | `src/experiments/verus_iterator.rs` | 5 | while ✅, loop ✅, for (range) ☐, for (general) ⚠️ | Custom ghost state for manual `while`/`loop` iteration. Defines same methods as `ForLoopGhostIterator` but designed for explicit `next()` calls. |
| `ExecIteratorTrait` | `src/experiments/verus_iterator.rs` | 18 | while ✅, loop ✅, for (range) ☐, for (general) ⚠️ | Custom executable iterator with `iter()` and `next()` methods. Provides ghost state connections via `requires`/`ensures`. |

## Structs

| Name | File | Line | Used in Loop Types | Purpose |
|------|------|------|---------------------------------|---------|
| `VecGhostIter` | `src/experiments/verus_vec_iterator.rs` | 6 | while ✅, loop ✅, for (range) ☐, for (general) ⚠️ | Ghost state: `cur: int`, `end: int`, `data: Seq<usize>`. |
| `VecExecIter` | `src/experiments/verus_vec_iterator.rs` | 59 | while ✅, loop ✅, for (range) ☐, for (general) ⚠️ | Exec state: `data: Vec<usize>`, `cur: usize`. |
| `VecCollection` | `src/experiments/verus_vec_iterator.rs` | 76 | while ✅, loop ✅, for (range) ☐, for (general) ⚠️ | Wrapper providing `iter()` method. |

## Implementations

| Name | File | Line | Used in Loop Types | Associated Types | Purpose |
|------|------|------|---------------------------------|------------------|---------|
| `GhostIteratorTrait for VecGhostIter` | `src/experiments/verus_vec_iterator.rs` | 12 | while ✅, loop ✅, for (range) ☐, for (general) ☐ | None | Implements custom ghost trait. |
| `View for VecExecIter` | `src/experiments/verus_vec_iterator.rs` | 64 | while ✅, loop ✅, for (range) ☐, for (general) ☐ | `V = VecGhostIter` | Connects exec to ghost state. |
| `ExecIteratorTrait for VecCollection` | `src/experiments/verus_vec_iterator.rs` | 80 | while ✅, loop ✅, for (range) ☐, for (general) ☐ | `ExecIter = VecExecIter` | Implements custom exec trait. |
| `ForLoopGhostIteratorNew for VecExecIter` | `src/experiments/verus_vec_iterator.rs` | 112 | while ☐, loop ☐, for (range) ☐, for (general) ⚠️ | `GhostIter = VecGhostIter` | Attempted for-loop support (buggy). |
| `ForLoopGhostIterator for VecGhostIter` | `src/experiments/verus_vec_iterator.rs` | 120 | while ☐, loop ☐, for (range) ☐, for (general) ⚠️ | `ExecIter = VecExecIter`, `Item = usize`, `Decrease = int` | Attempted for-loop support (buggy). |
| `Iterator for VecExecIter` | `src/experiments/verus_vec_iterator.rs` | 168 | while ✅, loop ✅, for (range) ☐, for (general) ☐ | `Item = usize` | Standard Rust Iterator. |

---

