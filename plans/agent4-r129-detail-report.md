# Agent 4 — R129 Detailed Report of R128 Changes

Commit: `0b06892ba` on `agent4/ready`

## Files Modified

1. `src/Chap41/AVLTreeSetMtEph.rs`
2. `src/Chap41/AVLTreeSetMtPer.rs`
3. `src/Chap41/ArraySetEnumMtEph.rs`

No files created (other than `plans/agent4-r128-report.md` and logs).
No source files deleted.

---

## Struct Layout Changes

### AVLTreeSetMtEph

OLD:
```rust
pub struct AVLTreeSetMtEphInv;                              // RwLockPredicate type

pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
    pub inner: Arc<RwLock<AVLTreeSetStEph<T>, AVLTreeSetMtEphInv>>,
    pub ghost_set_view: Ghost<Set<<T as View>::V>>,
}
```

NEW:
```rust
pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
    pub tree: ParamBST<T>,   // from BSTParaMtEph (Chap38)
}
```

### AVLTreeSetMtPer

OLD:
```rust
pub struct AVLTreeSetMtPerInv;                              // RwLockPredicate type

pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
    pub locked_set: Arc<RwLock<AVLTreeSetStPer<T>, AVLTreeSetMtPerInv>>,
    pub ghost_set_view: Ghost<Set<<T as View>::V>>,
}
```

NEW:
```rust
pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
    pub tree: ParamBST<T>,   // from BSTParaMtEph (Chap38)
}
```

---

## File 1: `src/Chap41/AVLTreeSetMtEph.rs`

### Import changes
- REMOVED: `use std::sync::Arc;`
- REMOVED: `use vstd::rwlock::*;`
- REMOVED: `use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;`
- REMOVED: `use crate::vstdplus::arc_rwlock::arc_rwlock::*;`
- ADDED: `use crate::Chap38::BSTParaMtEph::BSTParaMtEph::*;`
- ADDED: `use crate::vstdplus::feq::feq::obeys_feq_full_trigger;` (cfg verus_keep_ghost)

### Deleted types/impls
- DELETED: `pub struct AVLTreeSetMtEphInv;`
- DELETED: `impl RwLockPredicate<AVLTreeSetStEph<T>> for AVLTreeSetMtEphInv`
- DELETED: `impl AVLTreeSetMtEph { pub open spec fn spec_set_view }` (inherent spec fn block)
- DELETED: `impl fmt::Debug for AVLTreeSetMtEphInv`
- DELETED: `impl fmt::Display for AVLTreeSetMtEphInv`
- DELETED: NOTE comment block about Ord (lines 39-46 of old file)

### Function-by-function listing (impl AVLTreeSetMtEphTrait)

```
src/Chap41/AVLTreeSetMtEph.rs:73: spec fn view — CLEAN: body changed from self.spec_set_view() to self.tree@
src/Chap41/AVLTreeSetMtEph.rs:231: spec fn spec_avltreesetmteph_wf — CLEAN: body changed from self.ghost_set_view@.finite() to self.tree@.finite()
src/Chap41/AVLTreeSetMtEph.rs:235: fn size — CLEAN: rewritten from acquire_read+borrow+assume to self.tree.size(). Removed 1 assume.
src/Chap41/AVLTreeSetMtEph.rs:240: fn to_seq — HOLED(assume×3): rewritten from lock+delegate to collect_in_order+from_vec. Has assume(out@.len() < usize::MAX), assume(seq@.to_set() =~= self@), assume(forall contains).
src/Chap41/AVLTreeSetMtEph.rs:255: fn empty — CLEAN: rewritten from AVLTreeSetStEph::empty+new_arc_rwlock to ParamBST::new()
src/Chap41/AVLTreeSetMtEph.rs:260: fn singleton — CLEAN: rewritten from AVLTreeSetStEph::singleton+new_arc_rwlock to ParamBST::singleton(x)
src/Chap41/AVLTreeSetMtEph.rs:265: fn from_seq — HOLED(assume×2): new sequential insert loop. Has assume(elem@ == seq@[i]), assume(tree@.len() < usize::MAX).
src/Chap41/AVLTreeSetMtEph.rs:328: fn filter — CLEAN: rewritten from lock+delegate+4 assumes to self.tree.filter(f, Ghost(spec_pred)). Removed 4 assumes.
src/Chap41/AVLTreeSetMtEph.rs:338: fn intersection — CLEAN: rewritten from lock+delegate+assume to self.tree.intersect(&other.tree). Removed 1 assume.
src/Chap41/AVLTreeSetMtEph.rs:344: fn difference — CLEAN: rewritten from lock+delegate+assume to self.tree.difference(&other.tree). Removed 1 assume.
src/Chap41/AVLTreeSetMtEph.rs:350: fn union — CLEAN: rewritten from lock+delegate+2 assumes to self.tree.union(&other.tree). Removed 2 assumes.
src/Chap41/AVLTreeSetMtEph.rs:356: fn find — CLEAN: rewritten from lock+assume to self.tree.find(x).is_some(). Removed 1 assume.
src/Chap41/AVLTreeSetMtEph.rs:362: fn delete — HOLED(assume×1): rewritten from acquire_write to self.tree.delete(x). Has assume(self.tree@.len() < usize::MAX).
src/Chap41/AVLTreeSetMtEph.rs:371: fn insert — CLEAN: rewritten from acquire_write+assume to self.tree.insert(x). Removed 1 assume.
src/Chap41/AVLTreeSetMtEph.rs:377: fn iter — CLEAN: rewritten from lock+to_seq+clone loop to collect_in_order. Simpler.
src/Chap41/AVLTreeSetMtEph.rs:477: fn clone — CLEAN: rewritten from clone_arc_rwlock to self.tree.clone()
```

