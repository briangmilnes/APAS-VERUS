//! Runtime tests: #[derive(Clone)] — actually call .clone()

use apas_verus::experiments::derive_clone_struct_in_verus::derive_clone_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_clone_enum_in_verus::derive_clone_enum_in_verus::E as EEnum;

#[test]
fn test_clone_struct() {
    let s = SStruct { x: 42 };
    let t = s.clone();
    assert_eq!(s.x, t.x);
}

#[test]
fn test_clone_enum() {
    let e = EEnum::B(99);
    let f = e.clone();
    assert!(matches!(e, EEnum::B(_)));
    assert!(matches!(f, EEnum::B(99)));
}
