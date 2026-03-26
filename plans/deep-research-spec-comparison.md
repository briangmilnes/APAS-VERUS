# Deep Research: Specification Comparison Under Ownership-Aware Unification

## Question

What are the best known algorithms and tools for determining whether two formal
pre/postcondition specifications (requires/ensures) are equivalent or in a
refinement/subsumption relationship, when the specifications come from different
implementations of the same abstract data type that differ in ownership semantics
(ephemeral mutation vs persistent functional update)?

Specifically: unification modulo Rust-style linear + borrowing, with specification
subsumption/refinement as the primary goal.

## What This Is, Formally

**Unification modulo Rust linear + borrowing semantics, with specification
subsumption/refinement as the key goal.**

Given two trait declarations from different variants of the same algorithm, we want to
determine whether their specifications are equivalent, or whether one refines the other.
The specifications live in a first-order language with Rust's ownership structure, so the
unification must respect linear/borrow distinctions.

Concretely, given:
- Terms T1 (requires/ensures from variant A) and T2 (from variant B)
- A **known** substitution σ mapping variant-specific identifiers (type names, predicate
  names, wf names — mechanically derivable from naming conventions)
- An **inferred** substitution α mapping return value binders and local names
  (alpha-equivalence on specification binders)

The primary question is not just equality — it is **refinement**: is T1 ≤ T2 (T1's
contract is at least as strong as T2's)? Equality is the special case where T1 ≤ T2
and T2 ≤ T1.

Refinement here means: if a caller satisfies T1's requires, it satisfies T2's requires
(contravariant), and T1's ensures imply T2's ensures (covariant). This is behavioral
subtyping applied to specifications.

The ownership dimension is critical:
- Ephemeral (`&mut self`) specs use `old(self)` for pre-state and `self` for post-state
- Persistent (`&self → Self`) specs use `self` for pre-state and `result` for post-state
- These express the **same abstract state transition** under different ownership regimes
- The unification must treat `old(self) ↔ self` and `self (post-mutation) ↔ result` as
  equivalent under the linear/persistent rewriting

Additional structure:
- Requires/ensures are **conjunctions** where clause order is irrelevant (multiset comparison)
- Ghost parameters in one variant with no counterpart in the other indicate refinement
  (the ghost-carrying variant has a strictly stronger spec), not divergence — ghost
  parameters are witnesses that enable stronger postconditions

## What I Want From This Research

1. **Existing algorithms** for comparing/unifying formal specifications across module
   implementations. Particularly in:
   - ML module systems (signature matching, signature subtyping)
   - Design-by-contract systems (JML, Spec#, Dafny module refinement)
   - Behavioral subtyping / specification refinement calculi
   - Verus, F*, Liquid Haskell, or other refinement type systems

2. **Unification theory** relevant to this problem:
   - E-unification (unification modulo equational theories) — our equational theory is
     Rust ownership equivalences (`&mut self` ≡ `&self → Self`)
   - Unification with binders / nominal unification — for return-name alpha-equivalence
   - Multiset/commutative unification — for order-independent clause comparison
   - Baader and Snyder, "Unification Theory" (Handbook of Automated Reasoning, 2001) as
     the foundational survey

3. **Specification refinement/subsumption**:
   - When is spec A a behavioral subtype of spec B?
   - Liskov substitution principle — formal treatments, not the informal version
   - Refinement calculus approaches to comparing pre/post specs
   - How do module systems determine signature compatibility?

4. **Ownership-aware specification comparison**:
   - How does linear/affine typing affect specification equivalence?
   - Rust-specific: how do borrow semantics (`&self` vs `&mut self` vs `self`) change
     what a spec means?
   - Session types or linear logic approaches to comparing stateful interfaces

5. **Practical tools** that do anything close to this:
   - IDE tools that compare API contracts
   - Verification tool chains with cross-module spec checking
   - Dafny's refinement checking between abstract and concrete modules
   - Any F* or Verus tooling for comparing module interfaces

## What I Do NOT Want

- General verification tutorials
- Proof assistant comparisons
- Type inference algorithms (we're not inferring types, we're comparing specifications)
- Model checking approaches (we have symbolic specs, not state machines)
- PCC (proof-carrying code) — no trust boundary here, we control all variants

## Context

This is for building a tool in a Rust verification project. The tool will be implemented
in Rust. The specifications are in Verus syntax (requires/ensures clauses with Rust
expressions + ghost/spec/proof modes). The codebase has 244 modules, ~45 multi-variant
algorithm groups, ~4900 verified functions.
