# Verus Rules and Lessons Learned

## Building Verus

### Using vargo
- `vargo` is Verus's cargo wrapper that manages custom build paths and verification artifacts.
- **Build Verus from source**:
  ```bash
  cd source  # in verus-lang repository
  source ../tools/activate  # adds vargo to PATH (for bash/zsh)
  vargo build --release
  ```
- `vargo` accepts standard cargo commands: `build`, `test`, `run`, `clean`, `fmt`, `metadata`
- **Note**: `vargo` does NOT support `-j N` parallel build flag (unlike cargo)
- **No `--help` flag**: vargo is minimal and doesn't provide detailed help output
- To add `vargo` to PATH permanently, add to `~/.bashrc`:
  ```bash
  export PATH=~/projects/verus-lang/tools/vargo/target/release:$PATH
  ```

## Core Principles

### Don't Dig Holes
- Think before acting. Don't create unnecessary complications.
- Don't suggest `external_body` unless explicitly asked or after exhausting verification options.
- Explain *why* a proof fails before proposing workarounds.
- One step at a time. Read first, then make a single focused change.

### STEP Methodology

When the user requests "N STEP", complete the task in N iterations following this exact process:

**Each STEP consists of:**

0. **Read** - Read the current state of the file(s) we're working on
1. **Think** - Understand what change the user is requesting
2. **Implement** - Make that change without adding, deleting, or transforming unnecessarily
3. **Validate** - Run Verus verification on the change
4. **Show Errors** - Display the full validation output (no filtering, no grep)
5. **PAUSE** - Wait for user feedback before proceeding to next STEP

**Critical Rules:**
- Always read the file first - never assume you know its current state
- Never skip showing validation errors
- Never make multiple changes without validation between them
- Never summarize or hide error output - show the raw Verus output
- Each STEP is atomic: read, think, implement, validate, show errors, pause

### Proof Development Strategy
- Use **mixed top-down/bottom-up reasoning**:
  - Top-down: Identify core invariants the code needs
  - Assume temporarily: Strengthen postconditions to isolate problems
  - Bottom-up: Verify the body with strengthened assumptions
  - Refinement: Prove the strengthened assumptions hold
- Prove complex properties once at the appropriate abstraction level, not repeatedly in consuming code.

## SMT Solver Behavior

### Triggers
- `#[trigger]` annotations control when the SMT solver applies quantified axioms.
- Triggers fire during SMT reasoning, not just from explicit code paths.
- Other axioms being checked can trigger additional lemmas indirectly.
- Broadcasting (`#[verifier::proof]` with `broadcast use`) makes lemmas available globally.

### Assertions
- Use `assert` statements to guide the SMT solver through inductive steps.
- Place assertions at proof-critical points, not as documentation.
- Even simple Verus examples (like `triangle`) use explicit `assert` for induction.
- SMT proofs are "proof irrelevant" - we don't see the internal reasoning steps.

### Assumptions
- `assume` tells SMT to accept a fact without proof - use only for debugging.
- Comment out `assume` statements once the real proof is in place.
- Never ship code with uncommented `assume` statements.

## Iterator Verification

### Ghost vs Executable State
- **Ghost state**: Specification-only, exists at verification time only.
- **Executable state**: Real runtime data that also appears in specs via `View` trait.
- Iterator needs both: executable position/data, ghost abstract state.

### The Exhausted State Pattern
- Don't rely on `curr == None` alone to distinguish initial vs final iterator state.
- Add explicit `exhausted: bool` executable field.
- Four iterator states:
  1. Initial empty (`curr == None`, `!exhausted`, `future.is_empty()`)
  2. Initial non-empty (`curr == Some(...)`, `!exhausted`)
  3. Stepping (`curr == Some(...)`, `!exhausted`)
  4. Exhausted (`curr == None`, `exhausted`)

