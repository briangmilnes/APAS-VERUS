# veracity-iter-vs-rec — Transform Prompt

A veracity transformer that renames iterative implementations to `_iter` and installs
delegation wrappers under the original names.

## CLI Interface

```
veracity-iter-vs-rec [OPTIONS] --codebase <CODEBASE>

Options:
  -c, --codebase <CODEBASE>    Root of the APAS-VERUS codebase
  -d, --directory <DIRECTORY>  Process only this chapter directory
  -f, --file <FILE>            Process only this file
  -m, --manifest <MANIFEST>    Path to rename manifest (TOML)
  -n, --dry-run                Show what would change, don't write
  -h, --help                   Print help
```

## Rename Manifest

TOML file listing functions to rename per file. The tool reads this — it does NOT
guess which functions to rename.

```toml
# iter-vs-rec-manifest.toml

[[file]]
path = "src/Chap41/AVLTreeSetStEph.rs"
trait_name = "AVLTreeSetStEphTrait"
functions = ["find", "insert", "delete", "filter", "intersection", "union", "difference"]

[[file]]
path = "src/Chap41/AVLTreeSetStPer.rs"
trait_name = "AVLTreeSetStPerTrait"
functions = ["find", "insert", "delete", "filter", "intersection", "union", "difference"]

[[file]]
path = "src/Chap43/OrderedSetStEph.rs"
trait_name = "OrderedSetStEphTrait"
functions = ["first", "last", "previous", "next", "rank", "split", "get_range", "split_rank"]

[[file]]
path = "src/Chap43/OrderedSetStPer.rs"
trait_name = "OrderedSetStPerTrait"
functions = ["first", "last", "previous", "next", "rank", "split", "get_range", "split_rank"]

[[file]]
path = "src/Chap43/OrderedTableStEph.rs"
trait_name = "OrderedTableStEphTrait"
functions = ["find", "insert", "delete", "first_key", "last_key", "previous_key", "next_key", "rank_key", "split_key", "get_key_range", "split_rank_key"]

[[file]]
path = "src/Chap43/OrderedTableStPer.rs"
trait_name = "OrderedTableStPerTrait"
functions = ["find", "insert", "delete", "first_key", "last_key", "previous_key", "next_key", "rank_key", "split_key", "get_key_range", "split_rank_key"]
```

## Transform Steps (per function)

For each `fn foo(...)` in the manifest:

### Step 1: Parse trait block

Find the trait declaration matching `trait_name`. Locate `fn foo(` in it. Extract the
full signature: return type, requires, ensures, generic params, Ghost params.

### Step 2: Add `fn foo_iter(...)` to trait

Insert immediately after `fn foo(...)` in the trait:
```rust
/// Iterative alternative to `foo`.
fn foo_iter(SAME_PARAMS) -> (SAME_RETURN)
    requires SAME_REQUIRES,
    ensures SAME_ENSURES;
```

Emit: `FILE:LINE:info: ADDED fn foo_iter to trait TRAIT_NAME`

### Step 3: Parse impl block

Find the impl block for `trait_name`. Locate `fn foo(` in it. Extract the full body.

### Step 4: Rename impl body to `fn foo_iter(...)`

Change the function name from `foo` to `foo_iter`. Body, requires, ensures unchanged.

Emit: `FILE:LINE:info: RENAMED fn foo -> fn foo_iter in impl TRAIT_NAME`

### Step 5: Install delegation wrapper

Insert `fn foo(...)` immediately after `fn foo_iter(...)` in the impl:
```rust
fn foo(SAME_PARAMS) -> (SAME_RETURN) {
    self.foo_iter(FORWARDED_ARGS)
}
```

The delegation body forwards all parameters by name. For `&mut self` methods, the
delegation is still `self.foo_iter(args)`. No requires/ensures on the delegation body
— Verus infers them from the trait.

Emit: `FILE:LINE:info: ADDED delegation fn foo -> foo_iter in impl TRAIT_NAME`

### Step 6: Doc comment on `_iter`

Add `/// Iterative alternative to \`foo\`.` on the `_iter` in both trait and impl.
Move the original `///` doc comments (APAS cost spec, etc.) to stay on `fn foo` in the
trait.

Emit: `FILE:LINE:info: ADDED doc comment on fn foo_iter`

## Emacs Buffer Output

All output goes to stdout. Each action is a Rust-style diagnostic that emacs
compilation-mode can parse:

```
src/Chap41/AVLTreeSetStEph.rs:196:info: ADDED fn find_iter to trait AVLTreeSetStEphTrait
src/Chap41/AVLTreeSetStEph.rs:342:info: RENAMED fn find -> fn find_iter in impl AVLTreeSetStEphTrait
src/Chap41/AVLTreeSetStEph.rs:355:info: ADDED delegation fn find -> find_iter in impl AVLTreeSetStEphTrait
src/Chap41/AVLTreeSetStEph.rs:200:info: ADDED fn insert_iter to trait AVLTreeSetStEphTrait
src/Chap41/AVLTreeSetStEph.rs:360:info: RENAMED fn insert -> fn insert_iter in impl AVLTreeSetStEphTrait
src/Chap41/AVLTreeSetStEph.rs:385:info: ADDED delegation fn insert -> insert_iter in impl AVLTreeSetStEphTrait
...
```

Format: `FILE:LINE:info: ADDED|RENAMED DETAIL`

This lets `M-x compile` / `M-x next-error` step through every change.

## Logging

Per-file log written to `src/ChapNN/analyses/veracity-iter-vs-rec.log`:

```
$ veracity-iter-vs-rec -c ~/projects/APAS-VERUS
Full output: src/Chap41/analyses/veracity-iter-vs-rec.log
Full output: src/Chap43/analyses/veracity-iter-vs-rec.log

Summary:
|   # | Chap | File                     | Renamed | Added | Delegations |
|-----|------|--------------------------|---------|-------|-------------|
|   1 |   41 | AVLTreeSetStEph.rs       |       7 |     7 |           7 |
|   2 |   41 | AVLTreeSetStPer.rs       |       7 |     7 |           7 |
|   3 |   43 | OrderedSetStEph.rs       |       8 |     8 |           8 |
|   4 |   43 | OrderedSetStPer.rs       |       8 |     8 |           8 |
|   5 |   43 | OrderedTableStEph.rs     |      11 |    11 |          11 |
|   6 |   43 | OrderedTableStPer.rs     |      11 |    11 |          11 |
|     | TOTAL|                          |      52 |    52 |          52 |
```

## Dry Run

With `--dry-run`, emit all info lines but don't write files.

## Error Cases

- `fn foo` not found in trait → `error: fn foo not found in trait TRAIT_NAME`
- `fn foo` not found in impl → `error: fn foo not found in impl for TRAIT_NAME`
- `fn foo_iter` already exists → `warning: fn foo_iter already exists in TRAIT_NAME, skipping`
- Trait/impl not found → `error: trait TRAIT_NAME not found in FILE`
- Spec parse failure → `error: could not parse requires/ensures for fn foo (LINE)`

## Validation

The tool does NOT run `scripts/validate.sh`. The caller does that. The transform is
purely syntactic — parse, rename, insert. If specs are copied correctly and delegation
args are forwarded correctly, verification will pass with the same count.
