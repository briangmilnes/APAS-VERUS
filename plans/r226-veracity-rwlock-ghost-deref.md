# Veracity feedback (r226): RWLOCK_GHOST misses the dereferenced-borrow form

For the veracity maintainer. APAS-VERUS agents do not modify veracity.

## Observation

`scripts/holes.sh src/Chap37/BSTRBMtEph.rs` classifies

```rust
let (mut current, write_handle) = self.root.acquire_write();
proof { assume(self.ghost_root@ == current); }
```

as `structural_false_positive RWLOCK_GHOST`, but classifies the reader form

```rust
let handle = self.root.acquire_read();
let data = handle.borrow();
proof { assume(self.ghost_root@ == *data); }
```

as `assume() [algorithmic]`. Six such assumes are in `BSTRBMtEph.rs` after r226
(`minimum`, `maximum`, `in_order`, `pre_order`, `filter`, `reduce`), reported at
the lines of `src/Chap37/analyses/veracity-review-verus-proof-holes.log`.

## Why they are the same pattern

Both equate the ghost shadow `self.ghost_root@` with the value held inside the
`RwLock`: the writer with the value moved out by `acquire_write`, the reader with
the value borrowed by `handle.borrow()`. The only syntactic difference is the
dereference `*data` of the borrow.

## Request

Treat `assume(self.<ghost_field>@ == *<x>)`, where `<x>` is bound by
`<handle>.borrow()` of a read handle obtained from `acquire_read()`, as
RWLOCK_GHOST, like the writer form.
