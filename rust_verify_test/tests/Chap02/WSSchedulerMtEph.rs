//! Proof time tests for WSSchedulerMtEph

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// Test that pool.join returns the results of both closures
test_verify_one_file! {
    #[test] pool_join_preserves_ensures verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::*;
        
        fn test_pool_join_with_ensures() {
            let pool = Pool::new(4);
            
            let (a, b) = pool.join(
                (|| -> (r: u64) ensures r == 10u64 { 5 + 5 }),
                (|| -> (r: u64) ensures r == 20u64 { 10 + 10 }),
            );
            
            // Verus should prove these from the closure ensures
            assert(a == 10);
            assert(b == 20);
        }
    } => Ok(())
}

// Test pool spec_size
test_verify_one_file! {
    #[test] pool_spec_size verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::*;
        
        fn test_pool_spec_size() {
            let pool = Pool::new(8);
            proof {
                assert(pool.spec_size() == 8);
            }
        }
    } => Ok(())
}
