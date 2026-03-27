# R89 Agent 2 — Fix ETSPStEph rlimit matching loop, STEP 20

## Objective

Uncomment `ETSPStEph.rs` (Chap26) and fix the rlimit exceeded error on
`lemma_combined_cycle`. The root cause is a Z3 matching loop on modular
sequence indexing in `spec_edges_form_cycle`.

## The Matching Loop

`spec_edges_form_cycle` has trigger `tour[i]` with body containing
`tour[((i + 1) % tour.len())]`. When Z3 instantiates the forall for index `i`,
it produces a new `tour[...]` term at `(i+1) % n`, which triggers another
instantiation, which produces another, looping around the cycle endlessly.

The `lemma_combined_cycle` proof has an `assert forall` with trigger `combined[i]`
that interacts with two sub-tour foralls (`lt[...]`, `rt[...]`), each with their
own modular indexing. This creates a 3-way instantiation cascade.

## Strategy: Break the trigger chain

The key insight: **hide the modular indexing behind a closed spec fn** so Z3's
trigger engine can't chain instantiations.

### Step 1: Add a closed helper spec

```rust
pub closed spec fn spec_next_edge_from(tour: Seq<Edge>, i: int) -> Point {
    tour[((i + 1) % (tour.len() as int))].from
}
```

### Step 2: Rewrite spec_edges_form_cycle using the helper

```rust
pub open spec fn spec_edges_form_cycle(tour: Seq<Edge>) -> bool {
    tour.len() > 0 ==>
    forall|i: int| #![trigger tour[i]] 0 <= i < tour.len() ==>
        spec_point_eq(tour[i].to, spec_next_edge_from(tour, i))
}
```

Now when Z3 instantiates for index `i`, it gets `spec_next_edge_from(tour, i)` —
an opaque function call, NOT `tour[(i+1) % n]`. The chain is broken.

### Step 3: In lemma_combined_cycle, selectively reveal

Use `reveal(spec_next_edge_from)` only inside the specific `assert ... by` blocks
where you need to reason about the actual next index. This gives you control over
when Z3 sees the modular arithmetic.

### Alternative strategies (if Step 1-3 doesn't work)

**Split into segment lemmas:** Instead of one `assert forall` over all of combined,
write 4 small proof fns:
- `lemma_cycle_left_segment` — proves cycle property for indices 0..ln_i-2
- `lemma_cycle_left_bridge` — proves it for index ln_i-1
- `lemma_cycle_right_segment` — proves it for indices ln_i..ln_i+rn_i-2
- `lemma_cycle_right_bridge` — proves it for index ln_i+rn_i-1

Each sub-lemma has a simpler quantifier structure. Then the main lemma just calls
all 4 and assembles.

**Reduce rlimit with `#[verifier::rlimit(40)]`** — if the proof is close to working,
a small rlimit bump might get it through.

## Read first

- `src/Chap26/ETSPStEph.rs` — your file (read the whole thing)
- `src/Chap26/ETSPMtEph.rs` — working Mt version (same lemma is external_body there too)

## lib.rs

Uncomment `pub mod ETSPStEph;` — it's currently `// FIX: rlimit`.

## Isolation

```bash
scripts/validate.sh isolate Chap26
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify ETSPMtEph.rs.
- Do NOT add assume or accept.
- Do NOT just bump rlimit without structural changes — the matching loop will
  eat any finite rlimit. You must break the trigger chain.
- If the closed-spec approach works for lemma_combined_cycle but other functions
  still have issues, use external_body on those and report what blocks them.
- The MtEph version has the same lemma as external_body — if you prove it here,
  we can port the proof there later.

## STEP 20

## Report

Write `plans/agent2-r89-etsp-report.md`.
