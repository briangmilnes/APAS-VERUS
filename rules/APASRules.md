## APAS Project Rules

### Assistant Vec Prohibition
- Callers must never gain new `Vec` usage—exports stay on arrays/sequences.
- Inside a module, `Vec` may appear only in two cases:
  - Temporary builders (`from_vec`, `to_vec`, `collect`, etc.) where the final representation is converted into the array-based structure before returning.
  - Internal scratch space when the output length is unknown up front; once determined, data is copied/moved into the canonical array representation.
- Do not expose raw `Vec` or return structures backed by `Vec`; all public APIs operate on APAS array types.
- In sweeps, never add `Vec`/`vec![]`/`to_vec()`/`into_vec()` at call sites—use sequence APIs (`tabulate`, `nth`, `length`, `set`, `iter`, literals).
- Core operations must remain inherent methods on the data structure; no free-function wrappers.
- **Exception**: Purely functional modules may define a typeless trait for type-checking specification and algorithmic analysis documentation alongside free function implementations.
- Existing caller-facing `Vec` usage must remain localized and not expand beyond its current footprint.
- **Seq-First Rule**: When length is known, operate directly on the sequence without converting to `Vec`.
- **Vec-to-Seq Rule**: When consuming a `Vec`, allocate the target array structure immediately (via `tabulate` or constructors) rather than manipulating the `Vec` in place.
- If a module does not define a sequence type but you need storage for one, 
 use Chap19 sequences. 

### Element Shorthands and Delegation
- Use the APAS shorthands to avoid repeated bounds: `StT` (`Eq + Clone + Display + Debug + Sized`) for single-threaded data, `MtT` (`Sized + Send + Sync`) for multi-threaded contexts.
- **StT Deliberately Excludes Copy**: APAS data structures must support non-Copy types like `String`, `Vec<T>`, `Pair<String, i32>`, and custom structs representing real-world entities (cities, people, composite keys). Adding `Copy` to `StT` would severely limit practical usage and force users into primitive types only. The `Clone` bound provides sufficient copying semantics for APAS algorithms while maintaining flexibility.
- Prefer `Pair` from `crate::Types::Types` over raw tuples in public APIs.
- Maintain chapter delegation rules: Chap.19 ST traits delegate to Chap.18 ST traits; Chap.19 MT traits delegate to Chap.18 MT traits. Never mix ST and MT traits in delegation paths because their bounds differ.

### APAS Naming Conventions
- **Function and Method Names**: Use `snake_case` for all function and method names, following Rust conventions.
  - Examples: `from_vec`, `from_set`, `cartesian_product`, `is_empty`, `is_singleton`
  - Convert textbook notation to snake_case: N⁺(v) → `n_plus`, N⁻(v) → `n_minus`
- **Struct and Trait Names**: Use `PascalCase` for types, following Rust conventions.
- **APAS Text Fidelity**: When the APAS textbook specifies algorithm names, mirror the semantics exactly but apply Rust naming conventions (e.g., if textbook has `Fib`, use `fib` function in Rust).
### Iterator-based Assertions
- Validate sequence contents via `iter()`, `iter_mut()`, or `into_iter()` instead of exposing backing storage so tests stay aligned with the APAS abstractions.

### Criterion Bench Configuration
- Supply representative iterator benchmarks (e.g., `iter_sum_*`).
- Use APAS timing parameters: warm-up ≤ 1 s, measurement ≈ 6 s, sample size ≈ 30, total run ≤ 10 s.
- All benchmarks must have warmup and a total time limit. The defaults are too long.

### Chapter Trait Hoisting (Option A)
- Hoist shared bounds such as `T: StT`/`T: MtT` to chapter trait headers when every method shares the element type.
- Keep extra method generics only when the method truly changes the element type (e.g., `map<U: StT>`).
- Chap.19 traits may add local bounds, but prefer reusing the hoisted bounds whenever possible.

### Parallel Spawn/Join Model
- Implement multi-threaded chapter algorithms using `std::thread::spawn` for recursive branches and `join` to synchronize completion.
- Avoid alternative thread-pool abstractions (e.g., rayon) so the parallel structure mirrors the textbook and remains amenable to Verus proofs.
- **No Thresholding**: Do not use `PARALLEL_THRESHOLD` or similar input-size checks to decide whether to parallelize. APAS parallel algorithms should always use their parallel structure regardless of input size. The textbook's parallel algorithms are unconditionally parallel.

