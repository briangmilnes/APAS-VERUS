//! Runtime tests for standards::multi_struct_standard.

use apas_verus::standards::multi_struct_standard::multi_struct_standard::*;

#[test]
fn test_leaf_new() {
    let leaf = Leaf::new(42);
    assert_eq!(leaf.key, 42);
}

#[test]
fn test_leaf_set_key() {
    let mut leaf = Leaf::new(10);
    leaf.set_key(99);
    assert_eq!(leaf.key, 99);
}

#[test]
fn test_interior_new() {
    let left = Box::new(Node::LeafNode(Leaf::new(1)));
    let right = Box::new(Node::LeafNode(Leaf::new(3)));
    let interior = Interior::new(2, Some(left), Some(right));
    assert_eq!(interior.key, 2);
}

#[test]
fn test_interior_set_key() {
    let mut interior = Interior::new(5, None, None);
    interior.set_key(10);
    assert_eq!(interior.key, 10);
}

#[test]
fn test_tree_new_empty() {
    let tree = Tree::new();
    assert!(tree.child.is_none());
}

#[test]
fn test_tree_with_leaf() {
    let leaf = Node::LeafNode(Leaf::new(42));
    let tree = Tree { child: Some(Box::new(leaf)) };
    assert!(tree.child.is_some());
}
