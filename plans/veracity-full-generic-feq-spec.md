<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Spec: `veracity-full-generic-feq`

## Purpose

Automated refactoring tool that folds `obeys_feq_full::<T>()` into each module's
`spec_X_wf` predicate and cleans up the now-redundant feq lines from requires clauses,
loop invariants, and trigger assertions. Adds `#[verifier::loop_isolation(false)]` to
functions whose loop invariants lose their feq line.

## CLI

```
veracity-full-generic-feq [OPTIONS]
```

### Required (one of)

| Flag | Long | Description |
|---|---|---|
| `-d` | `--directory <DIR>` | Process all `.rs` files in DIR |
| `-f` | `--file <FILE>` | Process a single `.rs` file |

### Required

| Flag | Long | Description |
|---|---|---|
| `-c` | `--codebase <DIR>` | Project root (for resolving imports) |

### Optional

| Flag | Long | Description |
|---|---|---|
| `-e` | `--exclude <PATTERN>` | Exclude files matching glob (repeatable) |
| `-n` | `--dry-run` | Print changes without writing |
| | `--no-loop-isolation` | Skip adding `loop_isolation(false)` |
| | `--report` | Print summary table only, no changes |

## What It Does

For each `.rs` file in scope, the tool performs these transformations in order:

### Step 1: Identify the wf predicate

Find the module's well-formedness spec fn. Pattern:

```
open spec fn spec_<module>_wf(&self) -> bool {
    <body>
}
```

The `<module>` part is the module name lowercased with no underscores per the naming
convention (e.g., `spec_avltreesetstper_wf`, `spec_tablesteph_wf`).

If no `spec_*_wf` is found, skip the file (report it as skipped).

### Step 2: Determine type parameters

Scan the file for `obeys_feq_full::<X>()` occurrences. Collect the unique type parameters
used (e.g., `T`, `K`, `V`, `Pair<K, V>`). Classify:

- **Single-type**: Only `obeys_feq_full::<T>()` appears.
- **Two-type**: `obeys_feq_full::<K>()` and `obeys_feq_full::<V>()` appear (possibly
  also `obeys_feq_full::<Pair<K, V>>()`).
- **Other**: Any other combination — report and skip (human review needed).

### Step 3: Modify the wf predicate

Add the feq clause(s) to the wf predicate body, before the closing `}`:

**Single-type:**
```rust
open spec fn spec_foo_wf(&self) -> bool {
    // ... existing body ...
    && obeys_feq_full::<T>()
}
```

**Two-type:**
```rust
open spec fn spec_foo_wf(&self) -> bool {
    // ... existing body ...
    && obeys_feq_full::<K>()
    && obeys_feq_full::<V>()
    && obeys_feq_full::<Pair<K, V>>()   // only if Pair<K,V> feq was used in the file
}
```

If the wf predicate already contains `obeys_feq_full`, skip this step for that type param.

### Step 4: Remove feq from loop invariants

For every `while` or `loop` block, find lines in the `invariant` clause that are exactly:

```
                    obeys_feq_full::<T>(),
```

(with any amount of leading whitespace, and possibly `K`, `V`, or `Pair<K, V>` instead of `T`).

Delete these lines. Track which functions had invariant lines removed — they need
`loop_isolation(false)` in Step 6.

**Important**: Only remove feq lines from invariants. Do NOT remove other lines from
invariants. The line must match the pattern `\s*obeys_feq_full::<[^>]+>\(\),?\s*$`.

### Step 5: Remove standalone trigger assertions and feq from requires

For functions that have `self.spec_*_wf()` (or similar wf call) in their `requires` clause:

**5a.** Delete lines matching `obeys_feq_full::<X>()` from the `requires` clause.
These are now implied by wf.

**5b.** Delete `assert(obeys_feq_full_trigger::<X>());` statements from the function body.
These were only needed to establish feq for the solver; wf now provides it via the
requires clause.

**Exception — constructors**: Functions that do NOT have a wf predicate in their `requires`
but DO have wf in their `ensures` are constructors (e.g., `empty()`, `singleton()`,
`from_seq()`). For these:

- Do NOT remove `assert(obeys_feq_full_trigger::<X>());` — constructors need it to
  prove the feq part of wf.
- If the constructor lacks the trigger assert but now needs to prove feq in wf, ADD
  `assert(obeys_feq_full_trigger::<T>());` as the first statement in the function body.
  (For two-type: add one for each type param.)

**Detection of constructors**: A function is a constructor if:
- It returns `Self` or the module's main type
- Its `requires` clause does NOT mention `self.spec_*_wf()` or `old(self).spec_*_wf()`
- Its `ensures` clause DOES mention `spec_*_wf()`

### Step 6: Add `#[verifier::loop_isolation(false)]`

For every function that had `obeys_feq_full` removed from a loop invariant in Step 4,
add `#[verifier::loop_isolation(false)]` as an attribute on that function.

The attribute goes immediately before the `fn` keyword:

```rust
        #[verifier::loop_isolation(false)]
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
```

If the function already has the attribute, skip.

**Why**: Verus's loop isolation prevents facts from the function context (like wf implying
feq) from flowing into loop bodies. With isolation disabled, the solver can unfold wf inside
the loop and see that it implies feq, making the explicit invariant line unnecessary.

### Step 7: Clean up trailing commas and blank lines

After removing lines from invariants and requires blocks, fix formatting:
- Remove double blank lines left by deletions.
- Fix trailing commas: if the last invariant line now ends with `,` before `decreases`
  or the loop body `{`, that's fine (Verus accepts trailing commas in invariants).