### MT Module Discipline
- Any module whose filename contains `Mt` MUST deliver actual multi-threaded semantics: structure definitions must rely on `MtT` elements and internal synchronization (`Send + Sync`) rather than single-threaded `StT` shortcuts.
- Treat wrapper structs in `*Mt*` files as genuine MT types: their fields should employ `MtT` or thread-safe containers (e.g., mutexes, atomic references) and expose APIs safe for concurrent use.
- Never mirror a single-threaded implementation inside an `Mt` module; if functionality cannot be parallelised safely, move it to the `St` counterpart instead of duplicating it under the MT name.

### Persistent Mutation Ban
- Modules whose names end in `Per` represent persistent data structures. They must not expose in-place mutators such as `set`/`update`; persistent APIs always return a new value instead of mutating the receiver.
- `Per` implementations never expose slices or other borrowed views of private storage. Subsequence operations must allocate a fresh persistent value (e.g., `subseq_copy`) rather than returning `&[T]`.
- Treat every `*Per` file as persistent by definition: data structures are immutable, no `set`/`update`/`insert_in_place`; methods must return new structures.
- Treat every `*Eph` file as ephemeral: data structures may be mutated in place, and `set`/`update` are permitted when specified by the chapter API.

### Iteration vs. Recursion Hygiene
- When code naturally descends a structure or mirrors the textbook recursion, opt for a compact recursive implementation (often as a nested function) instead of piling logic into a `loop { … }`.
- Straightforward iterative loops are still fine for generators or linear scans; switch only when the recursion matches the idea more clearly.
- If only one call site uses the recursive routine, keep it local to that function; hoist it out only when multiple entry points need it shared.

### Graph Notation Convention
- **APAS uses semantic precision over mathematical tradition**: Use `(V, A)` for directed graphs and `(V, E)` for undirected graphs.
- **Directed graphs**: Always use `A:` for arcs (directed edges) in macros, documentation, and APIs.
  ```rust
  DirGraphLit!( V: [1, 2, 3], A: [(1, 2), (2, 3)] )
  WeightedDirGraphLit!( V: ["A", "B"], A: [("A", "B", 42)] )
  ```
- **Undirected graphs**: Always use `E:` for edges (undirected edges) in macros, documentation, and APIs.
  ```rust
  UnDirGraphLit!( V: [1, 2, 3], E: [(1, 2), (2, 3)] )
  WeightedUnDirGraphLit!( V: ["A", "B"], E: [("A", "B", 3.14)] )
  ```
- **Rationale**: While mathematics traditionally uses `(V, E)` for all graphs, APAS distinguishes directed arcs from undirected edges to avoid ambiguity about directedness in APIs and algorithms.

### Benchmark Macro Usage Patterns
- When using `StructLit!` macros in benchmark files, follow struct-specific patterns based on suffix:
  - `*Per` structs (persistent): Use `from_vec` pattern—collect data into `Vec`, then use `from_vec` or let macro handle `from_vec` internally
  - `*Eph` structs (ephemeral): Use constructor + set pattern—create with initial size/value using macro, then set individual values via `.set()` calls
  - Literal cases: Use direct macro form `StructLit![x, y, z]` when values are compile-time known
- **CRITICAL: Never replace the actual operation being benchmarked**—only replace setup/construction code that isn't the performance measurement target
- Benchmark files that specifically test `tabulate`, `map`, or other API methods must preserve those exact calls to maintain measurement validity
- This preserves performance characteristics and design patterns of persistent vs ephemeral data structures while maintaining benchmark accuracy.

### Parallel Pair Semantics
- Whenever an APAS example uses the `||` parallel pair notation, implement the corresponding Rust code with the project's Parallel Pair abstraction (not ad-hoc thread joins).
- Be sure to up the threads stacks as APAS uses lots of recursion.

### Exercise Benchmark Policy
- **Do not create benchmarks for exercises unless explicitly requested**. Exercises are learning-focused implementations that don't require performance measurement.
- **Only create benchmarks for**: Core algorithms, data structures, and implementations that are part of the main APAS library.
- **When requested**: Follow standard Criterion configuration and naming conventions (`BenchExercise_X_Y.rs`).

### Definsive interfaces
- APAS follows a defensive programming style where bad arguments (like out-of-bounds indices or invalid parameters) most don't panic but instead produce empty sequences or sets or shorter
results.

