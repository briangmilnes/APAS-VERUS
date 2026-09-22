// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Lemmas relating a sequence's `to_set()` view to `take`, `push`, `map`, and
//! `fold_left`, over vstd's finite `Set` (verus 0.2026.09.13).
//!
//! `Set::fold` unfolds to `ISet::fold` on `to_iset()`; the one private lemma
//! `lemma_no_dup_seq_fold_left_is_set_fold` carries a duplicate-free sequence's
//! `fold_left` across that bridge once, and the twelve public weighted-sum
//! lemmas that Chap06 calls are instances of it. The set-view lemmas that
//! Chap05 and Chap06 call are restated over `Seq::to_set_ensures` and
//! `Seq::lemma_push_to_set_commute`. Only the lemmas some file in Chap02
//! through Chap06 calls are defined here; `seq_set_pre_0913.rs` is the
//! pre-09.13 module kept as a record.
//!
//! These are plain proof functions, not broadcast lemmas: broadcast versions
//! caused matching loops (30M+ rlimit) in the callers.

//  Table of Contents
//	Section 2. imports
//	Section 3. broadcast use
//	Section 6. spec fns
//	Section 7. proof fns

//		Section 2. imports

use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::iset::fold::is_fun_commutative;

verus! {

//		Section 3. broadcast use

broadcast use {
    vstd::seq::group_seq_lemmas,
    vstd::seq_lib::group_seq_properties,
    vstd::set::group_set_lemmas,
    vstd::iset::group_iset_lemmas,
    Seq::to_set_ensures,
};

//		Section 6. spec fns

// Weighted sums: the third component of a triple, summed over a sequence or a set.

/// Sum of the third component over a sequence of triples (u32 weights).
pub open spec fn spec_weighted_seq_sum<A, B>(seq: Seq<(A, B, u32)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u32)| acc + t.2 as nat)
}

/// Sum of the third component over a set of triples (u32 weights).
pub open spec fn spec_weighted_set_sum<A, B>(s: Set<(A, B, u32)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u32)| acc + t.2 as nat)
}

pub open spec fn spec_weighted_seq_sum_u8<A, B>(seq: Seq<(A, B, u8)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u8)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u8<A, B>(s: Set<(A, B, u8)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u8)| acc + t.2 as nat)
}

pub open spec fn spec_weighted_seq_sum_u16<A, B>(seq: Seq<(A, B, u16)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u16)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u16<A, B>(s: Set<(A, B, u16)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u16)| acc + t.2 as nat)
}

pub open spec fn spec_weighted_seq_sum_u64<A, B>(seq: Seq<(A, B, u64)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u64)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u64<A, B>(s: Set<(A, B, u64)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u64)| acc + t.2 as nat)
}

pub open spec fn spec_weighted_seq_sum_u128<A, B>(seq: Seq<(A, B, u128)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, u128)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_u128<A, B>(s: Set<(A, B, u128)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, u128)| acc + t.2 as nat)
}

pub open spec fn spec_weighted_seq_sum_usize<A, B>(seq: Seq<(A, B, usize)>) -> nat {
    seq.fold_left(0nat, |acc: nat, t: (A, B, usize)| acc + t.2 as nat)
}
pub open spec fn spec_weighted_set_sum_usize<A, B>(s: Set<(A, B, usize)>) -> nat {
    s.fold(0nat, |acc: nat, t: (A, B, usize)| acc + t.2 as nat)
}

pub open spec fn spec_signed_weighted_seq_sum_i8<A, B>(seq: Seq<(A, B, i8)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i8)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i8<A, B>(s: Set<(A, B, i8)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i8)| acc + t.2 as int)
}

pub open spec fn spec_signed_weighted_seq_sum_i16<A, B>(seq: Seq<(A, B, i16)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i16)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i16<A, B>(s: Set<(A, B, i16)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i16)| acc + t.2 as int)
}

/// The i32 pair keeps the unsuffixed names its callers use.
pub open spec fn spec_signed_weighted_seq_sum<A, B>(seq: Seq<(A, B, i32)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i32)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum<A, B>(s: Set<(A, B, i32)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i32)| acc + t.2 as int)
}

pub open spec fn spec_signed_weighted_seq_sum_i64<A, B>(seq: Seq<(A, B, i64)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i64)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i64<A, B>(s: Set<(A, B, i64)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i64)| acc + t.2 as int)
}

pub open spec fn spec_signed_weighted_seq_sum_i128<A, B>(seq: Seq<(A, B, i128)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, i128)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_i128<A, B>(s: Set<(A, B, i128)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, i128)| acc + t.2 as int)
}

