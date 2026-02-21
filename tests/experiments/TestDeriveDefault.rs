//! Runtime tests: #[derive(Default)] — actually call default()

use apas_verus::experiments::derive_default_struct_in_verus::derive_default_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_default_enum_in_verus::derive_default_enum_in_verus::E as EEnum;

#[test]
fn test_default_struct() {
    let s = SStruct::default();
    assert_eq!(s.x, 0);
}

#[test]
fn test_default_enum() {
    let e = EEnum::default();
    assert!(matches!(e, EEnum::A));
}
