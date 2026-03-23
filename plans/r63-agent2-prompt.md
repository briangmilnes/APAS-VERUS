# Agent 2 — Round 63

You are Agent 2 working in `~/projects/APAS-VERUS-agent2`.
You are a **senior proof engineer**. Your job is to prove, not to catalog
reasons you can't. Read the code, read the error, write the proof.

## Baseline

- Main: 4504 verified, 0 errors, 6 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Target: Close `chain_lookup` clone bridge assume — 1 hole (Chap47, StructChainedHashTable.rs:201)

### Current code (lines 179-216)

```rust
fn chain_lookup<Key: Eq + View + Clone, Value: Clone>(
    chain: &Option<Box<Node<Key, Value>>>,
    key: &Key,
) -> (found: Option<Value>)
    ensures
        spec_chain_to_map(*chain).dom().contains(*key)
            ==> found == Some(spec_chain_to_map(*chain)[*key]),
        !spec_chain_to_map(*chain).dom().contains(*key)
            ==> found is None,
    decreases chain,
{
    match chain {
        ...
        Some(node) => {
            ...
            if eq {
                let v = node.value.clone();
                proof {
                    assume(v == node.value); // Clone bridge: behind EntryTrait, no feq path.
                }
                Some(v)
            } else { ... }
        }
    }
}
```

The assume at line 201 is `assume(v == node.value)` after `v = node.value.clone()`.
This is the standard clone bridge pattern.

### Before you write any code

**Read `src/standards/partial_eq_eq_clone_standard.rs`** in full. It shows the
`obeys_feq_clone` pattern for proving clone correctness.

### Fix path

1. **Add `obeys_feq_clone::<Value>()` to `chain_lookup`'s requires**:
   ```rust
   fn chain_lookup<Key: Eq + View + Clone, Value: Clone + View>(
       chain: &Option<Box<Node<Key, Value>>>,
       key: &Key,
   ) -> (found: Option<Value>)
       requires
           obeys_feq_clone::<Value>(),
       ensures ...
   ```

   Note: `Value` needs the `View` bound for `obeys_feq_clone` to apply.
   Check whether `Value` already has `View` bound on the struct/function.

2. **Replace the assume** with the standard clone proof:
   ```rust
   let v = node.value.clone();
   proof {
       assert(cloned(node.value, v));   // from Clone::clone ensures
       assert(v == node.value);          // from obeys_feq_clone
   }
   ```

   You may need to import `cloned` from the right place. Search for existing
   clone bridge patterns in the codebase: `grep -r "cloned(" src/Chap47/`.

3. **Propagate the new requires to callers.** `chain_lookup` is a free function.
   Find all call sites:
   ```bash
   grep -rn "chain_lookup" src/Chap47/
   ```
   Each caller must now prove `obeys_feq_clone::<Value>()`. If the caller already
   has `obeys_feq_clone::<Value>()` in its own requires (or the module's wf
   predicate includes it via `obeys_feq_full`), this is automatic. If not, add
   it to the caller's requires and propagate upward.

4. **Check the `Value: Clone` bound.** If `Value` doesn't have `View` currently,
   adding it may cascade. Look at the `Node` struct definition and
   `StructChainedHashTableStEphTrait` to see what bounds exist.

### Watch out for

- `chain_lookup` is marked `// veracity: no_requires`. After adding requires,
  remove that annotation.
- The function is recursive (`decreases chain`). The new requires must hold on
  the recursive call too — but `obeys_feq_clone::<Value>()` is a global property,
  so it propagates automatically.
- Other functions in this file may have similar clone bridges. If you see other
  `assume(v == ...)` patterns after `.clone()`, fix them the same way while
  you're here. But only fix what you can prove — don't add new assumes.

### Scope

Small — one free function, one assume, propagate requires to callers (likely 2-3
functions in the same file). The `obeys_feq_clone` pattern is well-established
in this codebase.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent2-round63-report.md`. Push to `agent2/ready`.
