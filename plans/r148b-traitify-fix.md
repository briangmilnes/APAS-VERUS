# R148b — You barely moved any functions. Fix it. DOT.

## What went wrong

You were told to move top-level free functions into trait methods. Most of you
left them as free functions or made them associated functions (not methods).
The net result: 155 free functions before → 152-155 after. That's 0-3 moved.

## The pattern works. R147 proved it.

`BSTPlainStEph.rs` moved ALL 7 functions including consuming ones (insert_node,
delete_node, delete_min_node). The technique:

```rust
// Before (free function):
fn insert_node<T: TotalOrder>(node: BalBinTree<T>, value: T) -> (inserted: BalBinTree<T>)
    ...
{
    match node {
        BalBinTree::Leaf => { ... }
        BalBinTree::Node(inner) => { ... }
    }
}

// After (trait method):
fn insert_node(self, value: T) -> (inserted: Self)
    ...
{
    let ghost node = self;  // preserve proof references to `node`
    match self {
        BalBinTree::Leaf => { ... }
        BalBinTree::Node(inner) => { ... }
    }
}
```

That's it. `node` → `self`. Add `let ghost node = self;`. Every `match node`
→ `match self`. Recursive calls: `insert_node(left, value)` → `left.insert_node(value)`.

For `&self` methods it's even simpler — no ghost alias needed:
```rust
fn contains_node(&self, target: &T) -> (found: bool) {
    match self {  // was match node
        ...
    }
}
```

## What to do now

Go back to your assigned files. For EVERY free function where the first
parameter is the tree type (`BalBinTree<T>`, `Link<T>`, `Box<Node<T>>`),
make it a trait METHOD with `self`, `&self`, or `&mut self`. NOT an
associated function. NOT `Self::fn_name(arg)`. A METHOD: `arg.fn_name()`.

Specifically:

- `fn foo(node: BalBinTree<T>, ...)` → `fn foo(self, ...)` + `match self`
- `fn foo(node: &BalBinTree<T>, ...)` → `fn foo(&self, ...)` + `match self`
- `fn foo(link: Link<T>, ...)` → `fn foo(self, ...)` + `match self`
- `fn foo(link: &Link<T>, ...)` → `fn foo(&self, ...)` + `match self`
- `fn foo(link: &mut Link<T>, ...)` → `fn foo(&mut self, ...)` + `match self`
- `fn foo(root: Box<Node<T>>, ...)` → `fn foo(self, ...)` on Box<Node<T>>

Rotations, rebalance, splay, flip_colors — ALL of these take the tree as
first arg. They ALL become methods.

The ONLY functions that stay free are ones where the first parameter is NOT
the tree type (e.g., a helper that takes `(key: T, left: Tree, right: Tree)`
with no natural self).

## Proof references

If proof blocks reference the old parameter name (`node`, `link`, `root`),
add `let ghost node = self;` (or `link`, `root`) at the top of the method
body. This costs nothing — it's ghost code, erased at compile time.

## AVL `tree_is_avl` issue (Agent 1)

You said `tree_is_avl` can't appear in trait requires because "the trait's
Self is abstract." Wrong. You're implementing the trait FOR `BalBinTree<T>`.
In the impl block, `Self` IS `BalBinTree<T>`. Use `tree_is_avl(self)` in
the impl's requires. In the trait declaration, use a spec method:
`spec fn is_avl(&self) -> bool;` that the impl defines as
`open spec fn is_avl(&self) -> bool { tree_is_avl(*self) }`.

## Splay associated functions (Agent 3)

You made them `Self::splay(root, target)` — associated functions, not methods.
Change to `root.splay(target)` with `self` as first param. The `Box<Node<T>>`
type can have trait impls.

## AVLTreeSeq node functions (Agent 4)

You said "Node functions stay free because their contracts reference struct
fields." That's not a blocker. The impl knows the concrete type. Struct fields
are accessible in the impl body and in `open spec fn` in the impl. Move them.

## Validate and report the count

After your changes, run:
```bash
grep -c "^    fn \|^    pub fn " src/Chap37/YOUR_FILES.rs
```
Report the count. It should be near zero for your assigned files.

## When done

RCP.
