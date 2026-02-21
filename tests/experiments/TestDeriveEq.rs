//! Runtime tests: #[derive(Eq)] — actually call == and !=

use apas_verus::experiments::derive_eq_struct_in_verus::derive_eq_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_eq_enum_in_verus::derive_eq_enum_in_verus::E as EEnum;

#[test]
fn test_eq_struct() {
    let a = SStruct { x: 42 };
    let b = SStruct { x: 42 };
    let c = SStruct { x: 99 };
    assert!(a == b);
    assert!(a != c);
}

#[test]
fn test_eq_enum() {
    let a = EEnum::A;
    let b = EEnum::A;
    let c = EEnum::B(42);
    assert!(a == b);
    assert!(a != c);
    assert!(EEnum::B(1) == EEnum::B(1));
    assert!(EEnum::B(1) != EEnum::B(2));
}
