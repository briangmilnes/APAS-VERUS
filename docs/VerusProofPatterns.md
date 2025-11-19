# Verus Proof Patterns: Complete Reference

## Three Proof Styles

### Style 1: `requires`/`ensures` Only (Explicit Call)
- Non-broadcast lemmas
- Called explicitly in proofs
- No `forall`/`==>` in ensures
- Example: `lemma_example(s, 5);`

### Style 2: Broadcast with `forall`/`==>` in Ensures
- Broadcast axioms/lemmas with quantifiers
- Auto-applied by SMT
- Contains `forall` or `exists` in ensures
- Example: `axiom_seq_ext_equal` uses `forall|i| ... ==> ...`

### Style 3: Broadcast with Simple Ensures (No Quantifiers)
- Broadcast axioms/lemmas
- Auto-applied by SMT
- Simple equations or properties
- Example: `axiom_seq_push_len` just says `s.push(a).len() == s.len() + 1`

---

## Master Table: vstd Seq Axioms (`seq.rs`)

| Axiom/Lemma | S1 | S2 | S3 | Req | Trigger | Notes |
|-------------|----|----|----|----|---------|-------|
| `axiom_seq_index_decreases` | | | ☑ | ☑ | `s[i]` | Decreases axiom |
| `axiom_seq_len_decreases` | | ☑ | | ☑ | `trigger(s2[i2])` | Has forall in requires |
| `axiom_seq_subrange_decreases` | | | ☑ | ☑ | `s.subrange(i,j)` | Has proof body |
| `axiom_seq_empty` | | | ☑ | | `Seq::<A>::empty().len()` | Always 0 |
| `axiom_seq_new_len` | | | ☑ | | `Seq::new(len, f).len()` | Constructor length |
| `axiom_seq_new_index` | | | ☑ | ☑ | `Seq::new(len, f)[i]` | Constructor indexing |
| `axiom_seq_push_len` | | | ☑ | | `s.push(a).len()` | Classic example |
| `axiom_seq_push_index_same` | | | ☑ | ☑ | `s.push(a)[i]` | When `i == s.len()` |
| `axiom_seq_push_index_different` | | | ☑ | ☑ | `s.push(a)[i]` | When `i < s.len()` |
| `axiom_seq_update_len` | | | ☑ | ☑ | `s.update(i, a).len()` | Length unchanged |
| `axiom_seq_update_same` | | | ☑ | ☑ | `s.update(i, a)[i]` | Same index |
| `axiom_seq_update_different` | | | ☑ | ☑ | `s.update(i2, a)[i1]` | Different index |
| `axiom_seq_ext_equal` | | ☑ | | | `s1 =~= s2` | Has forall in <==> |
| `axiom_seq_ext_equal_deep` | | ☑ | | | `s1 =~~= s2` | Deep equality |
| `axiom_seq_subrange_len` | | | ☑ | ☑ | `s.subrange(j, k).len()` | Subrange length |
| `axiom_seq_subrange_index` | | | ☑ | ☑ | `s.subrange(j, k)[i]` | Subrange indexing |
| `axiom_seq_add_len` | | | ☑ | | `s1.add(s2).len()` | Concatenation length |
| `axiom_seq_add_index1` | | | ☑ | ☑ | `s1.add(s2)[i]` | First part |
| `axiom_seq_add_index2` | | | ☑ | ☑ | `s1.add(s2)[i]` | Second part |

**Summary for `seq.rs`:**
- Style 1: 0
- Style 2: 2 (with forall/==>)
- Style 3: 16 (simple properties)
- **Total: 18**

---

## Master Table: vstd Set Axioms (`set.rs`)

