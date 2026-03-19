# Agent 2 Round 40 Report

## Task

Prove OrderedTableStPer delegation wrappers — remove `external_body` from methods
in `src/Chap43/OrderedTableStPer.rs` and provide verified implementations with the
AVLTreeSetStPer backing store introduced in R39.

## Results

| # | Metric | Before | After |
|---|--------|--------|-------|
| 1 | Verified | 4301 | 4305 |
| 2 | Errors | 0 | 0 |
| 3 | Total holes | 186 | 180 |
| 4 | RTT | 2613 pass | 2613 pass |

## OrderedTableStPer.rs Hole Count

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStPer.rs | 26 | 20 | -6 |
| 2 | 43 | AugOrderedTableStPer.rs | 1 | 1 | 0 |

## Methods Proven (6 external_body removed)

| # | Chap | File | Method | Technique |
|---|------|------|--------|-----------|
| 1 | 43 | OrderedTableStPer.rs | size | Direct delegation to elements.length() |
| 2 | 43 | OrderedTableStPer.rs | singleton | AVLTreeSetStPer::singleton + bridge lemma |
| 3 | 43 | OrderedTableStPer.rs | find | Loop + nth + reveal(obeys_view_eq) + .eq() |
| 4 | 43 | OrderedTableStPer.rs | insert | Loop search + delete-then-insert chain |
| 5 | 43 | OrderedTableStPer.rs | delete | Loop search + delete + bridge lemmas |
| 6 | 43 | OrderedTableStPer.rs | from_sorted_entries | AVLTreeSetStPer::from_seq delegation |

## Bridge Lemmas Written (section 7)

| # | Chap | Lemma | Purpose |
|---|------|-------|---------|
| 1 | 43 | lemma_keys_no_dups_preserved_by_set_eq | Keys uniqueness preserved under set equality |
| 2 | 43 | lemma_entries_to_map_set_determines_map | Set-equal sequences produce equal maps |
| 3 | 43 | lemma_keys_no_dups_after_set_remove | Keys uniqueness after Set::remove |
| 4 | 43 | lemma_entries_to_map_after_remove_pair | Map = old_map.remove(k) after pair removal |
| 5 | 43 | lemma_keys_no_dups_after_set_insert | Keys uniqueness after Set::insert |
| 6 | 43 | lemma_entries_to_map_dom_after_insert | Map domain = old_dom.insert(k) after pair insert |

Lemmas 1-2 were written in the first R40 session. Lemmas 3-6 were written in this session.

## Key Proof Patterns

1. **Loop + nth + early return**: Iterate over `base_set.elements` using `nth(i)`, compare
   keys with `reveal(obeys_view_eq)` + `.eq()`, return early on match.
2. **clone_plus()**: Required instead of `clone()` for `Pair<K,V>` to generate `cloned`
   fact needed by feq broadcast axioms.
3. **ext_equal for maps**: Proving `m1 =~= m2` requires two `assert forall` blocks:
   (a) domain biconditional with `dom().contains()`, (b) value equality. Must use
   `#[trigger]` on `m2.dom().contains(k2)` or `m1.remove(k)[k2]`. The solver cannot
   assemble `=~=` from separately proved parts without this structure.
4. **Not-found case**: Direct `AVLTreeSetStPer { elements: copy_elements }` construction
   (bypasses `AVLTreeSetStPer::clone` which doesn't ensure wf preservation).

## Remaining 20 external_body Methods — Blocking Analysis

### Missing `requires self.spec_orderedtablestper_wf()` (6 methods)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | domain | No wf requires; can't reason about entries |
| 2 | 43 | collect | No wf requires; can't call to_seq |
| 3 | 43 | first_key | No wf requires; can't call nth |
| 4 | 43 | last_key | No wf requires; can't call nth |
| 5 | 43 | previous_key | No wf requires; can't iterate |
| 6 | 43 | next_key | No wf requires; can't iterate |

These are in `OrderedTableTrait` which doesn't require wf. Adding wf requires changing
the shared trait definition (affects StEph, MtEph, MtPer variants).

### Missing `other.spec_orderedtablestper_wf()` (1 method)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | difference | Needs other's wf to iterate other's elements |

### Missing `keys@.finite()` (2 methods)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | restrict | ArraySetStEph::find requires self@.finite() |
| 2 | 43 | subtract | ArraySetStEph::find requires self@.finite() |

### Closure-based methods (4 methods)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | tabulate | Needs f.requires propagation |
| 2 | 43 | map | Needs f.requires propagation |
| 3 | 43 | filter | Needs f.requires + spec_pred bridge |
| 4 | 43 | intersection | Needs f.requires + other wf |

### Need additional infrastructure (5 methods)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | union | Needs f.requires + other wf + complex merge |
| 2 | 43 | split_key | Needs obeys_feq_clone for value cloning |
| 3 | 43 | join_key | Needs both tables' wf + merge proof |
| 4 | 43 | get_key_range | Needs wf requires |
| 5 | 43 | split_rank_key | Needs wf requires |

### Iterator (1 method)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | Iterator::next | Needs wf in iterator invariant |

### Standalone (1 method)

| # | Chap | Method | Issue |
|---|------|--------|-------|
| 1 | 43 | rank_key | Needs wf requires |
| 2 | 43 | select_key | Needs wf requires |

## What Would Unblock the Most Holes

1. **Add `requires self.spec_wf()` to OrderedTableTrait methods** (domain, collect,
   first_key, last_key, previous_key, next_key, get_key_range, rank_key, select_key,
   split_rank_key): Would unblock ~10 methods. This is a trait-level change affecting
   all variants.
2. **Add `requires other.spec_wf()` to difference/intersection/union**: Would unblock
   ~3 more methods.
3. **Add `requires keys@.finite()` to restrict/subtract**: Would unblock 2 methods.

## Techniques Used

- Loop + nth iteration over AVLTreeSeqStPerS backing
- Bridge lemmas between Seq/Set/Map representations
- reveal(obeys_view_eq) + .eq() for key comparison
- clone_plus() for Pair cloning with feq properties
- ext_equal proof pattern for Map =~= assertions
- Direct struct construction to bypass clone wf gaps
