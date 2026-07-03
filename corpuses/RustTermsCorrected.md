# Rust Terms (Programming-Languages Alignment)

This reference crosswalk pairs Rust vocabulary with analogous programming-language
terminology. Use it as a quick translation table when you move between systems
discussions and PL theory notes.

This is a comparison of the technical terms used in Rust against standard programming terminology.

Rust Term                    | Description                                        | Programming Language Term       
---------------------------- | -------------------------------------------------- | --------------------------------
Rust Term                      | Description                                          | Programming Language Term       
------------------------------ | ---------------------------------------------------- | --------------------------------
Ad-hoc Polymorphism            | behavior specialization via traits and impls         | typeclass-style overloading (ad-hoc polymorphism)
Arc<T>                         | atomic reference counting smart pointer              | concurrent shared pointer       
Associated Type                | trait-defined type member                            | type family member              
Blanket Implementation         | impl block applying to many target types at once     | typeclass blanket instance      
Block Scope                    | region controlled by a block for locals              | lexical scope extent            
Borrow                         | temporary access without taking ownership            | alias with affine capability    
Borrow Checker                 | compile-time analysis enforcing borrow rules         | affine type checker             
Box<T>                         | owning pointer allocating on the heap                | owned heap allocation           
Cargo                          | build system and dependency manager                  | package manager toolchain       
Clone Trait                    | explicit duplication via clone()                     | copy constructor interface      
Closure                        | lambda capturing lexical environment                 | Closure                         
Const Generic                  | compile-time constant parameter                      | indexed type parameter          
Copy Trait                     | marker for implicit bitwise copy semantics           | implicit copy qualifier         
Crate                          | primary compilation unit and package                 | top-level module artifact       
Debug Trait                    | formatting for developer diagnostics                 | debug-print interface           
Default Implementation         | method body provided in trait definition             | default method implementation           
Deref Trait                    | overloads * to follow smart pointers                 | implicit dereference protocol   
Display Trait                  | formatting as user-facing text                       | pretty-print interface          
Drop                           | deterministic cleanup when scope ends                | RAII finalizer                  
Dynamic Dispatch               | runtime dyn Trait method dispatch                    | subtype polymorphism            
Enum                           | tagged union storing one of several variants         | algebraic sum type              
Eq Trait                       | marks PartialEq as total equality                    | equivalence relation marker     
Free Function                  | standalone function not tied to a type or impl       | top-level function definition   
Function Signature             | declaration of function name, parameters, and return | function declaration header     
Function Type Signature        | input and output type of a function                  | function type annotation        
Future                         | value representing an asynchronous computation       | promise/future type             
Generic Parameter              | type-level variable for polymorphism                 | parametric type variable        
Impl Block                     | impl of trait or inherent methods                    | instance declaration            
Inherent Implementation        | impl block adding methods without implementing trait | type extension                  
Inner<T>                       | crate-internal struct housing shared storage         | backing container handle        
Inherent Method                | method defined in an impl without a trait            | method definition               
Iterator                       | object producing a sequence of items lazily          | iterator combinator             
Iterator Adapter               | combinator transforming iterators                    | higher-order sequence transformer
lib.rs                         | crate root file aggregating public modules           | module root file                
Lifetime                       | static region describing borrow validity             | region typing interval          
Macro                          | macro_rules! or macro definition                     | Macro                           
Macro Definition               | macro rules block expanding to new syntax            | macro definition (hygienic)     
Macro Rules                    | compile-time code expansion mechanism                | hygienic macro system           
Match Expression               | branching by destructuring values                    | pattern matching expression     
MaybeUninit<T>                 | wrapper allowing staged initialization of memory     | uninitialized memory guard      
Method Signature               | declaration of method name, receiver, and parameters | method declaration header       
Module                         | module declaration introducing a namespace           | module definition               
Module Scope                   | visibility region of a module item                   | namespace scope extent          
Move                           | transferring ownership to a new binding              | linear move semantics           
mut                            | mutable binding qualifier allowing reassignment      | mutable binding qualifier       
NoStd                          | crate mode without the standard library              | freestanding runtime profile    
Option<T>                      | value that may be present or absent                  | maybe type constructor          
Ord Trait                      | total ordering compatible with PartialOrd            | total ordering interface        
Ownership                      | each value has one controlling owner                 | affine resource ownership       
Parametric Polymorphism        | generic code uniform for all type parameters         | uniform polymorphism            
PartialEq Trait                | equality comparison that may be partial              | partial equality predicate      
PartialOrd Trait               | ordering comparison that may be partial              | partial ordering interface      
Pattern                        | syntax for destructuring values in bindings          | structural pattern descriptor   
PhantomData                    | zero-sized marker for ownership or variance          | phantom type witness            
Pin                            | wrapper forbidding moves after initialization        | pointer stability guarantee     
Place Expression               | expression referring to a storage location           | l-value expression              
Pragma                         | attribute applied to items or modules                | Pragma                          
Pub                            | visibility modifier for public items                 | Public                          
RAII                           | ensures resources released when scope ends           | scope-bound resource management 
Range<usize>                   | half-open interval used for slicing indices         | integer range descriptor        
Rc<T>                          | reference counting smart pointer without atomics     | single-thread shared pointer    
Result<T, E>                   | success-or-error value                               | either type for error handling  
Sealed Trait                   | trait closed for implementation via a private supertrait | closed inductive trait      
Self                           | type alias for the implementing type                 | self type (type-level)          
self                           | method receiver representing the current value       | self parameter (value-level)    
Send Trait                     | marker indicating a type can move across threads     | thread-transfer capability      
Slice                          | non-owning view into contiguous elements             | array view type                 
std::cell::Cell                | interior mutability for copy types                   | copy cell                       
std::cell::RefCell             | runtime borrow-checked interior mutability           | dynamic borrow cell             
std::sync::atomic::AtomicBool  | lock-free boolean with atomic operations             | atomic boolean cell             
std::sync::atomic::AtomicUsize | lock-free usize with atomic operations               | atomic integer cell             
std::sync::atomic::Ordering    | memory ordering specifier for atomics                | memory ordering lattice         
std::sync::Barrier             | thread rendezvous synchronization                    | barrier synchronization primitive
std::sync::Condvar             | condition variable for blocking waits                | condition variable primitive    
std::sync::mpsc::Receiver      | multi-producer, single-consumer channel receiver     | channel receive endpoint        
std::sync::mpsc::Sender        | multi-producer, single-consumer channel sender       | channel send endpoint           
std::sync::Mutex               | mutual exclusion primitive with poisoning            | blocking lock abstraction       
std::sync::Once                | one-time initialization guard                        | single-assignment initializer   
std::sync::RwLock              | reader-writer synchronization primitive              | shared/exclusive lock abstraction
std::thread                    | module containing Rust threading APIs                | threading library namespace     
std::thread::Builder           | configure thread name/stack before spawn             | thread builder utility          
std::thread::current           | handle to the currently running thread               | thread descriptor handle        
std::thread::JoinHandle        | handle used to join the spawned thread               | thread join handle              
std::thread::park              | suspend current thread until unparked                | thread parking primitive        
std::thread::scope             | spawn threads borrowing stack data safely            | scoped threading construct      
std::thread::sleep             | block current thread for a duration                  | thread sleep primitive          
std::thread::spawn             | start a new OS thread running a closure              | thread creation primitive       
std::thread::unpark            | resume a parked thread via its handle                | thread unparking primitive      
std::thread::yield_now         | hint scheduler to run another thread                 | scheduler yield hint            
Struct                         | collection of named fields stored together           | product record type             
Sync Trait                     | marker allowing shared references across threads     | thread-sharing capability       
Trait                          | shared behavior specification for multiple types     | typeclass interface             
Trait Implementation           | impl block implementing a trait for a type           | interface implementation        
Trait Item                     | function/constant/type member in a trait             | interface member                
Trait Object                   | dynamically-dispatched trait reference               | existential interface value     
Triggerfish Syntax             | <Type as Trait>::method UFCS notation                | fully-qualified call syntax     
Type Alias                     | binding a new name to an existing type               | type synonym declaration        
Type Inference                 | local inference requiring explicit generics          | Type Inference                  
Unsafe Block                   | region where compiler safety checks are relaxed      | unchecked region                
Use                            | bringing modules or items into scope                 | import statement                
Where Clause                   | trailing generic constraints for types or functions  | type constraint clause          
Matches: 4 of 94 Rust terms share the same name as their programming-language counterparts.
Matches detail: Closure, Macro, Pragma, Type Inference.
The terms that match are: Closure, Macro, Pragma, and Type Inference.