| Axiom/Lemma | S1 | S2 | S3 | Req | Trigger | Notes |
|-------------|----|----|----|----|---------|-------|
| `axiom_set_empty` | | | ☑ | | `Set::empty().contains(a)` | Always false |
| `axiom_set_new` | | | ☑ | | `Set::new(f).contains(a)` | Constructor |
| `axiom_set_insert_same` | | | ☑ | | `s.insert(a).contains(a)` | Always true |
| `axiom_set_insert_different` | | | ☑ | ☑ | `s.insert(a2).contains(a1)` | When `a1 != a2` |
| `axiom_set_remove_same` | | | ☑ | | `s.remove(a).contains(a)` | Always false |
| `axiom_set_remove_insert` | | ☑ | | ☑ | `s.remove(a)` | Has forall in proof |
| `axiom_set_remove_different` | | | ☑ | ☑ | `s.remove(a2).contains(a1)` | When `a1 != a2` |
| `axiom_set_union` | | | ☑ | | `s1.union(s2).contains(a)` | OR semantics |
| `axiom_set_intersect` | | | ☑ | | `s1.intersect(s2).contains(a)` | AND semantics |
| `axiom_set_difference` | | | ☑ | | `s1.difference(s2).contains(a)` | Difference semantics |
| `axiom_set_complement` | | | ☑ | | `s.complement().contains(a)` | NOT semantics |
| `axiom_set_ext_equal` | | ☑ | | | `s1 =~= s2` | Has forall in <==> |
| `axiom_set_ext_equal_deep` | | | ☑ | | `s1 =~~= s2` | Deep equality |
| `axiom_mk_map_domain` | | | ☑ | | `s.mk_map(f).dom()` | Map domain |
| `axiom_mk_map_index` | | | ☑ | ☑ | `s.mk_map(f)[key]` | Map indexing |
| `axiom_set_empty_finite` | | | ☑ | | `Set::<A>::empty().finite()` | Always finite |
| `axiom_set_insert_finite` | | ☑ | | ☑ | `s.insert(a).finite()` | Has forall in proof |
| `axiom_set_remove_finite` | | ☑ | | ☑ | `s.remove(a).finite()` | Has forall in proof |
| `axiom_set_union_finite` | | ☑ | | ☑ | `s1.union(s2).finite()` | Has choose/forall |
| `axiom_set_intersect_finite` | | ☑ | | ☑ | `s1.intersect(s2).finite()` | Has forall |
| `axiom_set_difference_finite` | | ☑ | | ☑ | `s1.difference(s2).finite()` | Has forall |
| `axiom_set_choose_infinite` | | | ☑ | ☑ | `s.contains(s.choose())` | For infinite sets |
| `axiom_set_empty_len` | | | ☑ | | `Set::<A>::empty().len()` | Length 0 |
| `axiom_set_insert_len` | | | ☑ | ☑ | `s.insert(a).len()` | Conditional +1 |
| `axiom_set_remove_len` | | | ☑ | ☑ | `s.remove(a).len()` | Conditional -1 |
| `axiom_set_contains_len` | | | ☑ | ☑ | `s.contains(a)`, `s.len()` | Non-zero length |
| `axiom_set_choose_len` | | | ☑ | ☑ | `s.len()`, `s.contains(s.choose())` | Choose from non-empty |

**Summary for `set.rs`:**
- Style 1: 0
- Style 2: 8 (with forall/==> in body)
- Style 3: 19 (simple properties)
- **Total: 27**

---

## Master Table: vstd Seq Lemmas (`seq_lib.rs` - Broadcast Only)

| Lemma | S1 | S2 | S3 | Req | Trigger | Notes |
|-------|----|----|----|----|---------|-------|
| `lemma_seq_union_to_multiset_commutative` | | | ☑ | | `(a + b).to_multiset()` | Multiset conversion |
| `lemma_seq_contains` | | ☑ | | | `s.contains(x)` | Has exists in <==> |
| `lemma_seq_empty_contains_nothing` | | | ☑ | | `Seq::<A>::empty().contains(x)` | Always false |
| `lemma_seq_empty_equality` | | | ☑ | | `s.len()` | Zero length implies empty |
| `lemma_seq_concat_contains_all_elements` | | | ☑ | | `(x + y).contains(elt)` | OR of contains |
| `lemma_seq_contains_after_push` | | | ☑ | | `s.push(v).contains(x)` | Contains after push |
| `lemma_seq_subrange_elements` | | ☑ | | ☑ | `s.subrange(start, stop).contains(x)` | Has exists in <==> |
| `lemma_seq_take_len` | | | ☑ | | `s.take(n).len()` | Take length |
| `lemma_seq_take_contains` | | | ☑ | ☑ | `s.take(n).contains(x)` | Contains in prefix |
| `lemma_seq_take_index` | | | ☑ | ☑ | `s.take(n)[i]` | Indexing in take |
| `lemma_seq_skip_len` | | | ☑ | | `s.skip(n).len()` | Skip length |
| `lemma_seq_skip_contains` | | ☑ | | ☑ | `s.skip(n).contains(x)` | Has exists in <==> |
| `lemma_seq_skip_index` | | | ☑ | ☑ | `s.skip(n)[i]` | Indexing in skip |
| `lemma_seq_skip_index2` | | | ☑ | ☑ | `s[i]` | Alternative form |
| `lemma_seq_append_take_skip` | | | ☑ | ☑ | `s.take(n).add(s.skip(n))` | Reconstruction |
| `lemma_seq_take_update_commut1` | | | ☑ | ☑ | `s.update(i, v).take(n)` | Update then take |
| `lemma_seq_take_update_commut2` | | | ☑ | ☑ | `s.take(n).update(i, v)` | Take then update |
| `lemma_seq_skip_update_commut1` | | | ☑ | ☑ | `s.update(i, v).skip(n)` | Update then skip |
| `lemma_seq_skip_update_commut2` | | | ☑ | ☑ | `s.skip(n).update(i, v)` | Skip then update |
| `lemma_seq_skip_build_commut` | | | ☑ | ☑ | Multiple triggers | Build/skip commute |
| `lemma_seq_skip_nothing` | | | ☑ | | `s.skip(0)` | Skip 0 is identity |
| `lemma_seq_take_nothing` | | | ☑ | | `s.take(0)` | Take 0 is empty |

**Summary for `seq_lib.rs` (broadcast only):**
- Style 1: 0
- Style 2: 3 (with exists/forall)
- Style 3: 19 (simple properties)
- **Total: 22**

