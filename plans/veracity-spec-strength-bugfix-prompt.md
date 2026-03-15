# Veracity Spec-Strength Tool — Bug Fix Prompt

## Context

`veracity-review-spec-strength` was just built and deployed to
`~/projects/veracity/target/release/`. First real run:

```bash
veracity-review-spec-strength -c -p prompts/ src/
```

The output has four critical bugs. All must be fixed before the tool is usable.

## Bug 1: Trait + Impl Double-Counting

**Problem**: Every function appears twice — once from the trait declaration (which has
the `ensures`) and once from the impl body (which inherits ensures from the trait and
has no explicit ensures of its own). The impl copy gets classified as "missing."

**Example** from `SetStEph.rs` output:
```
 5 | empty         | weak     | empty.spec_setsteph_wf()     | only wf/finite/len
...
23 | empty         | missing  |                              | no ensures
```

Line 5 is from the trait. Line 23 is from the impl. They are the same function.

**Impact**: Inflates total count (~6788 reported vs ~3400 real exec fns), inflates
"missing" count to 38% when the real number is much lower.

**Fix**: When a function appears in both a trait declaration and an impl block for that
trait, count it ONCE. Use the trait's ensures (that's the spec). The impl body is just
the proof — it inherits the trait's ensures. Specifically:

1. Parse trait declarations and collect `(trait_name, fn_name) -> ensures`.
2. Parse `impl TraitName for Type` blocks.
3. For each fn in the impl: look up the trait's ensures, use that.
4. Do NOT emit a separate entry for the impl fn.
5. Only emit standalone functions (not in any impl-for-trait block) as their own entries.

## Bug 2: "Weak" Classification Is Broken

**Problem**: The weak classifier triggers on any ensures that doesn't match the narrow
set of "strong" patterns, even when the ensures clearly encodes the operation's full
semantics. The tool's "strong" patterns are too specific (only matching things like
`self@ == old(self)@.insert(...)`) and missing the most common strong pattern:
`result == spec_fn(args)`.

**Examples of STRONG specs misclassified as "weak"**:

| Function | Ensures | Correct | Tool Says |
|----------|---------|---------|-----------|
| `fib_seq` | `fibonacci == spec_fib(n as nat)` | STRONG | weak |
| `mem_star` | `member == in_star(self@...)` | STRONG | weak |
| `is_functional` | `functional == is_functional(...)` | STRONG | weak |
| `eq` | `equal == (self@ == other@)` | STRONG | weak |
| `mem` / `relates` | `contains == self@.contains(...)` | STRONG | strong (sometimes) |
| `insertion_sort` | `sorted.len() == old(a).len()` | PARTIAL | weak |
| `insert` (Set) | `self.spec_setsteph_wf()` | WEAK (correct!) | weak |

**The classification logic should be**:

- **STRONG**: Ensures contains a functional postcondition that fully characterizes the
  result or mutation. Patterns:
  - `result == spec_fn(...)` or `result == expr` (named return bound to spec expression)
  - `result <==> predicate(...)` (boolean result bound to spec predicate)
  - `self@ == old(self)@.operation(...)` (mutation characterized functionally)
  - `forall|...| ... ==> ...` with content predicates (not just structural)
  - `result@ == self@.method(...)` (view-level characterization)
  - Any ensures where the return value or mutation is fully determined by the inputs

- **PARTIAL**: Ensures present and correct but missing key properties:
  - Has `contains(v@)` but no extremality/ordering
  - Has `len()` spec but no content spec
  - Has domain spec but no value spec
  - Characterizes some but not all outputs

- **WEAK**: Ensures is purely structural — only `spec_wf()`, `finite()`, `len`, or `true`.
  The ensures tells you nothing about what the operation computes.

- **MISSING**: No ensures clause at all.

**Key insight**: If the ensures binds the return value to ANY expression involving the
input arguments or `self@`, it's at least PARTIAL, not weak. `result == f(x)` is the
strongest possible spec — it fully determines the output. The current tool thinks
that's weak because it doesn't pattern-match `spec_fib` or `in_star` as "known strong"
functions. The fix: ANY `result == expr` or `result <==> expr` where `expr` references
inputs or self is STRONG unless `expr` is just `true`, a length, or `wf()`.

## Bug 3: All Chapters Lumped Under One Header

**Problem**: Running with `-c` on `src/` produces one section header
`Spec Strength Review: Chap02` and dumps all files from all chapters under it.

**Fix**: When processing `src/` (or any directory containing multiple `ChapNN/`
subdirectories), emit a separate section per chapter with its own header and summary:

```
=================================================================
Spec Strength Review: Chap02
=================================================================
1. Summary
   Functions: 13 total
   ...

=================================================================
Spec Strength Review: Chap05
=================================================================
1. Summary
   Functions: 95 total
   ...
```

Each chapter gets its own summary stats. At the end, emit a grand total.

## Bug 4: Counting Non-Exec Functions

**Problem**: The tool may be counting `spec fn`, `proof fn`, and lemmas. These should
be excluded — only `fn` (exec mode) functions matter for spec strength.

**Fix**: Skip any function declared as `spec fn`, `proof fn`, `open spec fn`,
`closed spec fn`, or `pub proof fn`. Only count `fn`, `pub fn`, `pub(crate) fn` —
exec-mode functions.

## Testing

After fixing, run:

```bash
# Single chapter — should show per-file breakdown, no duplicates
veracity-review-spec-strength -p prompts/ src/Chap05/

# Spot check: SetStEph.rs should show ~20 functions (not 41)
# mem and size should be STRONG
# empty and singleton should be WEAK (only wf ensures)
# No "missing" entries for impl bodies

# All chapters — should show per-chapter sections
veracity-review-spec-strength -c -p prompts/ src/

# Compare against known ground truth:
# Chap05/SetStEph.rs has these strong specs: size, mem, all_nonempty,
#   partition_on_elt, partition, split, choose
# Chap05/SetStEph.rs has these weak specs: empty, singleton, insert,
#   union, intersection, from_vec (only wf ensures)
```

## Priority

Fix Bug 1 (double-counting) and Bug 2 (weak misclassification) first — those make the
output useless. Bug 3 (chapter grouping) and Bug 4 (non-exec fns) are important but
the tool is at least readable with those unfixed.

Build with `cargo build --release` in `~/projects/veracity/`.
