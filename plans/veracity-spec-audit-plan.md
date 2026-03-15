# Plan: veracity-review-spec-strength — Automated Spec Audit Tool

## Problem

Agents need to compare every function's `requires`/`ensures` against APAS textbook prose
to classify spec strength. Currently this requires:
- Complex bash loops over chapter directories
- grep/diff/awk pipelines to extract and compare specs
- Manual reading of prose files and source files

Every one of these bash commands asks for Claude Code permission. Agents stall constantly.

## Solution

Add a new veracity subcommand: `veracity-review-spec-strength`. It does what agents
currently do manually, but as a single Rust binary invocation that's already in the
permission allow list.

## Usage

```bash
# Single chapter
veracity-review-spec-strength src/Chap43/

# All chapters
veracity-review-spec-strength src/

# With prose comparison (reads prompts/ChapNN.txt)
veracity-review-spec-strength -p prompts/ src/Chap43/

# Machine-readable output
veracity-review-spec-strength --json src/Chap43/
```

## Output: Human-Readable

```
=================================================================
Spec Strength Review: src/Chap43/
Prose source: prompts/Chap43.txt
=================================================================

1. Summary
   Functions:  142 total
   Strong:      38 (27%)  — ensures matches prose semantics
   Partial:     24 (17%)  — ensures present but missing key properties
   Weak:        48 (34%)  — ensures only wf/finite/true
   Missing:      8 (6%)   — no ensures at all
   External:    24 (17%)  — external_body (spec may be strong or weak)

2. Per-File Breakdown

   OrderedSetStEph.rs (18 fns)
   # | Function      | Strength | Ensures Summary              | Gap
   1 | new           | strong   | wf, empty set                | —
   2 | find          | strong   | contains iff                 | —
   3 | insert        | partial  | contains(v), wf              | missing: forall preservation
   4 | first         | partial  | contains(v@)                 | missing: minimality (TotalOrder)
   5 | last          | partial  | contains(v@)                 | missing: maximality (TotalOrder)
   6 | previous      | partial  | contains(v@)                 | missing: predecessor ordering
   7 | rank          | weak     | r <= len                     | missing: filter count
   8 | filter        | weak     | subset_of                    | missing: backward completeness
   9 | split         | external | (external_body)              | needs audit
   ...

3. Classification Criteria

   STRONG:  ensures contains view-level postconditions that encode the ADT semantics.
            Examples: `self@ == old(self)@.insert(k, v)`, `found <==> self@.contains(k@)`,
            `forall|x| self@.contains(x) ==> TotalOrder::le(min, x)`.

   PARTIAL: ensures present and correct but missing one or more key properties.
            Examples: `contains(v@)` without extremality, `subset_of` without completeness,
            domain correct but values unspecified.

   WEAK:    ensures only structural (`wf`, `finite()`, `len`, `true`) — does not encode
            the ADT operation's semantics.

   MISSING: no ensures clause at all.

   EXTERNAL: has #[verifier::external_body]. The ensures may be strong or weak —
             listed separately for audit. The tool classifies the ENSURES strength,
             not the proof status.

4. Actionable Items (Partial + Weak + Missing)

   # | File                    | Function  | Strength | Action Needed
   1 | OrderedSetStEph.rs      | first     | partial  | +TotalOrder minimality
   2 | OrderedSetStEph.rs      | last      | partial  | +TotalOrder maximality
   3 | OrderedSetStEph.rs      | rank      | weak     | +filter count spec
   4 | OrderedSetStEph.rs      | filter    | weak     | +Ghost(spec_fn) completeness
   5 | OrderedTableStEph.rs    | insert    | weak     | +value preservation
   ...

5. Comparison with Prose (when -p flag used)

   Prose ADT 43.1 defines:
     first(A) = min[|A|]
     last(A) = max[|A|]
     previous(A,k) = max{k' in A | k' < k}
     next(A,k) = min{k' in A | k' > k}
     rank(A,k) = |{k' in A | k' < k}|

   Prose Match Report:
   # | Prose Def          | Function  | File                   | Match
   1 | first = min[|A|]   | first     | OrderedSetStEph.rs     | PARTIAL (has contains, no min)
   2 | last = max[|A|]    | last      | OrderedSetStEph.rs     | PARTIAL (has contains, no max)
   3 | previous = max{<k} | previous  | OrderedSetStEph.rs     | PARTIAL (has contains, no ordering)
   4 | next = min{>k}     | next      | OrderedSetStEph.rs     | PARTIAL (has contains, no ordering)
   5 | rank = |{k'<k}|    | rank      | OrderedSetStEph.rs     | WEAK (only r <= len)
```

