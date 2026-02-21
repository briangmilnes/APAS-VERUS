//! Runtime tests: #[derive(Copy)] — actually use copy semantics

use apas_verus::experiments::derive_copy_struct_in_verus::derive_copy_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_copy_enum_in_verus::derive_copy_enum_in_verus::E as EEnum;

#[test]
fn test_copy_struct() {
    let s = SStruct { x: 42 };
    let t = s; // copy, not move
    assert_eq!(s.x, 42);
    assert_eq!(t.x, 42);
}

#[test]
fn test_copy_enum() {
    let e = EEnum::B(99);
    let f = e; // copy, not move
    assert!(matches!(e, EEnum::B(99)));
    assert!(matches!(f, EEnum::B(99)));
}
