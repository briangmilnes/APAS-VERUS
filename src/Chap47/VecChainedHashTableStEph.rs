//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Vec Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses Vec for separate chaining collision resolution.

pub mod VecChainedHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!)
    // 7. proof fns (inside verus!)
    // 9. impls (inside verus!: EntryTrait for Vec, ParaHashTableStEphTrait, ChainedHashTable)
    // 13. derive impls outside verus!

    // 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Chap47::ChainedHashTable::ChainedHashTable::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

        // 4. type definitions

        /// Vec Chained Hash Table implementation.
        pub struct VecChainedHashTableStEph;

        // 7. proof fns

        proof fn _vec_chained_hash_table_verified() {}

        /// Clones a Vec<(Key, Value)> with sequence equality ensures.
        /// Clone bridges inside this function follow the approved clone body pattern.
        fn clone_vec_pairs<Key: Clone, Value: Clone>(pairs: &Vec<(Key, Value)>) -> (cloned: Vec<(Key, Value)>)
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
                decreases pairs.len() - i,
            {
                let k = pairs[i].0.clone();
                let v = pairs[i].1.clone();
                proof {
                    assume(k == pairs@[i as int].0); // Clone bridge for Key.
                    assume(v == pairs@[i as int].1); // Clone bridge for Value.
                }
                new_vec.push((k, v));
                i += 1;
            }
            new_vec
        }

        // 9. impls

        impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for Vec<(Key, Value)> {
            open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
                spec_seq_pairs_to_map(self@)
            }

            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — empty Vec construction.
            fn new() -> (entry: Self)
                ensures entry@.len() == 0,
            { Vec::new() }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n) worst case, Span O(n) — linear scan for duplicate key, n = chain length.
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

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan of chain, n = chain length.
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

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n), Span O(n) — linear scan + Vec::remove (shifts elements), n = chain length.
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

        // 9. impls (ParaHashTableStEphTrait, ChainedHashTable)

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, Vec<(Key, Value)>, Metrics, H>
            for VecChainedHashTableStEph
        {
            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n) worst, Span O(n) — hash, clone bucket, dedup insert, set back.
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
                    let eq = bucket[scan_i].0 == key;
                    proof { assume(eq == (bucket@[scan_i as int].0 == key)); } // Eq bridge.
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

                table.table.set(index, bucket);

                proof {
                    assert(table.table@[index as int]@ == new_bucket_seq);
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().insert(key, value));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(key) by {}

                    lemma_table_to_map_update_insert::<Key, Value, Vec<(Key, Value)>>(
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
                }

                if !existed {
                    table.num_elements = table.num_elements + 1;
                }
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(1+α) expected, Span O(1+α) — hash, index bucket, scan chain.
            #[verifier::external_body]
            fn lookup(table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                EntryTrait::lookup(&table.table[index], key)
            }

            /// - APAS: Work O(1+α) expected, Span O(1+α).
            /// - Claude-Opus-4.6: Work O(n) worst, Span O(n) — hash, copy bucket entries, delete, set back.
            #[verifier::external_body]
            fn delete(table: &mut HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> (deleted: bool) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let bucket_len = table.table[index].len();
                let mut new_bucket: Vec<(Key, Value)> = Vec::new();
                let mut deleted = false;
                let mut i: usize = 0;
                while i < bucket_len
                    invariant
                        i <= bucket_len,
                        index < table.table.len(),
                        bucket_len == table.table@[index as int]@.len(),
                        table.table@.len() == table.current_size as int,
                        table.current_size == old(table).current_size,
                    decreases bucket_len - i,
                {
                    if table.table[index][i].0 == *key && !deleted {
                        deleted = true;
                    } else {
                        new_bucket.push((table.table[index][i].0.clone(), table.table[index][i].1.clone()));
                    }
                    i = i + 1;
                }
                table.table.set(index, new_bucket);
                if deleted && table.num_elements > 0 {
                    table.num_elements = table.num_elements - 1;
                }
                deleted
            }

            /// - APAS: Work O(n + m + m'), Span O(n + m + m').
            /// - Claude-Opus-4.6: Work O(n + m + m'), Span O(n + m + m') — collects n pairs, creates m' chains, reinserts.
            #[verifier::external_body]
            fn resize(
                table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>,
                new_size: usize,
            ) -> (resized: HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>) {
                let mut pairs: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                while i < table.table.len()
                    invariant
                        i <= table.table@.len(),
                        table.table@.len() == table.current_size as int,
                    decreases table.table.len() - i,
                {
                    let ghost table_len = table.table@.len();
                    let chain_len = table.table[i].len();
                    let mut j: usize = 0;
                    while j < chain_len
                        invariant
                            i < table_len,
                            table.table@.len() == table_len,
                            j <= chain_len,
                            chain_len == table.table@[i as int]@.len(),
                        decreases chain_len - j,
                    {
                        pairs.push((table.table[i][j].0.clone(), table.table[i][j].1.clone()));
                        j = j + 1;
                    }
                    i = i + 1;
                }

                let mut new_table_vec: Vec<Vec<(Key, Value)>> = Vec::new();
                let mut k: usize = 0;
                while k < new_size
                    invariant
                        k <= new_size,
                        new_table_vec@.len() == k as int,
                    decreases new_size - k,
                {
                    new_table_vec.push(Vec::new());
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

                let mut m: usize = 0;
                while m < pairs.len()
                    invariant
                        m <= pairs@.len(),
                        new_size > 0,
                        new_table.current_size == new_size,
                        new_table.table@.len() == new_table.current_size as int,
                        new_table.num_elements <= m,
                    decreases pairs.len() - m,
                {
                    let key = pairs[m].0.clone();
                    let value = pairs[m].1.clone();
                    Self::insert(&mut new_table, key, value);
                    m = m + 1;
                }

                new_table
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ChainedHashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>
            for VecChainedHashTableStEph
        {
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to stored hash function.
            fn hash_index(table: &HashTable<Key, Value, Vec<(Key, Value)>, Metrics, H>, key: &Key) -> (index: usize) {
                call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash)
            }
        }
    }

    // 13. derive impls outside verus!

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
