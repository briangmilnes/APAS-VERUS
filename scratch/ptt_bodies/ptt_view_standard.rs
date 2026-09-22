// Scratchpad copy of Proveview_standard.rs test bodies (crate:: paths).
use vstd::prelude::*;
use vstd::std_specs::iter::*;
use crate::standards::view_standard::view_standard::*;

verus! {

fn test_simple_view() {
    let s = SimpleS::new(42);
    assert(s@ == 42);
}

fn test_collection_view() {
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(20);
    let c = CollectionS { seq: v };
    assert(c@.len() == 2);
    assert(c@[0] == 10);
    assert(c@[1] == 20);
}

fn three() -> (a: CollectionS<u64>)
    ensures a.seq@.len() == 3,
{
    let mut v: Vec<u64> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    CollectionS { seq: v }
}

fn test_loop_borrow_iter() {
    let a = three();
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: VerusForLoopWrapper<std::slice::Iter<'_, u64>> = VerusForLoopWrapper::new(a.iter());
    loop
        invariant
            it.wf(),
            IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
            IteratorSpec::decrease(&it.iter) is Some,
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
        decreases IteratorSpec::decrease(&it.iter)->0,
    {
        match it.next() {
            Some(x) => { collected.push(*x); },
            None => { assert(collected@ =~= orig); break; },
        }
    }
}

fn test_loop_borrow_into() {
    let a = three();
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: VerusForLoopWrapper<std::slice::Iter<'_, u64>> = VerusForLoopWrapper::new((&a).into_iter());
    loop
        invariant
            it.wf(),
            IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
            IteratorSpec::decrease(&it.iter) is Some,
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
        decreases IteratorSpec::decrease(&it.iter)->0,
    {
        match it.next() {
            Some(x) => { collected.push(*x); },
            None => { assert(collected@ =~= orig); break; },
        }
    }
}

fn test_loop_consume() {
    let a = three();
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: VerusForLoopWrapper<std::vec::IntoIter<u64>> = VerusForLoopWrapper::new(a.into_iter());
    loop
        invariant
            it.wf(),
            IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
            IteratorSpec::decrease(&it.iter) is Some,
            it.seq() == orig,
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
        decreases IteratorSpec::decrease(&it.iter)->0,
    {
        match it.next() {
            Some(x) => { collected.push(x); },
            None => { assert(collected@ =~= orig); break; },
        }
    }
}

fn test_for_borrow_iter() {
    let a = three();
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a.iter()
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
    {
        collected.push(*x);
    }
    assert(collected@ =~= orig);
}

fn test_for_borrow_into() {
    let a = three();
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: (&a).into_iter()
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == *it.seq()[i],
    {
        collected.push(*x);
    }
    assert(collected@ =~= orig);
}

fn test_for_consume() {
    let a = three();
    let ghost orig: Seq<u64> = a.seq@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a.into_iter()
        invariant
            it.seq() == orig,
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == it.seq()[i],
    {
        collected.push(x);
    }
    assert(collected@ =~= orig);
}

} // verus!