### APAS Where Clause Simplification
- **APAS Boolean Type**: Use `B` (not `bool`) in APAS code as defined in `Types.rs`
- **Predicate Abbreviations**: Replace `F: Fn(&T) -> B` patterns with `F: Pred<T>` (includes Send + Sync + 'static)
- **APAS Type Abbreviations**: Apply `MtKey`, `MtVal`, `MtReduceFn`, `HashOrd`, `ArithmeticT` consistently
- **Remove Redundant APAS Constraints**: Remove `where T: 'static` when T is already `MtVal` (which includes 'static)
- **Target**: Minimize where clauses across APAS codebase using APAS type conventions

### Functional Module Pattern
- **Purely functional modules** (containing only stateless functions with no data structures) **must** define a **typeless trait** that declares the **exact signatures** of all public free functions in the module.
- **EXCLUSION**: This pattern does **NOT** apply to modules that define data structures, type aliases, or `impl` blocks. Such modules already have proper traits for their types and should not get an additional dummy trait.
- **Identification**: A functional module contains **only** `pub fn` declarations and imports - no `struct`, `enum`, `type`, or `impl` blocks.
- The trait serves as a **type-checking specification** and **algorithmic analysis documentation space**.
- Comment the trait with: `// A dummy trait as a minimal type checking comment and space for algorithmic analysis.`
- **CRITICAL**: The trait function signatures must **exactly match** the public free function signatures - same names, same parameters, same return types, same generic bounds.
- **No implementation required** - the trait exists purely for documentation and type specification.
- **Free functions**: Implement the actual functionality as free functions in the same module with signatures that exactly match the trait.

#### Example Pattern
```rust
pub mod SortingAlgorithms {
    use crate::Types::Types::*;
    
    // A dummy trait as a minimal type checking comment and space for algorithmic analysis.
    pub trait SortingAlgorithmsTrait<T: StT> {
        /// Claude Work: O(n²), Span: O(n²)
        fn insertion_sort(arr: &mut [T]);
        
        /// Claude Work: O(n log n), Span: O(log n)
        fn merge_sort(arr: &mut [T]);
        
        /// Claude Work: O(n log n) average, O(n²) worst, Span: O(log n)
        fn quick_sort(arr: &mut [T]);
    }
    
    // Free functions - actual implementations with EXACT same signatures as trait
    pub fn insertion_sort<T: StT>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let key = arr[i].clone();
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1].clone();
                j -= 1;
            }
            arr[j] = key;
        }
    }
    
    pub fn merge_sort<T: StT>(arr: &mut [T]) {
        if arr.len() <= 1 { return; }
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge_in_place(arr, mid);
    }
    
    pub fn quick_sort<T: StT>(arr: &mut [T]) {
        if arr.len() <= 1 { return; }
        let pivot = partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }
    
    // Private helper functions don't need to be in the trait
    fn merge_in_place<T: StT>(arr: &mut [T], mid: usize) {
        // Implementation details...
    }
    
    fn partition<T: StT>(arr: &mut [T]) -> usize {
        // Implementation details...
    }
}
```

#### Rationale
- **Type specification**: Documents expected function signatures for all module functions
- **Analysis space**: Provides clean location for algorithmic complexity documentation
- **API clarity**: Makes the module's public interface explicit through trait declarations
- **Minimal type checking**: Rust compiler validates trait function signatures are well-formed
- **Documentation anchor**: Tests and benchmarks can reference the trait for expected behavior

### Factory Pattern Ban
- **NEVER use "Factory" in struct, trait, or function names**. This is a Java anti-pattern that creates unnecessary complexity and indirection.
- **Rationale**: Factory patterns obscure simple construction logic behind unnecessary abstractions. Direct constructors, builder patterns, or free functions are clearer and more idiomatic in Rust.
- **Instead of**: `LinearProbingFactory::create_string_table()` 
- **Use**: `LinearProbingHashTable::new()` or free function `create_linear_probing_table()`
- **Exception**: None. Factory naming is banned in all contexts.

### Unit Struct Algorithm Pattern
- **Unit structs with algorithmic impl blocks should be converted to free functions with documentary traits**.
- **Identification**: Unit structs (`pub struct Name;`) that contain only algorithmic methods with no state.
- **Pattern**: Convert to module with documentary trait + free functions following the Functional Module Pattern (see above).
- **Keep unit structs for**: Data containers, behavioral patterns (strategies), type-level markers with PhantomData.
- **Convert unit structs for**: Algorithms, utilities, examples, analysis functions, validators.

### Documentation
- Always put this copyright in on line 1: "//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'."
- Always put in a few line summary of the module after that, if one sentence does the job great.
- Always put this copyright in on line 1: "//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'."
- If there are problems with the implementation, such as lacking parallelism, add a "//! Note: ..." line.
