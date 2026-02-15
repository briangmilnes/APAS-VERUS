// Copyright (c) 2025 Brian G. Milnes
//! Hash, Eq, Display, Debug implementations for the macro-generated CheckedU32
//! Also adds the axioms needed for use in hash collections (obeys_key_model, obeys_feq_full)

pub mod hash_checked_u32 {

#[cfg(verus_keep_ghost)]
use {
    vstd::prelude::*,
    vstd::std_specs::hash::obeys_key_model,
};
#[cfg(verus_keep_ghost)]
use {
    crate::vstdplus::checked_nat::checked_nat::*,
    crate::vstdplus::feq::feq::*,
};
#[cfg(verus_keep_ghost)]
use {
    std::hash::{Hash, Hasher},
    std::fmt::{Display, Debug, Formatter},
};

#[cfg(verus_keep_ghost)]
verus! {

broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

// Hash implementation - external_body since Hash trait isn't spec'd
impl Hash for CheckedU32 {
    #[verifier::external_body]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_option().hash(state);
    }
}

// PartialEq inside verus!
impl PartialEq for CheckedU32 {
    #[verifier::external_body]
    fn eq(&self, other: &Self) -> bool {
        self.to_option() == other.to_option()
    }
}

impl Eq for CheckedU32 {}

// Axioms for CheckedU32 to work in hash collections
pub open spec fn CheckedU32_feq_trigger() -> bool { true }

// Veracity: USED
pub broadcast proof fn axiom_CheckedU32_feq()
    requires #[trigger] CheckedU32_feq_trigger()
    ensures obeys_feq_full::<CheckedU32>()
{ admit(); }
// Veracity: USED

pub broadcast proof fn axiom_CheckedU32_key_model()
    requires #[trigger] CheckedU32_feq_trigger()
    ensures obeys_key_model::<CheckedU32>()
{ admit(); }

pub broadcast group group_CheckedU32_axioms {
    axiom_CheckedU32_feq,
    axiom_CheckedU32_key_model,
}

pub open spec fn valid_key_type_CheckedU32() -> bool {
    &&& obeys_key_model::<CheckedU32>()
    &&& obeys_feq_full::<CheckedU32>()
}

} // verus!

// Display and Debug outside verus! since they're pure Rust traits
#[cfg(verus_keep_ghost)]
impl Display for CheckedU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.to_option() {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "OVERFLOW"),
        }
    }
}

#[cfg(verus_keep_ghost)]
impl Debug for CheckedU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.to_option() {
            Some(v) => write!(f, "CheckedU32({})", v),
            None => write!(f, "CheckedU32(OVERFLOW)"),
        }
    }
}
}
