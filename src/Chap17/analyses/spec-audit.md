# Chap17 Spec Audit — MathSeq

## Summary

18 of 22 functions **strong**, 2 **partial**, 1 **weak**, 1 **missing** (iter_mut outside verus!).

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | MathSeq.rs | new | — | len == length, all elements cloned from init | **strong** |
| 2 | MathSeq.rs | set | — | success: index valid, element set, rest unchanged; fail: unchanged | **strong** |
| 3 | MathSeq.rs | length | — | len == spec_len() | **strong** |
| 4 | MathSeq.rs | nth | index < len | elem@ == self@[index] | **strong** |
| 5 | MathSeq.rs | empty | — | len == 0 | **strong** |
| 6 | MathSeq.rs | singleton | — | len == 1, [0] == item@ | **strong** |
| 7 | MathSeq.rs | add_last | — | len+1, last == value@, prefix preserved | **strong** |
| 8 | MathSeq.rs | delete_last | — | empty→None; non-empty→Some with last, len-1, prefix | **strong** |
| 9 | MathSeq.rs | is_empty | — | emptiness == spec_is_empty() | **strong** |
| 10 | MathSeq.rs | is_singleton | — | singularity == spec_is_singleton() | **strong** |
| 11 | MathSeq.rs | from_vec | — | spec_seq() == data@ | **strong** |
| 12 | MathSeq.rs | with_len | — | len == length, all elements cloned | **strong** |
| 13 | MathSeq.rs | subseq | — | result@ == spec_seq().subrange(clamped) | **strong** |
| 14 | MathSeq.rs | subseq_copy | start+len <= seq.len | result.spec_seq() == spec_seq().subrange(...) | **strong** |
| 15 | MathSeq.rs | domain | — | len == spec_len(), forall i: domain[i] == i | **strong** (R20 fix) |
| 16 | MathSeq.rs | range | valid_key_type | len <= seq.len, no_duplicates | **partial** |
| 17 | MathSeq.rs | multiset_range | valid_key_type, injectivity | len <= seq.len | **weak** |
| 18 | MathSeq.rs | iter | — | it@.0 == 0, it@.1 == spec_seq() | **strong** |
| 19 | MathSeq.rs | next (Iterator) | — | None/Some with index advance, element correct | **strong** |
| 20 | MathSeq.rs | clone | — | cloned@ == self@ | **strong** |
| 21 | MathSeq.rs | eq | — | equal == (self@ == other@) | **strong** |
| 22 | MathSeq.rs | iter_mut | — | (none) | **missing** |

## R20 Fixes Applied

- **domain**: Strengthened from `len == spec_len()` to also ensure `forall|i| domain@[i] == i as usize`. Loop invariant updated. Now **strong**.

## Remaining Gaps

- **range**: Missing membership spec. Should ensure: every range element is from the sequence and every sequence element appears in range. Would need loop invariant connecting `seen` set to processed input elements.
- **multiset_range**: Missing count semantics. Should ensure: each pair (count, elem) represents an element and its multiplicity. Would need substantial loop invariant work.
- **iter_mut**: Outside verus!, no spec possible. Acceptable limitation.
