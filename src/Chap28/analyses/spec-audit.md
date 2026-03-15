# Chap28 Spec Audit — Maximum Contiguous Subsequence Sum

## Summary

All 7 StEph algorithm variants have **strong** specifications.

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | MCSSSpec.rs | spec_range_sum | — | recursive sum definition | **strong** (spec) |
| 2 | MCSSSpec.rs | is_mcss_of | — | achieved + maximal (Def 28.4) | **strong** (spec) |
| 3 | MaxContigSubSumBruteStEph.rs | max_contig_sub_sum_brute | sums_fit_i32 | empty→None; non-empty→is_mcss_of | **strong** |
| 4 | MaxContigSubSumOptStEph.rs | max_contig_sub_sum_opt | sums_fit_i32, len < MAX | empty→None; non-empty→is_mcss_of | **strong** |
| 5 | MaxContigSubSumIterStEph.rs | max_contig_sub_sum_iter | sums_fit_i32 | empty→None; non-empty→is_mcss_of | **strong** |
| 6 | MaxContigSubSumDivConStEph.rs | max_contig_sub_sum_divcon | sums_fit_i32 | empty→None; non-empty→is_mcss_of | **strong** |
| 7 | MaxContigSubSumDivConOptStEph.rs | max_contig_sub_sum_divcon_opt | sums_fit_i32 | empty→None; non-empty→is_mcss_of | **strong** |
| 8 | MaxContigSubSumReducedStEph.rs | max_contig_sub_sum_reduced | sums_fit_i32 | empty→None; non-empty→is_mcss_of | **strong** |
| 9 | MaxContigSubSumReducedMcsseStEph.rs | max_contig_sub_sum_reduced_mcsse | sums_fit_i32 | empty→None; non-empty→is_mcss_of | **strong** |

## Notes

- All algorithms ensure two-part MCSS postcondition: (a) result achieved by some range, (b) result maximal.
- Shared spec `is_mcss_of` from MCSSSpec.rs encodes APAS Definition 28.4.
- Overflow precondition `sums_fit_i32` ensures all partial sums within i32 bounds.
- Helper lemmas (prefix_opt, divcon_combine, strength_combine) bridge algorithm internals to MCSS spec.
- Covers APAS Algorithms 28.8, 28.13, 28.14, 28.15, 28.16, 28.17, 28.19.
