//! APAS-VERUS library crate

pub mod Types;

pub mod vstdplus {
    pub mod set;
    pub mod set_with_view;
    pub mod hash_set_with_view_plus;
    pub mod total_order;
    pub mod partial_order;
    pub mod clone_view;
}

pub mod Chap03 {
    pub mod InsertionSortStEph;
}

pub mod Chap05 {
    pub mod SetStEph;
    pub mod RelationStEph;
}