### Iterator Traits and Specifications
- Define `exec_invariant` to link concrete and ghost state.
- Define `ghost_invariant` for additional abstract properties.
- Define `ghost_ensures` for final state conditions.
- Define `ghost_decrease` for termination measures.
- Define `ghost_peek_next` for next element specification.
- Define `ghost_advance` for state transition specification.
- Put the proof burden in `next()`'s postconditions, not in every consuming loop.

## Loop Verification

### Invariants
- State what must be true at loop start and after each iteration.
- Keep invariants minimal - only what's needed for the proof.
- Most properties should prove automatically from preconditions.
- Use `invariant_except_break` if property doesn't hold at `break` exits.

### Termination
- Every `while` loop needs a `decreases` clause (or `#[verifier::exec_allows_no_decreases_clause]`).
- Decreases clause must be a value with a well-founded order (e.g., `int`, not `Option<int>`).
- Common patterns: `end - cur`, `n - i`, `len - processed`.

### Inductive Proofs
- SMT solvers don't automatically handle complex inductive properties.
- Explicit `assert` statements are required to guide induction (see `triangle` example).
- Break induction into small, explicit steps.

## For-Loop Verification (Verus Desugaring)

For `for x in collection.iter() { body }`:

1. `let init_iter = collection.iter();` - requires `exec_invariant` holds on `init_iter@`
2. Loop entry - asserts `exec_invariant`, `ghost_invariant` (from `init_iter@`)
3. `while let Some(x) = iter.next() { body }`:
   - After `next()`: asserts `exec_invariant`, assumes element properties from `ghost_peek_next`
   - Before next iteration: asserts `exec_invariant`, `ghost_invariant`
4. Loop exit: asserts `ghost_ensures`
5. Termination: `ghost_decrease()` must decrease each iteration

## Type System and Traits

### Rust/Verus Trait Limitations
- **No method overriding in subtraits**: Can't redefine a method from a supertrait with different specs.
- **No specification refinement**: Can't add `requires` to an inherited trait method in an implementation.
- **No GATs (Generic Associated Types)**: Verus doesn't support `type Iter<'a>` patterns yet.
- **Workaround**: Use cloning instead of lifetimes, or build custom traits from scratch.

### View Trait
- `impl View for ConcreteType { type V = GhostType; fn view(&self) -> GhostType }`
- Access view with `concrete_value@` syntax.
- View connects executable state to specification state.
- **Critical**: `self@` returns the abstract type (`V`), not the struct. So `self@.len()` works for `Set`, but `self@.field.len()` doesn't.
- **Trait bounds**: Adding `: View` as a supertrait can cause ambiguity - compiler may resolve to `vstd::string::View` instead of `vstd::prelude::View`. Avoid View in trait supertraits if possible.
- **Return types**: `&mut Self` return types are **not supported** in Verus trait methods. Use `()` or `bool` instead.

## Generic Types

### Equality for Generic T
- Concrete types (`int`, `usize`): Verus knows executable `==` matches spec `==`.
- Generic `T: PartialEq`: Verus **cannot** axiomatize this connection automatically.
- Workaround: Use concrete types, or define trait with spec-level equality, or use `external_body` (if justified).

### Sized vs Unsized
- Associated types implicitly require `Sized`.
- Use `type T: ?Sized` to allow unsized types (e.g., `[int]`, `str`).
- Can't pass unsized types by value - use `&[T]` not `[T]`.

## Sequences and Collections

### Seq Operations
- `s.len()` - length (returns `nat`)
- `s[i]` - indexing (requires `0 <= i < s.len()`)
- `s.take(n)` - prefix, first `n` elements (exclusive: `subseq(0, n)`)
- `s.skip(n)` - suffix, elements after first `n`
- `s.push(x)` - append element
- `s@` - view of Vec/Array/Slice as Seq

### Set Operations
- `s.insert(x)` - add element (returns new set)
- `s.remove(x)` - remove element (returns new set)
- `s.contains(x)` - membership
- `s.union(t)` or `s + t` - union
- `s.disjoint(t)` - disjointness
- `s.is_empty()` - empty check
- Define `singleton(x)` helper for single-element sets.

### Recursive Collection Types
- Use `#[verifier::reject_recursive_types(T)]` for types that contain collections of themselves.
- Example: `SetStEph<T>` containing `HashSet<T>` needs this attribute to prevent infinite type expansion.
- Without it: `error: Type parameter T must be declared #[verifier::reject_recursive_types]`

