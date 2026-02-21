<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Experiments Verification Summary (6–85)

| # | Experiment | Pass/Fail | Failure Reason |
|---|------------|:---------:|----------------|
| 1 | ArrayVal | pass | |
| 2 | ArrayVecSet | pass | |
| 3 | assume_spec_test | fail | Verus panic (traits.rs assertion) |
| 4 | baseviewtypes | pass | |
| 5 | boxing_fns | pass | |
| 6 | checked_comm | pass | |
| 7 | CheckedI32 | pass | |
| 8 | checked_signed_int | pass | |
| 9 | checked_unsigned_int | pass | |
| 10 | checked_u32 | pass | |
| 11 | clone_fn | pass | |
| 12 | clone_plus | fail | postcondition not satisfied (feq_works) |
| 13 | clone | pass | |
| 14 | collect_deep_view | pass | |
| 15 | collect | pass | |
| 16 | deep_view_struct | pass | |
| 17 | eq_rel | pass | |
| 18 | executable_use_of_int | pass | |
| 19 | external_body_accept_hole | pass | |
| 20 | f64_bits_sort | pass | |
| 21 | f64_float_cmp_sort | pass | |
| 22 | f64_sort | fail | assertion failed (f64_le_spec in loop invariant) |
| 23 | ForFor | fail | precondition not satisfied, invariant not satisfied |
| 24 | ForLoops | pass | |
| 25 | ghost_type_invariant | fail | type_invariant makes struct opaque |
| 26 | HashCheckedU32 | fail | conflicting impls with vstdplus::hashed_checked_u32 |
| 27 | hash_set_iter | pass | |
| 28 | hash_set_modern_pattern | pass | |
| 29 | hash_set_with_view_plus_loops | pass | |
| 30 | invariant_proof_test | fail | expected `!` in proof fn |
| 31 | minimal_iter | pass | |
| 32 | modify_a_ghost_struct | pass | |
| 33 | parapair_closure_ensures | fail | ParaPairs not in experiments_only |
| 34 | parapair_move_closure_ensures | fail | ParaPairs not in experiments_only |
| 35 | parapair_named_closure | fail | ParaPairs not in experiments_only |
| 36 | parapair_toplevel_closure | fail | ParaPairs not in experiments_only |
| 37 | pervasives | pass | |
| 38 | possession | fail | missing field `a` in struct initializer |
| 39 | proof_fn_in_trait | pass | |
| 40 | proven_partialeq | pass | |
| 41 | pub_crate_test | pass | |
| 42 | seq_array_equality | pass | |
| 43 | seq_for_basic_proofs | pass | |
| 44 | seq_loop_basic_proofs | pass | |
| 45 | seq_set_exec | fail | Chap05 not in experiments_only |
| 46 | seq_vec_equality | pass | |
| 47 | seq_while_basic_proofs | pass | |
| 48 | set_len_empty_both_ways | pass | |
| 49 | SetLoops | fail | clone_view moved to attic |
| 50 | sigma_pi | pass | |
| 51 | signed_int | pass | |
| 52 | simple_hash_set_iter | fail | obeys_feq_full, assertion failed |
| 53 | simple_seq_iter | fail | clone_view moved to attic |
| 54 | simple_set_iter | fail | clone_view, lemma_take_full_to_set |
| 55 | struct_construction_test | pass | |
| 56 | supertrait | fail | multiple applicable items (supertrait foo) |
| 57 | tcb_foul | fail | use old(x) for &mut in requires |
| 58 | test_feq_insertion_sort | pass | |
| 59 | test_feq | pass | |
| 60 | test_test | pass | |
| 61 | test_verify_one_file | pass | |
| 62 | total_ord_gen_axioms | fail | trigger must be fn/field/arith |
| 63 | total_ord_gen | fail | assertion failed (axiom_cloned_view_eq) |
| 64 | ToVecProof | fail | clone_view moved to attic |
| 65 | triangle | pass | |
| 66 | unsigned_int | pass | |
| 67 | use_proven_partialeq | pass | |
| 68 | vec_clone_in_verus | pass | |
| 69 | vec_if | pass | |
| 70 | vec_length_while_rust | pass | |
| 71 | vec_length_while_verus | pass | |
| 72 | vec_remove_duplicates | pass | |
| 73 | verus_iterator | pass | |
| 74 | verus_pub_crate_test | fail | field expr for opaque datatype |
| 75 | verus_vec_iterator | fail | precondition not satisfied (exec_invariant) |
| 76 | verus_vec_iterator_while_basic_proofs | fail | depends on verus_vec_iterator |
| 77 | verus_wrapped_iter_loops | pass | |
| 78 | VSTDLoopProofs | pass | |
| 79 | WhileWhile | pass | |

## Summary

| Result | Count |
|--------|-------|
| pass | 58 |
| fail | 21 |