### Syntax Examples

Name                         | Syntax                                                                          
---------------------------- | --------------------------------------------------------------------------------
Associated function call     | Type::function()                                                                
Method call                  | value.method()                                                                  
Method signature             | fn method(&self, arg: ArgType) -> ReturnType                                    
Mutable method signature     | fn update(&mut self, arg: ArgType)                                              
Self return type             | fn clone(&self) -> Self                                                         
UFCS call                    | <Type as Trait>::method(receiver, args)                                         
Struct literal               | Type { field: value }                                                           
Tuple struct literal         | Type(value0, value1)                                                            
Enum variant construction    | Enum::Variant(value)                                                            
Reference type               | &T                                                                              
Mutable reference type       | &mut T                                                                          
Raw pointer type             | *const T / *mut T                                                               
Slice type                   | &[T]                                                                            
Array type                   | [T; N]                                                                          
Function type                | fn(ArgType) -> ReturnType                                                       
Function type declaration    | fn name<T>(args: Type) -> ReturnType                                            
FnOnce bound                 | F: FnOnce(Args) -> Return                                                       
FnMut bound                  | F: FnMut(Args) -> Return                                                        
Fn bound                     | F: Fn(Args) -> Return                                                           
Type alias declaration       | type AliasName = ExistingType;                                                  
Associated type declaration  | type Item;                                                                      
Module declaration (inline)  | mod module_name { /* items */ }                                                 
Module declaration (file)    | mod module_name;                                                                
Trait declaration            | pub trait TraitName { fn method(&self); }                                       
Inherent impl block          | impl TypeName { /* methods */ }                                                 
Trait impl block             | impl TraitName for TypeName { /* methods */ }                                   
Public item                  | pub fn function_name() { /* body */ }                                           
Macro definition (macro_rules!) | macro_rules! name { (pattern) => { expansion }; }                               
Module macro definition      | pub macro_rules! name { ... } // add #[macro_export] for exports                
Macro invocation             | macro_name!(arguments);                                                         
Use declaration (crate root) | use crate::module::{Item, *};                                                   
Use declaration (external)   | use external_crate::module::{Type, *};                                          
Crate root path              | crate::module::Item                                                             
External crate path          | external_crate::module::Item                                                    
Relative path (super)        | super::{Item, *}                                                                
Relative path (self)         | self::{Item, *}                                                                 
Where clause                 | fn foo<T>(x: T) where T: Trait                                                  

