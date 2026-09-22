// Scratchpad copy of Provetable_of_contents_standard.rs test bodies (crate:: paths).
use vstd::prelude::*;
use vstd::std_specs::iter::*;
use crate::standards::table_of_contents_standard::table_of_contents_standard::*;

verus! {

fn test_loop_borrow_iter() {
    let a: ExampleS<u64> = ExampleS::new(3, 42);
    let ghost orig: Seq<u64> = a@;
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
    let a: ExampleS<u64> = ExampleS::new(3, 55);
    let ghost orig: Seq<u64> = a@;
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
    let a: ExampleS<u64> = ExampleS::new(3, 33);
    let ghost orig: Seq<u64> = a@;
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
    let a: ExampleS<u64> = ExampleS::new(3, 99);
    let ghost orig: Seq<u64> = a@;
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
    let a: ExampleS<u64> = ExampleS::new(3, 77);
    let ghost orig: Seq<u64> = a@;
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
    let a: ExampleS<u64> = ExampleS::new(3, 66);
    let ghost orig: Seq<u64> = a@;
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
