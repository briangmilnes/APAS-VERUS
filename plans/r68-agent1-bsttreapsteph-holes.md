# R68 Agent 1: BSTTreapStEph Hole Burndown

## Goal

Reduce the 28 holes in `src/Chap39/BSTTreapStEph.rs` by converting assumes to requires
and proving structural properties from wf.

## Current Holes (28)

| # | Category | Count | Description |
|---|----------|-------|-------------|
| 1 | `spec_param_wf_link` type_invariant analog | 17 | `assume(spec_param_wf_link(&self.root))` at method entry |
| 2 | size ↔ view len bridge | 3 | `make_node_treap_st`, `param_size` |
| 3 | finite from wf | 3 | `self@.finite()` follows from wf |
| 4 | eq/clone workaround | 3 | `clone_elem_st`, `clone_with_view` — standard pattern |
| 5 | `obeys_feq_clone` | 2 | In `param_insert`, `param_delete` |

## Fix 1: Add `spec_param_wf` to the Trait (eliminates 17 holes)

The 17 `assume(spec_param_wf_link(&self.root))` calls are unnecessary. The standard APAS
pattern is: **require wf at entry, ensure wf at exit.** Every other module does this
(BSTParaStEph, AVLTreeSetStEph, OrderedTableStEph, etc.).

### How

Add an abstract spec fn to the trait:

```rust
pub trait ParamBSTTreapStEphTrait<T: StT + Ord + IsLtTransitive>:
    Sized + View<V = Set<<T as View>::V>>
{
    spec fn spec_parambsttreapsteph_wf(&self) -> bool;

    fn expose(&self) -> (exposed: ExposedTreap<T>)
        requires
            self.spec_parambsttreapsteph_wf(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent_st::<T>(),
        ensures ...;
    // etc for all methods
}
```

In the impl, provide the body:

```rust
impl<T: StT + Ord + IsLtTransitive> ParamBSTTreapStEphTrait<T> for BSTTreapStEph<T> {
    open spec fn spec_parambsttreapsteph_wf(&self) -> bool {
        spec_param_wf_link(&self.root)
    }
}
```

**No cycle**: `spec_param_wf_link` is a free spec fn that walks `Link<T>` recursively.
It does not call any trait methods.

Then in each impl method body, **delete** the `assume(spec_param_wf_link(&self.root))`
line — it's now provided by the requires.

For methods that return `Self` or produce new trees (join_mid, singleton, param_insert,
param_delete, param_split, param_join_pair, param_union, param_intersect,
param_difference, param_filter), add `ensures result.spec_parambsttreapsteph_wf()` (or
`self.spec_parambsttreapsteph_wf()` for `&mut self` methods). You'll need to prove wf
is preserved — the internal functions already maintain the structural invariant, so the
proof should flow from their ensures.

For `expose`: the returned `ExposedTreap::Node(l, k, r)` should ensure
`l.spec_parambsttreapsteph_wf() && r.spec_parambsttreapsteph_wf()`. This follows from
`spec_param_wf_link` being recursive (wf of parent implies wf of children).

For `join_mid`: requires wf on the children (from the ExposedTreap), ensures wf on
the result. The internal `join_with_priority_st` should maintain wf.

For constructors (`param_new`, `singleton`): ensures wf. Trivially true for empty tree
and single-node tree.

### What stays as assumes

Methods that are the FIRST entry point — where no caller can provide wf because the
tree was just constructed externally — may still need assumes. But `param_new` and
`singleton` are constructors that PRODUCE wf trees, so their callers get wf from ensures.
The only remaining case would be if external code creates a `BSTTreapStEph { root: ... }`
directly, bypassing the constructors. If that's not supported, all 17 can be eliminated.

## Fix 2: Prove `finite` from wf (eliminates 3 holes)

`spec_param_wf_link` includes `lv.finite() && rv.finite()` as conjuncts. So
`self.spec_parambsttreapsteph_wf()` implies `self@.finite()`. Write a proof lemma:

```rust
proof fn lemma_wf_implies_finite<T: StT + Ord + IsLtTransitive>(tree: &BSTTreapStEph<T>)
    requires spec_param_wf_link(&tree.root),
    ensures tree@.finite(),
{
    // Structural induction on tree.root
}
```

Or simply add `self@.finite()` as a direct conjunct to wf (then it's extractable by
assert). Either way, the 3 `assume(self@.finite())` become assertions.

## Fix 3: Prove size ↔ view len from wf (eliminates up to 3 holes)

`spec_param_wf_link` includes `node.size as nat == lv.len() + rv.len() + 1`. This means
the exec `size` field tracks the spec set length. Write a bridging lemma:

```rust
proof fn lemma_wf_size_eq_view_len<T: StT + Ord + IsLtTransitive>(link: &Link<T>)
    requires spec_param_wf_link(link),
    ensures spec_set_view_link(link).len() == BSTTreapStEph::<T>::spec_size_link(link),
{
    // Structural induction
}
```

Then replace the assumes in `make_node_treap_st` and `param_size` with calls to this lemma.

## Fix 4: eq/clone and feq_clone (5 holes — likely stay)

- 3 eq/clone workaround assumes — standard pattern, leave as-is
- 2 `obeys_feq_clone` — standard pattern, leave as-is

These are the accepted Verus workaround patterns.

## Summary of Expected Outcome

| Category | Before | After | Delta |
|----------|--------|-------|-------|
| spec_param_wf assumes | 17 | 0 | -17 |
| finite from wf | 3 | 0 | -3 |
| size ↔ len bridge | 3 | 0 | -3 |
| eq/clone + feq_clone | 5 | 5 | 0 |
| **Total** | **28** | **5** | **-23** |

## Approach

1. **Read** the current BSTTreapStEph.rs (your R67 work)
2. **Add** `spec fn spec_parambsttreapsteph_wf(&self) -> bool` to the trait
3. **Add** `requires self.spec_parambsttreapsteph_wf()` to all non-constructor methods
4. **Add** `ensures result.spec_parambsttreapsteph_wf()` to constructors and methods
   that return new trees
5. **Delete** all 17 `assume(spec_param_wf_link(&self.root))` lines
6. **Write** `lemma_wf_implies_finite` and `lemma_wf_size_eq_view_len`
7. **Replace** finite and size-len assumes with lemma calls
8. **Validate** — target 0 errors, 5 remaining holes
9. **RTT** and **PTT** — all pass

## Constraints

- Do NOT modify any file other than `src/Chap39/BSTTreapStEph.rs`.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- eq/clone and feq_clone assumes stay (accepted patterns).
- Run validate, rtt, ptt sequentially.
- **Do NOT revert your R67 work.** Build on it.