## Namespaces and Calling Syntax

UFCS highlights:
- Disambiguates trait methods when multiple traits share a name.
- Uses the syntax <Type as Trait>::method(receiver, args...); the receiver may be moved,
  borrowed, or mutably borrowed depending on the trait signature.
- Useful when the trait is not imported into scope or when inherent method names collide.

Use declarations and namespaces:
- use brings modules, types, or traits into the current scope.
- Paths inside the current crate start with crate::; paths to external crates begin with the
  Cargo package name once it is in scope.
- Relative paths use self:: for the current module and super:: for the parent module.
- Group imports with braces to reduce repetition, for example use crate::foo::{Bar, Baz};

Typographical conventions:

Rust rule: when referencing a Rust-specific term, include the corresponding programming-language term in parentheses.
- snake_case for functions, modules, variables, and file names (per Rust naming conventions).
- UpperCamelCase for types, traits, and enum variants.
- SCREAMING_SNAKE_CASE for constants and statics.
- Leading underscores mark intentionally unused bindings.

Rust uses crate:: inside your crate's source even though it knows the crate name.
So if you go looking at source file (through some search) you can't immediately
see which crate your source is in.

## Macros

Rust macros (syntactic macros) are defined with the macro_rules! form or the newer
macro form. They can appear as pub macro_rules! inside a module, but exported macros
usually add the #[macro_export] attribute (macro export pragma). Macro expansion happens
before type checking, so the paths and names inside a macro are validated only at the call
site where it expands.

Macro notes:
- Macros expand syntactically; they are not type-checked until the expansion is inserted at the use site.
- Attributes such as #[macro_export] or #[macro_use] control visibility, while procedural
  macros rely on #[proc_macro], #[proc_macro_attribute], or #[proc_macro_derive].
