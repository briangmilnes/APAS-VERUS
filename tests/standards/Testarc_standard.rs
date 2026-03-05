//! Runtime tests for standards::arc_standard.

use apas_verus::standards::arc_standard::arc_standard::*;

#[test]
fn test_view_arc_new_and_read() {
    let a = ViewArcS::new(42);
    assert_eq!(a.read_val(), 42);
}

#[test]
fn test_view_arc_display() {
    let a = ViewArcS::new(7);
    assert_eq!(format!("{}", a), "7");
    assert_eq!(format!("{:?}", a), "ViewArcS(SimpleS(7))");
}

#[test]
fn test_deep_view_arc_new_and_len() {
    let items: Vec<u64> = vec![10, 20, 30];
    let a: DeepViewArcS<u64> = DeepViewArcS::new(items);
    assert_eq!(a.read_len(), 3);
}

#[test]
fn test_deep_view_arc_empty() {
    let items: Vec<u64> = vec![];
    let a: DeepViewArcS<u64> = DeepViewArcS::new(items);
    assert_eq!(a.read_len(), 0);
}

#[test]
fn test_deep_view_arc_display() {
    let items: Vec<u64> = vec![1, 2, 3];
    let a: DeepViewArcS<u64> = DeepViewArcS::new(items);
    assert_eq!(format!("{}", a), "[1, 2, 3]");
    assert_eq!(format!("{:?}", a), "DeepViewArcS(CollectionS([1, 2, 3]))");
}
