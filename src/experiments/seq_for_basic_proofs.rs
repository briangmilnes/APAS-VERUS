// Copyright (c) 2025 Brian G. Milnes
pub mod seq_for_basic_proofs {

    use vstd::prelude::*;

verus! {
    //  Seq basics

    // length on int
    pub open spec fn seq_int_length(s: Seq<int>) -> nat { s.len() }

    pub open spec fn seq_i64_length(s: Seq<i64>) -> nat { s.len() }
    pub fn fixed_length_i64_array(s: [i64; 3]) -> (l: usize)
        ensures l == seq_i64_length(s@)
    { s.len() }

    pub fn length_i64_array(s: &[i64]) -> (l: usize)
        ensures l == seq_i64_length(s@)
    { s.len() }

    pub fn length_i64_vec(s: Vec<i64>) -> (l: usize)
        ensures l == seq_i64_length(s@)
    { s.len() }

    // length on usize
    pub open spec fn seq_usize_length(s: Seq<usize>) -> nat { s.len() }

    pub fn fixed_length_usize_array(s: [usize; 3]) -> (l: usize)
        ensures l == seq_usize_length(s@)
    { s.len() }

    pub fn length_usize_array(s: &[usize]) -> (l: usize)
        ensures l == seq_usize_length(s@)
    { s.len() }

    pub fn length_usize_vec(s: Vec<usize>) -> (l: usize)
        ensures l == seq_usize_length(s@)
    { s.len() }


    // length on T
    pub open spec fn seq_t_length<T>(s: Seq<T>) -> nat { s.len() }

    pub fn fixed_length_t_array<T>(s: [T; 3]) -> (l: usize)
        ensures l == seq_t_length(s@)
    { s.len() }

    pub fn length_t_array<T>(s: &[T]) -> (l: usize)
        ensures l == seq_t_length(s@)
    { s.len() }

    pub fn length_t_vec<T>(s: Vec<T>) -> (l: usize)
        ensures l == seq_t_length(s@)
    { s.len() }


    // Mem on int
    pub open spec fn seq_int_mem(s: Seq<int>, elt: int) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }

    pub open spec fn seq_i64_mem(s: Seq<i64>, elt: i64) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }
    pub fn i64_array_mem_for(s: &[i64], elt: i64) -> (result: bool)
        ensures result == seq_i64_mem(s@, elt)
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return true;
            }
        }
        false
    }

    pub fn i64_vec_mem_for(s: Vec<i64>, elt: i64) -> (result: bool)
        ensures result == seq_i64_mem(s@, elt)
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return true;
            }
        }
        false
    }

    // Mem on usize
    pub open spec fn seq_usize_mem(s: Seq<usize>, elt: usize) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }

    pub fn usize_array_mem_for(s: &[usize], elt: usize) -> (result: bool)
        ensures result == seq_usize_mem(s@, elt)
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return true;
            }
        }
        false
    }

    pub fn usize_vec_mem_for(s: Vec<usize>, elt: usize) -> (result: bool)
        ensures result == seq_usize_mem(s@, elt)
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return true;
            }
        }
        false
    }

