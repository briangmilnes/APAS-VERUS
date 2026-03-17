Veracity feature: detect and classify structural false positive proof holes.

Structural false positives are holes that cannot be removed due to
Verus/Rust language limitations, not missing proof effort.

Categories to detect:

1. STD_TRAIT_IMPL
   external_body on std trait method impls (Iterator::next, PartialOrd::partial_cmp,
   Ord::cmp, Display::fmt, Debug::fmt) where the trait signature cannot carry
   requires/ensures.
   Detection: fn is inside `impl <std_trait> for ...` block AND has external_body.
   Note: Some Iterator::next are fixable by delegating to a proved inner iterator.
   Only flag as FP if the body calls a function with preconditions that can't be
   satisfied without requires on next().

2. THREAD_SPAWN
   external_body on functions whose body is solely a std::thread::spawn or
   HFScheduler::run_fork_join call, wrapping a 'static closure boundary.
   Detection: external_body fn whose body contains thread::spawn,
   thread::JoinHandle, or HFScheduler.

3. EQ_CLONE_ASSUME
   assume() inside PartialEq::eq or Clone::clone bodies (the workaround
   pattern from partial_eq_eq_clone_standard.rs).
   Detection: assume() call inside fn eq() or fn clone() within
   impl PartialEq/Clone block.

4. RWLOCK_GHOST
   assume() in Mt module functions that bridge ghost state across RwLock
   read/write boundaries — the assume reconstructs ghost state that the
   RwLock predicate can't propagate.
   Detection: assume() in Mt module fn, where the function accesses an
   Arc<RwLock<...>> and the assume references ghost/spec state.

5. UNSAFE_SEND_SYNC
   unsafe impl Send/Sync for types with Ghost<...> fields where the inner
   ghost type doesn't satisfy Send/Sync bounds but is erased at runtime.
   Detection: `unsafe impl Send` or `unsafe impl Sync` on a type that
   contains Ghost<...> fields.

6. OPAQUE_EXTERNAL
   external_body on functions that call external Rust functions with no
   Verus spec (e.g., string formatting, I/O).
   Detection: external_body fn whose body calls functions from std:: that
   have no verus specification.

Output format per hole:
  file:line | category | fn_name | confidence (high/medium/low)

Confidence levels:
  high = pattern match is unambiguous
  medium = pattern match but edge cases possible
  low = heuristic, needs human review

Exclusions:
  - Skip *Example*.rs files
  - Skip Problem*.rs files
  - Skip experiments/ directory
  - assume(false); diverge() in thread-join is an info. This is how you handle divergence in verus.