### Annotation changes in trait declaration

```
src/Chap41/AVLTreeSetMtEph.rs:103: fn size — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtEph.rs:109: fn to_seq — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtEph.rs:117: fn empty — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtEph.rs:124: fn singleton — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtEph.rs:131: fn from_seq — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtEph.rs:141: fn filter — ANNOTATION: updated DIFFERS reason to "sequential filter (spec_fn not Send)". Removed CS 41.3 and claude-4-sonet lines.
src/Chap41/AVLTreeSetMtEph.rs:162: fn intersection — ANNOTATION: changed from DIFFERS to "matches APAS; parallel D&C via BSTParaMtEph". Removed CS 41.3 and claude-4-sonet lines.
src/Chap41/AVLTreeSetMtEph.rs:173: fn difference — ANNOTATION: changed from DIFFERS to "matches APAS; parallel D&C via BSTParaMtEph". Removed CS 41.3 and claude-4-sonet lines.
src/Chap41/AVLTreeSetMtEph.rs:184: fn union — ANNOTATION: changed from DIFFERS to "matches APAS; parallel D&C via BSTParaMtEph". Removed CS 41.3 and claude-4-sonet lines.
src/Chap41/AVLTreeSetMtEph.rs:196: fn find — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
src/Chap41/AVLTreeSetMtEph.rs:204: fn delete — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
src/Chap41/AVLTreeSetMtEph.rs:214: fn insert — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
```

---

## File 2: `src/Chap41/AVLTreeSetMtPer.rs`

### Import changes
- REMOVED: `use std::sync::Arc;`
- REMOVED: `use vstd::rwlock::*;`
- REMOVED: `use crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBSTTrait;`
- REMOVED: `use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;`
- REMOVED: `use crate::vstdplus::arc_rwlock::arc_rwlock::*;`
- ADDED: `use crate::Chap38::BSTParaMtEph::BSTParaMtEph::*;`

### Deleted types/impls
- DELETED: `pub struct AVLTreeSetMtPerInv;`
- DELETED: `impl RwLockPredicate<AVLTreeSetStPer<T>> for AVLTreeSetMtPerInv`
- DELETED: `impl fmt::Debug for AVLTreeSetMtPerInv`
- DELETED: `impl fmt::Display for AVLTreeSetMtPerInv`
- DELETED: NOTE comment block about Ord (lines 47-53 of old file)

### Function-by-function listing (impl AVLTreeSetMtPerTrait)