### Standard Library Specifications
- Many `std` types lack Verus specs (e.g., `HashSet::clone`).
- Use `assume_specification` to provide specs for unsupported std methods:
  ```rust
  pub assume_specification<T, S> [<std::collections::HashSet<T, S> as std::clone::Clone>::clone]
      (_0: &std::collections::HashSet<T, S>) -> std::collections::HashSet<T, S>
  where S: std::clone::Clone, T: std::clone::Clone,
  ;
  ```
- Place specs in a dedicated module (e.g., `vstdplus/hash_set_specs.rs`).

## Common Pitfalls

### Overflow
- Integer operations can overflow in executable code.
- Use `wrapping_add`, `wrapping_sub`, etc., or prove bounds prevent overflow.
- Spec functions use mathematical `int`, which doesn't overflow.

### Type Mismatches
- `usize` vs `int`: Cast with `as int` for specs, `as usize` for exec (with bounds check).
- `u32` vs `int`: Same casting rules.
- Seq indexing expects `int`, array indexing expects `usize`.

### Linter Warnings
- Clippy prefers `+=` over `i = i + 1`.
- Ignore false positive linter errors during verification (can't disable specific Verus constructs).

## Documentation and Communication

### Code Comments
- Don't use `{}` to say "this follows directly" - use for real proof blocks only.
- Terse comments before each assertion/invariant explaining what it does.
- Avoid "jejune" (obvious) comments that just restate the code.
- Don't spam with prose that assumes the reader can't read code.

### Variable Naming
- Prefer `elt` over `target` for collection elements (standard terminology).
- Use `i`, `j`, `k` for integer indices.
- Use descriptive names for complex ghost state (`previous`, `current`, `future`).

## Working with Verus Tooling

### Verification Command
- **Always use the full Verus command for verification:**
  ```bash
  cd ~/projects/APAS-VERUS && \
  ~/projects/verus-lang/source/target-verus/release/verus \
  --crate-type=lib src/lib.rs \
  --multiple-errors 20 \
  --expand-errors \
  --time-expanded
  ```
- **Never** use `verus` on a single file when the file has crate dependencies (will fail with "could not find X in crate root").
- **Never** use `cargo build` or `vargo build` for verification - they don't run the Verus verifier.

### Validation Output
- Always show validation errors in tool output, not hidden in terminal.
- Report exact error messages, line numbers, and failure context.

### Symbol Navigation
- Use `consult-lsp-symbols` (or `lsp-workspace-symbol`) for jumping to definitions.
- Ensure LSP indexes `vstd` and Verus standard library.

## File Organization

### Module Structure
- Separate concerns: traits in one file, implementations in another, examples in a third.
- Keep experimental code in `src/experiments/`.
- Document verification patterns in `docs/`.

### Dependency Management
- Verus builds on Rust's module system.
- Use `pub mod` in `lib.rs` to expose modules.
- Comment out modules that don't compile to keep the crate building.

## Philosophy

### Verification is Not Magic
- If a proof is too complicated, the specification or code structure might be wrong.
- Simplify before adding more proof complexity.
- Trust the SMT solver for simple facts, guide it explicitly for inductive reasoning.

### Learn from Examples
- Study Verus examples (`examples/`, `verus-lang/verus`) for patterns.
- Even simple examples use explicit proof steps - don't expect full automation.
- Copy patterns that work, adapt to your domain.

### Cognitive Load Management
- Minimize context switching.
- Direct symbol navigation preserves flow.
- One focused task per STEP cycle.
- Don't force the programmer to think about irrelevant details.