## Output: JSON (--json)

```json
{
  "chapter": "Chap43",
  "prose_file": "prompts/Chap43.txt",
  "files": [
    {
      "file": "OrderedSetStEph.rs",
      "functions": [
        {
          "name": "first",
          "line": 140,
          "strength": "partial",
          "ensures": ["self@.finite()", "self@.len() == 0 <==> first matches None", "first matches Some(v) ==> self@.contains(v@)"],
          "gap": "missing TotalOrder minimality",
          "prose_ref": "ADT 43.1: first(A) = min[|A|]",
          "is_external_body": false
        }
      ]
    }
  ],
  "summary": {
    "total": 142,
    "strong": 38,
    "partial": 24,
    "weak": 48,
    "missing": 8,
    "external": 24
  }
}
```

## Output: Per-Chapter Log (batch mode)

When run on `src/`, writes per-chapter output to:
`src/ChapNN/analyses/veracity-review-spec-strength.log`

This parallels the existing `veracity-review-verus-proof-holes.log` pattern.

## Classification Algorithm

The tool already parses Verus source (veracity has a Verus parser). Classification:

1. **Extract ensures clauses** from each exec fn (trait or impl).

2. **Pattern-match for weakness indicators**:
   - `ensures true` → MISSING (vacuous)
   - Only `wf`/`finite()`/`len` ensures → WEAK
   - No ensures clause → MISSING
   - `#[verifier::external_body]` on body → EXTERNAL (classify ensures separately)

3. **Pattern-match for strength indicators**:
   - `self@ == old(self)@.insert(...)` → STRONG (functional update)
   - `found <==> self@.contains(...)` → STRONG (bidirectional containment)
   - `TotalOrder::le(...)` → STRONG (ordering)
   - `forall|...|` with content/ordering → STRONG
   - `to_multiset() =~=` → STRONG (content preservation)
   - `Ghost(spec_fn)` parameter present → check for backward completeness

4. **Pattern-match for partial indicators**:
   - `self@.contains(v@)` without `forall` extremality → PARTIAL
   - `subset_of` without backward completeness → PARTIAL
   - Domain correct but no value spec → PARTIAL

5. **Prose comparison** (optional, -p flag):
   - Parse `prompts/ChapNN.txt` for ADT definitions (regex: `ADT \d+\.\d+`)
   - Extract operation names and definitions
   - Match to function names
   - Report whether ensures encodes the prose definition

## Implementation Scope

- **Parser**: Already exists in veracity (Verus source parser).
- **Ensures extraction**: Already done by `veracity-review-module-fn-impls`.
- **Classification logic**: New — ~500 lines of pattern matching.
- **Prose parser**: New — ~200 lines of regex extraction from text files.
- **Output formatters**: ~300 lines (human + JSON + per-chapter log).

Estimated: ~1000 lines of Rust in `~/projects/veracity/`.

## Integration with Agent Workflow

Once built, agents use:
```bash
# Instead of complex bash loops:
veracity-review-spec-strength src/Chap43/

# Instead of manual diff against prose:
veracity-review-spec-strength -p prompts/ src/Chap43/
```

Both commands are already covered by the `Bash(~/projects/veracity/target/release/* *)`
permission rule. No new permissions needed. No bash stalls.

## Scripts to Add

```bash
# scripts/all-spec-strength-by-chap.sh — parallel to all-holes-by-chap.sh
#!/bin/bash
for dir in src/Chap*/; do
    veracity-review-spec-strength -p prompts/ "$dir"
done
```

## Relation to Existing Tools

| Tool | What it does | What it doesn't do |
|------|-------------|-------------------|
| `veracity-review-proof-holes` | Finds assume/external_body/admit | Doesn't assess ensures strength |
| `veracity-review-module-fn-impls` | Lists all fns with their specs | Doesn't classify strength |
| `veracity-review-verus-style` | Checks style conventions | Doesn't check spec semantics |
| **`veracity-review-spec-strength`** | **Classifies spec strength against prose** | New |
