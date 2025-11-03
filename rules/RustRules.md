## APAS User Rules — Canonical (UserRules4)

### Code Elegance and Minimalism

#### Terminology alignment
- Prefer standard programming-languages terminology alongside Rust usage: write `rust-term (programming-languages-term)` when referencing Rust-specific jargon (e.g., `blanket impl (polymorphic extension)`).

#### Always choose the minimal solution (KISS)
- Extend existing types/traits rather than creating new ones; look first for the smallest viable patch
- Start every sketch from the minimal change that satisfies the requirement before considering embellishments
- Avoid over-engineering—most example code online is unnecessarily complex; remove anything that is not strictly required
- Elegance comes from simplicity: prefer direct, single-purpose code over layers of helpers or abstractions
- Strongly prefer simpler, more elegant solutions over feature-heavy alternatives; resist optional knobs unless the problem demands them
- When in doubt, ask “does this extra code buy us behaviour the user requested?”—if not, keep it out

#### Closure Mutation Patterns
- **FnMut vs Fn**: When closures need to mutate captured variables, they require `FnMut` trait bounds
- **Vec-based workaround**: If a function requires `Fn` but you need mutation (like `tabulate`), replace closure-based implementation with explicit `Vec` loops
- **Pattern**: `let mut acc = init; for i in 0..n { acc = f(&acc, &data[i]); results.push(acc.clone()); }`
- This avoids closure capture issues while maintaining functional semantics

#### Variable naming discipline
- **No "temp" variables**: Never use `temp_vec`, `temp_data`, `temp_result`, etc. Variable scope and data lifetime are clear from code context.
- **No rock band/song names**: Never use variable names like `led_zeppelin`, `pink_floyd`, `stairway_to_heaven`, etc. Use descriptive names that relate to the code's purpose.
- **Descriptive names**: Use meaningful names that describe the variable's purpose: `entries`, `result_vec`, `filtered_data`, `sorted_pairs`.
- **Pattern**: `let entries = ...;` not `let temp_entries = ...;`

#### Formatting discipline
- Do not run `cargo fmt` or `rustfmt`; leave formatting passes to the user.
- User formatting target: keep `rustfmt` max line width at 120 characters.

#### Zero Warnings Policy (MANDATORY)
- **ALL CODE MUST COMPILE WITH ZERO WARNINGS**: No `warning:` messages are acceptable in any build output.
- **Fix immediately**: Address all compiler warnings before considering any task complete.
- **Common fixes**:
  - Unused variables: prefix with underscore (`_var`) or remove if truly unused
  - Unused imports: remove or conditionally compile with `#[cfg(...)]`
  - Dead code: remove or mark with `#[allow(dead_code)]` only if intentionally kept
  - Deprecated items: update to non-deprecated alternatives
- **No blanket allows**: Do not use `#[allow(warnings)]` or similar broad suppressions.
- **CI/build requirement**: All builds must pass with `-D warnings` (warnings as errors).

### Imports and Module Scope

#### Standard Library Imports and Result usage (module-top; no aliasing)
- Put needed std imports at the top of each module. Don’t write `std::…` inside trait/impl bodies.
- Don’t alias std items. Import by their real names.
- Bounds in code should read minimally (see baseline rules below).
- Prefer importing frequently used std items (`Iter`, `Formatter`, etc.) rather than repeating long paths.
- Import order: after the module declaration add a blank line, then all `use std::…` lines, then a blank line, then `use` statements from external crates, then another blank line followed by `use crate::Types::Types::*;` if needed and the rest of the internal `crate::…` imports.

Result guidance
- Formatting-only files (no generic `Result<T, E>`):
  - Import `Display`, `Debug`, `Formatter`, `Result` and use bare `Result` in `fmt` methods.
  - Example:
    ```rust
    use std::fmt::{Display, Debug, Formatter, Result};
    impl Display for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { /* … */ }
    }
    ```
- Files that also use generic `Result<T, E>`:
  - Do NOT import `fmt::Result`. Keep generic `Result<T, E>` bare for APIs, and use `std::fmt::Result` only in `fmt` methods.
  - Example:
    ```rust
    use std::fmt::{Display, Debug, Formatter};
    fn do_work() -> Result<u32, &'static str> { /* … */ }
    impl Display for Foo {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { /* … */ }
    }
    ```
- We do not use mod.rs style. Just put the modules correctly in lib.rs.


