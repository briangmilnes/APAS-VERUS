//! Testing mixed exec/ghost field initialization in traits, i.e., demonic possession of a struct.

pub mod possession {
    use vstd::prelude::*;

    verus! {

        pub struct S { pub ghost a: nat, pub b: i32,}
        
        pub trait STrait: Sized {
            fn new_exorcised(b: i32) -> Self;
            fn new_possessed(b: i32) -> Self;
        }

        impl STrait for S {
            fn new_exorcised (b: i32) -> (s: Self)
                ensures s.a == 0, s.b == b,
            { S { b: 0i32, } }

            fn new_possessed(b: i32) -> (s: Self)
               ensures s.a == 0, s.b == b,
            {
                let ghost a_val = 0nat;
                S { a: a_val, b: b, }
            }

        }

/* 
    impl S {
        pub fn new_direct(b: i32) -> (s: Self)
            ensures
                s.a == 0,
                s.b == b,
        {
            let ghost a_val = 0nat;
            S {
                a: a_val,
                b: b,
            }
        }

        pub fn new_with_proof(b: i32) -> (s: Self)
            ensures
                s.a == 0,
                s.b == b,
        {
            proof {
                let ghost a_val = 0nat;
            }
            let ghost a_val = 0nat;
            S {
                a: a_val,
                b: b,
            }
        }
    }
*/

    } // verus!
}