pub open spec fn spec_signed_weighted_seq_sum_isize<A, B>(seq: Seq<(A, B, isize)>) -> int {
    seq.fold_left(0int, |acc: int, t: (A, B, isize)| acc + t.2 as int)
}
pub open spec fn spec_signed_weighted_set_sum_isize<A, B>(s: Set<(A, B, isize)>) -> int {
    s.fold(0int, |acc: int, t: (A, B, isize)| acc + t.2 as int)
}

//		Section 7. proof fns

// Set views of mapped sequences.

/// A valid index's view is in the mapped sequence's set.
pub proof fn lemma_seq_index_in_map_to_set<T: View>(seq: Seq<T>, i: int)
    requires
        0 <= i < seq.len(),
    ensures
        seq.map(|i: int, k: T| k@).to_set().contains(seq[i]@),
{
    let mapped = seq.map(|i: int, k: T| k@);
    assert(mapped.to_set().contains(mapped[i]));
}

/// An element of the mapped sequence's set is the view of some index.
pub proof fn lemma_map_to_set_contains_index<T: View>(seq: Seq<T>, s: T::V)
    requires
        seq.map(|i: int, k: T| k@).to_set().contains(s),
    ensures
        exists |i: int| #![trigger seq[i]] 0 <= i < seq.len() && s == seq[i]@,
{
    let mapped = seq.map(|i: int, k: T| k@);
    let idx = mapped.lemma_contains_to_index(s);
    assert(mapped[idx] == seq[idx]@);
}

/// - Taking one more element of a mapped sequence inserts that element's view
/// - into the set of the shorter prefix.
pub proof fn lemma_take_one_more_extends_the_seq_set_with_view<T: View>(seq: Seq<T>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n).map(|i: int, k: T| k@).to_set().insert(seq[n]@) == seq.take(n+1).map(|i: int, k: T| k@).to_set(),
{
    let mapped_n = seq.take(n).map(|i: int, k: T| k@);
    let mapped_n_plus_1 = seq.take(n+1).map(|i: int, k: T| k@);
    assert(mapped_n_plus_1 =~= mapped_n.push(seq[n]@));
    mapped_n.lemma_push_to_set_commute(seq[n]@);
}

/// - A duplicate-free sequence whose views are exactly the members of `target`
/// - maps to `target`: the bridge from an iterator's sequence to set equality.
pub proof fn lemma_seq_map_to_set_equality<T: View>(seq: Seq<T>, target: Set<T::V>)
    requires
        seq.no_duplicates(),
        forall|k: T| #![trigger seq.contains(k), target.contains(k@)] seq.contains(k) ==> target.contains(k@),
        forall|kv: T::V| #[trigger] target.contains(kv) ==> exists|k: T| #![trigger seq.contains(k)] seq.contains(k) && k@ == kv,
    ensures
        seq.map(|i: int, k: T| k@).to_set() == target,
{
    let mapped_seq = seq.map(|i: int, k: T| k@);
    let mapped_set = mapped_seq.to_set();

    assert forall |kv: T::V| #[trigger] mapped_set.contains(kv) implies target.contains(kv) by {
        let idx = mapped_seq.lemma_contains_to_index(kv);
        assert(seq.contains(seq[idx]));
    }

    assert forall |kv: T::V| #[trigger] target.contains(kv) implies mapped_set.contains(kv) by {
        let k = choose|k: T| #![trigger seq.contains(k)] seq.contains(k) && k@ == kv;
        let idx = seq.lemma_contains_to_index(k);
        assert(mapped_seq[idx] == seq[idx]@);
    }
    assert(mapped_set =~= target);
}