#### Specialized import rules
- Inside `src/` (library code):
  - **MUST** use `crate::…` for all intra-crate paths
  - **NEVER** use `apas_ai::` in library code
  - Prefer wildcard imports for your own modules: `use crate::Mod1::Mod1::*;`
  - Macros exported at crate root: import with `use crate::FooSeqLit;` (or call as `crate::FooSeqLit![…]`)
- Outside the crate (`tests/`, `benches/`, `examples/`, dependents):
  - **MUST** use the crate identifier `apas_ai::` (from package name `apas-ai`)
  - **NEVER** use `crate::` in integration tests or benchmarks
  - Prefer wildcard imports: `use apas_ai::Mod1::Mod1::*;`
  - Macros: `use apas_ai::FooSeqLit;` then `FooSeqLit![…]`
- Unit tests inside `src/` modules (`#[cfg(test)] mod tests`):
  - Treat as inside-crate: `use crate::…` and `use crate::FooSeqLit;`
- Never use `extern crate`. Do not add re-exports.

#### Module import style for this project
- In user modules, avoid importing individual symbols. Don’t use `use Foo::{Bar,Baz}`.
- Prefer wildcard imports `use Foo::*` for your own modules (includes traits); let the module control what’s public.
- Minimize repeated `use crate::...` lines: group them once with braces, e.g. `use crate::{Types::Types::*, LinkedListStPer::LinkedListStPer::*, LinkedListStPerChap18::LinkedListStPerChap18::*};`.
- Fall back to explicit symbol imports only to resolve name collisions.
- `PartialEq` and `Eq` definitions should be inside the file’s single module.
- Don’t create root shim files (e.g., `Chap3.rs`) purely to re-export a directory; declare inline modules with `#[path]` or reference the directory structure directly.

#### No trailing per-file re-exports (use lib.rs instead)
- Do not place lines like `pub use FooMod::FooModTrait;` at the end of source files.
- If a re-export is desired for public API ergonomics, add it in `src/lib.rs` only.
- Inside modules, import items via their module paths (e.g., `use crate::FooMod::FooMod::*;`) rather than relying on per-file re-exports.
- Macros remain defined inside their modules with `#[macro_export]`; do not add extra re-exports for them.

#### Wildcard-first imports; group to minimize `use crate`
- Default: wildcard-import module contents (including traits): `use crate::SomeMod::SomeMod::*;`.
- Prefer a single grouped import per file: `use crate::{A::A::*, B::B::*};`.
- Only name symbols explicitly when disambiguating or when a wildcard would pull conflicting items into scope.

#### Use Lit! macros for literal data construction
- Always construct fixed, small literal values using the provided `...Lit!` macros (e.g., `SetLit!`, `RelationLit!`, `MappingLit!`).
- For pair-like elements, use `Pair` inside the literal: `SetLit![Pair(a, b), Pair(c, d)]`.
- Do not hand-build literals with temporary vars, loops, or manual inserts in tests or examples. Prefer the literal macro for clarity and brevity.
- If a macro cannot express the literal you need, prefer adding/updating that macro rather than open-coding a one-off constructor.

#### Avoid "helper" terminology (code smell)
- Do not introduce a helper function unless it will be used in at least 3 distinct call sites (or across 2+ modules), or it eliminates clearly error-prone duplication.
- Otherwise, keep the code inline or use an existing macro/constructor. Exceptions: readability for >10 lines of complex logic.
- **Never use "helper" to describe types or functions** - it's vague and adds no information
  - ❌ BAD: "helper type", "helper function", "helper method"
  - ✓ GOOD: Describe the actual purpose or role
    - "mutex wrapper" (describes what it does)
    - "constructor" (describes its role)
    - "private internal type" (visibility + architectural role)
- **"Private" vs "Internal"** - both are appropriate but mean different things:
  - **Private** = Rust visibility scope (no `pub` keyword)
  - **Internal** = architectural role (implementation detail, not exported API)
  - Example: "private internal type" is precise - private visibility, internal purpose

#### Module/file layout and Mandatory Encapsulation
- **MANDATORY**: Each file should have a single module.
- **ALL CODE MUST BE WITHIN `pub mod M{...}`**: Every function, struct, enum, type alias, macro, and implementation must be defined inside the module block. The only exceptions are:
  - `src/main.rs`: May have a free `fn main()` function
  - `src/lib.rs`: May have module declarations and re-exports at file level
