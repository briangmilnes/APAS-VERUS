//! Runtime tests: #[derive(Debug)] — actually call fmt

use apas_verus::experiments::derive_debug_struct_in_verus::derive_debug_struct_in_verus::S as SStruct;
use apas_verus::experiments::derive_debug_enum_in_verus::derive_debug_enum_in_verus::E as EEnum;

#[test]
fn test_debug_struct() {
    let s = SStruct { x: 42 };
    let out = format!("{:?}", s);
    assert!(out.contains("42"));
}

#[test]
fn test_debug_enum() {
    let e = EEnum::B(99);
    let out = format!("{:?}", e);
    assert!(out.contains("99") || out.contains("B"));
}