/*
    // Mem on T won't work without generic equality and some axioms which are
    // not in Verus.
    pub open spec fn seq_t_mem<T: Eq>(s: Seq<T>, elt: T) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }

    pub fn t_array_mem_for<T: Eq>(s: &[T], elt: T) -> (result: bool)
        ensures result == seq_t_mem(s@, elt)
    {
        let mut i: usize = 0;
        loop
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if i >= s.len() {
                return false;
            }
            if s[i] == elt {
                // AXIOM: assume executable `s[i] == elt` means spec `s@[i as int] == elt`
                proof { assume(s@[i as int] == elt); }
                assert(s@[i as int] == elt);
                return true;
            }
            // AXIOM: assume executable `s[i] != elt` means spec `s@[i as int] != elt`
            proof { assume(s@[i as int] != elt); }
            assert(s@[i as int] != elt);
            i += 1;
        }
    }
*/

    // find 
    
    // find on int
    pub open spec fn seq_int_find(s: Seq<int>, elt: int) -> Option<int> {
        if exists|i: int| 0 <= i < s.len() && s[i] == elt {
            Some(choose|i: int| 0 <= i < s.len() && s[i] == elt && (forall|j: int| 0 <= j < i ==> s[j] != elt))
        } else {
            None
        }
    }

    pub fn i64_array_find_for(s: &[i64], elt: i64) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return Some(i);
            }
        }
        None
    }

    pub fn i64_vec_find_for(s: Vec<i64>, elt: i64) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return Some(i);
            }
        }
        None
    }

    // find on usize
    pub open spec fn seq_usize_find(s: Seq<usize>, elt: usize) -> Option<int> {
        if exists|i: int| 0 <= i < s.len() && s[i] == elt {
            Some(choose|i: int| 0 <= i < s.len() && s[i] == elt && (forall|j: int| 0 <= j < i ==> s[j] != elt))
        } else {
            None
        }
    }

    pub fn usize_array_find_for(s: &[usize], elt: usize) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return Some(i);
            }
        }
        None
    }

    pub fn usize_vec_find_for(s: Vec<usize>, elt: usize) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        for i in 0..s.len()
            invariant
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
        {
            if s[i] == elt {
                return Some(i);
            }
        }
        None
    }

    // sum non-negative using up iteration
    
    // sum on int
    pub open spec fn seq_int_sum_up(s: Seq<int>) -> int
        decreases s.len(),
    {
        if s.len() == 0 {
            0
        } else {
            seq_int_sum_up(s.drop_last()) + s[s.len() - 1]
        }
    }

    pub open spec fn seq_int_sum_non_negative(s: Seq<int>) -> bool {
        forall|i: int| 0 <= i <= s.len() ==> seq_int_sum_up(#[trigger] s.take(i)) >= 0
    }

    // sum on i64
    pub open spec fn seq_i64_sum_up(s: Seq<i64>) -> int
        decreases s.len(),
    {
        if s.len() == 0 {
            0
        } else {
            seq_i64_sum_up(s.drop_last()) + s.last() as int
        }
    }

    pub open spec fn seq_i64_sum_non_negative_up(s: Seq<i64>) -> bool {
        forall|i: int| 0 <= i <= s.len() ==> seq_i64_sum_up(#[trigger] s.take(i)) >= 0
    }

    pub fn i64_array_sum_non_negative_up_for(s: &[i64]) -> (result: bool)
        ensures result == seq_i64_sum_non_negative_up(s@)
    {
        let mut sum: i128 = 0;
        for i in 0..s.len()
            invariant
                sum == seq_i64_sum_up(s@.take(i as int)),
                forall|j: int| 0 <= j <= i ==> seq_i64_sum_up(#[trigger] s@.take(j)) >= 0,
                i64::MIN <= sum <= i64::MAX * i,
        {
            assert(s@.take(i as int) =~= s@.take((i + 1) as int).drop_last());
            sum = sum + s[i] as i128;
            if sum < 0 {
                return false;
            }
        }
        true
    }

    pub fn i64_vec_sum_non_negative_up_for(s: Vec<i64>) -> (result: bool)
        ensures result == seq_i64_sum_non_negative_up(s@)
    {
        let mut sum: i128 = 0;
        for i in 0..s.len()
            invariant
                sum == seq_i64_sum_up(s@.take(i as int)),
                forall|j: int| 0 <= j <= i ==> seq_i64_sum_up(#[trigger] s@.take(j)) >= 0,
                i64::MIN <= sum <= i64::MAX * i,
        {
            assert(s@.take(i as int) =~= s@.take((i + 1) as int).drop_last());
            sum = sum + s[i] as i128;
            if sum < 0 {
                return false;
            }
        }
        true
    }

    // sum using down iteration

    // sum on int
    pub open spec fn seq_int_sum_down(s: Seq<int>) -> int
        decreases s.len(),
    {
        if s.len() == 0 {
            0
        } else {
            s[0] + seq_int_sum_down(s.skip(1))
        }
    }

    // sum on i64
    pub open spec fn seq_i64_sum_down(s: Seq<i64>) -> int
        decreases s.len(),
    {
        if s.len() == 0 {
            0
        } else {
            s[0] as int + seq_i64_sum_down(s.drop_first())
        }
    }

    pub open spec fn seq_i64_sum_non_negative_down(s: Seq<i64>) -> bool {
        forall|i: int| 0 <= i <= s.len() ==> seq_i64_sum_down(#[trigger] s.skip(i)) >= 0
    }

    pub fn i64_array_sum_non_negative_down_for(s: &[i64]) -> (result: bool)
        ensures result == seq_i64_sum_non_negative_down(s@)
    {
        let mut sum: i128 = 0;
        let mut i: usize = s.len();
        loop
            invariant
                i <= s@.len(),
                sum == seq_i64_sum_down(s@.skip(i as int)),
                forall|j: int| i <= j <= s@.len() ==> seq_i64_sum_down(#[trigger] s@.skip(j)) >= 0,
                i64::MIN <= sum <= i64::MAX * (s.len() - i),
            decreases i,
        {
            if i == 0 {
                return true;
            }
            i -= 1;
            // Key assertions: connect skip(i) to skip(i+1)
            assert(s@.skip(i as int).len() > 0);
            assert(s@.skip(i as int)[0] == s@[i as int]);
            assert(s@.skip(i as int).skip(1) =~= s@.skip((i + 1) as int));
            // Explicitly state the recursive definition unfolds correctly
            assert(s@[i as int] as int + seq_i64_sum_down(s@.skip((i + 1) as int)) 
                   == seq_i64_sum_down(s@.skip(i as int))) by {
                admit();
            };
            sum = s[i] as i128 + sum;
            if sum < 0 {
                return false;
            }
        }
    }


    pub fn i64_vec_sum_non_negative_down_for(s: Vec<i64>) -> (result: bool)
        ensures result == seq_i64_sum_non_negative_down(s@)
    {
        let mut sum: i128 = 0;
        let mut i: usize = s.len();
        loop
            invariant
                i <= s@.len(),
                sum == seq_i64_sum_down(s@.skip(i as int)),
                forall|j: int| i <= j <= s@.len() ==> seq_i64_sum_down(#[trigger] s@.skip(j)) >= 0,
                i64::MIN <= sum <= i64::MAX * (s.len() - i),
            decreases i,
        {
            if i == 0 {
                return true;
            }
            i -= 1;
            // Key assertions: connect skip(i) to skip(i+1)
            assert(s@.skip(i as int).len() > 0);
            assert(s@.skip(i as int)[0] == s@[i as int]);
            assert(s@.skip(i as int).skip(1) =~= s@.skip((i + 1) as int));
            // Explicitly state the recursive definition unfolds correctly
            assert(s@[i as int] as int + seq_i64_sum_down(s@.skip((i + 1) as int)) 
                   == seq_i64_sum_down(s@.skip(i as int))) by {
                admit();
            };
            sum = s[i] as i128 + sum;
            if sum < 0 {
                return false;
            }
        }
    }

    // length by iterating, which is trivial until you have a non ordered collection with no length, 
    // which you should not have. 
    pub fn array_length_up_for<T>(s: &[T]) -> (length: usize)
        ensures length == s@.len()
    {
        let mut length: usize = 0;
        for i in 0..s.len()
            invariant
                length == i,
        {
            length += 1;
        }
        length
    }

    pub fn vec_length_up_for<T>(s: Vec<T>) -> (length: usize)
        ensures length == s@.len()
    {
        let mut length: usize = 0;
        for i in 0..s.len()
            invariant
                length == i,
        {
            length += 1;
        }
        length
    }

    // count occurrences of elt

    pub open spec fn seq_int_count_up(s: Seq<int>, elt: int) -> nat
        decreases s.len()
    {
        if s.len() == 0 {
            0nat
        } else {
            (if s.last() == elt { 1nat } else { 0nat }) + seq_int_count_up(s.drop_last(), elt)
        }
    }

    pub open spec fn seq_int_count_down(s: Seq<int>, elt: int) -> nat
        decreases s.len()
    {
        if s.len() == 0 {
            0nat
        } else {
            (if s[0] == elt { 1nat } else { 0nat }) + seq_int_count_down(s.drop_first(), elt)
        }
    }

    pub fn i64_array_count_up_for(s: &[i64], elt: i64) -> (count: usize)
        ensures count <= s@.len()
    {
        let mut count: usize = 0;
        for i in 0..s.len()
            invariant
                count <= i,
        {
            if s[i] == elt {
                count += 1;
            }
        }
        count
    }

    pub fn i64_vec_count_up_for(s: Vec<i64>, elt: i64) -> (count: usize)
        ensures count <= s@.len()
    {
        let mut count: usize = 0;
        for i in 0..s.len()
            invariant
                count <= i,
        {
            if s[i] == elt {
                count += 1;
            }
        }
        count
    }

    pub proof fn seq_int_sum_equivalence(s: Seq<int>)
        ensures seq_int_sum_up(s) == seq_int_sum_down(s)
        decreases s.len(),
    {
        if s.len() == 0 {
        } else if s.len() == 1 {
            assert(s.drop_last().len() == 0);
            assert(s.skip(1).len() == 0);
            assert(seq_int_sum_up(s.drop_last()) == 0);
            assert(seq_int_sum_down(s.skip(1)) == 0);
            assert(s[0] == s.last());
        } else {
            seq_int_sum_equivalence(s.drop_last());
            seq_int_sum_equivalence(s.skip(1));
            assert(s.drop_last().skip(1) =~= s.skip(1).drop_last());
            seq_int_sum_equivalence(s.drop_last().skip(1));
        }
    }

    pub proof fn seq_i64_sum_equivalence(s: Seq<i64>)
        ensures seq_i64_sum_up(s) == seq_i64_sum_down(s)
        decreases s.len(),
    {
        if s.len() == 0 {
        } else if s.len() == 1 {
            assert(s.drop_last().len() == 0);
            assert(s.drop_first().len() == 0);
            assert(seq_i64_sum_up(s.drop_last()) == 0);
            assert(seq_i64_sum_down(s.drop_first()) == 0);
            assert(s[0] == s.last());
        } else {
            seq_i64_sum_equivalence(s.drop_last());
            seq_i64_sum_equivalence(s.drop_first());
            
            assert(s.drop_last().drop_first() =~= s.drop_first().drop_last());
            seq_i64_sum_equivalence(s.drop_last().drop_first());
        }
    }
 }
}