- **NO FREE DEFINITIONS**: No definitions are allowed at file scope outside the module block in any `src/*.rs` file (except the exceptions above).
- **Violation is a build error**: Any code found outside module blocks should be moved inside immediately.
---

### Traits and Implementations (Mandatory Pattern)
- For every new public API in `src/` modules, define a public trait inside the module and implement it for the module's concrete type(s). Do not expose only free functions as the API surface.
- Hoist baseline bounds at the trait header (see Generalized lifting rule) and mirror them on the corresponding impl header.
- Keep both the trait and its impl(s) inside the module's single `pub mod` block (see Mandatory Encapsulation).
- Name traits and impls consistently with the module (e.g., `Chapter36StTrait` implemented for `ArraySeqStEphS<T>`).
- Free functions may exist for composition, but core operations must be available via trait methods.
- Tests: write at least one test per public trait item (see Tests Format).

#### Default Trait Implementations (Pattern)
- **One-line defaults in trait**: If a default implementation fits on one line (≤120 chars), provide it directly in the trait definition.
- **Multi-line defaults in impl**: If a default implementation requires multiple lines, provide only the method signature in the trait and implement it in the impl block.
- **Rationale**: Keeps trait definitions scannable and readable while isolating complex logic in impl blocks.
- **Pattern**:
  ```rust
  pub trait FooTrait<T: StT>: Sized {
      // Required primitives (no body)
      fn primitive1() -> Self;
      fn primitive2(&self) -> &[T];
      
      // One-line defaults (readable at a glance)
      fn empty()            -> Self { Self::primitive1() }
      fn length(&self)      -> N { self.primitive2().len() }
      
      // Multi-line defaults (signature only)
      fn complex_operation(&mut self, data: T) -> Result<&mut Self, &'static str>;
  }
  
  impl<T: StT> FooTrait<T> for FooS<T> {
      fn primitive1() -> Self { /* ... */ }
      fn primitive2(&self) -> &[T] { /* ... */ }
      
      // Complex default implementation
      fn complex_operation(&mut self, data: T) -> Result<&mut Self, &'static str> {
          // Multi-line logic here
          if self.validate(data) {
              self.update(data);
              Ok(self)
          } else {
              Err("Invalid data")
          }
      }
  }
  ```
- **Alignment encouraged**: Vertically align `->` in trait method signatures for improved readability (manual in Emacs; do not use `rustfmt`).

#### No Trait Method Duplication (MANDATORY)
- **NEVER** duplicate trait method implementations as inherent methods on the same type.
- Trait methods are the single source of truth for behavior.
- If a trait method has an implementation in the trait impl block, **DO NOT** create an inherent method with the same name and signature.
- Rationale: Eliminates redundant code paths, ensures consistent behavior, improves test coverage, and follows DRY principle.
- **Violating pattern** (WRONG):
  ```rust
  impl<T> MyType<T> {
      pub fn empty() -> Self { Self { data: Vec::new() } }  // ❌ DELETE THIS
  }
  impl<T> MyTrait<T> for MyType<T> {
      fn empty() -> Self { Self { data: Vec::new() } }      // ✓ KEEP ONLY THIS
  }
  ```
- **Correct pattern**:
  ```rust
  impl<T> MyTrait<T> for MyType<T> {
      fn empty() -> Self { Self { data: Vec::new() } }      // ✓ SINGLE SOURCE
  }
  // Call via trait: MyType::empty() or <MyType as MyTrait>::empty()
  ```

#### Inherent Impls - Single Implementation Pattern (MANDATORY)

**Core Rule: Types with custom traits MUST have only ONE implementation location.**

- **Import philosophy**: `use Module::*;` is encouraged - modules control exports via `pub`
  - Users get everything the module publicly exports
  - Don't need to track which names are types vs traits vs functions
  - Module author decides public API, not the importer
  - Conflicts are rare and resolved explicitly when they occur

