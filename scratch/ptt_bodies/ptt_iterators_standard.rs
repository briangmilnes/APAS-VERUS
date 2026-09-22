// Scratchpad copy of Proveiterators_standard.rs test bodies (crate:: paths).
use vstd::prelude::*;
use vstd::std_specs::iter::*;
use crate::standards::iterators_standard::iterators_standard::*;

verus! {

fn test_loop_borrow_iter() {
    let a: ExampleS<u64> = ExampleS::new(3, 42);
    let ghost orig: Seq<u64> = a@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: std::slice::Iter<'_, u64> = a.iter();
    let ghost mut pos: int = 0;
    loop
        invariant
            IteratorSpec::obeys_prophetic_iter_laws(&it),
            IteratorSpec::decrease(&it) is Some,
            0 <= pos <= orig.len(),
            IteratorSpec::remaining(&it).len() == orig.len() - pos,
            forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
            collected.len() == pos,
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof { pos = pos + 1; assert(orig[old_pos] == *x); }
                collected.push(*x);
            },
            None => { assert(pos == orig.len()); assert(collected@ =~= orig); break; },
        }
    }
}

fn test_loop_borrow_into() {
    let a: ExampleS<u64> = ExampleS::new(3, 55);
    let ghost orig: Seq<u64> = a@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: std::slice::Iter<'_, u64> = (&a).into_iter();
    let ghost mut pos: int = 0;
    loop
        invariant
            IteratorSpec::obeys_prophetic_iter_laws(&it),
            IteratorSpec::decrease(&it) is Some,
            0 <= pos <= orig.len(),
            IteratorSpec::remaining(&it).len() == orig.len() - pos,
            forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
            collected.len() == pos,
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof { pos = pos + 1; assert(orig[old_pos] == *x); }
                collected.push(*x);
            },
            None => { assert(pos == orig.len()); assert(collected@ =~= orig); break; },
        }
    }
}

fn test_loop_consume() {
    let a: ExampleS<u64> = ExampleS::new(3, 33);
    let ghost orig: Seq<u64> = a@;
    let mut collected: Vec<u64> = Vec::new();
    let mut it: std::vec::IntoIter<u64> = a.into_iter();
    let ghost mut pos: int = 0;
    loop
        invariant
            IteratorSpec::obeys_prophetic_iter_laws(&it),
            IteratorSpec::decrease(&it) is Some,
            0 <= pos <= orig.len(),
            IteratorSpec::remaining(&it).len() == orig.len() - pos,
            forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
            collected.len() == pos,
            forall|i: int| 0 <= i < collected.len() ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof { pos = pos + 1; assert(orig[old_pos] == x); }
                collected.push(x);
            },
            None => { assert(pos == orig.len()); assert(collected@ =~= orig); break; },
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