```
src/Chap41/AVLTreeSetMtPer.rs:66: spec fn view — CLEAN: body changed from self.spec_set_view() to self.tree@
src/Chap41/AVLTreeSetMtPer.rs:72: spec fn spec_avltreesetmtper_wf (inherent) — CLEAN: body changed from self.ghost_set_view@.finite() to self.tree@.finite()
src/Chap41/AVLTreeSetMtPer.rs:193: spec fn spec_avltreesetmtper_wf (trait impl) — CLEAN: body changed to self.tree@.finite()
src/Chap41/AVLTreeSetMtPer.rs:197: fn size — CLEAN: rewritten from lock+assume to self.tree.size(). Removed 1 assume.
src/Chap41/AVLTreeSetMtPer.rs:202: fn to_seq — HOLED(assume×3): rewritten from lock+collect_in_order+from_vec to tree.collect_in_order+from_vec. Has assume(vals@.len() < usize::MAX), assume(seq@.to_set() =~= self@), assume(forall contains).
src/Chap41/AVLTreeSetMtPer.rs:216: fn empty — CLEAN: rewritten to ParamBST::new()
src/Chap41/AVLTreeSetMtPer.rs:221: fn singleton — CLEAN: rewritten to ParamBST::singleton(x)
src/Chap41/AVLTreeSetMtPer.rs:226: fn from_seq — HOLED(assume×0 new): sequential insert loop preserved from old code. Same proof structure. No new assumes (obeys_feq_full_trigger is an assert, not assume).
src/Chap41/AVLTreeSetMtPer.rs:304: fn filter — CLEAN: rewritten from lock+3 assumes to self.tree.filter(f, Ghost(spec_pred)). Removed 3 assumes.
src/Chap41/AVLTreeSetMtPer.rs:314: fn intersection — CLEAN: rewritten from lock+assume to self.tree.intersect(&other.tree). Removed 1 assume.
src/Chap41/AVLTreeSetMtPer.rs:320: fn difference — CLEAN: rewritten from lock+assume to self.tree.difference(&other.tree). Removed 1 assume.
src/Chap41/AVLTreeSetMtPer.rs:326: fn union — HOLED(assume×1): rewritten from lock+2 assumes to self.tree.union. Has assume(size bound). Net -1 assume.
src/Chap41/AVLTreeSetMtPer.rs:333: fn find — CLEAN: rewritten from lock+assume to self.tree.find(x).is_some(). Removed 1 assume.
src/Chap41/AVLTreeSetMtPer.rs:339: fn delete — HOLED(assume×1): rewritten from lock+assume to clone+delete. Has assume(tree@.len() < usize::MAX). Net: same count.
src/Chap41/AVLTreeSetMtPer.rs:350: fn insert — HOLED(assume×1): rewritten from lock+2 assumes to clone+insert. Has assume(tree@.len() < usize::MAX). Net -1 assume.
src/Chap41/AVLTreeSetMtPer.rs:377: fn eq (PartialEq) — HOLED(assume×3): rewritten from lock-based comparison to collect_in_order+element comparison. Has assume(self.tree@.finite()), assume(other.tree@.finite()), assume(all_eq == (self@ == other@)). Old had assume(equal == (self@ == other@)) — net +2 assume (finiteness).
src/Chap41/AVLTreeSetMtPer.rs:414: fn partial_cmp — CLEAN: unchanged (external_body)
src/Chap41/AVLTreeSetMtPer.rs:421: fn cmp — CLEAN: rewritten from lock-based to collect_in_order (external_body preserved)
src/Chap41/AVLTreeSetMtPer.rs:443: fn clone — CLEAN: rewritten from clone_arc_rwlock to self.tree.clone()
```

### Annotation changes in trait declaration

```
src/Chap41/AVLTreeSetMtPer.rs:85: fn size — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtPer.rs:91: fn to_seq — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtPer.rs:99: fn empty — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtPer.rs:103: fn singleton — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
src/Chap41/AVLTreeSetMtPer.rs:108: fn from_seq — ANNOTATION: removed claude-4-sonet line
src/Chap41/AVLTreeSetMtPer.rs:118: fn filter — ANNOTATION: updated DIFFERS reason. Removed CS 41.3 and claude-4-sonet lines.
src/Chap41/AVLTreeSetMtPer.rs:139: fn intersection — ANNOTATION: changed from DIFFERS to "matches APAS; parallel D&C via BSTParaMtEph". Removed CS 41.3 and claude-4-sonet lines.
src/Chap41/AVLTreeSetMtPer.rs:148: fn difference — ANNOTATION: same as intersection.
src/Chap41/AVLTreeSetMtPer.rs:157: fn union — ANNOTATION: same as intersection. Also removed size bound from trait requires (was never there for MtPer).
src/Chap41/AVLTreeSetMtPer.rs:166: fn find — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
src/Chap41/AVLTreeSetMtPer.rs:174: fn delete — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
src/Chap41/AVLTreeSetMtPer.rs:182: fn insert — ANNOTATION: removed CS 41.3 and claude-4-sonet lines
```