- **Single Implementation Pattern**:
  - If a type has a custom trait: ALL public methods go in the trait impl ONLY
  - NO inherent impl block alongside trait impl for the same functionality
  - Prevents duplicate code paths and testing overhead
  - Eliminates confusion about method resolution (inherent vs trait)
  
  ```rust
  // ❌ WRONG - two implementations of same functionality
  pub struct SetStEph<T> { data: HashSet<T> }
  
  impl<T: Eq + Hash> SetStEph<T> {  // ❌ DELETE THIS - duplicate code path
      pub fn empty() -> Self { SetStEph { data: HashSet::new() } }
      pub fn insert(&mut self, x: T) -> bool { self.data.insert(x) }
      pub fn union(&self, other: &Self) -> Self { /* ... */ }
  }
  
  impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
      fn empty() -> Self { SetStEph { data: HashSet::new() } }
      fn insert(&mut self, x: T) -> bool { self.data.insert(x) }
      fn union(&self, other: &Self) -> Self { /* ... */ }
  }
  
  // ✓ CORRECT - single implementation location
  pub struct SetStEph<T> { data: HashSet<T> }
  
  impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {  // ✓ SINGLE SOURCE
      fn empty() -> Self { SetStEph { data: HashSet::new() } }
      fn insert(&mut self, x: T) -> bool { self.data.insert(x) }
      fn union(&self, other: &Self) -> Self { /* ... */ }
      // ... all methods here, tested once, used everywhere
  }
  ```

- **Public vs Private types**: Different rules for what's exported
  
  **Public types (exported with `pub`)**: ALWAYS use `pub trait` + trait impl
  - Enforces Single Implementation Pattern at API boundary
  - Users get traits via `use Module::*;` automatically
  - Prevents method hiding and ensures discoverability
  
  ```rust
  // ✓ CORRECT - public type with public trait
  pub struct ArraySeqMtEphSliceS<T> { /* ... */ }
  
  pub trait ArraySeqMtEphSliceTrait<T: StTInMtT> {
      fn empty() -> Self;
      fn length(&self) -> N;
      // ... all public methods
  }
  
  impl<T: StTInMtT> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
      // implementations
  }
  ```
  
  **Private types (no `pub`)**: Use trait if non-trivial, skip if tiny
  - Private types don't leak outside module (not in `use Module::*`)
  - Use **private trait** (no `pub`) if:
    - Multiple methods (>1)
    - Non-trivial logic (mutex locking, tree balancing, etc.)
    - Improves readability and maintainability
  - Skip trait (use inherent impl) only if:
    - Single trivial constructor (`fn new()` with basic field initialization)
    - ≤3 lines total, no complex logic
  
  ```rust
  // ✓ CORRECT - private internal type with private trait (mutex logic)
  struct Inner<T: StTInMtT> {
      data: Mutex<Box<[T]>>,
  }
  
  trait InnerTrait<T: StTInMtT> {  // private trait (no pub)
      fn new(data: Box<[T]>) -> Self;
      fn len(&self) -> N;
  }
  
  impl<T: StTInMtT> InnerTrait<T> for Inner<T> {
      fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }
      fn len(&self) -> N {
          let guard = self.data.lock().unwrap();
          guard.len()
      }
  }
  
  // ✓ ACCEPTABLE - single trivial constructor (pragmatic exception)
  struct AVLTreeNode<T> {
      value: T, height: usize, left: Option<Box<AVLTreeNode<T>>>, right: Option<Box<AVLTreeNode<T>>>,
  }
  
  impl<T: StT> AVLTreeNode<T> {
      fn new(value: T) -> Self {
          AVLTreeNode { value, height: 1, left: None, right: None }
      }
  }
  ```

- **Allowed inherent impls**: Only for types that have NO custom trait
  - Utility types (examples, analysis helpers, result containers)
  - Types that genuinely don't need/want a trait abstraction
  - These should be rare - most types benefit from trait abstraction

- **Standard library trait impls**: Always allowed (Clone, Debug, Display, Ord, PartialOrd, Hash, Eq, etc.)
  - These are trait impls (`impl Trait for Type`), not inherent impls
  - Place at bottom of file after custom trait impls

- **Detection**: `scripts/rust/src/find_inherent_impls.py` - currently 149 blocks across 106 files
- **Status**: Ongoing cleanup to eliminate duplicate implementations

### Types, Bounds, and Lifting

