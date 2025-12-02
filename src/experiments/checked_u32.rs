//! CheckedU32 implementing CheckedUnsignedInt trait.

#[allow(unused_imports)]
use vstd::prelude::*;
use crate::experiments::checked_unsigned_int::CheckedUnsignedInt;

verus! {

pub struct CheckedU32 { i: Ghost<int>, v: Option<u32> }

impl View for CheckedU32 {
    type V = int;

    closed spec fn view(&self) -> int { self.i@ }
}

impl Clone for CheckedU32 {
    fn clone(&self) -> (clone: Self)
        ensures clone@ == self@
    {
        proof { use_type_invariant(self); }
        Self { i: self.i, v: self.v }
    }
}

impl CheckedU32 {
    #[verifier::type_invariant]
    spec fn well_formed(self) -> bool {
        match self.v {
            Some(v) => self.i@ == v as int,
            None => self.i@ > (u32::MAX as int),
        }
    }

    pub closed spec fn spec_new(v: u32) -> CheckedU32 { 
        CheckedU32 { i: Ghost(v as int), v: Some(v) } 
    }

    #[verifier::when_used_as_spec(spec_new)]
    pub fn new(v: u32) -> (checked: Self)
        ensures checked@ == v as int
    {
        Self { i: Ghost(v as int), v: Some(v) }
    }

    pub fn new_overflow(Ghost(i): Ghost<int>) -> (checked: Self)
        requires i > (u32::MAX as int)
        ensures checked@ == i
    {
        Self { i: Ghost(i), v: None }
    }

    pub fn unwrap(&self) -> (value: u32)
        requires Self::spec_in_range(self@)
        ensures value as int == self@
    {
        proof { use_type_invariant(self); }
        self.v.unwrap()
    }
}

impl CheckedUnsignedInt for CheckedU32 {
    open spec fn spec_max() -> nat { u32::MAX as nat }

    fn is_normal(&self) -> (normal: bool)
        ensures normal == Self::spec_in_range(self@)
    {
        proof { use_type_invariant(self); }
        self.v.is_some()
    }

    fn is_overflow(&self) -> (overflow: bool)
        ensures overflow == (self@ > Self::spec_max() as int)
    {
        proof { use_type_invariant(self); }
        self.v.is_none()
    }

    fn from_value(v: u64) -> (checked: Self)
    {
        Self { i: Ghost(v as int), v: Some(v as u32) }
    }

    #[verifier::external_body]
    fn from_int(i: Ghost<int>) -> (checked: Self)
        ensures checked@ == i@
    {
        // Cannot implement without external_body: ghost int cannot be converted to exec u32
        unimplemented!()
    }

    fn to_value(&self) -> (value: u64)
    {
        proof { use_type_invariant(self); }
        self.v.unwrap() as u64
    }

    #[verifier::external_body]
    fn add_checked(&self, other: &Self) -> (sum: Self)
        ensures sum@ == self@ + other@
    {
        let new_i: Ghost<int> = Ghost(self@ + other@);
        match (&self.v, &other.v) {
            (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
            _ => Self { i: new_i, v: None },
        }
    }

    #[verifier::external_body]
    fn mul_checked(&self, other: &Self) -> (product: Self)
        ensures product@ == self@ * other@
    {
        let new_i: Ghost<int> = Ghost(self@ * other@);
        match (&self.v, &other.v) {
            (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
            (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
            (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
            _ => Self { i: new_i, v: None },
        }
    }

    fn clone_checked(&self) -> (clone: Self)
        ensures clone@ == self@
    {
        proof { use_type_invariant(self); }
        Self { i: self.i, v: self.v }
    }
}

} // verus!

