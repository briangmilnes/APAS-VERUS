<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# What Is the APAS Threading Model for Costs?

**Status:** Open question for Guy Blelloch and Umut Acar.

## The Question

APAS states Span costs for hash-based set operations (union, intersection,
etc.) as Theta(1). The current Verus implementations are sequential loops
with Span = Work. If we parallelize them with fork-join (the only model
available in safe Rust), the best achievable Span is Theta(log n) due to
binary divide overhead — not the Theta(1) that APAS states.

**Which cost model does APAS intend for these Span bounds?**

## Background: Three Models

### 1. PRAM (Parallel Random Access Machine)

- Fork is instantaneous: spawning n threads costs O(1) span.
- All n threads execute simultaneously.
- n independent O(1) operations in parallel: Work Theta(n), Span Theta(1).
- This gives the Theta(1) Span that APAS annotates.

### 2. Fork-Join DAG (Binary Divide)

- `par(f, g)` forks two tasks. Span = max(Span(f), Span(g)) + O(1).
- To fork n tasks, use balanced binary divide: `par(first_half, second_half)`.
- Depth of the divide tree: Theta(log n).
- n independent O(1) operations: Work Theta(n), Span Theta(log n).
- This is the model Chapter 2 of APAS defines for work-span analysis.

### 3. Sequential (Current Verus Implementation)

- `for x in iter { ... }`: each iteration depends on the previous.
- n independent O(1) operations: Work Theta(n), Span Theta(n).
- No parallelism. Span = Work.

## The Tension

Chapter 2 defines the fork-join DAG model and derives the greedy scheduling
bound T_P < W/P + S. Under this model, forking n tasks requires a binary
divide with Theta(log n) depth. There is no way to achieve Span Theta(1)
for n independent operations in the fork-join DAG model.

Yet the hash-based ADT cost tables state Span Theta(1) for operations like
union, intersection, and cartesian product. These bounds are only achievable
in a PRAM model with O(1)-cost fork.

Possible resolutions:

1. **APAS intends PRAM costs for ADT tables.** The fork-join model is for
   algorithm analysis; the ADT interface costs assume idealized parallelism.
   The log n factor is absorbed into the scheduler.

2. **APAS intends fork-join costs and Theta(1) is approximate.** The
   Theta(1) means "constant depth of hash operations, ignoring fork overhead"
   and the log n factor is considered a lower-order term that the scheduler
   handles.

3. **The Span Theta(1) annotations are for per-element costs.** The table
   states the span of a single hash operation (O(1)), not the span of the
   full compound operation. The compound operation's span depends on how it
   is composed (sequentially or in parallel).

4. **These are aspirational bounds** that assume a concurrent hash table with
   O(1) lock-free insert/lookup, where n threads can mutate the table
   simultaneously without sequential dependencies.

## Concrete Example: SetMtEph::union

APAS states: Work Theta(a + b), Span Theta(1).

| Model | Span | How |
|-------|------|-----|
| PRAM | Theta(1) | All inserts happen simultaneously |
| Fork-join DAG | Theta(log(a + b)) | Binary divide, O(1) per leaf |
| Sequential (current) | Theta(a + b) | for-each loop, no parallelism |

## What We Need to Know

1. Should the Verus Mt implementations target PRAM Span (Theta(1)) or
   fork-join Span (Theta(log n))?

2. If PRAM, should we annotate a separate "fork-join Span" to reflect
   what the implementation can actually achieve?

3. Is there a concurrent hash set model where Span Theta(1) is
   literally achievable in practice (e.g., lock-free concurrent hash map)?

## Chap05 SetMtEph Parallelism Audit

Current state of multi-threaded set operations:

| # | Function | APAS Span | Fork-join target | Current Span | Parallel? |
|---|----------|-----------|-----------------|--------------|-----------|
| 1 | from_vec | Theta(1) | Theta(log n) | Theta(n) | No |
| 2 | union | Theta(1) | Theta(log n) | Theta(a+b) | No |
| 3 | disjoint_union | Theta(1) | Theta(log n) | Theta(a+b) | No |
| 4 | intersection | Theta(1) | Theta(log n) | Theta(a+b) | No |
| 5 | partition | Theta(1) | Theta(log(a) + log(parts)) | Theta(a * parts) | No |
| 6 | cartesian_product | Theta(b) | Theta(b * log(a)) | Theta(a * b) | Partially |

Operation 6 spawns a threads in parallel but joins results sequentially.
The parallel phase achieves Span Theta(b), but the sequential join phase
is Theta(a * b), negating the benefit.

## Action Items

- [ ] Brian: Read and assess whether the question is clear.
- [ ] If unclear, draft a letter to Guy and Umut with the specific question.
- [ ] Decide on annotation convention: do we annotate PRAM Span, fork-join
      Span, or both?
- [ ] For each Mt operation, decide whether to implement true parallelism
      (fork-join with binary divide) or accept sequential with a TODO.