#### Types and Bounds (Baseline + Minimal Additions)
- Baseline (project‑wide): Public types, traits, and impls must bind element type parameters to `Eq + Clone + Copy + Display + Debug + Sized` at the declaration site (not on every method).
- Hash is opt‑in: Add `Hash` only when required by a specific API (e.g., `HashMap`/`HashSet` or hashing). Prefer the narrowest scope (a single method or specific impl). Do not hoist `Hash` to a trait header unless every item needs it.
- Minimal additions: Add extra bounds only when strictly required by stdlib contracts or called code, and keep them as local as possible (method/impl where‑clause).
- Associated types: When an associated type represents the element type, ensure it satisfies `Eq + Clone + Copy + Display + Debug + Sized`; add `Hash` only when that associated type is used in hashing contexts.

#### Generalized lifting rule (applies to every trait/impl)
- Project baseline for public element type parameters is: `Eq + Clone + Copy + Display + Debug + Sized`.
- Bind the baseline at the declaration site (trait/struct/impl header). Do not repeat baseline bounds on every method.
- Hoist only bounds that are common to all trait items’ signatures/where‑clauses and satisfiable at all call sites.
- Do NOT hoist `Hash` unless every item requires it.
- Method‑only extras stay local (on that item). If every item in an impl needs an extra bound, put it on that impl header.
- Mirror hoisted bounds from the trait header exactly on every corresponding impl header.
- Do not hoist lifetimes. Keep lifetime parameters where they logically belong.
- If multiple impl blocks repeat identical bounds, unify them on the impl header; avoid per‑method duplication.

Example
```rust
pub trait Foo<T: Eq + Clone + Copy + Display + Debug + Sized> {
    fn show(&self, x: T) -> String;

    fn bucket(&self, x: T) -> usize
    where
        T: std::hash::Hash;
}

impl<T: Eq + Clone + Copy + Display + Debug + Sized> Foo<T> for Bar<T> {
    fn show(&self, x: T) -> String { /* … */ }

    fn bucket(&self, x: T) -> usize
    where
        T: std::hash::Hash,
    {
        /* … */
    }
}
```

#### Type Creation Traits (align with baseline)
#### Function argument bounds without where-clauses (new)
- Prefer inline generic bounds directly on the function’s generic parameters and arguments; avoid trailing `where` clauses unless:
  - The bounds are too long to read inline, i.e., over  120 characters, you may put in a Where clause.
  - You need higher-ranked trait bounds/lifetimes making inline form unreadable.
- For methods returning sequence types, put the element bound inline on the method generic: `fn map<U: StT + Clone>(...) -> Seq<U>` not `fn map<U>(...) -> Seq<U> where U: StT + Clone`.
- For inherent/trait methods that repeat a single bound across many items in the same impl, hoist to the impl header per the hoisting rules above; otherwise keep inline.

#### Callable parameter style (`impl Fn` in parameter position)
- Prefer `impl Fn` in parameter position when a function takes a callable and you do not need to name its concrete type, unify it across parameters, or return it. This keeps signatures short and avoids a separate `where` clause.
- Use a named generic like `F: Fn(...) -> _` if the callable's type must be referenced in multiple places (e.g., two parameters must be the same closure type) or if the function returns the callable.
- Use a trait object like `&dyn Fn(...) -> _` for dynamic dispatch or heterogeneous storage behind a pointer; accept the virtual call overhead.
- Pick the correct trait: `Fn` for non‑mutating closures, `FnMut` if the closure mutates captured state, `FnOnce` if it consumes captured values.

Example (applies to `reduce`):
```rust
fn reduce<T: MtT + Clone + Eq>(
    a: &ArrayPerS<T>,
    f: impl Fn(&T, &T) -> T,
    id: T,
) -> T
```

#### Default element bound (StT by default)

- Default: Use `StT` (`Eq + Clone + Display + Debug + Sized`) for public data structures and chapter traits.
- MtT is exceptional: Use `MtT` (`Sized + Send + Sync`) only when concurrency primitives are stored (e.g., `Mutex`, parallel chap19 algorithms) or thread-safety is otherwise required.
- For `ArrayPerS<T>`: if parallel algorithms store thread-safe wrappers, constrain chapter traits/methods to `MtT` locally; otherwise keep the core type and common traits `StT`.
- `Hash` remains opt‑in; not part of `Elem`.
- `Hash` stays opt‑in: add `T: Hash` only on the specific methods/impls that use hashing; do not include `Hash` in `Elem`.

- For new public concrete types:
  - Derive `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`.
  - Implement `Display`.
  - Add `Hash` only if the type is used in hashed contexts.
