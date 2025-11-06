//! APAS-VERUS library crate

pub mod Types;

pub mod vstdadditions {
    pub mod TotalOrdered;
}

pub mod experiments {
    pub mod TraitRequiresPropagation;
}

pub mod Chap03 {
    pub mod InsertionSortStEph;
}

pub mod Chap05 {
    pub mod SetStEph;
    // TODO: Re-enable once we implement concrete type impls
    // pub mod MappingStEph;
    // pub mod RelationStEph;
}
