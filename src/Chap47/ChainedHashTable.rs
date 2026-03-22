//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses separate chaining for collision resolution.

pub mod ChainedHashTable {

    // Table of Contents
    // 1. module
    // 2. imports
    // 7. proof fns (inside verus!: lemma_seq_pairs_*)
    // 8. traits (inside verus!: ChainedHashTable)

    // 2. imports
    use vstd::prelude::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus! {

        // 7. proof fns

        /// Removing a pair at position i (where pairs[i].0 != k) does not affect the map for key k.
        pub proof fn lemma_seq_pairs_to_map_remove_preserves_other_keys<Key, Value>(
            pairs: Seq<(Key, Value)>,
            i: int,
            k: Key,
        )
            requires
                0 <= i < pairs.len(),
                pairs[i].0 != k,
            ensures
                spec_seq_pairs_to_map(pairs.remove(i)).dom().contains(k)
                    == spec_seq_pairs_to_map(pairs).dom().contains(k),
                spec_seq_pairs_to_map(pairs.remove(i)).dom().contains(k) ==>
                    spec_seq_pairs_to_map(pairs.remove(i))[k]
                        == spec_seq_pairs_to_map(pairs)[k],
            decreases pairs.len(),
        {
            if pairs.len() == 1 {
            } else if i == pairs.len() - 1 {
                assert(pairs.remove(i) =~= pairs.drop_last());
            } else {
                assert(pairs.remove(i).drop_last() =~= pairs.drop_last().remove(i));
                assert(pairs.remove(i).last() == pairs.last());
                assert(pairs.drop_last()[i] == pairs[i]);
                lemma_seq_pairs_to_map_remove_preserves_other_keys(pairs.drop_last(), i, k);
            }
        }

        /// Removing a pair at position i (where pairs[i].0 == key) and then pushing (key, value)
        /// produces the same map as inserting key->value into the original map.
        pub proof fn lemma_seq_pairs_remove_key_then_push<Key, Value>(
            pairs: Seq<(Key, Value)>,
            i: int,
            key: Key,
            value: Value,
        )
            requires
                0 <= i < pairs.len(),
                pairs[i].0 == key,
            ensures
                spec_seq_pairs_to_map(pairs.remove(i).push((key, value)))
                    =~= spec_seq_pairs_to_map(pairs).insert(key, value),
        {
            let removed = pairs.remove(i);
            let final_seq = removed.push((key, value));
            assert(final_seq.drop_last() == removed);
            assert(final_seq.last() == (key, value));
            assert forall |k: Key| k != key implies
                spec_seq_pairs_to_map(removed).dom().contains(k)
                    == spec_seq_pairs_to_map(pairs).dom().contains(k)
            by {
                lemma_seq_pairs_to_map_remove_preserves_other_keys(pairs, i, k);
            }
            assert forall |k: Key| k != key && #[trigger] spec_seq_pairs_to_map(removed).dom().contains(k) implies
                spec_seq_pairs_to_map(removed)[k] == spec_seq_pairs_to_map(pairs)[k]
            by {
                lemma_seq_pairs_to_map_remove_preserves_other_keys(pairs, i, k);
            }
        }

        /// If no pair in a sequence has key as its first element, the key is absent from the map.
        pub proof fn lemma_seq_pairs_no_key_not_in_map<Key, Value>(
            pairs: Seq<(Key, Value)>,
            key: Key,
        )
            requires
                forall |j: int| 0 <= j < pairs.len() ==> (#[trigger] pairs[j]).0 != key,
            ensures
                !spec_seq_pairs_to_map(pairs).dom().contains(key),
            decreases pairs.len(),
        {
            if pairs.len() > 0 {
                assert forall |j: int| 0 <= j < pairs.drop_last().len()
                    implies (#[trigger] pairs.drop_last()[j]).0 != key by {
                    assert(pairs.drop_last()[j] == pairs[j]);
                }
                lemma_seq_pairs_no_key_not_in_map::<Key, Value>(pairs.drop_last(), key);
            }
        }

        /// If a pair at index idx has key as its first element, the key is in the map.
        pub proof fn lemma_seq_pairs_has_key_in_map<Key, Value>(
            pairs: Seq<(Key, Value)>,
            key: Key,
            idx: int,
        )
            requires
                0 <= idx < pairs.len(),
                pairs[idx].0 == key,
            ensures
                spec_seq_pairs_to_map(pairs).dom().contains(key),
            decreases pairs.len(),
        {
            if idx == pairs.len() - 1 {
                // key is the last pair; map.insert(key, _) contains key.
            } else {
                assert(pairs.drop_last()[idx] == pairs[idx]);
                lemma_seq_pairs_has_key_in_map::<Key, Value>(pairs.drop_last(), key, idx);
            }
        }

        /// If pairs[i].0 == key and no later index has the same key, then the
        /// map value for key equals pairs[i].1.
        pub proof fn lemma_seq_pairs_last_key_gives_value<Key, Value>(
            pairs: Seq<(Key, Value)>,
            key: Key,
            i: int,
        )
            requires
                0 <= i < pairs.len(),
                pairs[i].0 == key,
                forall |j: int| i < j < pairs.len() ==> (#[trigger] pairs[j]).0 != key,
            ensures
                spec_seq_pairs_to_map(pairs).dom().contains(key),
                spec_seq_pairs_to_map(pairs)[key] == pairs[i].1,
            decreases pairs.len(),
        {
            if i == pairs.len() - 1 {
                // pairs.last() == (key, pairs[i].1).
                // spec_seq_pairs_to_map inserts last pair, so map[key] == pairs[i].1.
            } else {
                assert(pairs.drop_last()[i] == pairs[i]);
                assert forall |j: int| i < j < pairs.drop_last().len()
                    implies (#[trigger] pairs.drop_last()[j]).0 != key by {
                    assert(pairs.drop_last()[j] == pairs[j]);
                }
                lemma_seq_pairs_last_key_gives_value::<Key, Value>(pairs.drop_last(), key, i);
                // pairs.last().0 != key, so inserting it doesn't affect map[key].
            }
        }

        // 8. traits

        /// Chained Hash Table trait - extends ParaHashTableStEphTrait.
        /// Uses separate chaining (linked lists or sequences) for collision resolution.
        /// Entry type is parametric - can be Vec, LinkedList, or any type implementing EntryTrait.
        pub trait ChainedHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>:
            ParaHashTableStEphTrait<Key, Value, Entry, Metrics, H>
        {
            /// Computes the hash index for a key.
            /// - APAS: Work O(1), Span O(1).
            /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on hash function.
            fn hash_index(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (index: usize)
                requires
                    spec_hashtable_wf(table),
                ensures
                    index < table.current_size,
                    index as nat == (table.spec_hash@)(*key) % (table.current_size as nat);
        }

    }
}
