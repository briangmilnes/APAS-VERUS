# R104 Agent 1 — Fix `assert forall ... ==>` warnings, STEP 20

## Objective

Verus warns when `assert forall` uses `==>` instead of `implies`. The warning:

```
warning: using ==> in `assert forall` does not currently assume the antecedent
in the body; consider using `implies` instead of `==>`
```

With `implies`, the antecedent is assumed in the `by` block — almost always what
you want. Find and fix all instances across the codebase.

## The fix

There are two cases:

### Case 1: Direct `==>` as outermost implication

```rust
// Before:
assert forall|j: int| 0 <= j < n ==> foo(j) by { ... }

// After:
assert forall|j: int| 0 <= j < n implies foo(j) by { ... }
```

### Case 2: Nested `==>` inside the conclusion

```rust
// Before:
assert forall|j: int| 0 <= j < n implies (A(j) ==> B(j)) by { ... }

// After — fold inner condition into antecedent:
assert forall|j: int| 0 <= j < n && A(j) implies B(j) by { ... }
```

**Important**: `implies` is only valid as the outermost implication of
`assert forall`. Inside parens, `(A implies B)` is a parse error. So nested
`==>` must be restructured by moving the inner antecedent into the outer
antecedent with `&&`.

## Method

1. Run `scripts/validate.sh 2>&1 | grep -c "using ==>"` to count total warnings
2. For each warning, read the file and line, apply the fix
3. After all fixes, run `scripts/validate.sh` to confirm zero warnings and zero errors
4. Do NOT change any `==>` that is NOT inside an `assert forall` — regular
   `requires`/`ensures`/`spec fn` implications use `==>` and that's correct

## Already fixed

- `src/Chap52/AdjTableGraphStEph.rs:616` — done
- `src/Chap52/AdjTableGraphStPer.rs:574` — done

## Isolation

Use full validation since warnings may be in any chapter:

```bash
scripts/validate.sh
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 20

## Report

Write `plans/agent1-r104-implies-report.md`.
