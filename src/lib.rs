//! APAS-VERUS library crate

pub mod Types;

pub mod vstdplus {
    pub mod set;
    pub mod set_with_view;
    pub mod hash_set_with_view;
    pub mod total_order;
    pub mod partial_order;
}

pub mod Chap03 {
    pub mod InsertionSortStEph;
}

pub mod Chap05 {
    // All commented out while testing vstdplus::Set
    // pub mod SetStEph;
    // pub mod SetStEph2;
    // pub mod SetStEphPlus;
    // pub mod RelationStEph;
    // pub mod MappingStEph;
}
