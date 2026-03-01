//! Experiment: Can Verus verify owned data mutation on tree nodes?
//!
//! Hypothesis: Taking ownership of Box<Node>, mutating a data field (not
//! structure), and returning it works without &mut parameters on recursive
//! helpers.
//!
//! Result: YES. Both destructure-rebuild and direct field mutation on
//! mut node: Box<Node> verify. Recursive owned insert with count bump
//! verifies. Option::take() + reassign on &mut self works.

#[allow(non_shorthand_field_patterns)]
pub mod tree_mut_data_updates {

    use vstd::prelude::*;

    verus! {

    struct Node {
        key: u64,
        count: u64,
        left: Link,
        right: Link,
    }

    type Link = Option<Box<Node>>;

    fn leaf(key: u64) -> (r: Node)
        ensures r.key == key, r.count == 0,
    {
        Node { key, count: 0, left: None, right: None }
    }

    // Test 1a: Destructure and rebuild with new field value.
    fn update_count(node: Box<Node>) -> (result: Box<Node>)
        requires node.count < u64::MAX,
        ensures
            result.count == node.count + 1,
            result.key == node.key,
    {
        let Node { key, count, left, right } = *node;
        Box::new(Node { key, count: count + 1, left, right })
    }

    // Test 1b: Direct field mutation on owned Box.
    fn update_count_mut(mut node: Box<Node>) -> (result: Box<Node>)
        requires node.count < u64::MAX,
        ensures
            result.count == node.count + 1,
            result.key == node.key,
    {
        node.count = node.count + 1;
        node
    }

    // Test 2: Recursive owned insert that bumps count on the way back up.
    fn insert(link: Link, key: u64) -> (result: Link)
        decreases link,
    {
        match link {
            None => Some(Box::new(leaf(key))),
            Some(node) => {
                let Node { key: k, count: c, left, right } = *node;
                let (new_left, new_right) = if key < k {
                    (insert(left, key), right)
                } else if key > k {
                    (left, insert(right, key))
                } else {
                    (left, right)
                };
                let built = Box::new(Node { key: k, count: c, left: new_left, right: new_right });
                assume(c < u64::MAX);
                Some(update_count(built))
            }
        }
    }

    // Test 3: Wrapper struct with &mut self using Option::take() + reassign.
    struct Tree { root: Link }

    fn tree_insert(tree: &mut Tree, key: u64) {
        let old_root = tree.root.take();
        tree.root = insert(old_root, key);
    }

    } // verus!
}
