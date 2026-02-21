# Experiments Hypothesis/Result Plan

For each experiment missing Hypothesis or Result:
1. Read the file
2. Add missing Hypothesis (and Result if missing)
3. Uncomment it alone in lib.rs (comment out all other experiments)
4. Validate: `verus --cfg 'feature="experiments_only"'`
5. If validation fails: mark Result = failed, explain why
6. If validation succeeds: run `tests/experiments/TestX` if exists, mark Test = succeeds/fails

## Tracking Table

| # | File | Has Hyp | Has Result | Val Result | Test | Test Result |
|---|------|:-------:|:----------:|------------|------|-------------|
| 1 | abstract_set_iter.rs | ❌ | ❌ | | - | |
| 2 | accept_external_body.rs | ❌ | ✅ | | - | |
| 3 | accept.rs | ❌ | ❌ | | - | |
| 4 | arc_clone_deref.rs | ❌ | ✅ | | - | |
| 5 | arc_rwlock_ninject.rs | ❌ | ✅ | | - | |
| 6 | ArrayVal.rs | ❌ | ❌ | | - | |
| 7 | ArrayVecSet.rs | ❌ | ❌ | | - | |
| 8 | assume_spec_test.rs | ❌ | ✅ | | - | |
| 9 | baseviewtypes.rs | ❌ | ✅ | | - | |
| 10 | boxing_fns.rs | ❌ | ❌ | | - | |
| 11 | checked_comm.rs | ❌ | ✅ | | - | |
| 12 | CheckedI32.rs | ❌ | ✅ | | TestCheckedI32 | |
| 13 | checked_signed_int.rs | ❌ | ❌ | | - | |
| 14 | checked_u32.rs | ❌ | ❌ | | TestCheckedU32 | |
| 15 | checked_unsigned_int.rs | ❌ | ❌ | | - | |
| 16 | clone_fn.rs | ❌ | ❌ | | - | |
| 17 | clone_plus.rs | ❌ | ❌ | | - | |
| 18 | clone.rs | ❌ | ❌ | | - | |
| 19 | collect_deep_view.rs | ❌ | ❌ | | - | |
| 20 | collect.rs | ❌ | ❌ | | - | |
| 21 | deep_view_struct.rs | ❌ | ❌ | | - | |
| 22 | eq_rel.rs | ❌ | ❌ | | - | |
| 23 | executable_use_of_int.rs | ❌ | ✅ | | - | |
| 24 | external_body_accept_hole.rs | ❌ | ✅ | | - | |
| 25 | f64_bits_sort.rs | ❌ | ✅ | | - | |
| 26 | f64_float_cmp_sort.rs | ❌ | ✅ | | - | |
| 27 | f64_sort.rs | ❌ | ✅ | | - | |
| 28 | ForFor.rs | ❌ | ❌ | | - | |
| 29 | ForLoops.rs | ❌ | ❌ | | - | |
| 30 | ghost_type_invariant.rs | ❌ | ✅ | | - | |
| 31 | HashCheckedU32.rs | ❌ | ❌ | | - | |
| 32 | hash_set_iter.rs | ❌ | ❌ | | - | |
| 33 | hash_set_modern_pattern.rs | ❌ | ❌ | | - | |
| 34 | hash_set_with_view_plus_loops.rs | ❌ | ❌ | | - | |
| 35 | invariant_proof_test.rs | ❌ | ❌ | | - | |
| 36 | minimal_iter.rs | ❌ | ❌ | | - | |
| 37 | modify_a_ghost_struct.rs | ❌ | ✅ | | - | |
| 38 | parapair_closure_ensures.rs | ❌ | ✅ | | - | |
| 39 | parapair_move_closure_ensures.rs | ❌ | ✅ | | - | |
| 40 | parapair_named_closure.rs | ❌ | ✅ | | - | |
| 41 | parapair_toplevel_closure.rs | ❌ | ✅ | | - | |
| 42 | pervasives.rs | ❌ | ❌ | | - | |
| 43 | possession.rs | ❌ | ❌ | | - | |
| 44 | proof_fn_in_trait.rs | ❌ | ✅ | | - | |
| 45 | proven_partialeq.rs | ❌ | ✅ | | - | |
| 46 | pub_crate_test.rs | ❌ | ❌ | | - | |
| 47 | seq_for_basic_proofs.rs | ❌ | ❌ | | - | |
| 48 | seq_loop_basic_proofs.rs | ❌ | ❌ | | - | |
| 49 | seq_set_exec.rs | ❌ | ❌ | | - | |
| 50 | seq_while_basic_proofs.rs | ❌ | ❌ | | - | |
| 51 | set_len_empty_both_ways.rs | ❌ | ❌ | | - | |
| 52 | SetLoops.rs | ❌ | ❌ | | - | |
| 53 | sigma_pi.rs | ❌ | ❌ | | - | |
| 54 | signed_int.rs | ❌ | ❌ | | - | |
| 55 | simple_hash_set_iter.rs | ❌ | ❌ | | - | |
| 56 | simple_seq_iter.rs | ❌ | ❌ | | - | |
| 57 | simple_set_iter.rs | ❌ | ❌ | | - | |
| 58 | struct_construction_test.rs | ❌ | ❌ | | - | |
| 59 | supertrait.rs | ❌ | ❌ | | - | |
| 60 | tcb_foul.rs | ❌ | ❌ | | - | |
| 61 | test_feq_insertion_sort.rs | ❌ | ❌ | | - | |
| 62 | test_feq.rs | ❌ | ✅ | | - | |
| 63 | test_test.rs | ❌ | ✅ | | - | |
| 64 | test_verify_one_file.rs | ❌ | ❌ | | - | |
| 65 | total_ord_gen_axioms.rs | ❌ | ❌ | | - | |
| 66 | total_ord_gen.rs | ❌ | ❌ | | - | |
| 67 | ToVecProof.rs | ❌ | ✅ | | - | |
| 68 | trait_decreases.rs | ❌ | ✅ | | - | |
| 69 | triangle.rs | ❌ | ❌ | | - | |
| 70 | unsigned_int.rs | ❌ | ❌ | | - | |
| 71 | use_proven_partialeq.rs | ❌ | ✅ | | - | |
| 72 | vec_clone_in_verus.rs | ❌ | ✅ | | - | |
| 73 | vec_if.rs | ❌ | ❌ | | - | |
| 74 | vec_length_while_rust.rs | ❌ | ❌ | | - | |
| 75 | vec_length_while_verus.rs | ❌ | ❌ | | - | |
| 76 | vec_remove_duplicates.rs | ❌ | ❌ | | - | |
| 77 | verus_iterator.rs | ❌ | ✅ | | - | |
| 78 | verus_pub_crate_test.rs | ❌ | ✅ | | - | |
| 79 | verus_vec_iterator.rs | ❌ | ❌ | | - | |
| 80 | verus_vec_iterator_while_basic_proofs.rs | ❌ | ✅ | | - | |
| 81 | verus_wrapped_iter_loops.rs | ❌ | ❌ | | - | |
| 82 | VSTDLoopProofs.rs | ❌ | ❌ | | - | |
| 83 | WhileWhile.rs | ❌ | ❌ | | - | |

Note: TestChecked* tests use vstdplus::checked_int, not experiments. CheckedI32 is in experiments.
Derive experiments and mut_refs already have both. verus_keep_ghost_and_test has TestVerusKeepGhostAndTest.
