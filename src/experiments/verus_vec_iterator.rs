// Copyright (c) 2025 Brian G. Milnes
use vstd::prelude::*;

verus! {

// Inlined from verus_iterator so this experiment is self-contained when run alone.
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
        ensures result@.exec_invariant(&result);
    fn next(it: &mut Self::ExecIterator) -> (result: Option<Self::Item>)
        requires old(it)@.exec_invariant(&old(it)),
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

pub struct VecGhostIter {
    pub cur: int,
    pub end: int,
    pub data: Seq<usize>,
}

impl GhostIteratorTrait for VecGhostIter {
    type Item = usize;
    type Collection = VecCollection;
    type ExecIter = VecExecIter;
    
    open spec fn exec_invariant(&self, exec_iter: &VecExecIter) -> bool {
        &&& 0 <= self.cur <= self.end
        &&& self.cur == exec_iter.cur as int
        &&& self.end == exec_iter.data.len() as int
        &&& self.data == exec_iter.data@
    }
    
    open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
        match init {
            Some(init_iter) => {
                &&& self.end == init_iter.end
                &&& init_iter.cur <= self.cur <= self.end
            },
            None => true,
        }
    }
    
    open spec fn ghost_ensures(&self) -> bool {
        self.cur == self.end
    }
    
    open spec fn ghost_decrease(&self) -> Option<int> {
        Some(self.end - self.cur)
    }
    
    open spec fn ghost_peek_next(&self) -> Option<usize> {
        if self.cur < self.end {
            Some(self.data[self.cur])
        } else {
            None
        }
    }
    
    open spec fn ghost_advance(&self, exec_iter: &VecExecIter) -> Self {
        VecGhostIter {
            cur: self.cur + 1,
            end: self.end,
            data: self.data,
        }
    }
}

pub struct VecExecIter {
    pub data: Vec<usize>,
    pub cur: usize,
}

impl View for VecExecIter {
    type V = VecGhostIter;
    
    open spec fn view(&self) -> VecGhostIter {
        VecGhostIter {
            cur: self.cur as int,
            end: self.data.len() as int,
            data: self.data@,
        }
    }
}

pub struct VecCollection {
    pub data: Vec<usize>,
}

impl ExecIteratorTrait for VecCollection {
    type Item = usize;
    type Collection = VecCollection;
    type GhostIterator = VecGhostIter;
    type ExecIterator = VecExecIter;
    
    fn iter(&self) -> (result: Self::ExecIterator)
        ensures
            result@.exec_invariant(&result),
            result@.data == self.data@,
            result@.cur == 0,
            result.cur == 0,
    {
        VecExecIter {
            data: self.data.clone(),
            cur: 0,
        }
    }
    
    fn next(it: &mut Self::ExecIterator) -> (result: Option<Self::Item>)
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
            },
    {
        if it.cur < it.data.len() {
            let val = it.data[it.cur];
            it.cur = it.cur + 1;
            Some(val)
        } else {
            None
        }
    }
}

impl vstd::pervasive::ForLoopGhostIteratorNew for VecExecIter {
    type GhostIter = VecGhostIter;
    
    open spec fn ghost_iter(&self) -> VecGhostIter {
        self@
    }
}

impl vstd::pervasive::ForLoopGhostIterator for VecGhostIter {
    type ExecIter = VecExecIter;
    type Item = usize;
    type Decrease = int;
    
    open spec fn exec_invariant(&self, exec_iter: &VecExecIter) -> bool {
        &&& 0 <= self.cur <= self.end
        &&& self.cur == exec_iter.cur as int
        &&& self.end == exec_iter.data.len() as int
        &&& self.data == exec_iter.data@
    }
    
    open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
        match init {
            Some(init_iter) => {
                &&& self.end == init_iter.end
                &&& self.data == init_iter.data
                &&& init_iter.cur <= self.cur <= self.end
            },
            None => true,
        }
    }
    
    open spec fn ghost_ensures(&self) -> bool {
        self.cur == self.end
    }
    
    open spec fn ghost_decrease(&self) -> Option<int> {
        Some(self.end - self.cur)
    }
    
    open spec fn ghost_peek_next(&self) -> Option<usize> {
        if self.cur < self.end {
            Some(self.data[self.cur])
        } else {
            None
        }
    }
    
    open spec fn ghost_advance(&self, _exec_iter: &VecExecIter) -> VecGhostIter {
        VecGhostIter {
            cur: self.cur + 1,
            end: self.end,
            data: self.data,
        }
    }
}

impl Iterator for VecExecIter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        VecCollection::next(self)
    }
}

}
