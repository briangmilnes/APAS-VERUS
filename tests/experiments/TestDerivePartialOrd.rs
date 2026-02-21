//! Runtime tests: #[derive(PartialOrd)] — actually call partial_cmp, <, >, <=, >=

use apas_verus::experiments::derive_partial_ord_struct_in_verus::derive_partial_ord_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_partial_ord_enum_in_verus::derive_partial_ord_enum_in_verus::E as EEnum;

#[test]
fn test_partial_ord_struct() {
    let a = SStruct { x: 1 };
    let b = SStruct { x: 2 };
    assert!(a < b);
    assert!(a <= b);
    assert!(b > a);
    assert!(b >= a);
    assert!(a.partial_cmp(&b).unwrap().is_lt());
}

#[test]
fn test_partial_ord_enum() {
    let a = EEnum::A;
    let b = EEnum::B(0);
    assert!(a < b); // A < B variant
    assert!(EEnum::B(1) < EEnum::B(2));
}
