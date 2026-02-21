# Experiments: Has Hypothesis / Has Result

Check: first 30 lines contain "hypothesis" / "result" (case-insensitive).

| # | File | Has Hyp | Has Result |
|---|------|:-------:|:----------:|
| 1 | abstract_set_iter.rs | ❌ | ❌ |
| 2 | accept_external_body.rs | ❌ | ✅ |
| 3 | accept.rs | ❌ | ❌ |
| 4 | arc_clone_deref.rs | ❌ | ✅ |
| 5 | arc_rwlock_ninject.rs | ❌ | ✅ |
| 6 | ArrayVal.rs | ❌ | ❌ |
| 7 | ArrayVecSet.rs | ❌ | ❌ |
| 8 | assume_spec_test.rs | ❌ | ✅ |
| 9 | baseviewtypes.rs | ❌ | ✅ |
| 10 | biconditional_spec_fun.rs | ✅ | ✅ |
| 11 | boxing_fns.rs | ❌ | ❌ |
| 12 | checked_comm.rs | ❌ | ✅ |
| 13 | CheckedI32.rs | ❌ | ✅ |
| 14 | checked_signed_int.rs | ❌ | ❌ |
| 15 | checked_u32.rs | ❌ | ❌ |
| 16 | checked_unsigned_int.rs | ❌ | ❌ |
| 17 | clone_fn.rs | ❌ | ❌ |
| 18 | clone_plus.rs | ❌ | ❌ |
| 19 | clone_plus_vs_deep_clone.rs | ✅ | ✅ |
| 20 | clone.rs | ❌ | ❌ |
| 21 | collect2.rs | ✅ | ✅ |
| 22 | collect_deep_view.rs | ❌ | ❌ |
| 23 | collect.rs | ❌ | ❌ |
| 24 | deep_view_2_tuple.rs | ✅ | ✅ |
| 25 | deep_view_struct.rs | ❌ | ❌ |
| 26 | derive_clone_enum_in_verus.rs | ✅ | ✅ |
| 27 | derive_clone_struct_in_verus.rs | ✅ | ✅ |
| 28 | derive_copy_enum_in_verus.rs | ✅ | ✅ |
| 29 | derive_copy_struct_in_verus.rs | ✅ | ✅ |
| 30 | derive_debug_enum_in_verus.rs | ✅ | ✅ |
| 31 | derive_debug_struct_in_verus.rs | ✅ | ✅ |
| 32 | derive_default_enum_in_verus.rs | ✅ | ✅ |
| 33 | derive_default_struct_in_verus.rs | ✅ | ✅ |
| 34 | derive_display_enum_in_verus.rs | ✅ | ✅ |
| 35 | derive_display_struct_in_verus.rs | ✅ | ✅ |
| 36 | derive_eq_enum_in_verus.rs | ✅ | ✅ |
| 37 | derive_eq_struct_in_verus.rs | ✅ | ✅ |
| 38 | derive_hash_enum_in_verus.rs | ✅ | ✅ |
| 39 | derive_hash_struct_in_verus.rs | ✅ | ✅ |
| 40 | derive_ord_enum_in_verus.rs | ✅ | ✅ |
| 41 | derive_ord_struct_in_verus.rs | ✅ | ✅ |
| 42 | derive_partial_eq_enum_in_verus.rs | ✅ | ✅ |
| 43 | derive_partial_eq_struct_in_verus.rs | ✅ | ✅ |
| 44 | derive_partial_ord_enum_in_verus.rs | ✅ | ✅ |
| 45 | derive_partial_ord_struct_in_verus.rs | ✅ | ✅ |
| 46 | eq_rel.rs | ❌ | ❌ |
| 47 | executable_use_of_int.rs | ❌ | ✅ |
| 48 | external_body_accept_hole.rs | ❌ | ✅ |
| 49 | f64_bits_sort.rs | ❌ | ✅ |
| 50 | f64_float_cmp_sort.rs | ❌ | ✅ |
| 51 | f64_sort.rs | ❌ | ✅ |
| 52 | ForFor.rs | ❌ | ❌ |
| 53 | ForLoops.rs | ❌ | ❌ |
| 54 | ghost_type_invariant.rs | ❌ | ✅ |
| 55 | HashCheckedU32.rs | ❌ | ❌ |
| 56 | hash_set_iter.rs | ❌ | ❌ |
| 57 | hash_set_modern_pattern.rs | ❌ | ❌ |
| 58 | hash_set_with_view_plus_loops.rs | ❌ | ❌ |
| 59 | invariant_proof_test.rs | ❌ | ❌ |
| 60 | minimal_iter.rs | ❌ | ❌ |
| 61 | modify_a_ghost_struct.rs | ❌ | ✅ |
| 62 | mut_refs_and_mut_returns.rs | ✅ | ✅ |
| 63 | parapair_closure_ensures.rs | ❌ | ✅ |
| 64 | parapair_move_closure_ensures.rs | ❌ | ✅ |
| 65 | parapair_named_closure.rs | ❌ | ✅ |
| 66 | parapair_toplevel_closure.rs | ❌ | ✅ |
| 67 | pervasives.rs | ❌ | ❌ |
| 68 | possession.rs | ❌ | ❌ |
| 69 | proof_fn_in_trait.rs | ❌ | ✅ |
| 70 | proven_partialeq.rs | ❌ | ✅ |
| 71 | pub_crate_test.rs | ❌ | ❌ |
| 72 | seq_array_equality.rs | ❌ | ✅ |
| 73 | seq_for_basic_proofs.rs | ❌ | ❌ |
| 74 | seq_loop_basic_proofs.rs | ❌ | ❌ |
| 75 | seq_set_exec.rs | ❌ | ❌ |
| 76 | seq_vec_equality.rs | ❌ | ✅ |
| 77 | seq_while_basic_proofs.rs | ❌ | ❌ |
| 78 | set_len_empty_both_ways.rs | ❌ | ❌ |
| 79 | SetLoops.rs | ❌ | ❌ |
| 80 | sigma_pi.rs | ❌ | ❌ |
| 81 | signed_int.rs | ❌ | ❌ |
| 82 | simple_hash_set_iter.rs | ❌ | ❌ |
| 83 | simple_seq_iter.rs | ❌ | ❌ |
| 84 | simple_set_iter.rs | ❌ | ❌ |
| 85 | spec_fun_argument.rs | ✅ | ✅ |
| 86 | spec_loop.rs | ✅ | ✅ |
| 87 | struct_construction_test.rs | ❌ | ❌ |
| 88 | supertrait.rs | ❌ | ❌ |
| 89 | tcb_foul.rs | ❌ | ❌ |
| 90 | test_feq_insertion_sort.rs | ❌ | ❌ |
| 91 | test_feq.rs | ❌ | ✅ |
| 92 | test_test.rs | ❌ | ✅ |
| 93 | test_verify_one_file.rs | ❌ | ❌ |
| 94 | total_ord_gen_axioms.rs | ❌ | ❌ |
| 95 | total_ord_gen.rs | ❌ | ❌ |
| 96 | ToVecProof.rs | ❌ | ✅ |
| 97 | trait_decreases.rs | ❌ | ✅ |
| 98 | triangle.rs | ❌ | ❌ |
| 99 | unsigned_int.rs | ❌ | ❌ |
| 100 | use_proven_partialeq.rs | ❌ | ✅ |
| 101 | vec_clone_in_verus.rs | ❌ | ✅ |
| 102 | vec_filter.rs | ✅ | ✅ |
| 103 | vec_if.rs | ❌ | ❌ |
| 104 | vec_length_while_rust.rs | ❌ | ❌ |
| 105 | vec_length_while_verus.rs | ❌ | ❌ |
| 106 | vec_remove_duplicates.rs | ❌ | ❌ |
| 107 | verus_iterator.rs | ❌ | ✅ |
| 108 | verus_keep_ghost_and_test.rs | ✅ | ✅ |
| 109 | verus_pub_crate_test.rs | ❌ | ✅ |
| 110 | verus_vec_iterator_for_basic_proofs.rs | ❌ | ✅ |
| 111 | verus_vec_iterator_loop_basic_proofs.rs | ❌ | ✅ |
| 112 | verus_vec_iterator.rs | ❌ | ❌ |
| 113 | verus_vec_iterator_while_basic_proofs.rs | ❌ | ✅ |
| 114 | verus_wrapped_iter_loops.rs | ❌ | ❌ |
| 115 | VSTDLoopProofs.rs | ❌ | ❌ |
| 116 | WhileWhile.rs | ❌ | ❌ |

## Summary

| | Count |
|---|--------|
| Has Hyp ✅ | 24 |
| Has Hyp ❌ | 92 |
| Has Result ✅ | 48 |
| Has Result ❌ | 68 |
| Both ✅ | 24 |
| Missing both | 68 |
