//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Arc Standard: using vstd's View and DeepView specs through Arc.
//!
//! vstd provides View and DeepView impls for Arc<T> that forward to the
//! inner type: `arc@ == (*arc)@` and `arc.deep_view() == (*arc).deep_view()`.
//! Arc::new(t) ensures `result == t` (structural equality), so the view of
//! the Arc equals the view of the value passed to new.
//!
//! This standard shows two patterns:
//! - View through Arc: Arc<SimpleS> where SimpleS has View.
//!   `arc@ == inner@` — the view sees through the Arc.
//! - DeepView through Arc: Arc<CollectionS<T>> where CollectionS has DeepView.
//!   `arc.deep_view() == inner.deep_view()` — deep view also sees through.
//!
//! Key facts:
//! - Arc::new(t) ensures result == t (vstd std_specs/smart_ptrs.rs).
//! - Arc<A: View> has `view(&self) -> A::V { (**self).view() }` (vstd view.rs).
//! - Arc<A: DeepView> has `deep_view(&self) -> A::V { (**self).deep_view() }`.
//! - Arc::clone has no explicit spec, but deref gives the same value.
//!
//! References:
//! - vstd/view.rs (View and DeepView impls for Arc)
//! - vstd/std_specs/smart_ptrs.rs (Arc::new spec)
//! - src/experiments/arc_clone_deref.rs (Arc deref experiment)

//  Table of Contents
//	1. module
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	13. derive impls outside verus!

//		1. module

pub mod arc_standard {

    use std::sync::Arc;
    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    //		4. type definitions

    // View example: a simple struct whose View is a single value.
    pub struct SimpleS {
        pub val: u64,
    }

    // Wraps Arc<SimpleS> — specs use arc@ which forwards to SimpleS@.
    pub struct ViewArcS {
        pub data: Arc<SimpleS>,
    }

    // DeepView example: a generic collection whose DeepView maps elements.
    #[verifier::reject_recursive_types(T)]
    pub struct CollectionS<T> {
        pub items: Vec<T>,
    }

    // Wraps Arc<CollectionS<T>> — specs use arc.deep_view().
    #[verifier::reject_recursive_types(T)]
    pub struct DeepViewArcS<T> {
        pub data: Arc<CollectionS<T>>,
    }


    //		5. view impls

    impl View for SimpleS {
        type V = u64;

        open spec fn view(&self) -> u64 {
            self.val as u64
        }
    }


    //		8. traits

    /// View through Arc: arc@ forwards to the inner SimpleS@.
    pub trait ViewArcTrait: Sized {
        spec fn spec_val(&self) -> u64;

        fn new(val: u64) -> (s: Self)
            ensures
                s.spec_val() == val,
        ;

        fn read_val(&self) -> (val: u64)
            ensures
                val == self.spec_val(),
        ;
    }

    /// DeepView through Arc: arc.deep_view() forwards to the inner CollectionS.deep_view().
    pub trait DeepViewArcTrait<T: DeepView>: Sized {
        spec fn spec_items(&self) -> Seq<T::V>;

        fn new(items: Vec<T>) -> (s: Self)
            ensures
                s.spec_items() == items.deep_view(),
        ;

        fn read_len(&self) -> (len: usize)
            ensures
                len == self.spec_items().len(),
        ;
    }


    //		9. impls

    impl<T: DeepView> DeepView for CollectionS<T> {
        type V = Seq<T::V>;

        open spec fn deep_view(&self) -> Seq<T::V> {
            self.items.deep_view()
        }
    }

    impl ViewArcTrait for ViewArcS {
        // Arc@ forwards to SimpleS@, which is self.val.
        open spec fn spec_val(&self) -> u64 {
            self.data@
        }

        fn new(val: u64) -> (s: Self) {
            ViewArcS { data: Arc::new(SimpleS { val }) }
        }

        fn read_val(&self) -> (val: u64) {
            self.data.val
        }
    }

    impl<T: DeepView> DeepViewArcTrait<T> for DeepViewArcS<T> {
        // Arc deep_view forwards to CollectionS deep_view.
        open spec fn spec_items(&self) -> Seq<T::V> {
            self.data.deep_view()
        }

        fn new(items: Vec<T>) -> (s: Self) {
            DeepViewArcS { data: Arc::new(CollectionS { items }) }
        }

        fn read_len(&self) -> (len: usize) {
            self.data.items.len()
        }
    }

    } // verus!

    //		13. derive impls outside verus!

    impl Debug for SimpleS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "SimpleS({})", self.val)
        }
    }

    impl Display for SimpleS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }

    impl Debug for ViewArcS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "ViewArcS({:?})", *self.data)
        }
    }

    impl Display for ViewArcS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", *self.data)
        }
    }

    impl<T: Debug> Debug for CollectionS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "CollectionS({:?})", self.items)
        }
    }

    impl<T: Display> Display for CollectionS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.items.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    impl<T: Debug> Debug for DeepViewArcS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "DeepViewArcS({:?})", *self.data)
        }
    }

    impl<T: Display> Display for DeepViewArcS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", *self.data)
        }
    }
} // pub mod arc_standard
