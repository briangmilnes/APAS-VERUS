// Scratchpad copy of rust_verify_test/tests/standards/Proveprophetic_iterators_standard.rs
// test bodies, with `apas_verus::` replaced by `crate::`, verified through the harness.
use vstd::prelude::*;
use vstd::std_specs::iter::*;
use crate::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

verus! {

fn test_for_borrow_iter() {
    let a: ExampleS<u64> = ExampleS::new(4, 42);
    let ghost orig: Seq<u64> = a@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a.iter()
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == *it.seq()[i],
    {
        collected.push(*x);
    }
    assert(collected@ =~= orig);
}

fn test_for_borrow_into() {
    let a: ExampleS<u64> = ExampleS::new(4, 55);
    let ghost orig: Seq<u64> = a@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: &a
        invariant
            it.seq() == orig.as_ref(),
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == *it.seq()[i],
    {
        collected.push(*x);
    }
    assert(collected@ =~= orig);
}

fn test_for_consume() {
    let a: ExampleS<u64> = ExampleS::new(4, 66);
    let ghost orig: Seq<u64> = a@;
    let mut collected: Vec<u64> = Vec::new();
    for x in it: a.into_iter()
        invariant
            it.seq() == orig,
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == it.seq()[i],
    {
        collected.push(x);
    }
    assert(collected@ =~= orig);
}

fn test_loop_borrow() {
    let a: ExampleS<u64> = ExampleS::new(4, 77);
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
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof {
                    pos = pos + 1;
                    assert(orig[old_pos] == *x);
                }
                collected.push(*x);
            },
            None => {
                assert(pos == orig.len());
                assert(collected@ =~= orig);
                break;
            },
        }
    }
}

fn test_loop_consume() {
    let a: ExampleS<u64> = ExampleS::new(4, 88);
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
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof {
                    pos = pos + 1;
                    assert(orig[old_pos] == x);
                }
                collected.push(x);
            },
            None => {
                assert(pos == orig.len());
                assert(collected@ =~= orig);
                break;
            },
        }
    }
}

fn test_for_custom(n: u64) {
    let mut collected: Vec<u64> = Vec::new();
    for x in it: count_iter(0, n)
        invariant
            collected.len() == it.index(),
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == it.seq()[i],
    {
        collected.push(x);
    }
    assert(collected@ =~= Seq::new(n as nat, |i: int| i as u64));
}

fn test_loop_custom(n: u64) {
    let mut collected: Vec<u64> = Vec::new();
    let ghost orig: Seq<u64> = Seq::new(n as nat, |i: int| i as u64);
    let mut it: CountIter = count_iter(0, n);
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
            forall|i: int| 0 <= i < collected.len()
                ==> #[trigger] collected@[i] == orig[i],
        decreases IteratorSpec::decrease(&it)->0,
    {
        let ghost old_pos = pos;
        match it.next() {
            Some(x) => {
                proof {
                    pos = pos + 1;
                    assert(orig[old_pos] == x);
                }
                collected.push(x);
            },
            None => {
                assert(pos == orig.len());
                assert(collected@ =~= orig);
                break;
            },
        }
    }
}

} // verus!