### Step 8: Handle `obeys_view_eq` (optional, flag-gated)

`obeys_feq_full` implies `obeys_view_eq` (it's a stronger property). When `--cleanup-view-eq`
is passed:

- For functions with wf in requires, remove `obeys_view_eq::<X>()` from their requires
  clause (since wf -> feq_full -> view_eq).
- This is a separate flag because it's riskier — the solver may not automatically derive
  view_eq from feq_full without a lemma.

## Output

### Dry-run mode (`-n`)

Print a per-file summary:

```
src/Chap41/AVLTreeSetStPer.rs:
  wf predicate: spec_avltreesetstper_wf (line 162)
  type params: T (single-type)
  wf modification: +1 line (obeys_feq_full::<T>())
  loop invariant removals: 10 lines in 8 functions
  trigger assert removals: 8 lines in 8 functions
  requires removals: 0 lines (feq was not standalone in requires)
  loop_isolation(false) added: 8 functions
  constructor triggers kept: 2 (empty, singleton)
  net lines saved: 18
```

### Report mode (`--report`)

Print a summary table across all files:

```
| # | Chap | File                    | Type | WF Lines | Inv Rm | Trig Rm | Req Rm | Iso Add | Net |
|---|------|-------------------------|------|----------|--------|---------|--------|---------|-----|
| 1 | 41   | AVLTreeSetStPer.rs      | T    | +1       | -10    | -8      | 0      | +8      | -10 |
| 2 | 42   | TableStEph.rs           | K,V  | +3       | -2     | -4      | -3     | +2      | -4  |
...
```

### Normal mode

Apply all changes, print the summary table, and report any files skipped with reasons.

## Edge Cases

### No wf predicate

Some files use `obeys_feq_full` but have no `spec_*_wf`. Examples: `Types.rs` (defines
the broadcast axioms), `MathSeq.rs` (only 1 occurrence). Skip these — report as:

```
SKIP src/Types.rs: no spec_*_wf predicate found (feq axiom definitions, not a consumer)
SKIP src/Chap17/MathSeq.rs: no spec_*_wf predicate found
```

### Already migrated

If the wf predicate already contains `obeys_feq_full::<T>()`, skip Step 3 for that type
param but still do Steps 4-6 (there may be leftover invariant/trigger lines from a partial
manual migration).

### Free functions (not in trait impl)

Some files have free functions (not inside `impl Trait for Type`) that use feq. These
functions won't have `self.spec_*_wf()` in requires. Handle them like constructors: leave
their feq lines alone unless they call a method that requires wf (in which case they must
establish wf, which now implies feq).

### Multiple wf predicates

Some files (e.g., graph files) may have multiple wf predicates for different types.
Each wf predicate gets its own feq clause matching its type parameter.

### `obeys_feq_clone` and `obeys_feq_eq`

These are different from `obeys_feq_full`. Do NOT touch them. Only process
`obeys_feq_full::<X>()` and `obeys_feq_full_trigger::<X>()`.

### Hash table files

`src/Chap47/*.rs` files use `obeys_feq_full` but have complex wf predicates involving
hash functions. Process them the same way — add feq to wf, remove from invariants/requires.

## Reference Implementation

The manual test on `src/Chap41/AVLTreeSetStPer.rs` produced:

**Before** (original file):
- `obeys_feq_full::<T>()` in 10 loop invariants
- `assert(obeys_feq_full_trigger::<T>())` in 10 function bodies
- wf predicate: 3 clauses (avltreeseqstper_wf, no_duplicates, finite)

**After** (verified, 87s, 4480 verified, 0 AVLTreeSetStPer errors):
- `obeys_feq_full::<T>()` added to wf predicate (+1 line)
- `obeys_feq_full::<T>()` removed from all 10 loop invariants (-10 lines)
- `assert(obeys_feq_full_trigger::<T>())` removed from 8 non-constructor functions (-8 lines)
- `assert(obeys_feq_full_trigger::<T>())` KEPT in `empty()` and `singleton()` (constructors)
- `#[verifier::loop_isolation(false)]` added to 8 functions (+8 annotations)
- Net: -10 feq lines, +8 annotations

The reference file is at: `src/Chap41/AVLTreeSetStPer.rs` (current working tree state).

## Validation

After running the tool:

1. `scripts/validate.sh` — must pass with same verified count (±12 per file for removed
   obligations) and no new errors.
2. `scripts/rtt.sh` — must pass (feq changes are spec-only, no runtime impact).
3. `scripts/ptt.sh` — must pass.

If verification fails after the tool runs, the likely causes are:

1. **Constructor missing trigger**: A constructor now needs to prove feq as part of wf but
   lacks `assert(obeys_feq_full_trigger::<T>())`. Fix: add the trigger assert.
2. **Free function needs feq**: A function outside the trait impl uses feq but doesn't have
   wf in its requires. Fix: add `obeys_feq_full::<T>()` back to that specific function's
   requires.
3. **Loop isolation issue**: A loop body uses feq (calls `feq()` or `lemma_cloned_view_eq`)
   but `#[verifier::loop_isolation(false)]` wasn't added. Fix: add the attribute.

## Files to Exclude by Default

These files define feq infrastructure — they should never be modified by this tool:

- `src/vstdplus/feq.rs`
- `src/vstdplus/feq_stub.rs`
- `src/Types.rs` (contains broadcast axioms for concrete types)
- `src/standards/*.rs`
- `src/experiments/*.rs`
- `src/lib.rs`

Pass these as `-e` exclusions, or hardcode them.
