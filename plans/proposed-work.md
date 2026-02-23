# Proposed Step 2: Move standalone functions into the trait

1. **Remove `pub(crate)`**: Change `pub(crate) fn insert_at_link` to just `fn insert_at_link`.
2. **Add to `AVLTreeSeq` trait**: Add the signatures for `h_fn`, `size_link_fn`, `update_size_and_height`, `rotate_right_fn`, `rotate_left_fn`, `rebalance_fn`, `insert_at_link`, `nth_link`, `set_link`, `push_inorder`, and `compare_trees` to the `AVLTreeSeq` trait.
3. **Move specs to trait**: Move their `requires` and `ensures` clauses from the implementations into the trait definition.
4. **Move implementations**: Move the actual function bodies (keeping `#[verifier::external_body]` where it currently exists) into the `impl<T: StT> AVLTreeSeq<T> for AVLTreeS<T>` block.

Note: `push_left_iter` is related to the iterator and might need to stay separate or go into an iterator-specific trait/impl, but the core tree manipulation functions will move.