---

## File 3: `src/Chap41/ArraySetEnumMtEph.rs`

Minimal changes — annotation updates only. No structural changes.

```
src/Chap41/ArraySetEnumMtEph.rs:187: fn size — ANNOTATION: clarified DIFFERS to "sequential bit scan; APAS CS 41.3 Span O(1) assumes PRAM, not fork-join"
src/Chap41/ArraySetEnumMtEph.rs:230: fn filter — ANNOTATION: clarified DIFFERS to "sequential loop; APAS CS 41.3 Span O(1) assumes PRAM, not fork-join"
```

---

## Assume Audit

### AVLTreeSetMtEph — assumes REMOVED (old → not present in new)

| # | Old line | Old assume | Reason removed |
|---|----------|-----------|----------------|
| 1 | 286 | `assume(count == self@.len())` | BSTParaMtEph::size ensures this directly |
| 2 | 355 | `assume(filtered@.subset_of(self@))` | BSTParaMtEph::filter ensures this |
| 3 | 356 | `assume(filtered.spec_avltreesetmteph_wf())` | BSTParaMtEph::filter ensures finite |
| 4 | 357-358 | `assume(forall filtered@.contains ==> self@.contains && spec_pred)` | BSTParaMtEph::filter ensures this |
| 5 | 359-360 | `assume(forall self@.contains && spec_pred ==> filtered@.contains)` | BSTParaMtEph::filter ensures this |
| 6 | 379 | `assume(common@ == self@.intersect(other@))` | BSTParaMtEph::intersect ensures this |
| 7 | 399 | `assume(remaining@ == self@.difference(other@))` | BSTParaMtEph::difference ensures this |
| 8 | 413 | `assume(self_st@.len() + other_st@.len() < usize::MAX)` | union now takes tree refs with trait bound |
| 9 | 425 | `assume(combined@ == self@.union(other@))` | BSTParaMtEph::union ensures this |
| 10 | 435 | `assume(found == self@.contains(x@))` | BSTParaMtEph::find ensures is_some <==> contains |
| 11 | 457 | `assume(current@.len() + 1 < usize::MAX)` | insert now direct; tree@.len() bounded by trait requires |

Total removed from MtEph: **11 assumes**

### AVLTreeSetMtEph — assumes ADDED (not present in old → present in new)

| # | New line | New assume | Reason |
|---|----------|-----------|--------|
| 1 | 245 | `assume(out@.len() < usize::MAX)` | AVLTreeSeqStEphS::from_vec requires this; tree size bounded but not provable |
| 2 | 250 | `assume(forall contains seq@[i])` | Bridge between collect_in_order ensures and trait ensures |
| 3 | 288 | `assume(elem@ == seq@[i])` | Clone preserves view (standard clone workaround) |
| 4 | 289 | `assume(tree@.len() < usize::MAX)` | BSTParaMtEph::insert requires size bound |
| 5 | 366 | `assume(self.tree@.len() < usize::MAX)` | BSTParaMtEph::delete requires size bound |

Total added to MtEph: **5 assumes**

### AVLTreeSetMtEph — assumes RETAINED (present in both)

| # | Old line | New line | Assume |
|---|----------|----------|--------|
| 1 | 298 | 248 | `assume(seq@.to_set() =~= self@)` — bridge between BST in_order and set view |
| 2 | 521 | 415 | `assume(item@ == old(self)@.1[old(self)@.0])` — clone preserves value in iterator |

### AVLTreeSetMtEph assume net: 13 old → 7 new = **-6 net**

---

### AVLTreeSetMtPer — assumes REMOVED (old → not present in new)