- Module-level macros may be declared with pub macro_rules! and invoked via module::macro_name!(...).

### Common macro attributes

### General attributes (pragmas)

Attribute                      | Purpose
------------------------------ | --------------------------------------------------------------
#[allow(...)]                 | adjust lint level to allow selected warnings
#[deny(...)]                  | escalate selected warnings to hard errors
#[warn(...)]                  | enable additional warnings for selected lints
#[forbid(...)]                | prohibit selected lints even for downstream code
#[derive(Trait)]              | generate default implementations for the listed traits
#[must_use]                   | emit a warning if the annotated value is unused
#[inline] / #[inline(always)] | hint to the optimizer about inlining behavior
#[cold]                       | mark a function as unlikely to be executed
#[repr(...)]                  | control the layout or calling convention of a type
#[non_exhaustive]             | prevent downstream crates from exhaustively matching the item
#[cfg(...)]                   | conditionally include code based on compile-time configuration
#[macro_export]                  | export macro at the crate root
#[macro_use]                     | import macros from a module or crate
#[proc_macro]                    | mark function as procedural macro entry point
#[proc_macro_attribute]          | procedural attribute macro entry point
#[proc_macro_derive(Name)]       | derive macro entry point for Name

## Sealed Trait Idiom

**sealed trait** (Rust idiom): a trait *closed for implementation* — only the defining crate can `impl` it, because a private supertrait bound prevents external impls. Same idea as Java's `sealed interface` (JEP 409, Java 17) and SML's opaquely-ascribed datatype (where `:>` hides the constructors from outside the structure). Type-theoretically: a closed inductive sum, where the set of cases is fixed at the type's definition site.

Common Rust 1.95 examples: `core::slice::SliceIndex` (sealed via a private supertrait `slice::index::private_slice_index::Sealed`), `core::ops::OneSidedRange` (sealed via `core::range::iter::private_range::SliceIndexHelper`), `std::os::fd::AsFd` (formerly sealed).

**Trustd terminology**: use "sealed trait" or "closed trait" in prose. The PFPL-precise term is *closed inductive trait*; the Java-aligned term is *sealed trait*. Both are acceptable.

**Verus interaction**: when writing an `external_trait_specification` for a sealed trait, the private supertrait bound is unreachable from outside the defining crate. Use the `#[verifier::external_trait_private_bound(<path>)]` attribute (verus-trustd local patch) to drop the named private supertrait from the spec's supertrait list during VIR translation.

## Limitations

### Type System Limitations

Rust's type aliases (`type Alias = ExistingType;`) are purely compile-time nicknames: they inherit exactly the methods and trait implementations of the underlying type, but you cannot add new inherent methods or blanket trait impls to the alias itself. As a result, using a type abbreviation to replace a wrapper struct prevents you from extending the API—any new behavior must still be implemented on the original type. Whenever you need a distinct method/trait surface (for example, to expose multithreaded semantics or tailor bounds per module), you must keep a real `struct` newtype rather than a type alias.

### No Disjunctive/Union Types

Rust lacks disjunctive types (union types) found in languages like TypeScript or Ceylon. You cannot express `i32 | i64` or `String | &str` directly in the type system. This limitation forces verbose boilerplate when implementing the same behavior across multiple primitive types.

**Example Problem:** The `clone_mt` method in the `MtT` trait (`src/Types.rs`) requires 12+ nearly identical implementations for primitive types:

```rust
impl MtT for i32 {
    type Inner = i32;
    fn clone_mt(&self) -> Self { *self }      // Same code
    fn new_mt(inner: Self::Inner) -> Self { inner }  // Same code
}

impl MtT for i64 {
    type Inner = i64;
    fn clone_mt(&self) -> Self { *self }      // Same code  
    fn new_mt(inner: Self::Inner) -> Self { inner }  // Same code
}

impl MtT for u32 {
    type Inner = u32;
    fn clone_mt(&self) -> Self { *self }      // Same code
    fn new_mt(inner: Self::Inner) -> Self { inner }  // Same code
}

impl MtT for bool {
    type Inner = bool;
    fn clone_mt(&self) -> Self { *self }      // Same code
    fn new_mt(inner: Self::Inner) -> Self { inner }  // Same code
}
// ... 8 more identical implementations for u64, usize, isize, char, etc.
```

