//! APAS-VERUS library crate

pub mod Types;

pub mod vstdplus {
    pub mod TotalOrdered;
    pub mod SetView;
    pub mod SetTrait;
    pub mod HashSetWithViewPlus;
}

pub mod Chap03 {
    pub mod InsertionSortStEph;
}

pub mod Chap05 {
    pub mod SetStEph;
    pub mod SetStEphPlus;
    pub mod RelationStEph;
    // TODO: Apply trusted pattern to MappingStEph
    // pub mod MappingStEph;
}