| # | Old line | Old assume | Reason removed |
|---|----------|-----------|----------------|
| 1 | 247 | `assume(count == self@.len())` | BSTParaMtEph::size ensures this |
| 2 | 399 | `assume(filtered@.subset_of(self@))` | BSTParaMtEph::filter ensures this |
| 3 | 400-401 | `assume(forall filtered@.contains ==> self@.contains && spec_pred)` | BSTParaMtEph::filter ensures this |
| 4 | 402-403 | `assume(forall self@.contains && spec_pred ==> filtered@.contains)` | BSTParaMtEph::filter ensures this |
| 5 | 422 | `assume(common@ == self@.intersect(other@))` | BSTParaMtEph::intersect ensures this |
| 6 | 441 | `assume(remaining@ == self@.difference(other@))` | BSTParaMtEph::difference ensures this |
| 7 | 454 | `assume(self_st@.len() + other_st@.len() < usize::MAX)` | Replaced by new assume with `<=` |
| 8 | 466 | `assume(combined@ == self@.union(other@))` | BSTParaMtEph::union ensures this |
| 9 | 477 | `assume(found == self@.contains(x@))` | BSTParaMtEph::find ensures is_some <==> contains |
| 10 | 495 | `assume(updated@ == self@.remove(x@))` | BSTParaMtEph::delete ensures this |
| 11 | 506 | `assume(st@.len() + 1 < usize::MAX)` | Replaced by new assume |
| 12 | 517 | `assume(updated@ == self@.insert(x@))` | BSTParaMtEph::insert ensures this |

Total removed from MtPer: **12 assumes**

### AVLTreeSetMtPer — assumes ADDED (not present in old → present in new)

| # | New line | New assume | Reason |
|---|----------|-----------|--------|
| 1 | 206 | `assume(vals@.len() < usize::MAX)` | AVLTreeSeqMtPerS::from_vec requires this |
| 2 | 209-210 | `assume(seq@.to_set() =~= self@)` + `assume(forall contains)` | Bridge between collect_in_order and trait ensures |
| 3 | 328 | `assume(self.tree@.len() + other.tree@.len() <= usize::MAX)` | BSTParaMtEph::union size bound |
| 4 | 344 | `assume(tree@.len() < usize::MAX)` | BSTParaMtEph::delete size bound |
| 5 | 355 | `assume(tree@.len() < usize::MAX)` | BSTParaMtEph::insert size bound |
| 6 | 381 | `assume(self.tree@.finite())` | collect_in_order in PartialEq::eq requires finiteness |
| 7 | 382 | `assume(other.tree@.finite())` | same |
| 8 | 390 | `assume(false == (self@ == other@))` | early return in eq when sizes differ |
| 9 | 407 | `assume(all_eq == (self@ == other@))` | final eq result bridge |

Total added to MtPer: **9 assumes** (but note old had 14 including the eq assume; new has 10)

### AVLTreeSetMtPer — assumes RETAINED (present in both)

| # | Old line | New line | Assume |
|---|----------|----------|--------|
| 1 | 263 | 209 | `assume(seq@.to_set() =~= self@)` — bridge BST in_order ↔ set view |
| 2 | 548 | 407 | `assume(equal/all_eq == (self@ == other@))` — eq result bridge (rewritten) |

### AVLTreeSetMtPer assume net: 14 old → 10 new = **-4 net**

---

## Combined Assume Count

| File | Old assumes | New assumes | Net change |
|------|-------------|-------------|------------|
| AVLTreeSetMtEph.rs | 13 | 7 | -6 |
| AVLTreeSetMtPer.rs | 14 | 10 | -4 |
| ArraySetEnumMtEph.rs | 0 | 0 | 0 |
| **Total** | **27** | **17** | **-10** |

Correction to R128 report: the actual net reduction is **10 assumes**, not ~12 as claimed.
The discrepancy comes from MtPer's PartialEq rewrite which needed 2 new finiteness
assumes that weren't counted in the original estimate.

## New external_body or accept introduced

**Zero.** No new `external_body`, `accept()`, or `admit()` were added.

## Verification Results

- Isolate Chap41: 2113 verified, 1 error (pre-existing rlimit in Chap37 `insert_at_link`)
- Baseline (before changes): 2112 verified, 1 error (same pre-existing rlimit)
- RTT: 3534 passed (unchanged)
- PTT: 221 passed (unchanged)