The `clone_mt` method exists because `Mutex<T>` cannot use regular `.clone()` - it needs special handling to lock, clone the inner value, and create a new `Mutex`. However, for primitive types, `clone_mt` is just a copy operation, leading to repetitive stub implementations.

**Root Cause:** Rust's type system cannot express "any type that is `Copy + Send + Sync`" as a union. Each primitive type requires its own explicit `impl` block to avoid trait coherence conflicts.

**Workarounds:**
1. **Macro-generated implementations** (cleanest approach for `clone_mt` stubs):
   ```rust
   macro_rules! impl_mtt_for_copy {
       ($($t:ty),*) => {
           $(impl MtT for $t {
               type Inner = $t;
               fn clone_mt(&self) -> Self { *self }
               fn new_mt(inner: Self::Inner) -> Self { inner }
           })*
       };
   }
   impl_mtt_for_copy!(i32, i64, u32, u64, usize, isize, bool, char);
   ```
   This would eliminate 8+ repetitive `clone_mt` implementations while preserving the special-case handling for `Mutex<T>` and `String`.

2. **Blanket implementation** (conflicts with specific impls):
   ```rust
   impl<T: Copy + Send + Sync> MtT for T { ... }  // Would conflict with Mutex<T> impl
   ```

3. **Newtype wrapper** (changes API):
   ```rust
   struct CopyMt<T: Copy>(T);
   impl<T: Copy + Send + Sync> MtT for CopyMt<T> { ... }
   ```

**Impact:** This limitation leads to repetitive stub implementations whenever uniform behavior is needed across multiple primitive types, making the codebase more verbose and harder to maintain than languages with proper union types.

### Wildcard Imports and Trait Ambiguity

Rust's wildcard imports (`use Module::*`) import ALL public items including traits, but multiple traits with the same method names create ambiguity that Rust refuses to resolve automatically.

**Problem:** Wildcard imports are additive, not overriding. When multiple traits define the same method name, ALL are in scope simultaneously, causing ambiguity errors.

**Example Problem:**
```rust
// Module M1 defines Trait1 with method foo()
use crate::M1::*;  // Brings Trait1 into scope

// Module M2 defines Trait2 with method foo()
use crate::M2::*;  // Brings Trait2 into scope (ADDS to scope, doesn't replace M1)

fn test(obj: &SomeType) {
    obj.foo();  // ERROR E0034: multiple applicable items in scope
                //   - M1::Trait1::foo
                //   - M2::Trait2::foo
                // Neither "overwrites" the other - BOTH are in scope
}
```

**Critical Semantic:** Unlike many languages where later imports shadow earlier ones, Rust keeps ALL imported traits in scope. The second `use` does NOT overwrite the first - it adds to the namespace, creating potential conflicts.

**Inherent Methods as Workaround:** Inherent methods (methods defined directly on a type in an `impl Type` block) have **higher priority** than trait methods in method resolution. This provides a disambiguation mechanism:

```rust
struct SomeType;

impl SomeType {
    fn foo(&self) { }  // INHERENT method - ALWAYS wins
}

use M1::*;  // Trait1::foo now in scope
use M2::*;  // Trait2::foo now in scope

fn test(obj: &SomeType) {
    obj.foo();  // ✓ Calls inherent method, ignores BOTH trait methods
}
```

**Method Resolution Order:**
1. Inherent methods on the type (highest priority - always chosen if present)
2. Trait methods from traits in scope (only if no inherent method)
3. If multiple traits match → compilation error (ambiguity)

**Impact on APAS Codebase:** The widespread use of wildcard imports (`use Module::*`) combined with multiple sequence/collection traits defining common method names (length, nth, empty, etc.) makes inherent methods necessary for disambiguation. What appears to be "duplicate" code (inherent method + trait method with same signature) is actually an **intentional disambiguation pattern** - the inherent method shadows all trait methods to resolve ambiguity.

**Example from APAS-VERUS:**
```rust
// ArraySeqStEph defines both inherent and trait methods
impl<T> ArraySeqStEphS<T> {
    pub fn length(&self) -> N { self.data.len() }  // Inherent - for callers
}

impl<T> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
    fn length(&self) -> N { ArraySeqStEphS::length(self) }  // Trait - for generics
}

// In caller code with wildcard imports:
use crate::Chap18::ArraySeq::ArraySeq::*;           // ArraySeq trait with length()
use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*; // ArraySeqStEphTrait with length()

let arr = ArraySeqStEphS::empty();
arr.length();  // ✓ Calls inherent method, avoiding ambiguity between TWO trait methods
```

