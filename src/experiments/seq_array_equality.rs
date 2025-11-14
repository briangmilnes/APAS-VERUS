#![allow(unused_imports)]
use vstd::prelude::*;

verus! {

pub assume_specification<T: Clone> [<[T]>::to_vec] (s: &[T]) -> (result: Vec<T>)
    ensures
        result@ == s@,
;

pub fn array_copy_equality<T: Copy>(s: &[T]) -> (result: Vec<T>)
    ensures
        result@ == s@,
        result@ =~= s@,
{
    s.to_vec()
}

pub fn array_clone_equality<T: Clone>(s: &[T]) -> (result: Vec<T>)
    ensures
        result@ == s@,
        result@ =~= s@,
{
    s.to_vec()
}

}