**Note:** `seq_lib.rs` also has ~50+ Style 1 lemmas (non-broadcast) that are called explicitly.

---

## Master Table: APAS vstdplus Lemmas/Axioms

| Lemma/Axiom | S1 | S2 | S3 | Req | Trigger | Notes |
|-------------|----|----|----|----|---------|-------|
| **seq_set.rs** |
| `lemma_seq_index_in_to_set` | ☑ | | | ☑ | N/A (call explicitly) | Seq index → to_set |
| `lemma_push_not_contains_to_set_subset` | ☑ | | | ☑ | N/A | Push preserves subset |
| `lemma_push_not_contains_to_set_superset` | ☑ | | | ☑ | N/A | Push preserves superset |
| `lemma_push_not_contains_to_set` | ☑ | | | ☑ | N/A | Push equality (calls above 2) |
| `lemma_take_full` | ☑ | | | | N/A | take(len) == seq |
| `lemma_take_full_to_set` | ☑ | | | | N/A | take(len).to_set() == seq.to_set() |
| `lemma_seq_equal_to_set_equal` | ☑ | | | ☑ | N/A | Seq equal → set equal |
| `lemma_take_extends_set_subset` | ☑ | | | ☑ | N/A | take(n) + elem ⊆ take(n+1) |
| `lemma_take_extends_set_superset` | ☑ | | | ☑ | N/A | take(n+1) ⊆ take(n) + elem |
| `lemma_take_extends_set` | ☑ | | | ☑ | N/A | Equality (calls above 2) |
| **set_axioms.rs** |
| `lemma_singleton_len` | ☑ | | | | N/A | singleton(x).len() == 1 |
| `axiom_set_len_zero_iff_empty` | | ☑ | | | `s.len()` | len == 0 <==> empty |
| `lemma_set_split_element` | | ☑ | | ☑ | `s.remove(x)` | s == {x} + s.remove(x) |
| `lemma_set_move_element_preserves_union` | ☑ | | | ☑ | N/A | Move elem preserves union |
| `lemma_union_insert_commute` | | ☑ | | | `a.union(b.insert(x))` | Union distributes insert |
| **clone_view.rs** |
| `lemma_clone_preserves_view` | | ☑ | | ☑ | `x@`, `y@` | cloned(x,y) ==> x@ == y@ |

**Summary for vstdplus:**
- Style 1: 11 (explicit call lemmas)
- Style 2: 4 (broadcast with forall/requires)
- Style 3: 0
- **Total: 15**

---

## Grand Summary

| File | Style 1 | Style 2 | Style 3 | Total |
|------|---------|---------|---------|-------|
| `seq.rs` | 0 | 2 | 16 | 18 |
| `set.rs` | 0 | 8 | 19 | 27 |
| `seq_lib.rs` (broadcast) | 0 | 3 | 19 | 22 |
| **vstd subtotal** | **0** | **13** | **54** | **67** |
| `vstdplus` (all) | 11 | 4 | 0 | 15 |
| **Grand Total** | **11** | **17** | **54** | **82** |

**Additional:** `seq_lib.rs` has ~50+ Style 1 non-broadcast lemmas.

---

## Style Selection Guide

### Choose Style 1 When:
- ☐ Proof is complex with multiple assertions
- ☐ Need explicit control over application
- ☐ Would be expensive to auto-apply
- ☐ Combining multiple concepts

### Choose Style 2 When:
- ☐ Property involves quantifiers (forall/exists)
- ☐ Bridging different representations (e.g., extensional equality)
- ☐ Simple enough to auto-apply
- ☐ Trigger pattern is specific

### Choose Style 3 When:
- ☐ Simple equation or property
- ☐ Frequently needed fact
- ☐ No quantifiers needed
- ☐ Trigger is very specific operation

---

## Trigger Design Checklist

### Good Triggers ✓
- ☑ Specific operations: `s.push(a).len()`
- ☑ Left side of equations in ensures
- ☑ Antecedent of implications (LHS of `==>`)
- ☑ Unique combinations: `s.insert(a).contains(b)`

### Bad Triggers ✗
- ☐ Too general: just `s.contains(x)`
- ☐ Multiple independent patterns
- ☐ Both sides of `==>`
- ☐ Deeply nested expressions

---

## Your Code Example

### ❌ Original (30M rlimit):
```rust
pub broadcast proof fn lemma_take_extends_set<T>(seq: Seq<T>, n: int)
    // Complex proof body with assert forall
```
**Problem:** Style 2/3 (broadcast) with complex body

### ✓ Fixed (922K rlimit):
```rust
pub proof fn lemma_take_extends_set<T>(seq: Seq<T>, n: int)
    requires 0 <= n < seq.len(),
    ensures seq.take(n).to_set().insert(seq[n]) == seq.take(n+1).to_set(),
{
    lemma_take_extends_set_subset(seq, n);
    lemma_take_extends_set_superset(seq, n);
}
```
**Solution:** Style 1 (non-broadcast) with explicit calls
