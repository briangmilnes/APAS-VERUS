//! Runtime tests: #[derive(Hash)] — actually use in hash context

use std::collections::HashSet;

use apas_verus::experiments::derive_hash_struct_in_verus::derive_hash_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_hash_enum_in_verus::derive_hash_enum_in_verus::E as EEnum;

#[test]
fn test_hash_struct() {
    let mut set = HashSet::new();
    set.insert(SStruct { x: 42 });
    set.insert(SStruct { x: 99 });
    assert_eq!(set.len(), 2);
    assert!(set.contains(&SStruct { x: 42 }));
}

#[test]
fn test_hash_enum() {
    let mut set = HashSet::new();
    set.insert(EEnum::A);
    set.insert(EEnum::B(42));
    assert_eq!(set.len(), 2);
    assert!(set.contains(&EEnum::B(42)));
}
