# Agent 1 — Round 20: Prove Chap19 iterate/reduce/scan (24 holes)

## Mission

Chap19 has 24 holes across 3 files (StEph, StPer, MtEph), all from R19's spec
strengthening. Every hole is in iterate/reduce/scan functions. The proofs already
exist in Chap18 — copy and adapt them. This is the highest-leverage task in the
project: Chap19 is the root dependency for 17 downstream chapters.

## Required Reading

- `src/Chap18/ArraySeqStEph.rs` lines 667-780 — **the proven iterate/reduce/scan**.
  This is your template. Read it first.
- `src/standards/using_closures_standard.rs` — Ghost(spec_fn) threading pattern.
- `src/Chap19/ArraySeqStEph.rs` lines 785-863 — current external_body implementations.

## The 24 Holes (8 per file × 3 files)

Each file has the same 5 functions with the same 8 holes:

| # | Function | Hole Type | Fix |
|---|----------|-----------|-----|
| 1 | iterate_iter | external_body | Add Chap18-style while-loop proof |
| 2 | iterate | external_body | Thread Ghost(spec_f) through recursive call |
| 3 | iterate | assume_new() | Replaced by threading Ghost(spec_f) |
| 4 | reduce_iter | external_body | Add Chap18-style while-loop proof |
| 5 | reduce | external_body | Thread Ghost(spec_f) through recursive calls |
| 6 | reduce | assume_new() ×2 | Replaced by threading Ghost(spec_f) |
| 7 | scan | external_body | Add Chap18-style while-loop proof |

Files: `ArraySeqStEph.rs`, `ArraySeqStPer.rs`, `ArraySeqMtEph.rs`

## How to Fix Each Function

### iterate_iter — Copy Chap18's iterate proof

Chap18's `iterate` (line 667) IS the iterative proof. Chap19's `iterate_iter` is
the same algorithm without the proof. Copy the invariants and proof blocks:

```rust
fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A) {
    let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
    let len = a.seq.len();
    let mut acc = seed;
    let mut i: usize = 0;
    while i < len
        invariant
            i <= len,
            len == a.seq@.len(),
            forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
            forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) ==> ret == spec_f(a, t),
            s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
            acc == s.take(i as int).fold_left(seed, spec_f),
        decreases len - i,
    {
        proof {
            a.lemma_spec_index(i as int);
            assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
        }
        acc = f(&acc, &a.seq[i]);
        proof {
            let ghost t = s.take(i as int + 1);
            assert(t.len() > 0);
            assert(t.drop_last() =~= s.take(i as int));
            assert(t.last() == s[i as int]);
            reveal(Seq::fold_left);
        }
        i += 1;
    }
    proof {
        assert(s.take(len as int) =~= s);
    }
    acc
}
```

### iterate (recursive) — Thread Ghost(spec_f)

Replace `Ghost::assume_new()` with `Ghost(spec_f)` in the recursive call:

```rust
fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
    where T: Clone + Eq
{
    let len = a.seq.len();
    if len == 0 {
        seed
    } else {
        let new_seed = f(&seed, &a.seq[0]);
        let tail = Self::subseq(a, 1, len - 1);
        Self::iterate(&tail, f, Ghost(spec_f), new_seed)  // was Ghost::assume_new()
    }
}
```

This may need proof assertions to show the fold_left unrolling. If it doesn't
verify, add proof blocks showing `subseq(1, len-1).fold_left(f(seed, a[0]), spec_f)
== a.fold_left(seed, spec_f)`. Or wrap with external_body if the recursive proof
is too hard — the iterative version is the primary proof target.

### reduce_iter — Same as iterate_iter pattern

Same while-loop with fold_left invariant. The difference: reduce uses `(T, T) -> T`
instead of `(A, T) -> A`, and starts with `id` instead of `seed`.

### reduce (recursive, divide-and-conquer) — Thread Ghost(spec_f)

Replace both `Ghost::assume_new()` calls with `Ghost(spec_f)`:

```rust
let rb = Self::reduce(&b, f, Ghost(spec_f), id.clone());
let rc = Self::reduce(&c, f, Ghost(spec_f), id);
```

This may need lemmas about `fold_left` over split sequences. If the D&C proof is
too hard, wrap with external_body — but try threading Ghost first.

### scan — Copy Chap18's scan proof

Chap18's scan (line 739) has the full proof with pointwise prefix invariant. Copy it.

## Key Differences Between Chap18 and Chap19

Chap18 uses a two-trait structure (`BaseTrait` + `RedefinableTrait`), avoiding cyclic
self-references in ensures. Chap19 uses a single trait (`ArraySeqStEphTrait`), so
agent 3 had to use `a.seq@` directly instead of `a.spec_len()`. Your proofs should
similarly use `a.seq@.len()` and `a.seq@[i]` instead of `a.spec_len()` and
`a.spec_index(i)` to avoid the cycle.

Check whether `a.lemma_spec_index(i)` exists in Chap19. If not, inline the assertion:
`assert(a.seq@[i as int] == a.seq[i as int]@)` or whatever bridge is needed.

## MtEph Specifics

MtEph wraps through RwLock. If the MtEph iterate/reduce/scan functions are
`external_body` wrappers that delegate to the St version, you may be able to prove
them by calling the St method through the lock. If they have independent
implementations, apply the same proof patterns.

## Procedure

1. Read Chap18's proven iterate/reduce/scan (lines 667-780).
2. Read Chap19 StEph's current external_body versions.
3. Fix StEph first — remove external_body, add proofs.
4. `scripts/validate.sh` — fix any errors.
5. Repeat for StPer (should be nearly identical).
6. Repeat for MtEph.
7. `scripts/validate.sh` — 0 errors.

## Important

- Chap18's proofs are the template. Copy them.
- `Ghost(spec_f)` replaces `Ghost::assume_new()`. That's the fix for all assume_new holes.
- If recursive proofs are too hard, keep external_body on the recursive versions but
  prove the `_iter` versions. The `_iter` proofs are straightforward while-loops.
- Do NOT modify any Chap18 files or any non-Chap19 files.
- Do NOT add `assume` or `accept`.

## Deliverables

- 24 holes eliminated (or maximally reduced) across 3 files.
- `plans/agent1-round20-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
