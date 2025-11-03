## Algorithmic Analysis Rules

### Baseline Reporting
- Always report **Work** and **Span** (parallel time) for every algorithm, matching APAS conventions.
- Provide Big-O or better Big-Theta bounds plus the supporting recurrence or summation; simplify algebra so the reader can follow each step.
- State the assumptions behind the analysis (input size `n`, element comparison cost, deterministic vs. probabilistic models).

### Recurrence Handling
- Derive the recurrence from the algorithm before solving; note base cases explicitly.
- Use the Master Theorem, tree method, or substitution as appropriate; show the reasoning, not just the answer.
- When combining subproblems, document how work and span compose (e.g., parallel forks vs. sequential chaining).

### Data Structure Costs
- Reference the established APAS cost tables for primitive operations (sequence access, tree rotations, etc.).
- If you introduce a new data structure, tabulate its operation costs and justify them (amortized or worst-case as appropriate).
- Highlight any hidden constants or memory overhead that impact practicality even when asymptotics match.

### Parallelism Details
- Identify which steps run in parallel, which are sequential bottlenecks, and how they influence span.
- Note synchronization costs (spawn/join, atomics) and include them in the recurrence or additive terms.
- Clarify how work is distributed across processors and discuss load balancing when non-trivial.

### Edge Cases and Stability
- Discuss pathological inputs (already sorted, skewed trees, empty data) and whether they change the bounds.
- Call out probabilistic assumptions (hashing, randomized pivots) and provide expected vs. worst-case costs.
- If the algorithm relies on preconditions (e.g., sorted input), state them alongside the analysis.

### Presentation Standards
- Use precise language: “Work Θ(n log n), Span O(log n)” instead of vague qualitative terms.
- Present final results in a summary table when analyzing multiple functions in one module.
- Link back to the relevant code sections or lemmas so readers can cross-reference implementation and analysis.

### Documentation
- Each AI agent is to mark their estimate before each function and method in the traits with a /// <AGENT-NAME> Work: ... Span: .... .
- If an Agent's analysis differs from one entered by APAS, mark it with a 
  /// BUG: <AGENT-NAME>'s algorithmic analysis differs from APAS.
- If a prompt contains APAS's algorithmic analysis mark each function and method with an /// APAS: Work: ... Span: .... .

### Simplified Annotation Format
- When both APAS and an AI agent have analyzed the same function, use simplified format:
  - If they agree: `/// APAS: Work Θ(n), Span Θ(1), claude agrees`
  - If they disagree: `/// APAS: Work Θ(n), Span Θ(1), claude disagrees: Work Θ(n log n), Span Θ(log n)`
- **Parallelism annotations are ONLY allowed in `*Mt*` files** (mutable/parallel data structures)
  - In `*Mt*` files: Abbreviate `Parallelism` as `Par` (e.g., `Par Θ(n/log n)`)
  - In `*St*` files: Parallelism annotations are stripped entirely (sequential implementations have no parallelism)
- Remove trailing explanatory comments (e.g., "- parallel divide-and-conquer", "- dominated by", "- sequential")
- Use `scripts/rust/src/fix_complexity_annotations.py` to automatically convert double-line annotations to simplified format

### Algorithmic Analysis Process
- **Source Files Only**: Algorithmic analysis should be performed on `src/` files, not test files.
- **Consolidated Documentation**: Comprehensive algorithmic analysis should be documented in `chatlogs/AlgorithmicAnalysis.txt`.
- **Test File Scope**: Test files should focus on correctness verification, not algorithmic analysis documentation.
- **Analysis Extraction**: If algorithmic analysis is accidentally added to test files, extract it to the consolidated documentation file.
