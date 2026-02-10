//! Proof tests for ArraySeq
//!
//! Loop patterns tested (see docs/APASLoops.md):
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in iter: it`
//!
//! Higher-order function tests (spec_fn bridge / closure ensures):
//!   - iterate, reduce, scan, scan_inclusive, filter, iterate_prefixes, map, tabulate

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let
test_verify_one_file! {
    #[test] arrayseq_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;
        
        fn test_loop_loop() {
            let a: ArraySeqS<u64> = ArraySeqS::new(3, 42);
            
            let mut it: ArraySeqIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof {
                        items = items.push(*x);
                    }
                } else {
                    break;
                }
            }
            
            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-iter: `for x in iter: it` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] arrayseq_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        fn test_for_iter() {
            let a: ArraySeqS<u64> = ArraySeqS::new(3, 99);
            
            let it: ArraySeqIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();
            
            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(*x);
                }
            }
            
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// iterate: Call with a concrete spec_fn and verify the result.
test_verify_one_file! {
    #[test] arrayseq_iterate verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        fn test_iterate() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(10);

            // Bitwise OR is total and has a clear spec.
            let f = |acc: &u64, x: &u64| -> (ret: u64)
                ensures ret == (*acc | *x)
            { *acc | *x };

            let ghost spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| (a | b);

            let result = <ArraySeqS<u64> as ArraySeqTrait<u64>>::iterate(&a, &f, Ghost(spec_f), 0u64);

            // The call succeeds, proving the requires (spec_fn bridge) is satisfiable.
            // The ensures gives: result == spec_iterate(Seq::new(1, |i| a.spec_index(i)), spec_f, 0).
            let ghost _r = result;
        }
    } => Ok(())
}

// reduce: Call with bitwise OR (a monoid with identity 0) and verify the spec_fn bridge.
test_verify_one_file! {
    #[test] arrayseq_reduce verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        // Bitwise OR on u64: associative, 0 is left and right identity, no overflow.
        proof fn bitor_is_monoid()
            ensures spec_monoid(|a: u64, b: u64| a | b, 0u64),
        {
            let spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| a | b;
            assert forall|x: u64, y: u64, z: u64| #[trigger] spec_f(spec_f(x, y), z) == spec_f(x, spec_f(y, z)) by {
                assert(spec_f(spec_f(x, y), z) == ((x | y) | z));
                assert(spec_f(x, spec_f(y, z)) == (x | (y | z)));
                assert((x | y) | z == x | (y | z)) by (bit_vector);
            }
            assert forall|x: u64| #[trigger] spec_f(0u64, x) == x by {
                assert(0u64 | x == x) by (bit_vector);
            }
            assert forall|x: u64| #[trigger] spec_f(x, 0u64) == x by {
                assert(x | 0u64 == x) by (bit_vector);
            }
        }

        fn test_reduce() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(7);

            let f = |acc: &u64, x: &u64| -> (ret: u64)
                ensures ret == (*acc | *x)
            { *acc | *x };

            let ghost spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| a | b;

            proof { bitor_is_monoid(); }

            let result = <ArraySeqS<u64> as ArraySeqTrait<u64>>::reduce(&a, &f, Ghost(spec_f), 0u64);

            // The call succeeds, proving the requires (spec_fn bridge + monoid) is satisfiable.
            let ghost _r = result;
        }
    } => Ok(())
}

// scan: Call with bitwise OR monoid and verify the spec_fn bridge.
test_verify_one_file! {
    #[test] arrayseq_scan verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        proof fn bitor_is_monoid()
            ensures spec_monoid(|a: u64, b: u64| a | b, 0u64),
        {
            let spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| a | b;
            assert forall|x: u64, y: u64, z: u64| #[trigger] spec_f(spec_f(x, y), z) == spec_f(x, spec_f(y, z)) by {
                assert((x | y) | z == x | (y | z)) by (bit_vector);
            }
            assert forall|x: u64| #[trigger] spec_f(0u64, x) == x by {
                assert(0u64 | x == x) by (bit_vector);
            }
            assert forall|x: u64| #[trigger] spec_f(x, 0u64) == x by {
                assert(x | 0u64 == x) by (bit_vector);
            }
        }

        fn test_scan() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(7);

            let f = |acc: &u64, x: &u64| -> (ret: u64)
                ensures ret == (*acc | *x)
            { *acc | *x };

            let ghost spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| a | b;

            proof { bitor_is_monoid(); }

            let (scanned, total) = <ArraySeqS<u64> as ArraySeqTrait<u64>>::scan(&a, &f, Ghost(spec_f), 0u64);

            // The call succeeds, proving the requires (spec_fn bridge + monoid + obeys_feq_clone) is satisfiable.
            let ghost _s = scanned;
            let ghost _t = total;
        }
    } => Ok(())
}

// scan_inclusive: Call with bitwise OR monoid and verify the spec_fn bridge.
test_verify_one_file! {
    #[test] arrayseq_scan_inclusive verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        proof fn bitor_is_monoid()
            ensures spec_monoid(|a: u64, b: u64| a | b, 0u64),
        {
            let spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| a | b;
            assert forall|x: u64, y: u64, z: u64| #[trigger] spec_f(spec_f(x, y), z) == spec_f(x, spec_f(y, z)) by {
                assert((x | y) | z == x | (y | z)) by (bit_vector);
            }
            assert forall|x: u64| #[trigger] spec_f(0u64, x) == x by {
                assert(0u64 | x == x) by (bit_vector);
            }
            assert forall|x: u64| #[trigger] spec_f(x, 0u64) == x by {
                assert(x | 0u64 == x) by (bit_vector);
            }
        }

        fn test_scan_inclusive() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(7);

            let f = |acc: &u64, x: &u64| -> (ret: u64)
                ensures ret == (*acc | *x)
            { *acc | *x };

            let ghost spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| a | b;

            proof { bitor_is_monoid(); }

            let result = <ArraySeqS<u64> as ArraySeqTrait<u64>>::scan_inclusive(&a, &f, Ghost(spec_f), 0u64);

            // The call succeeds, proving the requires (spec_fn bridge + monoid + obeys_feq_clone) is satisfiable.
            let ghost _r = result;
        }
    } => Ok(())
}

// filter: Call with a concrete predicate and verify the spec_fn bridge.
test_verify_one_file! {
    #[test] arrayseq_filter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        fn test_filter() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(7);

            // Keep values > 5.
            let pred = |x: &u64| -> (ret: bool)
                ensures ret == (*x > 5)
            { *x > 5 };

            let ghost spec_pred: spec_fn(u64) -> bool = |x: u64| x > 5;

            let filtered = <ArraySeqS<u64> as ArraySeqTrait<u64>>::filter(&a, &pred, Ghost(spec_pred));

            // The call succeeds, proving the requires (spec_fn bridge + obeys_feq_clone) is satisfiable.
            let ghost _f = filtered;
        }
    } => Ok(())
}

// iterate_prefixes: Call with bitwise OR and verify the spec_fn bridge.
test_verify_one_file! {
    #[test] arrayseq_iterate_prefixes verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        fn test_iterate_prefixes() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(10);

            let f = |acc: &u64, x: &u64| -> (ret: u64)
                ensures ret == (*acc | *x)
            { *acc | *x };

            let ghost spec_f: spec_fn(u64, u64) -> u64 = |a: u64, b: u64| (a | b);

            let (prefixes, total) = iterate_prefixes(&a, &f, Ghost(spec_f), 0u64);

            // The call succeeds, proving the requires (spec_fn bridge + obeys_feq_clone) is satisfiable.
            let ghost _p = prefixes;
            let ghost _t = total;
        }
    } => Ok(())
}

// map: Call with a concrete closure and verify the ensures bridge.
test_verify_one_file! {
    #[test] arrayseq_map verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        fn test_map() {
            let a: ArraySeqS<u64> = ArraySeqS::singleton(7);

            // Bitwise NOT each element (total, no overflow).
            let f = |x: &u64| -> (ret: u64)
                ensures ret == !*x
            { !*x };

            let mapped = map(&a, &f);

            // The call succeeds, proving the requires (closure totality) is satisfiable.
            // The ensures gives: mapped.spec_len() == 1 and f.ensures((&a.spec_index(0),), mapped.spec_index(0)).
            let ghost _m = mapped;
        }
    } => Ok(())
}

// tabulate: Call with a concrete closure and verify the ensures bridge.
test_verify_one_file! {
    #[test] arrayseq_tabulate verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::ArraySeq::ArraySeq::*;

        fn test_tabulate() {
            // Build [0, 1, 2] by applying the identity function to each index.
            let f = |i: usize| -> (ret: u64)
                ensures ret == i as u64
            { i as u64 };

            let tab = tabulate(&f, 3);

            // The call succeeds, proving the requires (closure totality + length bound) is satisfiable.
            // The ensures gives: tab.spec_len() == 3 and f.ensures((i,), tab.spec_index(i)) for i in 0..3.
            let ghost _t = tab;
        }
    } => Ok(())
}