- Sequence‑like types (wrap or behave like a collection):
  - Provide `iter()` and `iter_mut()`.
  - Implement `IntoIterator` for owned, `&Self`, and `&mut Self`.
  - Implement `ExactSizeIterator`/`DoubleEndedIterator` where applicable.

---

### APIs, Macros, Constructors, Encapsulation

#### Macro Normalization (Exported and Type‑Checked)
- Define at crate root:
  - Use `#[macro_export]` and place `macro_rules!` at crate root (end of the module file).
  - Inside the macro body, use `$crate::` fully qualified paths to all types/functions.
- One definition: no module‑local duplicates or wrappers; one canonical macro per type family.
- Call‑site ergonomics:
  - Non‑empty forms rely on element types (ints default to `i32`); empty forms require a minimal type annotation at the call site.
  - Import macros from the crate root in tests/benches with `use my_crate::MacroName;`; inside `src` you can use `crate::MacroName` or invoke directly.
- Macros are all `pub`; used to make datatype literals (`Lit!`).
- Dead‑code type‑check helper (required; must include empty form):
```rust
#[allow(dead_code)]
fn _MyMacro_type_checks() {
    let _ = MyMacro![1];               // non‑empty infers (e.g., i32)
    let _: MyType<i32> = MyMacro![];   // empty form requires explicit type
}
```
- Naming: keep macro names consistent and descriptive (e.g., `FooSeqLit!`), aligned with the type they construct.

#### Struct Field Encapsulation
- Default: struct fields are non‑public; hide representation by default.
- Access via API: expose state through inherent methods and trait impls (e.g., `iter`, `iter_mut`, `len`, `as_slice`), not public fields.
- Construction: use constructors/macros; disallow struct literals outside the defining module.
- Exceptions: a field may be public only with explicit user approval and documented invariants.
- Visibility scope: prefer private; use `pub(crate)` only when necessary and justified in docs.
- Testing: write tests against the public API, not internal fields.

#### No Free‑Function Wrappers
- Do not create free functions that merely forward to trait or inherent methods (e.g., `fn select(a,b,i) → <Type as Trait>::method`).
- If a method isn't available on the receiver, add an extension trait implemented for the concrete type to expose `value.method(...)`.
- Allowed: free functions only when they add real semantics (compose multiple types, add logic not tied to a single receiver, or break dependency cycles). Do not duplicate method names as free functions.
- Do not add stub functions inside traits or impls that call the same module that simply call another function/method in that module. Call sites should invoke the original API directly rather than indirection stubs.
- A 'helper' function that is called once from a trait or impl is a stub and should be in the impl, if not public.
- **Exception**: Purely functional modules may define a typeless trait for type-checking specification and algorithmic analysis documentation alongside free function implementations.

#### Type Conversions and Naming
- Prefer traits over ad‑hoc functions:
  - Implement `From<Src>` for `Dst` or `TryFrom<Src>` for `Dst`.
  - Call via `Dst::from(src)` or `src.into()`.
- Use `to_*` only when cloning/allocating an owned value is required (e.g., `to_string`, `to_vec`, `to_owned`).
- Use `as_*` for cheap borrows/views with no allocation (e.g., `as_str`, `as_slice`).
- Use `into_*` only when consuming `self` clarifies intent or returns a non‑obvious type (e.g., `into_inner`, `into_boxed_slice`).
- Do not add `to_Type` or extra `from_*` helpers if `From/Into/TryFrom/TryInto` suffices.
- Constructor exception: allow inherent `from_vec` where idiomatic; otherwise prefer trait‑based conversions.

---

### Iteration, Iterators, and Tests

#### Iterator impls (three forms)
- Implement all three `IntoIterator` forms for your sequence type (owned, `&Self`, `&mut Self`).
- Provide inherent `iter()` and `iter_mut()` helpers that delegate to the backing collection.
- Avoid unnecessary bounds; add `T: Default` only if methods (e.g., `set`) require it.
- Add `ExactSizeIterator` when length is known in O(1) and stable during iteration.
- Add `DoubleEndedIterator` when items can be traversed from both ends without extra allocation or violating semantics.

#### Tests Format
- One test per public function/trait item in a module (include iterator and formatting coverage).
- Equality: test `PartialEq`/`Eq` behavior explicitly where defined.
- Iterator tests: cover forward and reverse traversal where applicable; assert lengths for `ExactSizeIterator`.
- Formatting:
  - `Display`: assert exact formatted string.
  - `Debug`: assert it contains the type name or key structure elements, as appropriate.
