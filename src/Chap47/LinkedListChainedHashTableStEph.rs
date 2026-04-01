//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! LinkedList Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses LinkedListStEphS (Chap18) for separate chaining collision resolution.

pub mod LinkedListChainedHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions (inside verus!)
    // 7. proof fns (inside verus!)
    // 9. impls (inside verus!: EntryTrait for LinkedListStEphS, ParaHashTableStEphTrait, ChainedHashTable)

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap18::LinkedListStEph::LinkedListStEph::*;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_clone, obeys_feq_full_trigger, lemma_reveal_view_injective};

    verus! {

        // 3. broadcast use

        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

        // 4. type definitions

        /// LinkedList Chained Hash Table implementation.
        pub struct LinkedListChainedHashTableStEph;

        // 7. proof fns

        proof fn _linked_list_chained_hash_table_verified() {}

        /// Clones a LinkedListStEphS<(Key, Value)> with sequence equality ensures.
        fn clone_linked_list_entry<Key: Eq + Clone, Value: Eq + Clone>(
            entry: &LinkedListStEphS<(Key, Value)>,
        ) -> (cloned: LinkedListStEphS<(Key, Value)>)
            requires
                obeys_feq_clone::<Key>(),
                obeys_feq_clone::<Value>(),
            ensures cloned.seq@ =~= entry.seq@,
        {
            let mut new_seq: Vec<(Key, Value)> = Vec::new();
            let mut i: usize = 0;
            while i < entry.seq.len()
                invariant
                    i <= entry.seq@.len(),
                    new_seq@.len() == i as int,
                    forall |j: int| 0 <= j < i as int
                        ==> #[trigger] new_seq@[j] == entry.seq@[j],
                    obeys_feq_clone::<Key>(),
                    obeys_feq_clone::<Value>(),
                decreases entry.seq.len() - i,
            {
                let k = clone_elem(&entry.seq[i].0);
                let v = clone_elem(&entry.seq[i].1);
                new_seq.push((k, v));
                i += 1;
            }
            LinkedListStEphS { seq: new_seq }
        }

        // 9. impls

        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for LinkedListStEphS<(Key, Value)> {
            open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
                spec_seq_pairs_to_map(self.seq@)
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty LinkedListStEphS construction.
            fn new() -> (entry: Self) { LinkedListStEphS { seq: Vec::new() } }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan for duplicate key, n = chain length.
            fn insert(&mut self, key: Key, value: Value)
                ensures
                    self.seq@.len() >= 1,
                    old(self).seq@.len() <= self.seq@.len(),
                    self.seq@.len() <= old(self).seq@.len() + 1,
                    self.seq@.last() == (key, value),
                    self.spec_entry_to_map()[key] == value,
            {
                let mut i: usize = 0;
                while i < self.seq.len()
                    invariant
                        i <= self.seq@.len(),
                        self.seq@ == old(self).seq@,
                    decreases self.seq.len() - i,
                {
                    if self.seq[i].0 == key {
                        self.seq.remove(i);
                        self.seq.push((key, value));
                        return;
                    }
                    i += 1;
                }
                self.seq.push((key, value));
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>) {
                let mut i: usize = 0;
                while i < self.seq.len()
                    decreases self.seq.len() - i,
                {
                    if self.seq[i].0 == *key {
                        return Some(self.seq[i].1.clone());
                    }
                    i += 1;
                }
                None
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan + Vec::remove, n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: bool)
                ensures
                    !deleted ==> self.seq@ == old(self).seq@,
                    deleted ==> self.seq@.len() + 1 == old(self).seq@.len(),
            {
                let mut i: usize = 0;
                while i < self.seq.len()
                    invariant
                        i <= self.seq@.len(),
                        self.seq@ == old(self).seq@,
                    decreases self.seq.len() - i,
                {
                    if self.seq[i].0 == *key {
                        self.seq.remove(i);
                        return true;
                    }
                    i += 1;
                }
                false
            }

            /// Element-wise clone avoiding Verus tuple-Clone limitation.
            fn clone_entry(&self) -> (cloned: Self) {
                let mut new_seq: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < self.seq.len()
                    invariant
                        i <= self.seq@.len(),
                    decreases self.seq.len() - i,
                {
                    new_seq.push((self.seq[i].0.clone(), self.seq[i].1.clone()));
                    i += 1;
                }
                LinkedListStEphS { seq: new_seq }
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>
            for LinkedListChainedHashTableStEph
        {
            /// - Alg Analysis: APAS (Ch47 ref): Work O(n) worst, Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst, Span O(n) — matches APAS
            fn insert(table: &mut HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: Key, value: Value) {
                let index = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let cloned = clone_linked_list_entry(&table.table[index]);
                let mut bucket_seq = cloned.seq;
                let ghost original = bucket_seq@;

                let bucket_len = bucket_seq.len();
                let mut existed = false;
                let mut found_idx: usize = 0;
                let mut scan_i: usize = 0;
                while scan_i < bucket_len
                    invariant
                        scan_i <= bucket_len,
                        bucket_len == original.len(),
                        bucket_seq@ == original,
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
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&bucket_seq[scan_i].0, &key);
                    if eq {
                        existed = true;
                        found_idx = scan_i;
                        break;
                    }
                    scan_i += 1;
                }

                if existed {
                    bucket_seq.remove(found_idx);
                }
                let ghost pre_push = bucket_seq@;
                bucket_seq.push((key, value));

                let ghost new_bucket_seq = bucket_seq@;

                proof {
                    assert(new_bucket_seq == pre_push.push((key, value)));
                    if existed {
                        assert(pre_push =~= original.remove(found_idx as int));
                        lemma_seq_pairs_remove_key_then_push::<Key, Value>(
                            original, found_idx as int, key, value);
                    } else {
                        assert(pre_push =~= original);
                        assert(new_bucket_seq.drop_last() =~= original);
                        assert(new_bucket_seq.last() == (key, value));
                    }
                    assert(spec_seq_pairs_to_map(new_bucket_seq)
                        =~= spec_seq_pairs_to_map(original).insert(key, value));
                }

                let new_entry = LinkedListStEphS { seq: bucket_seq };
                table.table.set(index, new_entry);

                proof {
                    assert(table.table@[index as int].seq@ == new_bucket_seq);
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().insert(key, value));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(key) by {}

                    lemma_table_to_map_update_insert::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                        old_table, index as int, table.table@[index as int], key, value);

                    assert(table.table@.len() == table.current_size as int);
                    assert(table.current_size > 0);
                    assert forall |j: int, k: Key| 0 <= j < table.table@.len()
                        && j != (table.spec_hash@)(k) as int % table.current_size as int
                        implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k) by {
                        if j != index as int {
                            assert(table.table@[j] == old_table[j]);
                        }
                    }
                    // One-slot modification witness for trait ensures.
                    assert(old_table =~= old(table).table@);
                    assert(spec_other_slots_preserved(old(table).table@, table.table@, index as int));
                }

                if !existed {
                    table.num_elements = table.num_elements + 1;
                }
                proof {
                    assert(spec_other_slots_preserved(old(table).table@, table.table@, index as int));
                }
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — hash, backward scan bucket for last-wins match.
            fn lookup(table: &HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let bucket_len = table.table[index].seq.len();
                // Ghost alias: definitionally == table.table@[index].seq@.
                let ghost bv: Seq<(Key, Value)> = table.table@[index as int].seq@;
                if bucket_len == 0 {
                    proof {
                        assert(bv =~= Seq::<(Key, Value)>::empty());
                        lemma_seq_pairs_no_key_not_in_map::<Key, Value>(bv, *key);
                        lemma_table_to_map_not_contains::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                            table.table@, *key);
                    }
                    return None;
                }
                let mut i: usize = bucket_len;
                while i > 0
                    invariant
                        0 <= i <= bv.len(),
                        bucket_len == bv.len(),
                        bv == table.table@[index as int].seq@,
                        index < table.table@.len(),
                        spec_hashtable_wf(table),
                        index as nat == (table.spec_hash@)(*key) % (table.current_size as nat),
                        forall |j: int| i as int <= j < bv.len()
                            ==> (#[trigger] bv[j]).0 != *key,
                        obeys_feq_clone::<Value>(),
                    decreases i,
                {
                    i = i - 1;
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&table.table[index].seq[i].0, key);
                    if eq {
                        let v = clone_elem(&table.table[index].seq[i].1);
                        proof {
                            lemma_seq_pairs_last_key_gives_value::<Key, Value>(
                                bv, *key, i as int);
                            // bv == table.table@[index].seq@ by definition, so
                            // spec_entry_to_map (= spec_seq_pairs_to_map(self.seq@)) matches.
                            assert(table.table@[index as int].spec_entry_to_map().dom().contains(*key));
                            assert forall |j: int| 0 <= j < table.table@.len() && j != index as int
                                implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {}
                            lemma_table_to_map_unique_entry_value::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                                table.table@, index as int, *key);
                        }
                        return Some(v);
                    }
                }
                proof {
                    lemma_seq_pairs_no_key_not_in_map::<Key, Value>(bv, *key);
                    lemma_table_to_map_not_contains::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                        table.table@, *key);
                }
                None
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(n) worst, Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst, Span O(n) — matches APAS
            fn delete(table: &mut HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: &Key) -> (deleted: bool) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let cloned = clone_linked_list_entry(&table.table[index]);
                let bucket_seq = cloned.seq;
                let ghost original = bucket_seq@;
                let bucket_len = bucket_seq.len();

                let mut new_seq: Vec<(Key, Value)> = Vec::new();
                let mut deleted = false;
                let ghost mut prefix_map: Map<Key, Value> = Map::empty();
                let mut i: usize = 0;

                while i < bucket_len
                    invariant
                        i <= bucket_len,
                        bucket_len == original.len(),
                        bucket_seq@ =~= original,
                        original =~= old_table[index as int].seq@,
                        index < old_table.len(),
                        table.table@ == old_table,
                        table.current_size == old(table).current_size,
                        table.num_elements == old(table).num_elements,
                        prefix_map =~= spec_seq_pairs_to_map(original.subrange(0, i as int)),
                        spec_seq_pairs_to_map(new_seq@) =~= prefix_map.remove(*key),
                        !deleted ==> forall |j: int| 0 <= j < i as int
                            ==> (#[trigger] original[j]).0 != *key,
                        deleted ==> exists |j: int| 0 <= j < i as int
                            && (#[trigger] original[j]).0 == *key,
                        obeys_feq_clone::<Key>(),
                        obeys_feq_clone::<Value>(),
                    decreases bucket_len - i,
                {
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&bucket_seq[i].0, key);

                    proof {
                        assert(original.subrange(0, (i + 1) as int).drop_last()
                            =~= original.subrange(0, i as int));
                        assert(original.subrange(0, (i + 1) as int).last()
                            == original[i as int]);
                    }

                    if !eq {
                        let k = clone_elem(&bucket_seq[i].0);
                        let v = clone_elem(&bucket_seq[i].1);
                        let ghost old_new_seq = new_seq@;
                        new_seq.push((k, v));
                        proof {
                            let ghost pair_key = original[i as int].0;
                            let ghost pair_val = original[i as int].1;
                            assert(new_seq@.drop_last() =~= old_new_seq);
                            assert(new_seq@.last() == (pair_key, pair_val));
                            assert(prefix_map.insert(pair_key, pair_val).remove(*key)
                                =~= prefix_map.remove(*key).insert(pair_key, pair_val));
                            prefix_map = prefix_map.insert(pair_key, pair_val);
                        }
                    } else {
                        proof {
                            let ghost pair_val = original[i as int].1;
                            assert(prefix_map.insert(*key, pair_val).remove(*key)
                                =~= prefix_map.remove(*key));
                            prefix_map = prefix_map.insert(original[i as int].0, original[i as int].1);
                        }
                        deleted = true;
                    }
                    i += 1;
                }

                proof {
                    assert(original.subrange(0, bucket_len as int) =~= original);
                }

                let new_entry = LinkedListStEphS { seq: new_seq };
                table.table.set(index, new_entry);

                proof {
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().remove(*key));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(*key) by {}

                    lemma_table_to_map_update_remove::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                        old_table, index as int, table.table@[index as int], *key);

                    assert(table.table@.len() == table.current_size as int);
                    assert(table.current_size > 0);
                    assert forall |j: int, k: Key| 0 <= j < table.table@.len()
                        && j != (table.spec_hash@)(k) as int % table.current_size as int
                        implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k) by {
                        if j == index as int {
                            assert(!old_table[j].spec_entry_to_map().dom().contains(k));
                        } else {
                            assert(table.table@[j] == old_table[j]);
                        }
                    }

                    if deleted {
                        let j_witness = choose |j: int| 0 <= j < original.len()
                            && (#[trigger] original[j]).0 == *key;
                        lemma_seq_pairs_has_key_in_map::<Key, Value>(original, *key, j_witness);
                        lemma_table_to_map_update_contains::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                            old_table, index as int, old_table[index as int], *key);
                        assert(old_table.update(index as int, old_table[index as int]) =~= old_table);
                    } else {
                        lemma_seq_pairs_no_key_not_in_map::<Key, Value>(original, *key);
                        lemma_table_to_map_not_contains::<Key, Value, LinkedListStEphS<(Key, Value)>>(old_table, *key);
                    }
                }

                if deleted && table.num_elements > 0 {
                    table.num_elements = table.num_elements - 1;
                }
                deleted
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' lists, reinserts.
            fn resize(
                table: &HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>,
                new_size: usize,
            ) -> (resized: HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>) {
                // Phase 1: collect all pairs from all chains.
                let mut pairs: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < table.table.len()
                    invariant
                        i <= table.table@.len(),
                        table.table@.len() == table.current_size as int,
                        spec_seq_pairs_to_map(pairs@) =~=
                            spec_table_to_map::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                                table.table@.subrange(0, i as int)),
                        obeys_feq_clone::<Key>(),
                        obeys_feq_clone::<Value>(),
                    decreases table.table.len() - i,
                {
                    let ghost outer_map = spec_seq_pairs_to_map(pairs@);
                    let chain_len = table.table[i].seq.len();
                    let mut j: usize = 0;
                    while j < chain_len
                        invariant
                            i < table.table@.len(),
                            j <= chain_len,
                            chain_len == table.table@[i as int].seq@.len(),
                            spec_seq_pairs_to_map(pairs@) =~=
                                outer_map.union_prefer_right(
                                    spec_seq_pairs_to_map(
                                        table.table@[i as int].seq@.subrange(0, j as int))),
                            obeys_feq_clone::<Key>(),
                            obeys_feq_clone::<Value>(),
                        decreases chain_len - j,
                    {
                        let ghost old_pairs = pairs@;
                        let ghost old_map = spec_seq_pairs_to_map(old_pairs);
                        let ghost chain = table.table@[i as int].seq@;
                        let k = clone_elem(&table.table[i].seq[j].0);
                        let v = clone_elem(&table.table[i].seq[j].1);
                        pairs.push((k, v));
                        proof {
                            assert(pairs@.drop_last() =~= old_pairs);
                            assert(pairs@.last() == (k, v));
                            assert(spec_seq_pairs_to_map(pairs@) =~= old_map.insert(k, v));
                            let ghost chain_sub = chain.subrange(0, j as int);
                            let ghost chain_sub_next = chain.subrange(0, (j + 1) as int);
                            assert(chain_sub_next.drop_last() =~= chain_sub);
                            assert(chain_sub_next.last() == chain[j as int]);
                            let ghost n = spec_seq_pairs_to_map(chain_sub);
                            assert(outer_map.union_prefer_right(n).insert(k, v) =~=
                                outer_map.union_prefer_right(n.insert(k, v)));
                        }
                        j = j + 1;
                    }
                    proof {
                        assert(table.table@[i as int].seq@.subrange(
                            0, table.table@[i as int].seq@.len() as int)
                            =~= table.table@[i as int].seq@);
                        let ghost sub_next = table.table@.subrange(0, (i + 1) as int);
                        assert(sub_next.drop_last()
                            =~= table.table@.subrange(0, i as int));
                        assert(sub_next.last() == table.table@[i as int]);
                    }
                    i = i + 1;
                }
                proof {
                    assert(table.table@.subrange(0, table.table@.len() as int)
                        =~= table.table@);
                }

                // Phase 2: create new table with empty entries.
                let mut new_table_vec: Vec<LinkedListStEphS<(Key, Value)>> = Vec::new();
                let mut k: usize = 0;
                while k < new_size
                    invariant
                        k <= new_size,
                        new_table_vec@.len() == k as int,
                        forall |idx: int| 0 <= idx < new_table_vec@.len()
                            ==> (#[trigger] new_table_vec@[idx]).seq@.len() == 0,
                        spec_table_to_map::<Key, Value, LinkedListStEphS<(Key, Value)>>(
                            new_table_vec@) == Map::<Key, Value>::empty(),
                    decreases new_size - k,
                {
                    let ghost old_vec = new_table_vec@;
                    let empty_chain = LinkedListStEphS { seq: Vec::new() };
                    new_table_vec.push(empty_chain);
                    proof {
                        lemma_table_to_map_push_empty::<
                            Key, Value, LinkedListStEphS<(Key, Value)>>(
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
                proof {
                    assert forall |key: Key, idx: int|
                        0 <= idx < new_table.table@.len()
                        && idx != (new_table.spec_hash@)(key) as int
                            % new_table.current_size as int
                        implies !#[trigger] new_table.table@[idx]
                            .spec_entry_to_map().dom().contains(key) by {
                        assert(new_table.table@[idx].seq@.len() == 0);
                    }
                    assert(spec_hashtable_wf(&new_table));
                }

                // Phase 3: reinsert all pairs.
                let mut m: usize = 0;
                while m < pairs.len()
                    invariant
                        m <= pairs@.len(),
                        new_size > 0,
                        new_table.current_size == new_size,
                        new_table.table@.len() == new_table.current_size as int,
                        new_table.num_elements <= m,
                        Self::spec_parahashtablesteph_wf(&new_table),
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
                    proof {
                        assert(pairs@.subrange(0, (m + 1) as int).drop_last()
                            =~= pairs@.subrange(0, m as int));
                        assert(pairs@.subrange(0, (m + 1) as int).last()
                            == pairs@[m as int]);
                    }
                    m = m + 1;
                }
                proof {
                    assert(pairs@.subrange(0, pairs@.len() as int) =~= pairs@);
                }

                new_table
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ChainedHashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>
            for LinkedListChainedHashTableStEph
        {
            /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to stored hash function.
            fn hash_index(table: &HashTable<Key, Value, LinkedListStEphS<(Key, Value)>, Metrics, H>, key: &Key) -> (index: usize) {
                call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash)
            }
        }
    }

    // 13. derive impls outside verus!

    impl std::fmt::Debug for LinkedListChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinkedListChainedHashTableStEph")
        }
    }

    impl std::fmt::Display for LinkedListChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LinkedListChainedHashTableStEph")
        }
    }
}
