pub mod seq_basics {

    use vstd::prelude::*;
    use crate::experiments::verus_iterator::*;
    use crate::experiments::verus_vec_iterator::*;

verus! {
    //  Seq basics

    // length on int
    pub open spec fn seq_int_length(s: Seq<int>) -> nat { s.len() }

    pub fn fixed_length_int_array(s: [int; 3]) -> (l: usize)
        ensures l == seq_int_length(s@)
    { s.len() }

    pub fn length_int_array(s: &[int]) -> (l: usize)
        ensures l == seq_int_length(s@)
    { s.len() }

    pub fn length_int_vec(s: Vec<int>) -> (l: usize)
        ensures l == seq_int_length(s@)
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


    // Member on int
    pub open spec fn seq_int_member(s: Seq<int>, elt: int) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }

    pub fn int_array_member(s: &[int], elt: int) -> (result: bool)
        ensures result == seq_int_member(s@, elt)
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn int_vec_member(s: Vec<int>, elt: int) -> (result: bool)
        ensures result == seq_int_member(s@, elt)
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return true;
            }
            i += 1;
        }
        false
    }

    // Member on usize
    pub open spec fn seq_usize_member(s: Seq<usize>, elt: usize) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }

    pub fn usize_array_member(s: &[usize], elt: usize) -> (result: bool)
        ensures result == seq_usize_member(s@, elt)
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn usize_vec_member(s: Vec<usize>, elt: usize) -> (result: bool)
        ensures result == seq_usize_member(s@, elt)
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return true;
            }
            i += 1;
        }
        false
    }

    // Member on T
/*
    pub open spec fn seq_t_member<T: Eq>(s: Seq<T>, elt: T) -> bool {
        exists|i: int| 0 <= i < s.len() && s[i] == elt
    }

    pub fn t_array_member<T: Eq>(s: &[T], elt: T) -> (result: bool)
        ensures result == seq_t_member(s@, elt)
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                assert(s@[i as int] == elt);
                return true;
            }
            assert(s@[i as int] != elt);
            i += 1;
        }
        false
    }

    pub fn t_vec_member<T: Eq>(s: Vec<T>, elt: T) -> (result: bool)
        ensures result == seq_t_member(s@, elt)
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                assert(s@[i as int] == elt);
                return true;
            }
            assert(s@[i as int] != elt);
            i += 1;
        }
        false
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

    pub fn int_array_find(s: &[int], elt: int) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn int_vec_find(s: Vec<int>, elt: int) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return Some(i);
            }
            i += 1;
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

    pub fn usize_array_find(s: &[usize], elt: usize) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn usize_vec_find(s: Vec<usize>, elt: usize) -> (result: Option<usize>)
        ensures
            match result {
                Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
                None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
            }
    {
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                forall|j: int| 0 <= j < i ==> s@[j] != elt,
            decreases s@.len() - i,
        {
            if s[i] == elt {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    // sum non-negative
    
    // sum on int
    pub open spec fn seq_int_sum(s: Seq<int>) -> int
        decreases s.len(),
    {
        if s.len() == 0 {
            0
        } else {
            s[0] + seq_int_sum(s.skip(1))
        }
    }

    pub open spec fn seq_int_sum_non_negative(s: Seq<int>) -> bool {
        forall|i: int| 0 <= i <= s.len() ==> seq_int_sum(#[trigger] s.take(i)) >= 0
    }

    // sum on i64
    pub open spec fn seq_i64_sum(s: Seq<i64>) -> int
        decreases s.len(),
    {
        if s.len() == 0 {
            0
        } else {
            s[0] as int + seq_i64_sum(s.skip(1))
        }
    }

    pub open spec fn seq_i64_sum_non_negative(s: Seq<i64>) -> bool {
        forall|i: int| 0 <= i <= s.len() ==> seq_i64_sum(#[trigger] s.take(i)) >= 0
    }

    pub fn i64_array_sum_non_negative(s: &[i64]) -> (result: bool)
        ensures result == seq_i64_sum_non_negative(s@)
    {
        let mut sum: i128 = 0;
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                sum == seq_i64_sum(s@.take(i as int)),
                forall|j: int| 0 <= j <= i ==> seq_i64_sum(#[trigger] s@.take(j)) >= 0,
                i64::MIN <= sum <= i64::MAX * i,
            decreases s@.len() - i,
        {
            assert(s@.take(i as int) =~= s@.take((i + 1) as int).drop_last());
            sum = sum + s[i] as i128;
            if sum < 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    pub fn i64_vec_sum_non_negative(s: Vec<i64>) -> (result: bool)
        ensures result == seq_i64_sum_non_negative(s@)
    {
        let mut sum: i128 = 0;
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                sum == seq_i64_sum(s@.take(i as int)),
                forall|j: int| 0 <= j <= i ==> seq_i64_sum(#[trigger] s@.take(j)) >= 0,
                i64::MIN <= sum <= i64::MAX * i,
            decreases s@.len() - i,
        {
            assert(s@.take(i as int) =~= s@.take((i + 1) as int).drop_last());
            sum = sum + s[i] as i128;
            if sum < 0 {
                return false;
            }
            i += 1;
        }
        true
    }

/*
    // count

    // count on int
    pub open spec fn seq_int_count(s: Seq<int>, elt: int) -> nat 
        decreases s.len()
    {
        if s.len() == 0 {
            0nat
        } else {
            (if s[0] == elt { 1nat } else { 0nat }) + seq_int_count(s.skip(1), elt)
        }
    }

    pub fn int_array_count(s: &[int], elt: int) -> (result: usize)
        ensures result == seq_int_count(s@, elt)
    {
        let mut count: usize = 0;
        let mut i: usize = 0;
        while i < s.len()
            invariant
                i <= s@.len(),
                count as int == seq_int_count(s@.take(i as int), elt),
            decreases s@.len() - i,
        {
            if s[i] == elt {
                count = count.wrapping_add(1);
            }
            i += 1;
        }
        count
    }
*/ 
}
}

