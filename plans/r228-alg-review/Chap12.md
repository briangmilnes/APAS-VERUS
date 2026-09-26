# r228 Alg Analysis Review: Chap12

Reviewer: Claude Opus 5.5, 2026-09-26.

## 1. Textbook cost specifications (`prompts/Chap12.txt`)

| # | Item | Cost stated |
|---|---|---|
| 1 | Def 12.5 compare-and-swap | atomic instruction, no cost |
| 2 | Def 12.6 fetch-and-add | atomic instruction, no cost |
| 3 | Ex 12.1 spin lock from faa | none |
| 4 | Ex 12.2 faa from cas | asks for a comparison, no cost |
| 5 | Ex 12.5 concurrent stack via cas | none |

No function in the chapter has a textbook cost, so every verdict is "no
textbook cost". Costs of CAS loops are stated with r = number of failed CAS
attempts, which is 0 without contention and has no bound under contention
(lock-free, not wait-free). Lock costs use w = busy-wait time.

## 2. Reviewed functions

| # | Chap | File | Function | APAS | Old review | New review | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | 12 | Exercise12_2.rs | fetch_add_cas (T) | none | am. O(1), unbounded | W 1+r, S 1+r | no textbook cost |
| 2 | 12 | Exercise12_2.rs | fetch_add_cas (I) | none | W 1 am., S 1 am. | W 1+r, S 1+r | no textbook cost |
| 3 | 12 | Exercise12_1.rs | new (T) | none | O(1) | W 1, S 1 | no textbook cost |
| 4 | 12 | Exercise12_1.rs | lock (T) | none | am. O(1), unbounded | W 1+w, S 1+w | no tb; not old [1] |
| 5 | 12 | Exercise12_1.rs | unlock (T) | none | O(1) | W 1, S 1 | no textbook cost |
| 6 | 12 | Exercise12_1.rs | with_lock (T) | none | O(1) + action | W 1+w+W(a), S 1+w+S(a) | no textbook cost |
| 7 | 12 | Exercise12_1.rs | new (I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 8 | 12 | Exercise12_1.rs | lock (I) | none | W 1 am., S 1 am. | W 1+w, S 1+w | no tb; not old [1] |
| 9 | 12 | Exercise12_1.rs | unlock (I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 10 | 12 | Exercise12_1.rs | with_lock (I) | none | W 1+W(a), S 1+S(a) | W 1+w+W(a), S 1+w+S(a) | no textbook cost |
| 11 | 12 | Exercise12_1.rs | parallel_increment | none | W it, S it | W it + spin, S it | no textbook cost [2] |
| 12 | 12 | Exercise12_5.rs | new (T) | none | O(1) | W 1, S 1 | no textbook cost |
| 13 | 12 | Exercise12_5.rs | push (T) | none | am. O(1), unbounded | W 1+r, S 1+r | no textbook cost |
| 14 | 12 | Exercise12_5.rs | pop (T) | none | am. O(1), unbounded | W 1+r, S 1+r | no textbook cost |
| 15 | 12 | Exercise12_5.rs | is_empty (T) | none | O(1) | W 1, S 1 | no textbook cost |
| 16 | 12 | Exercise12_5.rs | drain (T) | none | O(n) | W n+r, S n+r | no textbook cost |
| 17 | 12 | Exercise12_5.rs | new (I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 18 | 12 | Exercise12_5.rs | push (I) | none | W 1 am., S 1 am. | W 1+r, S 1+r | no textbook cost |
| 19 | 12 | Exercise12_5.rs | pop (I) | none | W 1 am., S 1 am. | W 1+r, S 1+r | no textbook cost |
| 20 | 12 | Exercise12_5.rs | is_empty (I) | none | W 1, S 1 | W 1, S 1 | no textbook cost |
| 21 | 12 | Exercise12_5.rs | drain (I) | none | W n, S n | W n+r, S n+r | no textbook cost |

Footnotes:

1. `SpinLock::lock` takes a ticket with one `fetch_add`, then spins on
   `turn` until its ticket comes up. The spin executes instructions for the
   whole wait, so the work is proportional to the wait, not amortized O(1).
2. `parallel_increment` runs 4 threads, but every increment happens inside
   the spin lock, so the 4 x iterations critical sections are serialized:
   span O(iterations) and no speedup over one thread.

The CAS-loop old lines ("O(1) amortized; O(contention) worst case") state
the same bound as the new O(1 + r) form, so they are not counted as
mismatches.

## 3. Counts

| # | Measure | Count |
|---|---|---|
| 1 | matches textbook | 0 |
| 2 | does not match textbook | 0 |
| 3 | does not match old analysis | 2 |
| 4 | no textbook cost | 21 |
| 5 | unannotated functions | 9 |
| 6 | malformed annotations | 10 |

Unannotated: `Exercise12_1.rs` `Default::default` and two `fmt`;
`Exercise12_5.rs` `Drop::drop`, `Default::default`, and four `fmt`.

Malformed (old Code-review lines with no `Work O(...), Span O(...)` form):
`Exercise12_2.rs` trait `fetch_add_cas`; `Exercise12_1.rs` trait `new`,
`lock`, `unlock`, `with_lock`; `Exercise12_5.rs` trait `new`, `push`, `pop`,
`is_empty`, `drain`. They were left unchanged and each received a new line.

## 4. Notable findings

- `SpinLock::lock` is a busy-wait; the old "amortized O(1)" is wrong.
- `parallel_increment` is a correctness demonstration: the spin lock
  serializes all work.
- Correctness defect outside the cost review: `ConcurrentStackMt::pop` in
  `Exercise12_5.rs` reads `(*head).next` after loading `head` without any
  reclamation scheme, so a concurrent pop can free `head` first
  (use-after-free), and the CAS on raw pointers is exposed to the ABA
  problem. The file also uses `unsafe` blocks, which the no-unsafe standard
  forbids.
- All ten trait-level old lines in the chapter use a non-standard format.
