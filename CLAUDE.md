# CLAUDE.md - Project Instructions for Claude Code

## Project Goal

APAS-VERUS formally verifies algorithms from "A Practical Approach to Data Structures" using Verus. The primary objective is getting code to verify (prove) with Verus.

## Commands

```bash
# Run Verus verification
cd ~/projects/APAS-VERUS && ~/projects/verus-lang/source/target-verus/release/verus --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors

# Check for proof holes after completing a proof
~/projects/veracity/target/release/veracity-review-proof-holes -d src/

# Search for lemmas before writing new ones
veracity-search 'proof fn .*len.*'
veracity-search 'fn _ types Seq'
veracity-search -C ~/projects/APAS-VERUS 'proof fn lemma'
```

## Interaction Keywords

| Keyword | Meaning |
|---------|---------|
| "discuss" | Chat only - DO NOT modify files |
| "sketch" | Show proposed code in markdown blocks - DO NOT modify files |
| "implement" | Proceed with file changes |
| "STEP n" | At most n edit/verify cycles, then stop |
| "execute relentlessly" / "AFK" | Run without stopping for approval |

## Before Commits/Pushes

Always ask for confirmation before `git commit` or `git push`. Show what will be committed/pushed first.

## Output Requirements

- Show complete command output in responses (user has vision limitations for terminal popups)
- Index all tables and suggestion lists with numbers for easy reference ("do #2")
- Show reasoning explicitly, not just in hidden thinking blocks

## Verus Rules

1. **Fix all warnings and errors** - don't leave them unresolved
2. **Search before writing lemmas** - use `veracity-search` first
3. **Check for proof holes** - run `veracity-review-proof-holes` before claiming complete
4. **No nested functions** - Verus proof limitations; keep helpers at module level
5. **Use trigger proposals** - when Verus suggests trigger fixes, apply them
6. **Don't assume limitations** - if you think Verus can't do X, search `src/experiments/` or propose a test

## Reference Locations

| Resource | Path |
|----------|------|
| vstd (verified std lib) | `~/projects/verus/source/vstd/` |
| Verus tests | `~/projects/verus/source/rust_verify_test/tests/` |
| Verus examples | `~/projects/verus/examples/` |
| Reference codebases | `~/projects/VerusCodebases/` |
| Verus guide | https://verus-lang.github.io/verus/guide/ |

**Never modify files in `~/projects/verus/`** - suggest upstream changes instead.

## Code Style

### Naming
- **Meaningful return names**: `fn size(&self) -> (count: N)` not `(result: N)`
- **No helper names**: Don't use `helper`, `inner`, `do_it` - describe what it does
- **Preserve user names**: Don't rename parameters/fields from user sketches without asking

### Comments
- Full sentences for own-line comments (capital letter, period)
- Fragments OK for end-of-line comments
- No jejune comments that just restate the obvious from the signature
- No decorative separator lines (`// ====` or `// ----`)

### Structure
- No trivial wrappers that just forward to another function
- Use work-stealing scheduler (`src/Chap02/WSSchedulerMtEph.rs`), not raw thread spawns
- No thread threshold optimization - this is textbook code demonstrating algorithms

## File Layout

| Type | Location |
|------|----------|
| Source | `src/` |
| Runtime tests (RTT) | `tests/` |
| Proof time tests (PTT) | `rust_verify_test/tests/` |

## Module Header Format

```rust
// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Brief module description.
```

## Macros for Constructing Values

Use literal macros in tests: `SetLit![1, 2, 3]`, `RelationLit![(1, 'a')]`, `MappingLit![(1, 'a')]`, `PairLit!(1, 'a')`

## File Operations

- Use `mv` to rename files, not read/write/delete
- Keep `Cargo.toml` in sync when adding/deleting/renaming registered files

## Wrap vs Specify (for adding Verus specs to Rust types)

- **SPECIFY** (`external_type_specification`): Adds specs to existing type, user uses std type directly
- **WRAP** (new struct): Creates wrapper, allows View transformation (e.g., `Set<K::V>`), can add methods

Use WRAP when you need `K::V` in view, need extra methods, or need to enforce invariants.

## Language

- **can** = ability (is it possible?)
- **may** = permission/option (is it allowed?)