Without the inherent method, `arr.length()` would fail with `E0034: multiple applicable items in scope` because both `ArraySeq::length` and `ArraySeqStEphTrait::length` are in scope via wildcard imports.

**Alternatives:**
1. **Explicit trait imports:** Import specific traits instead of wildcards to avoid bringing multiple conflicting traits into scope
2. **UFCS (Universal Function Call Syntax):** Explicitly disambiguate: `<Type as Trait>::method(&obj)`
3. **Inherent method delegation pattern:** Current APAS approach - inherent methods shadow trait conflicts

**Why This is Problematic:** This behavior is counterintuitive because:
- Most languages allow later imports to shadow earlier ones
- The "duplication" of inherent + trait methods appears redundant but is actually necessary for disambiguation
- Code review tools flag this as a DRY violation when it's actually an architectural necessity
- No explicit syntax indicates "this inherent method exists for disambiguation purposes"

### Trait Delegation and Module Composition Limitations

Rust lacks elegant mechanisms for trait delegation and module composition, forcing verbose workarounds when one module wants to re-export and extend functionality from another module.

**Common Pattern:** Module M2 wants to include all functions from Module M1's trait while adding its own methods, similar to trait inheritance or module functors in other languages.

**Problem:** Rust provides no direct way to express "M2Trait extends M1Trait" or "module M2 = M1 with additional methods."

**Available Workarounds:**

1. **UFCS Delegation** (verbose but explicit):
   ```rust
   // M2 must manually delegate every M1 method
   impl<T> M2Trait<T> for M2Struct<T> {
       fn m1_function(x: T) -> T {
           <M1Struct<T> as M1Trait<T>>::m1_function(x)  // UFCS stub
       }
       fn another_m1_function(a: &T, b: &T) -> T {
           <M1Struct<T> as M1Trait<T>>::another_m1_function(a, b)  // More boilerplate
       }
       // ... repeat for every M1 method
   }
   ```

2. **Type Aliases** (loses module identity):
   ```rust
   pub use M1::M1Trait as M2Trait;  // Just re-export, can't extend
   pub type M2Struct<T> = M1::M1Struct<T>;
   ```

3. **Composition + Deref** (limited to `&self` methods):
   ```rust
   pub struct M2Struct<T> { m1: M1Struct<T> }
   impl<T> Deref for M2Struct<T> {
       type Target = M1Struct<T>;
       fn deref(&self) -> &Self::Target { &self.m1 }
   }
   // Doesn't work for associated functions or `&mut self` methods
   ```

**What Other Languages Provide:**

- **ML Functors**: `module M2 = M1 with additional_functions`
- **Trait Inheritance**: `trait M2Trait: M1Trait { /* additional methods */ }`
- **Mixin Composition**: Automatic method forwarding with selective override
- **F* Function Aliasing**: `let ffromM2 = ffromM1` (simple function re-export)

**Impact on APAS Codebase:** This limitation requires extensive UFCS delegation stubs when Chapter N+1 modules want to re-export Chapter N functionality, leading to hundreds of lines of boilerplate code that simply forwards method calls. The verbosity obscures the actual algorithmic content and makes maintenance more difficult.

**Recommendation:** Use explicit UFCS delegation despite the verbosity, as it maintains clear module boundaries and allows selective re-export of only needed methods.

### Functional Module Documentation Limitations

Rust lacks built-in mechanisms for documenting and type-checking purely functional modules (modules containing only stateless functions with no data structures). Unlike languages with module signatures or interface files, Rust provides no way to specify the expected function signatures of a module separately from their implementations.

**Problem:** Purely functional modules have no natural place to document algorithmic complexity or provide type specifications for their functions. Free functions exist in isolation without a unifying interface declaration.

**Example Problem:** A sorting algorithms module with multiple functions has no central location to document expected signatures and complexity:

