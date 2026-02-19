//! Proof tests for KleeneStPer
//!
//! KleeneStPer does NOT have an iterator — it has `mem_star`, `mem_plus`, and
//! `alphabet` methods. This PTT tests that the spec functions verify correctly
//! when called.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] kleene_mem_star verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap05::KleeneStPer::KleeneStPer::*;
        use apas_verus::SetLit;

        fn test_mem_star()
            requires valid_key_type::<u64>()
        {
            let alpha: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
            let kleene: KleeneStPer<u64> = KleeneStPer::new(alpha);

            // Empty sequence is in Σ*
            let empty: [u64; 0] = [];
            let r_empty = kleene.mem_star(&empty);
            assert(r_empty == true);

            // Single element in alphabet is in Σ*
            let single: [u64; 1] = [1u64];
            let r_single = kleene.mem_star(&single);
            assert(r_single == true);

            // Sequence of alphabet elements is in Σ*
            let seq: [u64; 3] = [1u64, 2u64, 3u64];
            let r_seq = kleene.mem_star(&seq);
            assert(r_seq == true);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] kleene_mem_plus verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap05::KleeneStPer::KleeneStPer::*;
        use apas_verus::SetLit;

        fn test_mem_plus()
            requires valid_key_type::<u64>()
        {
            let alpha: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
            let kleene: KleeneStPer<u64> = KleeneStPer::new(alpha);

            // Empty sequence is NOT in Σ+
            let empty: [u64; 0] = [];
            let r_empty = kleene.mem_plus(&empty);
            assert(r_empty == false);

            // Single element in alphabet is in Σ+
            let single: [u64; 1] = [1u64];
            let r_single = kleene.mem_plus(&single);
            assert(r_single == true);

            // Sequence of alphabet elements is in Σ+
            let seq: [u64; 3] = [1u64, 2u64, 3u64];
            let r_seq = kleene.mem_plus(&seq);
            assert(r_seq == true);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] kleene_alphabet verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap05::KleeneStPer::KleeneStPer::*;
        use apas_verus::SetLit;

        fn test_alphabet()
            requires valid_key_type::<u64>()
        {
            let alpha: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
            let kleene: KleeneStPer<u64> = KleeneStPer::new(alpha);

            let alpha_ref = kleene.alphabet();
            assert(alpha_ref@ == kleene@);
            assert(alpha_ref@.len() == 3);
        }
    } => Ok(())
}