- Prefer `assert_eq!` on data values wherever possible.
- Prefer `<MacroName>Lit![…]` for non‑empty literals; use `T::new()` for empty cases instead of `<MacroName>Lit![]`.
- The only test we put in source code is for a macro with deadcode allowed to check it's typing. 

#### Integration Test Structure (MANDATORY)
- **Integration tests** (files in `tests/` directory) must have test functions at the **root level** of the file.
- **NEVER use `#[cfg(test)]` modules** in integration test files - this prevents test discovery.
- **Correct pattern**: `use` statements at file root, followed by `#[test]` functions at file root.
- **Incorrect pattern**: Wrapping tests in `#[cfg(test)] mod TestName { ... }` - this causes tests to be filtered out.
- **File structure**: `tests/ChapXX/TestModuleName.rs` should contain direct `#[test]` functions, not nested modules.
- **Import placement**: All `use` statements must be at the top of the file, not inside any module.

#### Test via Public API Only
- Write tests against exposed methods/traits/macros; never against private fields.

#### CamlCase not SnakeCase
- Functions/structures of more than one English word use CamlCase.
- One‑word functions may be all lower case.
- File names should be in CamlCase and start with a capital.

#### Type Inference Cleanup
- Avoid UFCS/turbofish unless required: don’t use `<Type as Trait>::method(...)` or `method::<T, _>(...)` at call sites if method‑call syntax with the trait in scope suffices.
- Prefer inferred bindings: `let x = expr;` when the type is deducible from the expression or later usage.
- Minimal annotations: when needed, use `let x: T = expr;`.
- Constructors: prefer inherent constructors (`Type::new`, `Type::from_vec`) or `Lit!` macros for literals.
- Numeric guidance: rely on defaults (`i32` for integers, `f64` for floats); add literal suffixes only when necessary.
- Iteration ergonomics: favor `iter()/iter_mut()` and `for` loops over explicit `into_iter()` unless consuming ownership is intentional.
- Eliminate redundant annotations/turbofish where later statements already constrain the type.

#### Contain UFCS (call‑site elimination)
- Replace `<Type as Trait>::item(...)` at call sites with method‑call syntax wherever possible.
- Ensure traits are imported; add inherent constructors or small extension traits and impls if needed.
- Keep UFCS inside impls/traits for disambiguation; minimize UFCS in callers (tests/benches should not need UFCS).

#### Where Clause Simplification (MANDATORY)
- **Inline Simple Bounds**: Replace `fn method<F>(...) where F: Fn(...);` with `fn method<F: Fn(...)>(...);`
- **Apply Predicate Abbreviations**: Replace `F: Fn(&T) -> bool` patterns with predicate trait bounds
- **Remove Redundant Constraints**: Remove `where T: 'static` when T is already constrained by traits that include 'static
- **Inline Type Constraints**: Replace `fn method<T>(...) where T: Clone + Eq;` with `fn method<T: Clone + Eq>(...);`
- **Apply Function Abbreviations**: Replace verbose function trait bounds with appropriate abbreviations
- **Use Type Abbreviations**: Apply consistent type abbreviations to reduce repetitive bounds
- **Target**: Minimize where clauses across codebase by inlining bounds and using abbreviations

### Script Metadata and Version Control (MANDATORY)

#### Git Commit ID in Scripts
- **ALL Python scripts** (especially in `scripts/` directories) MUST include a header comment with the git commit ID from when they were created/last modified
- **Format**: Add at the top of the file after the shebang and module docstring:
  ```python
  #!/usr/bin/env python3
  """
  Script description here.
  """
  # Git commit: <commit-hash>
  # Date: <commit-date>
  ```
- **Purpose**: Enable rollback and reapplication of scripts during research and debugging
- **Rationale**: Scripts are often generated during complex refactoring sessions. Having the exact commit ID allows:
  - Rolling back to the state before the script was applied
  - Understanding the context in which the script was created
  - Reapplying scripts in different branches or after rebases
  - Documenting the evolution of automated refactoring tools
- **Automation**: Use `scripts/add_git_metadata.py` to add/update metadata for all Python scripts
- **When to Update**: Update the commit ID whenever the script's logic is modified
- **Exception**: Scripts in `scripts/onetime/` are one-time use and may have stale commit IDs after application

---
