//! Experiment: Can Verus verify owned structural mutation on trees?
//!
//! Hypothesis: Rotations and recursive insert-with-rotation can be verified
//! by mutating left/right links directly on owned Box<Node>, using no &mut
//! parameters on recursive helpers.
//!
//! Result: YES. Direct link mutation (node.left.take(), node.left = ...,
//! y.right = Some(x)) on owned Box<Node> verifies. Rotations, recursive
//! insert with conditional rotation, and Option::take() wrapper all work.
//! No &mut parameters needed on recursive helpers — owned Box<Node> suffices.

pub mod tree_mut_structure_updates {

    use vstd::prelude::*;

    verus! {

    struct Node {
        key: u64,
        priority: u64,
        size: usize,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    spec fn spec_size(link: &Option<Box<Node>>) -> nat
        decreases *link,
    {
        match link {
            None => 0nat,
            Some(node) => 1 + spec_size(&node.left) + spec_size(&node.right),
        }
    }

    fn size_link(link: &Option<Box<Node>>) -> (r: usize)
        ensures r as nat == spec_size(link),
        decreases *link,
    {
        match link {
            None => 0,
            Some(node) => {
                let l = size_link(&node.left);
                let r = size_link(&node.right);
                assume(1 + l + r <= usize::MAX);
                1 + l + r
            }
        }
    }

    // Test 1: Update size by mutating the field on an owned Box.
    fn update_size(node: &mut Box<Node>)
    {
        let l = size_link(&node.left);
        let r = size_link(&node.right);
        assume(1 + l + r <= usize::MAX);
        node.size = 1 + l + r;
    }

    // Test 2: Right rotation by mutating links on owned nodes.
    //   x               y
    //  / \             / \
    // y   xr   =>    yl   x
    // / \                / \
    // yl  yr             yr  xr
    fn rotate_right(mut x: Box<Node>) -> (result: Box<Node>)
    {
        if let Some(mut y) = x.left.take() {
            x.left = y.right.take();
            update_size(&mut x);
            y.right = Some(x);
            update_size(&mut y);
            y
        } else {
            x
        }
    }

    // Test 3: Left rotation by mutating links on owned nodes.
    fn rotate_left(mut x: Box<Node>) -> (result: Box<Node>)
    {
        if let Some(mut y) = x.right.take() {
            x.right = y.left.take();
            update_size(&mut x);
            y.left = Some(x);
            update_size(&mut y);
            y
        } else {
            x
        }
    }

    // Test 4: Recursive insert that mutates links and conditionally rotates.
    fn insert_link(link: Option<Box<Node>>, key: u64, priority: u64) -> (result: Option<Box<Node>>)
        decreases link,
    {
        match link {
            None => {
                Some(Box::new(Node { key, priority, size: 1, left: None, right: None }))
            }
            Some(mut node) => {
                if key < node.key {
                    node.left = insert_link(node.left.take(), key, priority);
                    update_size(&mut node);
                    let needs_rotate = match &node.left {
                        Some(l) => l.priority < node.priority,
                        None => false,
                    };
                    if needs_rotate { Some(rotate_right(node)) } else { Some(node) }
                } else if key > node.key {
                    node.right = insert_link(node.right.take(), key, priority);
                    update_size(&mut node);
                    let needs_rotate = match &node.right {
                        Some(r) => r.priority < node.priority,
                        None => false,
                    };
                    if needs_rotate { Some(rotate_left(node)) } else { Some(node) }
                } else {
                    Some(node)
                }
            }
        }
    }

    // Test 5: Wrapper struct using Option::take().
    struct Tree { root: Option<Box<Node>> }

    fn tree_insert(tree: &mut Tree, key: u64, priority: u64) {
        let old_root = tree.root.take();
        tree.root = insert_link(old_root, key, priority);
    }

    } // verus!
}