/// - Intersecting the set of one more mapped element with `s2` either inserts
/// - that element's view (when `s2` holds it) or leaves the intersection as it was.
pub proof fn lemma_take_one_more_intersect<T: View>(seq: Seq<T>, s2: Set<T::V>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        seq.take(n+1).map(|i: int, k: T| k@).to_set().intersect(s2) ==
            if s2.contains(seq[n]@) {
                seq.take(n).map(|i: int, k: T| k@).to_set().intersect(s2).insert(seq[n]@)
            } else {
                seq.take(n).map(|i: int, k: T| k@).to_set().intersect(s2)
            },
{
    let set_n = seq.take(n).map(|i: int, k: T| k@).to_set();
    let set_n_plus_1 = seq.take(n+1).map(|i: int, k: T| k@).to_set();
    lemma_take_one_more_extends_the_seq_set_with_view(seq, n);

    if s2.contains(seq[n]@) {
        assert(set_n_plus_1.intersect(s2) =~= set_n.intersect(s2).insert(seq[n]@));
    } else {
        assert(set_n_plus_1.intersect(s2) =~= set_n.intersect(s2));
    }
}

// Folds: a duplicate-free sequence folds to the same value as its set.

/// - `Set::fold` is `ISet::fold` on `to_iset()`. Induction on the sequence:
/// - `to_set` of a push is an insert, `to_iset` of an insert is an `ISet` insert
/// - (`axiom_make_set`), and `lemma_fold_insert` steps the `ISet` fold.
proof fn lemma_no_dup_seq_fold_left_is_set_fold<A, B>(seq: Seq<A>, z: B, f: spec_fn(B, A) -> B)
    requires
        seq.no_duplicates(),
        is_fun_commutative(f),
    ensures
        seq.fold_left(z, f) == seq.to_set().fold(z, f),
    decreases seq.len(),
{
    if seq.len() == 0 {
        assert(seq.to_set() =~= Set::<A>::empty());
        assert(Set::<A>::empty().to_iset() =~= ISet::<A>::empty());
        vstd::iset::fold::lemma_fold_empty::<A, B>(z, f);
    } else {
        let prefix = seq.drop_last();
        let last = seq.last();
        assert(prefix.no_duplicates());
        assert(!prefix.contains(last)) by {
            if prefix.contains(last) {
                let i = choose|i: int| 0 <= i < prefix.len() && prefix[i] == last;
                assert(seq[i] == seq[seq.len() - 1]);
            }
        }
        lemma_no_dup_seq_fold_left_is_set_fold(prefix, z, f);
        assert(seq =~= prefix.push(last));
        prefix.lemma_push_to_set_commute(last);
        assert(prefix.to_set().to_iset().finite());
        assert(prefix.to_set().to_iset().insert(last).finite());
        assert(seq.to_set().to_iset() == prefix.to_set().to_iset().insert(last));
        vstd::iset::fold::lemma_fold_insert(prefix.to_set().to_iset(), z, f, last);
    }
}

/// - For no-dup sequences representing a set, the sequence fold equals the set
/// - fold for weighted tuples.
pub proof fn lemma_weighted_seq_fold_equals_set_fold<A, B>(seq: Seq<(A, B, u32)>)
    requires
        seq.no_duplicates(),
    ensures
        spec_weighted_seq_sum(seq) == spec_weighted_set_sum(seq.to_set()),
{
    let f = |acc: nat, t: (A, B, u32)| acc + t.2 as nat;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0nat, f);
}

pub proof fn lemma_weighted_seq_fold_equals_set_fold_u8<A, B>(seq: Seq<(A, B, u8)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u8(seq) == spec_weighted_set_sum_u8(seq.to_set()),
{
    let f = |acc: nat, t: (A, B, u8)| acc + t.2 as nat;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0nat, f);
}

pub proof fn lemma_weighted_seq_fold_equals_set_fold_u16<A, B>(seq: Seq<(A, B, u16)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u16(seq) == spec_weighted_set_sum_u16(seq.to_set()),
{
    let f = |acc: nat, t: (A, B, u16)| acc + t.2 as nat;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0nat, f);
}

pub proof fn lemma_weighted_seq_fold_equals_set_fold_u64<A, B>(seq: Seq<(A, B, u64)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u64(seq) == spec_weighted_set_sum_u64(seq.to_set()),
{
    let f = |acc: nat, t: (A, B, u64)| acc + t.2 as nat;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0nat, f);
}

pub proof fn lemma_weighted_seq_fold_equals_set_fold_u128<A, B>(seq: Seq<(A, B, u128)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_u128(seq) == spec_weighted_set_sum_u128(seq.to_set()),
{
    let f = |acc: nat, t: (A, B, u128)| acc + t.2 as nat;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0nat, f);
}

pub proof fn lemma_weighted_seq_fold_equals_set_fold_usize<A, B>(seq: Seq<(A, B, usize)>)
    requires seq.no_duplicates(),
    ensures spec_weighted_seq_sum_usize(seq) == spec_weighted_set_sum_usize(seq.to_set()),
{
    let f = |acc: nat, t: (A, B, usize)| acc + t.2 as nat;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0nat, f);
}

pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i8<A, B>(seq: Seq<(A, B, i8)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i8(seq) == spec_signed_weighted_set_sum_i8(seq.to_set()),
{
    let f = |acc: int, t: (A, B, i8)| acc + t.2 as int;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0int, f);
}

pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i16<A, B>(seq: Seq<(A, B, i16)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i16(seq) == spec_signed_weighted_set_sum_i16(seq.to_set()),
{
    let f = |acc: int, t: (A, B, i16)| acc + t.2 as int;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0int, f);
}

pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold<A, B>(seq: Seq<(A, B, i32)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum(seq) == spec_signed_weighted_set_sum(seq.to_set()),
{
    let f = |acc: int, t: (A, B, i32)| acc + t.2 as int;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0int, f);
}

pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i64<A, B>(seq: Seq<(A, B, i64)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i64(seq) == spec_signed_weighted_set_sum_i64(seq.to_set()),
{
    let f = |acc: int, t: (A, B, i64)| acc + t.2 as int;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0int, f);
}

pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_i128<A, B>(seq: Seq<(A, B, i128)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_i128(seq) == spec_signed_weighted_set_sum_i128(seq.to_set()),
{
    let f = |acc: int, t: (A, B, i128)| acc + t.2 as int;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0int, f);
}

pub proof fn lemma_signed_weighted_seq_fold_equals_set_fold_isize<A, B>(seq: Seq<(A, B, isize)>)
    requires seq.no_duplicates(),
    ensures spec_signed_weighted_seq_sum_isize(seq) == spec_signed_weighted_set_sum_isize(seq.to_set()),
{
    let f = |acc: int, t: (A, B, isize)| acc + t.2 as int;
    assert(is_fun_commutative(f));
    lemma_no_dup_seq_fold_left_is_set_fold(seq, 0int, f);
}

// Folds over a sequence of viewed edges equal the weighted sum of the mapped sequence.

/// - Folding `e@.2` over the elements equals the weighted sum of the
/// - view-mapped sequence.
pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum<T: View<V = (A, B, u32)>, A, B>(seq: Seq<T>)
    ensures
        seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat)
            == spec_weighted_seq_sum(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 {
    } else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

/// - The `int` accumulator and the `nat` accumulator agree when every step
/// - adds a non-negative value.
pub proof fn lemma_fold_left_int_equals_nat_as_int<T: View<V = (A, B, u32)>, A, B>(seq: Seq<T>)
    ensures
        seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat)
            == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() == 0 {
    } else {
        let n = (seq.len() - 1) as int;
        lemma_fold_left_int_equals_nat_as_int::<T, A, B>(seq.take(n));
    }
}

pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u8<T: View<V = (A, B, u8)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u8(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u8::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_fold_left_int_equals_nat_as_int_u8<T: View<V = (A, B, u8)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u8::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}

pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u16<T: View<V = (A, B, u16)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u16(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u16::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_fold_left_int_equals_nat_as_int_u16<T: View<V = (A, B, u16)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u16::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}

pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u64<T: View<V = (A, B, u64)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u64(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u64::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
        assert(view_seq[n] == seq[n]@);
    }
}

pub proof fn lemma_fold_left_int_equals_nat_as_int_u64<T: View<V = (A, B, u64)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u64::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}

pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_u128<T: View<V = (A, B, u128)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_u128(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_u128::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_fold_left_int_equals_nat_as_int_u128<T: View<V = (A, B, u128)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_u128::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}

pub proof fn lemma_seq_fold_left_plus_is_weighted_seq_sum_usize<T: View<V = (A, B, usize)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) == spec_weighted_seq_sum_usize(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_seq_fold_left_plus_is_weighted_seq_sum_usize::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_fold_left_int_equals_nat_as_int_usize<T: View<V = (A, B, usize)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as nat) == seq.fold_left(0nat, |acc: nat, e: T| acc + e@.2 as nat) as int,
    decreases seq.len(),
{
    if seq.len() > 0 { lemma_fold_left_int_equals_nat_as_int_usize::<T, A, B>(seq.take((seq.len() - 1) as int)); }
}

pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i8<T: View<V = (A, B, i8)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i8(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i8::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i16<T: View<V = (A, B, i16)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i16(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i16::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
        assert(view_seq[n] == seq[n]@);
    }
}

pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum<T: View<V = (A, B, i32)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i64<T: View<V = (A, B, i64)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i64(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i64::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i128<T: View<V = (A, B, i128)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_i128(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_i128::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

pub proof fn lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_isize<T: View<V = (A, B, isize)>, A, B>(seq: Seq<T>)
    ensures seq.fold_left(0int, |acc: int, e: T| acc + e@.2 as int) == spec_signed_weighted_seq_sum_isize(seq.map(|_i: int, e: T| e@)),
    decreases seq.len(),
{
    let view_seq = seq.map(|_i: int, e: T| e@);
    if seq.len() == 0 { assert(seq =~= Seq::empty()); assert(view_seq =~= Seq::empty()); }
    else {
        let n = (seq.len() - 1) as int;
        lemma_signed_seq_fold_left_plus_is_weighted_seq_sum_isize::<T, A, B>(seq.take(n));
        assert(seq.take(n).map(|_i: int, e: T| e@) =~= view_seq.take(n));
    }
}

} // verus!
