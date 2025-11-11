use vstd::prelude::*;
use crate::experiments::verus_iterator::*;

verus! {

// Ghost state for Vec iterator
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

// Concrete iterator for Vec
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

// Collection wrapper
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
            it@.data == old(it)@.data,
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

}

