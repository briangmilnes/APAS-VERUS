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
    // TODO: Fix vstd::string::View name resolution bug in trait
    // pub mod SetStEph;
    pub mod SetStEph2;  // Test SetTraitWithView implementation
    // pub mod SetStEphPlus;  // Temporarily commented to test SetStEph2
    // pub mod RelationStEph;  // Depends on SetStEph
    // TODO: Apply trusted pattern to MappingStEph
    // pub mod MappingStEph;
}
