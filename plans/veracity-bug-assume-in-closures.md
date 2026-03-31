# Veracity Bug: assumes inside closure bodies not detected

## Problem

`veracity-review-proof-holes` does not detect `assume(...)` calls inside
closure bodies. These are real proof holes that should be counted.

## Evidence

`src/Chap52/AdjTableGraphMtPer.rs:440` has:

```rust
move |neighbors: &AVLTreeSetMtPer<V>| -> (r: AVLTreeSetMtPer<V>)
    ensures r@ == neighbors@.remove(v_clone@)
{
    proof {
        assume(neighbors.spec_avltreesetmtper_wf());
    }
    neighbors.delete(&v_clone)
},
```

Veracity marks `AdjTableGraphMtPer.rs` as `✓` (clean). Same pattern in
`src/Chap53/GraphSearchMtPer.rs` around line 168.

## Scope of the problem

This project uses closures extensively for:
- `join()` fork-join parallelism (every Mt file)
- `map`/`filter`/`tabulate` callbacks (Chap42 Table, Chap43 OrderedTable)
- Graph traversal closures (Chap52, Chap53)

Any `assume` inside these closure bodies is invisible to veracity. This
could be hiding a significant number of holes across the codebase.

## What to scan for

Search for `assume(` inside closure bodies. The pattern is:
- A closure `|args| -> (ret) ensures ... { ... assume(...) ... }`
- Or a closure `move |args| { proof { assume(...) } ... }`

These can appear as arguments to `join()`, `map()`, `filter()`, `tabulate()`,
`ordered_table.map(closure)`, etc.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

## Priority

High. Silent hole undercounting undermines the daily proof table accuracy.
