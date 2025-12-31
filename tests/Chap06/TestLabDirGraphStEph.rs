//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
use apas_verus::LabDirGraphStEphLit;
use apas_verus::Types::Types::*;

#[test]
fn test_labdirgraphstephlit_macro_functionality() {
    // Test empty graph creation
    let empty: LabDirGraphStEph<i32, String> = LabDirGraphStEphLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.labeled_arcs().size(), 0);

    // Test graph creation with vertices and arcs
    let with_data: LabDirGraphStEph<i32, String> = LabDirGraphStEphLit!(
        V: [1, 2, 3],
        A: [(1, 2, "arc1".to_string()), (2, 3, "arc2".to_string())]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.labeled_arcs().size(), 2);
}

#[test]
fn test_labelled_dir_graph_empty() {
    let g = LabDirGraphStEph::<i32, &str>::empty();
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_arcs().size(), 0);
    assert_eq!(format!("{g}"), "LabDirGraph(V: Set(0), A: Set(0))");
}

#[test]
fn test_labelled_dir_graph_add_vertex() {
    let mut g = LabDirGraphStEph::<i32, &str>::empty();
    g.add_vertex(1);
    g.add_vertex(2);

    assert_eq!(g.vertices().size(), 2);
    assert!(g.vertices().mem(&1));
    assert!(g.vertices().mem(&2));
    assert_eq!(g.labeled_arcs().size(), 0);
}

#[test]
fn test_labelled_dir_graph_add_labeled_arc() {
    let mut g = LabDirGraphStEph::<i32, &str>::empty();
    g.add_labeled_arc(1, 2, "edge12");
    g.add_labeled_arc(2, 3, "edge23");

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 2);
    assert!(g.has_arc(&1, &2));
    assert!(g.has_arc(&2, &3));
    assert!(!g.has_arc(&3, &1));

    assert_eq!(g.get_arc_label(&1, &2), Some(&"edge12"));
    assert_eq!(g.get_arc_label(&2, &3), Some(&"edge23"));
    assert_eq!(g.get_arc_label(&3, &1), None);
}

#[test]
fn test_labelled_dir_graph_neighbors() {
    let mut g = LabDirGraphStEph::<i32, &str>::empty();
    g.add_labeled_arc(1, 2, "a");
    g.add_labeled_arc(1, 3, "b");
    g.add_labeled_arc(3, 1, "c");

    let out_neighbors_1 = g.out_neighbors(&1);
    assert_eq!(out_neighbors_1.size(), 2);
    assert!(out_neighbors_1.mem(&2));
    assert!(out_neighbors_1.mem(&3));

    let in_neighbors_1 = g.in_neighbors(&1);
    assert_eq!(in_neighbors_1.size(), 1);
    assert!(in_neighbors_1.mem(&3));

    let out_neighbors_2 = g.out_neighbors(&2);
    assert_eq!(out_neighbors_2.size(), 0);

    let in_neighbors_2 = g.in_neighbors(&2);
    assert_eq!(in_neighbors_2.size(), 1);
    assert!(in_neighbors_2.mem(&1));
}

#[test]
fn test_labelled_dir_graph_arcs() {
    let mut g = LabDirGraphStEph::<i32, &str>::empty();
    g.add_labeled_arc(1, 2, "label");
    g.add_labeled_arc(2, 3, "another");

    let arcs = g.arcs();
    assert_eq!(arcs.size(), 2);
    assert!(arcs.mem(&Edge(1, 2)));
    assert!(arcs.mem(&Edge(2, 3)));
}

#[test]
fn test_labelled_dir_graph_macro_empty() {
    let g: LabDirGraphStEph<i32, &str> = LabDirGraphStEphLit!();
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_arcs().size(), 0);
}

#[test]
fn test_labelled_dir_graph_macro_with_data() {
    let g = LabDirGraphStEphLit!(
        V: [1, 2, 3],
        A: [(1, 2, "first"), (2, 3, "second")]
    );

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 2);
    assert!(g.has_arc(&1, &2));
    assert!(g.has_arc(&2, &3));
    assert_eq!(g.get_arc_label(&1, &2), Some(&"first"));
    assert_eq!(g.get_arc_label(&2, &3), Some(&"second"));
}

#[test]
fn test_labelled_dir_graph_different_label_types() {
    // Test with integer labels
    let g1 = LabDirGraphStEphLit!(
        V: ["a", "b"],
        A: [("a", "b", 42)]
    );
    assert_eq!(g1.get_arc_label(&"a", &"b"), Some(&42));

    // Test with string labels
    let g2 = LabDirGraphStEphLit!(
        V: [1, 2],
        A: [(1, 2, "hello")]
    );
    assert_eq!(g2.get_arc_label(&1, &2), Some(&"hello"));
}

#[test]
fn test_labelled_dir_graph_display() {
    let g = LabDirGraphStEphLit!(
        V: [1, 2],
        A: [(1, 2, "test")]
    );
    let display_str = format!("{g}");
    assert!(display_str.contains("LabDirGraph"));
    assert!(display_str.contains("V:"));
    assert!(display_str.contains("A:"));
}

#[test]
fn test_labelled_dir_graph_debug() {
    let g = LabDirGraphStEphLit!(
        V: [1],
        A: [(1, 1, "self")]
    );
    let debug_str = format!("{g:?}");
    assert!(debug_str.contains("LabDirGraph"));
    assert!(debug_str.contains("vertices"));
    assert!(debug_str.contains("labeled_arcs"));
}

#[test]
fn test_labelled_dir_graph_self_loop() {
    let mut g = LabDirGraphStEph::<i32, &str>::empty();
    g.add_labeled_arc(1, 1, "self_loop");

    assert!(g.has_arc(&1, &1));
    assert_eq!(g.get_arc_label(&1, &1), Some(&"self_loop"));

    let out_neighbors = g.out_neighbors(&1);
    assert_eq!(out_neighbors.size(), 1);
    assert!(out_neighbors.mem(&1));

    let in_neighbors = g.in_neighbors(&1);
    assert_eq!(in_neighbors.size(), 1);
    assert!(in_neighbors.mem(&1));
}
