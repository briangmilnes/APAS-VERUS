// Copyright (c) 2025 Brian G. Milnes
pub mod supertrait {
    use vstd::prelude::*;
    verus! {
    pub struct Concrete { pub i: u64 }

    pub trait Base {
        fn foo(&self) -> (r: u64)
            ensures r > 0;
    }

    pub trait Super: Base {
        fn foo_strict(&self) -> (r: u64)
            ensures r > 10;
    }

    impl Base for Concrete {
        fn foo(&self) -> (r: u64)
            ensures r > 0,
        { 5 }
    }

    impl Super for Concrete {
        fn foo_strict(&self) -> (r: u64)
            ensures r > 10,
        { 15 }
    }
    }
}
