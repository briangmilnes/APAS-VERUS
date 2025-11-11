pub mod minimal_iter {
    use vstd::prelude::*;
    verus! {
    
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    pub trait Collection {
        type Item;
        type Iter: Iterator<Item = Self::Item>;
        fn iter(&self) -> Self::Iter;
    }

    }
}