```rust
pub mod SortingAlgorithms {
    // No way to declare expected interface or document complexity centrally
    
    pub fn insertion_sort<T: StT>(arr: &mut [T]) { /* O(n²) but where to document? */ }
    pub fn merge_sort<T: StT>(arr: &mut [T]) { /* O(n log n) but where to document? */ }
    pub fn quick_sort<T: StT>(arr: &mut [T]) { /* O(n log n) average but where to document? */ }
}
```

**What Other Languages Provide:**

- **ML Module Signatures**: `module type SORTING = sig val insertion_sort : 'a array -> unit end`
- **Haskell Module Exports**: Explicit export lists with type signatures in interface files
- **F# Interface Files**: `.fsi` files declaring function signatures separately from implementation
- **OCaml Interface Files**: `.mli` files providing module type specifications

**APAS Workaround:** Use a "typeless trait" pattern for documentation and type specification:

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
    
    // Free functions - actual implementations
    pub fn insertion_sort<T: StT>(arr: &mut [T]) {
        // Implementation...
    }
    
    pub fn merge_sort<T: StT>(arr: &mut [T]) {
        // Implementation...
    }
    
    pub fn quick_sort<T: StT>(arr: &mut [T]) {
        // Implementation...
    }
}
```

**Benefits of Workaround:**
- **Type specification**: Documents expected function signatures
- **Analysis space**: Clean location for algorithmic complexity documentation  
- **API clarity**: Makes module interface explicit
- **Minimal type checking**: Rust validates trait signatures are well-formed

**Limitations of Workaround:**
- **No enforcement**: Trait signatures and free function signatures can diverge
- **Duplication**: Function signatures must be written twice (trait + implementation)
- **No automatic checking**: Compiler doesn't verify trait matches free functions
- **Unusual pattern**: Violates typical Rust idioms of implementing traits

**Impact:** This limitation forces verbose documentation patterns in algorithmic codebases where clear interface specifications and complexity analysis are essential for understanding and verification.


Modules may contain:
- type definitions (struct, enum, type alias)
- trait definitions
- impl blocks
- free functions
- constants and statics
- nested modules

A file named foo.rs is treated as mod foo { … } automatically, while lib.rs and explicit
mod statements create module boundaries. Module items must be declarations, not loose
statements. Keep source lines no longer than 120 characters.

Traits may contain:
- associated types
- associated constants
- method signatures (required or default bodies)

Traits may not contain:
- stored fields or data members
- arbitrary executable statements outside default method bodies
- implementations for types (those go in impl blocks)

Impl blocks may include:
- method definitions (with bodies)
- associated type defaults
- associated constant definitions

Impl blocks may not include:
- new trait item declarations
- free functions unrelated to the impl target

Type inference limitations:
- Rust performs local, statement-by-statement inference; function signatures must state
  generics explicitly, unlike ML’s global inference.
- Trait methods that return concrete associated types (for example, fn build() -> Widget)
  do not identify the implementor, so callers often need explicit type annotations or
  turbofish syntax (explicit type application).
- Associated functions on traits cannot be called without specifying the concrete impl type 
 (e.g., `<ArrayStPerS<_> as ArraySeqStPerChap18Trait<_>>::filter(...)`); 
  Hindley–Milner would infer the target automatically.
- Trait object upcasts (dyn Trait) erase concrete types, so methods that return Self are
  disallowed and callers must rely on associated types or where bounds.
- Inference does not speculate across trait obligations; when multiple traits provide the
  same method name, you must use UFCS or explicit annotations to disambiguate.

Rust vs. ML type inference:
- Rust requires explicit generic parameters and monomorphizes code; ML keeps polymorphic
  functions generic via Hindley-Milner inference.
- Rust traits provide ad-hoc polymorphism; ML modules or functors offer similar
  flexibility, but core ML inference does not account for trait-style obligations.
- Rust’s associated types and trait bounds often need manual annotations, whereas ML
  freely infers type variables without trait coherence constraints.

## Rust vs. ML polymorphism:

- Rust combines ad-hoc polymorphism (traits/impls) with monomorphized generics; ML leans
  on parametric polymorphism with global type inference.
- Rust allows multiple trait implementations per type (subject to coherence) and default
  methods; ML module signatures describe structure and functors instantiate behavior
  differently.
- Rust generics require explicit type parameters (no global inference) but permit
  specialization; ML infers type variables but lacks trait-style overloading.

