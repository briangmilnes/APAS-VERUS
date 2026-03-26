# veracity-compare-par-mut Phase 3: Compare traits across variants (Round 2)

## Context

Phase 1 (file groups) and Phase 2 (data type comparison) are done.
Their output is in `src/ChapNN/analyses/` logs. Do NOT re-run phases 1 or 2.

## Phase 3: Identify and compare module traits

For each file group identified in Phase 1, extract the module trait from each
variant file and compare across the group.

### What to extract per variant file

- Trait name (e.g., `ArraySeqStEphTrait`)
- Type bounds on the trait's generic parameters (e.g., `T: StT + Ord`)
- Supertraits (e.g., `Sized + View<V = Seq<T>>`)
- Function list: name, parameters, return type (with named return if present)
- For each function: has requires (yes/no), has ensures (yes/no)

### What to compare across variants in a group

**Function set alignment**: Does MtEph have every function that StEph has?
Missing functions are gaps. Extra functions are noted but not flagged.

**Type bound alignment**: Do all variants require the same bounds on T?
Extra bounds on one variant (e.g., MtEph adds `Clone`) are warnings.

**Supertrait alignment**: Do all variants have the same View type in their
supertrait? Mismatches are errors.

**Parameter alignment**: For matched functions, do parameter counts and types
agree (modulo `&self` vs `&mut self` for Eph vs Per)?

## Output format

Emacs compilation-mode format: `file:line: LEVEL: message`

```
src/Chap18/ArraySeqStEph.rs:45: info: trait ArraySeqStEphTrait<T: StT> — 12 fns
src/Chap18/ArraySeqMtEph.rs:89: info: trait ArraySeqMtEphTrait<T: StT> — 12 fns
src/Chap18/ArraySeqStEph.rs:0: info: matched functions: new, length, nth, set, push, pop, ...
src/Chap18/ArraySeqMtEph.rs:0: warning: missing function `append` (present in StEph)
src/Chap18/ArraySeqMtEph.rs:102: warning: `insert` extra type bound: MtEph has T: StT + Clone, StEph has T: StT
src/Chap18/ArraySeqStPer.rs:60: error: supertrait View<V = Seq<T>> but MtPer has View<V = Seq<T::V>>
```

Severity:
- `info:` — each trait identified, function count, matched function sets
- `warning:` — missing functions, extra type bounds, different param counts
- `error:` — supertrait View type mismatch, return type mismatch

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All parsing must be token-aware or AST-aware. Parse trait blocks with
brace/comma/semicolon awareness. A string-hacking detector will flag and kill
tools that corrupt source syntax.
