use vstd::prelude::*;

verus! {

pub trait GhostIteratorTrait: Sized {
    type Item;
    type Collection;
    type ExecIter;
    
    spec fn exec_invariant(&self, exec_iter: &Self::ExecIter) -> bool;
    spec fn ghost_invariant(&self, init: Option<&Self>) -> bool;
    spec fn ghost_ensures(&self) -> bool;
    spec fn ghost_decrease(&self) -> Option<int>;
    spec fn ghost_peek_next(&self) -> Option<Self::Item>;
    spec fn ghost_advance(&self, exec_iter: &Self::ExecIter) -> Self;
}
    
pub trait ExecIteratorTrait {
    type Item;
    type Collection;
    type GhostIterator: GhostIteratorTrait<Item = Self::Item, Collection = Self::Collection, ExecIter = Self::ExecIterator>;
    type ExecIterator: View<V = Self::GhostIterator>;
    
    fn iter(&self) -> (result: Self::ExecIterator)
        ensures
            result@.exec_invariant(&result);
    
    fn next(it: &mut Self::ExecIterator) -> (result: Option<Self::Item>)
        requires
            old(it)@.exec_invariant(&old(it)),
        ensures
            it@.exec_invariant(it),
            
            match result {
                Some(x) => old(it)@.ghost_peek_next() == Some(x),
                None => old(it)@.ghost_peek_next() is None,
            },
            
            result is Some ==> it@ == old(it)@.ghost_advance(&old(it)),
            
            match (old(it)@.ghost_decrease(), it@.ghost_decrease()) {
                (Some(old_m), Some(new_m)) => new_m <= old_m,
                _ => true,
            };
}

}

