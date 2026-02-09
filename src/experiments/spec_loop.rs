//  Experiment: spec_loop
//
//  HYPOTHESIS: Verus will reject a single trait that mixes Self-returning
//  methods (filter) with concrete-type-returning methods whose type parameter
//  differs (map returning WrapS<U>), due to a cycle in trait resolution.
//
//  RESULT: The simple case verifies cleanly — no cycle error.  The concern
//  that motivated the BaseTrait / RedefinableTrait split in Chap18 does not
//  reproduce at this level of complexity.  The split remains justified on
//  software-engineering grounds (base methods are never redefined; redefinable
//  ones are) but is not forced by a Verus type-system limitation in this
//  minimal example.

#[allow(unused_imports)]
pub mod spec_loop {

    use vstd::prelude::*;

    verus! {

    // A trivial wrapper around Vec<T>.
    #[verifier::reject_recursive_types(T)]
    pub struct WrapS<T> {
        pub v: Vec<T>,
    }

    impl<T: View> View for WrapS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            self.v@.map(|_i: int, t: T| t@)
        }
    }

    // --- Single trait that mixes Self-returning and concrete-returning methods ---
    //
    // `filter` returns Self  — fine.
    // `map`    returns WrapS<U> — causes the cycle.

    pub trait WrapTrait<T>: Sized {
        spec fn spec_len(&self) -> int;

        fn filter<F: Fn(&T) -> bool>(a: &WrapS<T>, pred: &F) -> (filtered: Self)
            where T: Clone
            requires
                forall|i: int| 0 <= i < a.v@.len()
                    ==> #[trigger] pred.requires((&a.v@[i],)),
            ensures
                filtered.spec_len() <= a.v@.len();

        fn map<U: Clone, F: Fn(&T) -> U>(a: &WrapS<T>, f: &F) -> (mapped: WrapS<U>)
            requires
                forall|i: int| 0 <= i < a.v@.len()
                    ==> #[trigger] f.requires((&a.v@[i],)),
            ensures
                mapped.v@.len() == a.v@.len();
    }

    impl<T> WrapTrait<T> for WrapS<T> {
        open spec fn spec_len(&self) -> int {
            self.v@.len() as int
        }

        fn filter<F: Fn(&T) -> bool>(a: &WrapS<T>, pred: &F) -> (filtered: WrapS<T>)
            where T: Clone
        {
            let len = a.v.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.v@.len(),
                    seq@.len() <= i,
                    forall|j: int| 0 <= j < a.v@.len()
                        ==> #[trigger] pred.requires((&a.v@[j],)),
                decreases len - i,
            {
                if pred(&a.v[i]) {
                    seq.push(a.v[i].clone());
                }
                i += 1;
            }
            WrapS { v: seq }
        }

        fn map<U: Clone, F: Fn(&T) -> U>(a: &WrapS<T>, f: &F) -> (mapped: WrapS<U>)
        {
            let len = a.v.len();
            let mut seq: Vec<U> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.v@.len(),
                    seq@.len() == i as int,
                    forall|j: int| 0 <= j < a.v@.len()
                        ==> #[trigger] f.requires((&a.v@[j],)),
                decreases len - i,
            {
                seq.push(f(&a.v[i]));
                i += 1;
            }
            WrapS { v: seq }
        }
    }

    } // verus!
}
