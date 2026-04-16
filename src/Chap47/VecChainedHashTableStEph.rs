// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Vec Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses Vec for separate chaining collision resolution.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 7. proof fns/broadcast groups
//	Section 9. impls
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod VecChainedHashTableStEph {

    //		Section 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger, lemma_reveal_view_injective};

    verus! 
{

    //		Section 3. broadcast use


        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4. type definitions


        /// Vec Chained Hash Table implementation.
        pub struct VecChainedHashTableStEph;

    //		Section 7. proof fns/broadcast groups


        proof fn _vec_chained_hash_table_verified() {}

    //		Section 9. impls


        /// Clones a Vec<(Key, Value)> with sequence equality ensures.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear copy of all pairs.
        fn clone_vec_pairs<Key: Eq + Clone, Value: Eq + Clone>(pairs: &Vec<(Key, Value)>) -> (cloned: Vec<(Key, Value)>)
            requires
                obeys_feq_clone::<Key>(),
                obeys_feq_clone::<Value>(),
            ensures cloned@ =~= pairs@,
        {
            let mut new_vec: Vec<(Key, Value)> = Vec::new();
            let mut i: usize = 0;
            while i < pairs.len()
                invariant
                    i <= pairs@.len(),
                    new_vec@.len() == i as int,
                    forall |j: int| 0 <= j < i as int
                        ==> #[trigger] new_vec@[j] == pairs@[j],
                    obeys_feq_clone::<Key>(),
                    obeys_feq_clone::<Value>(),
                decreases pairs.len() - i,
            {
                let k = clone_elem(&pairs[i].0);
                let v = clone_elem(&pairs[i].1);
                new_vec.push((k, v));
                i += 1;
            }
            new_vec
        }


        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for Vec<(Key, Value)> {
            open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
                spec_seq_pairs_to_map(self@)
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty Vec construction.
            fn new() -> (entry: Self)
                ensures entry@.len() == 0,
            { Vec::new() }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst case
            fn insert(&mut self, key: Key, value: Value)
                ensures
                    self@.len() >= 1,
                    old(self)@.len() <= self@.len(),
                    self@.len() <= old(self)@.len() + 1,
                    self@.last() == (key, value),
                    self.spec_entry_to_map()[key] == value,
            {
                let mut i: usize = 0;
                while i < self.len()
                    invariant
                        i <= self@.len(),
                        self@ == old(self)@,
                    decreases self.len() - i,
                {
                    if self[i].0 == key {
                        self.remove(i);
                        self.push((key, value));
                        return;
                    }
                    i += 1;
                }
                self.push((key, value));
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>) {
                let mut i: usize = 0;
                while i < self.len()
                    decreases self.len() - i,
                {
                    if self[i].0 == *key {
                        return Some(self[i].1.clone());
                    }
                    i += 1;
                }
                None
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan + Vec::remove (shifts elements), n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: bool)
                ensures
                    !deleted ==> self@ == old(self)@,
                    deleted ==> self@.len() + 1 == old(self)@.len(),
            {
                let mut i: usize = 0;
                while i < self.len()
                    invariant
                        i <= self@.len(),
                        self@ == old(self)@,
                    decreases self.len() - i,
                {
                    if self[i].0 == *key {
                        self.remove(i);
                        return true;
                    }
                    i += 1;
                }
                false
            }

            /// Element-wise clone avoiding Verus tuple-Clone limitation.
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear copy of all pairs.
            fn clone_entry(&self) -> (cloned: Self) {
                let mut new_vec: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < self.len()
                    invariant
                        i <= self@.len(),
                    decreases self.len() - i,
                {
                    new_vec.push((self[i].0.clone(), self[i].1.clone()));
                    i += 1;
                }
                new_vec
            }
        }


        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, Vec<(Key, Value)>, Metrics, H>
            for VecChainedHashTableStEph
        {
            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst
            fn insert(table: &mut HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: Key, value: Value) {
                let index = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let mut bucket = clone_vec_pairs(&table.table[index]);
                let ghost original = bucket@;

                let bucket_len = bucket.len();
                let mut existed = false;
                let mut found_idx: usize = 0;
                let mut scan_i: usize = 0;
                while scan_i < bucket_len
                    invariant
                        scan_i <= bucket_len,
                        bucket_len == original.len(),
                        bucket@ == original,
                        table.table@ == old_table,
                        table.current_size == old(table).current_size,
                        table.num_elements == old(table).num_elements,
                        index < table.table@.len(),
                        !existed ==> forall |j: int| 0 <= j < scan_i as int
                            ==> (#[trigger] original[j]).0 != key,
                        existed ==> found_idx < bucket_len
                            && original[found_idx as int].0 == key,
                    decreases bucket_len - scan_i,
                {
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED proof block
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&bucket[scan_i].0, &key);
                    if eq {
                        existed = true;
                        found_idx = scan_i;
                        break;
                    }
                    scan_i += 1;
                }

                if existed {
                    bucket.remove(found_idx);
                }
                let ghost pre_push = bucket@;
                bucket.push((key, value));

                let ghost new_bucket_seq = bucket@;

                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    if existed {
                        lemma_seq_pairs_remove_key_then_push::<Key, Value>(
                            original, found_idx as int, key, value);
                    } else {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(new_bucket_seq.drop_last() =~= original);
                    }
                }

                table.table.set(index, bucket);
// Veracity: NEEDED proof block

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().insert(key, value));

                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(key) by {}

                    lemma_table_to_map_update_insert::<Key, Value, Vec<(Key, Value)>>(
                        old_table, index as int, table.table@[index as int], key, value);

                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |j: int, k: Key| 0 <= j < table.table@.len()
                        && j != (table.spec_hash@)(k) as int % table.current_size as int
                        implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k) by {
                        if j != index as int {
                        }
                    }
                    // One-slot modification witness for trait ensures.
                }

                if !existed {
                    // Veracity: NEEDED proof block
                    table.num_elements = table.num_elements + 1;
                }
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(spec_other_slots_preserved(old(table).table@, table.table@, index as int));
                }
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — hash, backward scan bucket for last-wins match.
            fn lookup(table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let bucket_len = table.table[index].len();
                // Veracity: NEEDED proof block
                // Ghost alias: definitionally == table.table@[index]@.
                let ghost bv: Seq<(Key, Value)> = table.table@[index as int]@;
                if bucket_len == 0 {
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_seq_pairs_no_key_not_in_map::<Key, Value>(bv, *key);
                        lemma_table_to_map_not_contains::<Key, Value, Vec<(Key, Value)>>(
                            table.table@, *key);
                    }
                    return None;
                }
                // Scan backward so the first match found is the last occurrence,
                // matching spec_seq_pairs_to_map's last-wins semantics.
                let mut i: usize = bucket_len;
                while i > 0
                    invariant
                        0 <= i <= bv.len(),
                        bucket_len == bv.len(),
                        bv == table.table@[index as int]@,
                        index < table.table@.len(),
                        spec_hashtable_wf(table),
                        index as nat == (table.spec_hash@)(*key) % (table.current_size as nat),
                        forall |j: int| i as int <= j < bv.len()
                            ==> (#[trigger] bv[j]).0 != *key,
                        obeys_feq_clone::<Value>(),
                    // Veracity: NEEDED proof block
                    decreases i,
                {
                    i = i - 1;
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&table.table[index][i].0, key);
                    if eq {
                        let v = clone_elem(&table.table[index][i].1);
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_seq_pairs_last_key_gives_value::<Key, Value>(
                                bv, *key, i as int);
                            // bv == table.table@[index]@ by definition, so
                            // spec_entry_to_map (= spec_seq_pairs_to_map(self@)) matches.
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |j: int| 0 <= j < table.table@.len() && j != index as int
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {}
                            // Veracity: NEEDED proof block
                            lemma_table_to_map_unique_entry_value::<Key, Value, Vec<(Key, Value)>>(
                                table.table@, index as int, *key);
                        }
                        return Some(v);
                    }
                }
                // Veracity: NEEDED proof block
                proof {
                    lemma_seq_pairs_no_key_not_in_map::<Key, Value>(bv, *key);
                    lemma_table_to_map_not_contains::<Key, Value, Vec<(Key, Value)>>(
                        table.table@, *key);
                }
                None
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst
            fn delete(table: &mut HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> (deleted: bool) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let bucket = clone_vec_pairs(&table.table[index]);
                let ghost original = bucket@;
                let bucket_len = bucket.len();

                let mut new_bucket: Vec<(Key, Value)> = Vec::new();
                let mut deleted = false;
                let ghost mut prefix_map: Map<Key, Value> = Map::empty();
                let mut i: usize = 0;

                while i < bucket_len
                    invariant
                        i <= bucket_len,
                        bucket_len == original.len(),
                        bucket@ =~= original,
                        original =~= old_table[index as int]@,
                        index < old_table.len(),
                        table.table@ == old_table,
                        table.current_size == old(table).current_size,
                        table.num_elements == old(table).num_elements,
                        prefix_map =~= spec_seq_pairs_to_map(original.subrange(0, i as int)),
                        spec_seq_pairs_to_map(new_bucket@) =~= prefix_map.remove(*key),
                        !deleted ==> forall |j: int| 0 <= j < i as int
                            ==> (#[trigger] original[j]).0 != *key,
                        // Veracity: NEEDED proof block
                        deleted ==> exists |j: int| 0 <= j < i as int
                            && (#[trigger] original[j]).0 == *key,
                        obeys_feq_clone::<Key>(),
                        // Veracity: NEEDED proof block
                        obeys_feq_clone::<Value>(),
                    decreases bucket_len - i,
                {
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&bucket[i].0, key);

                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED proof block
                        assert(original.subrange(0, (i + 1) as int).drop_last()
                            =~= original.subrange(0, i as int));
                    }

                    if !eq {
                        let k = clone_elem(&bucket[i].0);
                        let v = clone_elem(&bucket[i].1);
                        let ghost old_new_bucket = new_bucket@;
                        new_bucket.push((k, v));
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED proof block
                            let ghost pair_key = original[i as int].0;
                            let ghost pair_val = original[i as int].1;
                            // new_bucket map = old map + (pair_key, pair_val).
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(new_bucket@.drop_last() =~= old_new_bucket);
                            // insert/remove commute on different keys.
                            prefix_map = prefix_map.insert(pair_key, pair_val);
                        }
                    } else {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                            let ghost pair_val = original[i as int].1;
                            // insert(key, v).remove(key) = remove(key).
                            prefix_map = prefix_map.insert(original[i as int].0, original[i as int].1);
                        }
                        deleted = true;
                    }
                    // Veracity: NEEDED proof block
                    i += 1;
                }

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(original.subrange(0, bucket_len as int) =~= original);
                }

                table.table.set(index, new_bucket);

                // Veracity: NEEDED proof block
                proof {

                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(*key) by {}

                    lemma_table_to_map_update_remove::<Key, Value, Vec<(Key, Value)>>(
                        old_table, index as int, table.table@[index as int], *key);

                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |j: int, k: Key| 0 <= j < table.table@.len()
                        && j != (table.spec_hash@)(k) as int % table.current_size as int
                        implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k) by {
                        if j == index as int {
                        } else {
                        }
                    }

                    if deleted {
                        let j_witness = choose |j: int| 0 <= j < original.len()
                            && (#[trigger] original[j]).0 == *key;
                        lemma_seq_pairs_has_key_in_map::<Key, Value>(original, *key, j_witness);
                        lemma_table_to_map_update_contains::<Key, Value, Vec<(Key, Value)>>(
                            old_table, index as int, old_table[index as int], *key);
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(old_table.update(index as int, old_table[index as int]) =~= old_table);
                    } else {
                        lemma_seq_pairs_no_key_not_in_map::<Key, Value>(original, *key);
                        lemma_table_to_map_not_contains::<Key, Value, Vec<(Key, Value)>>(old_table, *key);
                    }
                }

                if deleted && table.num_elements > 0 {
                    table.num_elements = table.num_elements - 1;
                }
                deleted
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' chains, reinserts.
            fn resize(
                table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>,
                new_size: usize,
            ) -> (resized: HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>) {
                // Phase 1: collect all pairs from all chains.
                let mut pairs: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < table.table.len()
                    invariant
                        i <= table.table@.len(),
                        table.table@.len() == table.current_size as int,
                        spec_seq_pairs_to_map(pairs@) =~=
                            spec_table_to_map::<Key, Value, Vec<(Key, Value)>>(
                                table.table@.subrange(0, i as int)),
                        obeys_feq_clone::<Key>(),
                        obeys_feq_clone::<Value>(),
                    decreases table.table.len() - i,
                {
                    let ghost outer_map = spec_seq_pairs_to_map(pairs@);
                    let chain_len = table.table[i].len();
                    let mut j: usize = 0;
                    while j < chain_len
                        invariant
                            i < table.table@.len(),
                            j <= chain_len,
                            chain_len == table.table@[i as int]@.len(),
                            spec_seq_pairs_to_map(pairs@) =~=
                                // Veracity: NEEDED proof block
                                outer_map.union_prefer_right(
                                    spec_seq_pairs_to_map(
                                        table.table@[i as int]@.subrange(0, j as int))),
                            obeys_feq_clone::<Key>(),
                            obeys_feq_clone::<Value>(),
                        decreases chain_len - j,
                    {
                        let ghost old_pairs = pairs@;
                        let ghost old_map = spec_seq_pairs_to_map(old_pairs);
                        let ghost chain = table.table@[i as int]@;
                        let k = clone_elem(&table.table[i][j].0);
                        let v = clone_elem(&table.table[i][j].1);
                        pairs.push((k, v));
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(pairs@.drop_last() =~= old_pairs);
                            let ghost chain_sub = chain.subrange(0, j as int);
                            let ghost chain_sub_next = chain.subrange(0, (j + 1) as int);
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert(chain_sub_next.drop_last() =~= chain_sub);
                            let ghost n = spec_seq_pairs_to_map(chain_sub);
                        }
                        j = j + 1;
                    }
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(table.table@[i as int]@.subrange(
                            0, table.table@[i as int]@.len() as int)
                            =~= table.table@[i as int]@);
                        let ghost sub_next = table.table@.subrange(0, (i + 1) as int);
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(sub_next.drop_last()
                            =~= table.table@.subrange(0, i as int));
                    }
                    i = i + 1;
                }
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(table.table@.subrange(0, table.table@.len() as int)
                        =~= table.table@);
                }

                // Veracity: NEEDED proof block
                // Phase 2: create new table with empty entries.
                let mut new_table_vec: Vec<Vec<(Key, Value)>> = Vec::new();
                let mut k: usize = 0;
                while k < new_size
                    invariant
                        k <= new_size,
                        new_table_vec@.len() == k as int,
                        forall |idx: int| 0 <= idx < new_table_vec@.len()
                            ==> (#[trigger] new_table_vec@[idx])@.len() == 0,
                        spec_table_to_map::<Key, Value, Vec<(Key, Value)>>(new_table_vec@)
                            == Map::<Key, Value>::empty(),
                    decreases new_size - k,
                {
                    let ghost old_vec = new_table_vec@;
                    let empty_chain: Vec<(Key, Value)> = Vec::new();
                    new_table_vec.push(empty_chain);
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_table_to_map_push_empty::<Key, Value, Vec<(Key, Value)>>(
                            old_vec, new_table_vec@.last());
                    }
                    k = k + 1;
                }
                let mut new_table = HashTable {
                    table: new_table_vec,
                    hash_fn: table.hash_fn.clone(),
                    initial_size: table.initial_size,
                    current_size: new_size,
                    num_elements: 0,
                    metrics: Metrics::default(),
                    spec_hash: table.spec_hash,
                    _phantom: PhantomData,
                };
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall |key: Key, idx: int|
                        0 <= idx < new_table.table@.len()
                        && idx != (new_table.spec_hash@)(key) as int
                            % new_table.current_size as int
                        implies !#[trigger] new_table.table@[idx]
                            .spec_entry_to_map().dom().contains(key) by {
                    }
                    // Veracity: NEEDED assert
                    assert(spec_hashtable_wf(&new_table));
                }

                // Phase 3: reinsert all pairs.
                let mut m: usize = 0;
                // Veracity: NEEDED proof block
                while m < pairs.len()
                    invariant
                        m <= pairs@.len(),
                        new_size > 0,
                        new_table.current_size == new_size,
                        new_table.table@.len() == new_table.current_size as int,
                        new_table.num_elements <= m,
                        Self::spec_parahashtablesteph_wf(&new_table),
                        // Veracity: NEEDED proof block
                        new_table@ =~= spec_seq_pairs_to_map(
                            pairs@.subrange(0, m as int)),
                        new_table.spec_hash == table.spec_hash,
                        obeys_feq_clone::<Key>(),
                        obeys_feq_clone::<Value>(),
                    decreases pairs.len() - m,
                {
                    let key = clone_elem(&pairs[m].0);
                    let value = clone_elem(&pairs[m].1);
                    Self::insert(&mut new_table, key, value);
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(pairs@.subrange(0, (m + 1) as int).drop_last()
                            =~= pairs@.subrange(0, m as int));
                    }
                    m = m + 1;
                }
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(pairs@.subrange(0, pairs@.len() as int) =~= pairs@);
                }

                new_table
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ChainedHashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>
            for VecChainedHashTableStEph
        {
            /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to stored hash function.
            fn hash_index(table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> (index: usize) {
                call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash)
            }
        }
    }

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for VecChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VecChainedHashTableStEph")
        }
    }

    impl std::fmt::Display for VecChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VecChainedHashTableStEph")
        }
    }
}
