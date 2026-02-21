//! Runtime tests: #[derive(Ord)] — actually call cmp, <, >, <=, >=

use std::cmp::Ordering;

use apas_verus::experiments::derive_ord_struct_in_verus::derive_ord_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_ord_enum_in_verus::derive_ord_enum_in_verus::E as EEnum;

#[test]
fn test_ord_struct() {
    let a = SStruct { x: 1 };
    let b = SStruct { x: 2 };
    assert!(a < b);
    assert_eq!(a.cmp(&b), Ordering::Less);
}

#[test]
fn test_ord_enum() {
    let a = EEnum::A;
    let b = EEnum::B(0);
    assert!(a < b);
    assert_eq!(a.cmp(&b), Ordering::Less);
}
