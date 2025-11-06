//! Minimal example showing that trait `requires` clauses don't propagate to impl bodies
//! for satisfying preconditions of function calls.

pub mod TraitRequiresPropagation {
    use vstd::prelude::*;
    use vstd::hash_set::*;
    use vstd::std_specs::hash::obeys_key_model;

    verus! {

// A simple trait with a requires clause
pub trait SimpleTrait<T: View + Eq + std::hash::Hash> {
    fn make_set() -> HashSetWithView<T>
        requires obeys_key_model::<T>();
}

// Implementation of the trait
pub struct SimpleImpl;

impl<T: View + Eq + std::hash::Hash> SimpleTrait<T> for SimpleImpl {
    fn make_set() -> HashSetWithView<T> {
        // ERROR: The trait has `requires obeys_key_model::<T>()`, but this doesn't
        // satisfy the precondition for HashSetWithView::new() in the impl body.
        HashSetWithView::new()  // precondition not satisfied
    }
}

// Same thing with assume - doesn't help
pub struct SimpleImplWithAssume;

impl<T: View + Eq + std::hash::Hash> SimpleTrait<T> for SimpleImplWithAssume {
    fn make_set() -> HashSetWithView<T> {
        // ERROR: assume() doesn't satisfy preconditions of function calls
        assume(obeys_key_model::<T>());
        HashSetWithView::new()  // precondition still not satisfied
    }
}

// What DOES work: concrete types with axioms
pub fn make_u64_set() -> (result: HashSetWithView<u64>)
    ensures result@ == Set::<u64>::empty()
{
    // This works because vstd has axiom_u64_obeys_hash_table_key_model
    HashSetWithView::new()
}

// Also works: trait with concrete type
pub trait ConcreteSetTrait {
    fn make_set() -> HashSetWithView<u64>;
}

pub struct ConcreteImpl;

impl ConcreteSetTrait for ConcreteImpl {
    fn make_set() -> HashSetWithView<u64> {
        // This works! Concrete types have axioms that satisfy obeys_key_model
        HashSetWithView::new()
    }
}

// CONCLUSION: You can't wrap HashSetWithView in traits with generic type parameters.
// The obeys_key_model precondition can only be satisfied for concrete types where
// vstd has proven the axioms. Use HashSetWithView directly, or only provide trait
// implementations for specific concrete types.

    } // verus!
}

