//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Struct Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses custom linked list struct for separate chaining collision resolution.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 8b. traits
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 6c. spec fns
//	Section 7c. proof fns/broadcast groups
//	Section 9c. impls
//	Section 12a. derive impls in verus!
//	Section 12b. derive impls in verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!


//		Section 1. module

pub mod StructChainedHashTable {

    //		Section 2. imports
    use std::marker::PhantomData;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
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

    //		Section 4a. type definitions


        /// Custom linked list node.
        #[verifier::reject_recursive_types(Key)]
        #[verifier::reject_recursive_types(Value)]
        pub struct Node<Key, Value> {
            pub key: Key,
            pub value: Value,
            pub next: Option<Box<Node<Key, Value>>>,
        }

    //		Section 4b. type definitions


        /// Custom linked list for chained hash table.
        #[verifier::reject_recursive_types(Key)]
        #[verifier::reject_recursive_types(Value)]
        pub struct ChainList<Key, Value> {
            pub head: Option<Box<Node<Key, Value>>>,
        }

    //		Section 8b. traits


        /// Spec trait for ChainList abstract state.
        pub trait ChainListTrait<Key, Value>: Sized {
            spec fn spec_to_map(&self) -> Map<Key, Value>;
        }

    //		Section 9b. impls


        impl<Key, Value> ChainListTrait<Key, Value> for ChainList<Key, Value> {
            open spec fn spec_to_map(&self) -> Map<Key, Value> {
                spec_chain_to_map(self.head)
            }
        }

        /// Inserts key-value into chain, updating if key exists, appending if not.
        /// Returns (new_chain, existed) where existed is true if key was already present.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive scan of chain; n = chain length.
        // veracity: no_requires
        fn chain_insert<Key: Eq + View + Clone, Value>(
            chain: Option<Box<Node<Key, Value>>>,
            key: Key,
            value: Value,
        ) -> (inserted: (Option<Box<Node<Key, Value>>>, bool))
            ensures
                inserted.0 is Some,
                spec_chain_to_map(inserted.0) == spec_chain_to_map(chain).insert(key, value),
                inserted.1 == spec_chain_to_map(chain).dom().contains(key),
                spec_chain_keys_unique(chain) ==> spec_chain_keys_unique(inserted.0),
            decreases chain,
        {
            match chain {
                None => {
                    let out = Some(Box::new(Node { key, value, next: None }));
                    proof {
                        reveal_with_fuel(spec_chain_to_map, 2);
                        reveal_with_fuel(spec_chain_keys_unique, 2);
                    }
                    (out, false)
                }
                Some(node) => {
                    let Node { key: nk, value: nv, next: nn } = *node;
                    proof { assert(obeys_feq_full_trigger::<Key>()); }
                    let eq = feq(&nk, &key);
                    if eq {
                        let out = Some(Box::new(Node { key, value, next: nn }));
                        proof {
                            lemma_reveal_view_injective::<Key>();
                            reveal_with_fuel(spec_chain_to_map, 2);
                            reveal_with_fuel(spec_chain_keys_unique, 2);
                        }
                        (out, true)
                    } else {
                        let (updated, existed) = chain_insert(nn, key, value);
                        let out = Some(Box::new(Node { key: nk, value: nv, next: updated }));
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            reveal_with_fuel(spec_chain_keys_unique, 2);
                            assert(spec_chain_to_map(nn).insert(key, value).insert(nk, nv)
                                =~= spec_chain_to_map(nn).insert(nk, nv).insert(key, value));
                            // Uniqueness: nk not in updated's map domain.
                            // spec_chain_to_map(updated) == spec_chain_to_map(nn).insert(key, value)
                            // nk ∉ spec_chain_to_map(nn).dom() (from unique(chain)) and nk != key.
                            assert(spec_chain_to_map(updated).dom() =~=
                                spec_chain_to_map(nn).dom().insert(key));
                        }
                        (out, existed)
                    }
                }
            }
        }

        /// Looks up key in chain, returning value if found.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive scan of chain; n = chain length.
        fn chain_lookup<Key: Eq + View + Clone, Value: Eq + Clone>(
            chain: &Option<Box<Node<Key, Value>>>,
            key: &Key,
        ) -> (found: Option<Value>)
            requires
                obeys_feq_clone::<Value>(),
            ensures
                spec_chain_to_map(*chain).dom().contains(*key)
                    ==> found == Some(spec_chain_to_map(*chain)[*key]),
                !spec_chain_to_map(*chain).dom().contains(*key)
                    ==> found is None,
            decreases chain,
        {
            match chain {
                None => {
                    proof { reveal_with_fuel(spec_chain_to_map, 1); }
                    None
                }
                Some(node) => {
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&node.key, key);
                    if eq {
                        let v = clone_elem(&node.value);
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                        }
                        Some(v)
                    } else {
                        let result = chain_lookup(&node.next, key);
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            // spec_chain_to_map(*chain) = spec_chain_to_map(node.next).insert(node.key, node.value)
                            // node.key != *key, so value at *key comes from node.next's map.
                        }
                        result
                    }
                }
            }
        }

        /// Removes all nodes matching key, returns updated chain and whether any found.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive scan of chain; n = chain length.
        // veracity: no_requires
        fn chain_delete<Key: Eq + View + Clone, Value>(
            chain: Option<Box<Node<Key, Value>>>,
            key: &Key,
        ) -> (remaining_and_deleted: (Option<Box<Node<Key, Value>>>, bool))
            ensures
                spec_chain_to_map(remaining_and_deleted.0)
                    == spec_chain_to_map(chain).remove(*key),
                remaining_and_deleted.1
                    == spec_chain_to_map(chain).dom().contains(*key),
                spec_chain_keys_unique(chain) ==> spec_chain_keys_unique(remaining_and_deleted.0),
            decreases chain,
        {
            match chain {
                None => {
                    proof { reveal_with_fuel(spec_chain_to_map, 1); }
                    (None, false)
                }
                Some(node) => {
                    let Node { key: nk, value: nv, next: nn } = *node;
                    proof { assert(obeys_feq_full_trigger::<Key>()); lemma_reveal_view_injective::<Key>(); }
                    let eq = feq(&nk, key);
                    let (new_next, tail_deleted) = chain_delete(nn, key);
                    if eq {
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            reveal_with_fuel(spec_chain_keys_unique, 2);
                            assert(spec_chain_to_map(nn).insert(*key, nv).remove(*key)
                                =~= spec_chain_to_map(nn).remove(*key));
                        }
                        (new_next, true)
                    } else {
                        let out = Some(Box::new(Node { key: nk, value: nv, next: new_next }));
                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            reveal_with_fuel(spec_chain_keys_unique, 2);
                            assert(spec_chain_to_map(nn).insert(nk, nv).remove(*key)
                                =~= spec_chain_to_map(nn).remove(*key).insert(nk, nv));
                            // nk ∉ dom(spec_chain_to_map(new_next)): remove can only shrink domain.
                            assert(spec_chain_to_map(new_next).dom() =~=
                                spec_chain_to_map(nn).dom().remove(*key));
                        }
                        (out, tail_deleted)
                    }
                }
            }
        }

        impl<Key: Eq + View + Clone, Value: Eq + View + Clone> EntryTrait<Key, Value> for ChainList<Key, Value> {
            open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
                spec_chain_to_map(self.head)
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty list.
            fn new() -> (entry: Self) { ChainList { head: None } }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive scan for duplicate key, n = chain length.
            fn insert(&mut self, key: Key, value: Value)
                ensures spec_chain_to_map(self.head).dom().contains(key),
            {
                let (new_head, _existed) = chain_insert(self.head.take(), key, value);
                self.head = new_head;
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive scan of chain, n = chain length.
            fn lookup(&self, key: &Key) -> (found: Option<Value>)
            {
                proof { assert(obeys_feq_full_trigger::<Value>()); }
                chain_lookup(&self.head, key)
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — recursive scan + rebuild, n = chain length.
            fn delete(&mut self, key: &Key) -> (deleted: bool)
                ensures
                    !deleted ==> spec_chain_to_map(self.head) == spec_chain_to_map(old(self).head),
            {
                let (new_head, found) = chain_delete(self.head.take(), key);
                self.head = new_head;
                found
            }

            /// Delegates to ChainList's Clone impl.
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clones entire chain; n = chain length.
            fn clone_entry(&self) -> (cloned: Self) {
                self.clone()
            }
        }

    //		Section 4c. type definitions


        /// Struct Chained Hash Table implementation.
        pub struct StructChainedHashTableStEph;

    //		Section 6c. spec fns


        /// Maps a chain (linked list of nodes) to its abstract Map representation.
        /// Head node's entry is inserted last, so it wins on duplicate keys.
        pub open spec fn spec_chain_to_map<Key, Value>(
            chain: Option<Box<Node<Key, Value>>>,
        ) -> Map<Key, Value>
            decreases chain,
        {
            match chain {
                None => Map::empty(),
                Some(node) => spec_chain_to_map(node.next).insert(node.key, node.value),
            }
        }

        /// True when no key appears more than once in the chain.
        pub open spec fn spec_chain_keys_unique<Key, Value>(
            chain: Option<Box<Node<Key, Value>>>,
        ) -> bool
            decreases chain,
        {
            match chain {
                None => true,
                Some(node) => {
                    !spec_chain_to_map(node.next).dom().contains(node.key)
                    && spec_chain_keys_unique(node.next)
                },
            }
        }

    //		Section 7c. proof fns/broadcast groups


        proof fn _struct_chained_hash_table_verified() {}

    //		Section 9c. impls


        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ParaHashTableStEphTrait<Key, Value, ChainList<Key, Value>, Metrics, H>
            for StructChainedHashTableStEph
        {
            /// Strengthened well-formedness: spec_hashtable_wf plus per-chain key uniqueness.
            open spec fn spec_parahashtablesteph_wf(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>) -> bool {
                spec_hashtable_wf(table)
                && forall |j: int| 0 <= j < table.table@.len()
                    ==> spec_chain_keys_unique(#[trigger] table.table@[j].head)
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(n) worst, Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst, Span O(n) — hash, clone chain, insert into clone, set back.
            fn insert(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: Key, value: Value) {
                let index = call_hash_fn(&table.hash_fn, &key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let mut entry = table.table[index].clone();
                let ghost old_entry_map = entry.spec_entry_to_map();

                let chain = entry.head.take();
                let (new_head, existed) = chain_insert(chain, key, value);
                entry.head = new_head;

                table.table.set(index, entry);

                proof {
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().insert(key, value));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(key) by {}

                    lemma_table_to_map_update_insert::<Key, Value, ChainList<Key, Value>>(
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
                    // Chain uniqueness preserved.
                    assert forall |j: int| 0 <= j < table.table@.len()
                        implies spec_chain_keys_unique(#[trigger] table.table@[j].head) by {
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
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(1+α) expected, Span O(1+α).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1+α) expected, Span O(1+α) — hash, index bucket, scan chain.
            fn lookup(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> (found: Option<Value>) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let result = chain_lookup(&table.table[index].head, key);
                proof {
                    // chain_lookup ensures correctness against spec_chain_to_map(head).
                    // spec_entry_to_map for ChainList == spec_chain_to_map(self.head).
                    if spec_chain_to_map(table.table@[index as int].head).dom().contains(*key) {
                        // Key in this bucket. By wf, not in any other bucket.
                        assert forall |j: int| 0 <= j < table.table@.len() && j != index as int
                            implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {}
                        lemma_table_to_map_unique_entry_value::<Key, Value, ChainList<Key, Value>>(
                            table.table@, index as int, *key);
                    } else {
                        // Key not in this bucket. By wf, not in any other bucket either.
                        assert forall |j: int| 0 <= j < table.table@.len()
                            implies !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(*key) by {
                            if j == index as int {
                            } else {
                            }
                        }
                        lemma_table_to_map_not_contains::<Key, Value, ChainList<Key, Value>>(
                            table.table@, *key);
                    }
                }
                result
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(n) worst, Span O(n).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst, Span O(n) — hash, clone chain, delete from clone, set back.
            fn delete(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> (deleted: bool) {
                let index = call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash);
                let ghost old_table = table.table@;

                let mut entry = table.table[index].clone();
                let ghost old_entry_map = entry.spec_entry_to_map();

                let chain = entry.head.take();
                let (new_head, found) = chain_delete(chain, key);
                entry.head = new_head;

                table.table.set(index, entry);

                proof {
                    assert(table.table@[index as int].spec_entry_to_map()
                        =~= old_table[index as int].spec_entry_to_map().remove(*key));

                    assert forall |j: int| 0 <= j < old_table.len() && j != index as int
                        implies !#[trigger] old_table[j].spec_entry_to_map().dom().contains(*key) by {}

                    lemma_table_to_map_update_remove::<Key, Value, ChainList<Key, Value>>(
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
                    // Chain uniqueness preserved.
                    assert forall |j: int| 0 <= j < table.table@.len()
                        implies spec_chain_keys_unique(#[trigger] table.table@[j].head) by {
                        if j != index as int {
                            assert(table.table@[j] == old_table[j]);
                        }
                    }

                    // Prove found == old(table)@.dom().contains(*key).
                    if found {
                        lemma_table_to_map_update_contains::<Key, Value, ChainList<Key, Value>>(
                            old_table, index as int, old_table[index as int], *key);
                        assert(old_table.update(index as int, old_table[index as int]) =~= old_table);
                    } else {
                        lemma_table_to_map_not_contains::<Key, Value, ChainList<Key, Value>>(old_table, *key);
                    }
                }

                if found && table.num_elements > 0 {
                    table.num_elements = table.num_elements - 1;
                }
                found
            }

            /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m').
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') — traverses all chains, creates m' lists, reinserts.
            fn resize(
                table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>,
                new_size: usize,
            ) -> (resized: HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>) {
                // Phase 1: collect all (key, value) pairs from all chains.
                let mut pairs: Vec<(Key, Value)> = Vec::new();
                let mut i: usize = 0;
                let ghost mut pairs_map = Map::<Key, Value>::empty();

                while i < table.table.len()
                    invariant
                        i <= table.table@.len(),
                        table.table@.len() == table.current_size as int,
                        Self::spec_parahashtablesteph_wf(table),
                        new_size > 0,
                        pairs_map =~= spec_seq_pairs_to_map::<Key, Value>(pairs@),
                        forall |k: Key| #[trigger] pairs_map.dom().contains(k) ==> (
                            (table.spec_hash@)(k) as int % (table.current_size as int) < (i as int)
                            && table.table@[(table.spec_hash@)(k) as int % (table.current_size as int)]
                                .spec_entry_to_map().dom().contains(k)
                            && pairs_map[k] == table.table@[
                                (table.spec_hash@)(k) as int % (table.current_size as int)
                            ].spec_entry_to_map()[k]
                        ),
                        forall |j: int, k: Key| 0 <= j < (i as int)
                            && #[trigger] table.table@[j].spec_entry_to_map().dom().contains(k)
                            ==> pairs_map.dom().contains(k),
                        obeys_feq_clone::<Key>(),
                        obeys_feq_clone::<Value>(),
                    decreases table.table.len() - i,
                {
                    let chain_clone = table.table[i].clone();
                    let mut current = chain_clone.head;
                    let ghost original_chain = table.table@[i as int].head;
                    let ghost mut inner_collected = Map::<Key, Value>::empty();
                    let ghost old_outer_pairs_map = pairs_map;

                    while current.is_some()
                        invariant
                            spec_chain_keys_unique(current),
                            inner_collected.union_prefer_right(spec_chain_to_map(current))
                                =~= spec_chain_to_map(original_chain),
                            forall |k: Key| #[trigger] inner_collected.dom().contains(k)
                                ==> !spec_chain_to_map(current).dom().contains(k),
                            pairs_map =~= spec_seq_pairs_to_map::<Key, Value>(pairs@),
                            pairs_map =~= old_outer_pairs_map.union_prefer_right(inner_collected),
                            forall |k: Key| #[trigger] inner_collected.dom().contains(k)
                                ==> !old_outer_pairs_map.dom().contains(k),
                            // Immutable context.
                            Self::spec_parahashtablesteph_wf(table),
                            i < table.table@.len(),
                            table.table@.len() == table.current_size as int,
                            table.current_size > 0,
                            new_size > 0,
                            original_chain == table.table@[i as int].head,
                            forall |k: Key| #[trigger] old_outer_pairs_map.dom().contains(k) ==> (
                                (table.spec_hash@)(k) as int % (table.current_size as int) < (i as int)
                                && table.table@[
                                    (table.spec_hash@)(k) as int % (table.current_size as int)
                                ].spec_entry_to_map().dom().contains(k)
                                && old_outer_pairs_map[k] == table.table@[
                                    (table.spec_hash@)(k) as int % (table.current_size as int)
                                ].spec_entry_to_map()[k]
                            ),
                            forall |j: int, k: Key| 0 <= j < (i as int)
                                && #[trigger] table.table@[j].spec_entry_to_map().dom().contains(k)
                                ==> old_outer_pairs_map.dom().contains(k),
                            obeys_feq_clone::<Key>(),
                            obeys_feq_clone::<Value>(),
                        decreases current,
                    {
                        let node_box = current.unwrap();
                        let Node { key: nk, value: nv, next: nn } = *node_box;
                        let ck = clone_elem(&nk);
                        let cv = clone_elem(&nv);
                        let ghost old_pairs = pairs@;
                        let ghost old_inner = inner_collected;

                        pairs.push((ck, cv));

                        proof {
                            reveal_with_fuel(spec_chain_to_map, 2);
                            reveal_with_fuel(spec_chain_keys_unique, 2);

                            // nk is in entry[i] (from the chain partition).
                            assert(spec_chain_to_map(original_chain).dom().contains(nk));
                            assert(table.table@[i as int].spec_entry_to_map().dom().contains(nk));
                            // By wf contrapositive: i == hash(nk) % size.
                            assert(
                                (table.spec_hash@)(nk) as int % (table.current_size as int)
                                    == (i as int)
                            );
                            // nk not in old_outer (which has keys from buckets < i only).
                            assert(!old_outer_pairs_map.dom().contains(nk));

                            // Update ghost state.
                            inner_collected = old_inner.insert(nk, nv);
                            pairs_map = pairs_map.insert(nk, nv);

                            // Maintain pairs_map =~= spec_seq_pairs_to_map(pairs@).
                            assert(pairs@ =~= old_pairs.push((nk, nv)));
                            assert(pairs@.drop_last() =~= old_pairs);
                            assert(pairs@.last() == (nk, nv));

                            // Maintain pairs_map =~= old_outer.upr(inner_collected).
                            // M.upr(N).insert(k,v) =~= M.upr(N.insert(k,v)).
                            assert(
                                old_outer_pairs_map.union_prefer_right(old_inner).insert(nk, nv)
                                =~= old_outer_pairs_map.union_prefer_right(
                                    old_inner.insert(nk, nv)
                                )
                            );

                            // Maintain chain partition invariant.
                            // inner.insert(nk,nv).upr(chain_to_map(nn))
                            //   = inner.upr(chain_to_map(nn)).insert(nk,nv)  [nk ∉ chain_to_map(nn)]
                            assert(
                                old_inner.insert(nk, nv)
                                    .union_prefer_right(spec_chain_to_map(nn))
                                =~= old_inner
                                    .union_prefer_right(spec_chain_to_map(nn))
                                    .insert(nk, nv)
                            );

                            // Maintain inner/current disjointness.
                            assert forall |k: Key|
                                #[trigger] inner_collected.dom().contains(k)
                                implies !spec_chain_to_map(nn).dom().contains(k)
                            by {
                                if k == nk {
                                    // By chain uniqueness: nk ∉ chain_to_map(nn).
                                } else {
                                    // k ∈ old_inner, k ∉ chain_to_map(old_current).
                                    // chain_to_map(nn) ⊂ chain_to_map(old_current),
                                    // so k ∉ chain_to_map(nn).
                                    assert(
                                        spec_chain_to_map(nn).dom().contains(k) ==>
                                        spec_chain_to_map(nn)
                                            .insert(nk, nv).dom().contains(k)
                                    );
                                }
                            }
                        }

                        current = nn;
                    }

                    proof {
                        // After inner loop: current == None.
                        reveal_with_fuel(spec_chain_to_map, 1);
                        // inner_collected.upr(Map::empty()) =~= inner_collected
                        //   =~= chain_to_map(original_chain).
                        assert(inner_collected =~= spec_chain_to_map(original_chain));
                        assert(pairs_map =~= old_outer_pairs_map.union_prefer_right(
                            spec_chain_to_map(original_chain)
                        ));

                        // Re-establish outer forward invariant at i+1.
                        assert forall |k: Key| #[trigger] pairs_map.dom().contains(k) implies (
                            (table.spec_hash@)(k) as int % (table.current_size as int)
                                < ((i + 1) as int)
                            && table.table@[
                                (table.spec_hash@)(k) as int % (table.current_size as int)
                            ].spec_entry_to_map().dom().contains(k)
                            && pairs_map[k] == table.table@[
                                (table.spec_hash@)(k) as int % (table.current_size as int)
                            ].spec_entry_to_map()[k]
                        ) by {
                            let bucket = (table.spec_hash@)(k) as int
                                % (table.current_size as int);
                            if inner_collected.dom().contains(k) {
                                // k from bucket i. hash(k)%size == i.
                                assert(table.table@[i as int]
                                    .spec_entry_to_map().dom().contains(k));
                                assert(bucket == i as int);
                                // pairs_map[k] via upr: inner_collected wins.
                                assert(pairs_map[k] == inner_collected[k]);
                                assert(inner_collected[k] ==
                                    spec_chain_to_map(original_chain)[k]);
                            } else {
                                // k from old_outer. Preserved.
                                assert(old_outer_pairs_map.dom().contains(k));
                                assert(pairs_map[k] == old_outer_pairs_map[k]);
                            }
                        }

                        // Re-establish outer backward invariant at i+1.
                        assert forall |j: int, k: Key| 0 <= j < ((i + 1) as int)
                            && #[trigger] table.table@[j]
                                .spec_entry_to_map().dom().contains(k)
                            implies pairs_map.dom().contains(k)
                        by {
                            if j < (i as int) {
                                assert(old_outer_pairs_map.dom().contains(k));
                            } else {
                                // j == i. k in entry[i] = chain_to_map(original_chain).
                                assert(inner_collected.dom().contains(k));
                            }
                        }
                    }

                    i = i + 1;
                }

                proof {
                    // After Phase 1: pairs_map =~= table@.
                    assert forall |k: Key|
                        (#[trigger] pairs_map.dom().contains(k)) <==>
                        (#[trigger] spec_table_to_map::<Key, Value, ChainList<Key, Value>>(
                            table.table@
                        )).dom().contains(k)
                    by {
                        if pairs_map.dom().contains(k) {
                            let bucket = (table.spec_hash@)(k) as int
                                % (table.current_size as int);
                            assert(table.table@[bucket]
                                .spec_entry_to_map().dom().contains(k));
                            assert forall |jj: int|
                                0 <= jj < table.table@.len() && jj != bucket
                                implies !#[trigger] table.table@[jj]
                                    .spec_entry_to_map().dom().contains(k)
                            by {}
                            lemma_table_to_map_unique_entry_value::<
                                Key, Value, ChainList<Key, Value>,
                            >(table.table@, bucket, k);
                        }
                        if spec_table_to_map::<Key, Value, ChainList<Key, Value>>(
                            table.table@
                        ).dom().contains(k)
                            && !pairs_map.dom().contains(k)
                        {
                            assert forall |j: int| 0 <= j < table.table@.len()
                                implies !#[trigger] table.table@[j]
                                    .spec_entry_to_map().dom().contains(k)
                            by {}
                            lemma_table_to_map_not_contains::<
                                Key, Value, ChainList<Key, Value>,
                            >(table.table@, k);
                        }
                    }
                    assert forall |k: Key|
                        #[trigger] pairs_map.dom().contains(k)
                        && spec_table_to_map::<Key, Value, ChainList<Key, Value>>(
                            table.table@
                        ).dom().contains(k)
                        implies pairs_map[k] == spec_table_to_map::<
                            Key, Value, ChainList<Key, Value>,
                        >(table.table@)[k]
                    by {
                        let bucket = (table.spec_hash@)(k) as int
                            % (table.current_size as int);
                        assert forall |jj: int|
                            0 <= jj < table.table@.len() && jj != bucket
                            implies !#[trigger] table.table@[jj]
                                .spec_entry_to_map().dom().contains(k)
                        by {}
                        lemma_table_to_map_unique_entry_value::<
                            Key, Value, ChainList<Key, Value>,
                        >(table.table@, bucket, k);
                    }
                    assert(pairs_map =~= table@);
                }

                // Phase 2: create new empty table.
                let mut new_table_vec: Vec<ChainList<Key, Value>> = Vec::new();
                let mut k: usize = 0;
                while k < new_size
                    invariant
                        k <= new_size,
                        new_table_vec@.len() == k as int,
                        spec_table_to_map::<Key, Value, ChainList<Key, Value>>(new_table_vec@)
                            == Map::<Key, Value>::empty(),
                        forall |j: int| 0 <= j < new_table_vec@.len() ==>
                            (#[trigger] new_table_vec@[j]).spec_entry_to_map()
                                == Map::<Key, Value>::empty(),
                        forall |j: int| 0 <= j < new_table_vec@.len() ==>
                            spec_chain_keys_unique((#[trigger] new_table_vec@[j]).head),
                    decreases new_size - k,
                {
                    let ghost old_vec = new_table_vec@;
                    new_table_vec.push(ChainList { head: None });
                    proof {
                        reveal_with_fuel(spec_chain_keys_unique, 1);
                        reveal_with_fuel(spec_chain_to_map, 1);
                        lemma_table_to_map_push_empty::<Key, Value, ChainList<Key, Value>>(
                            old_vec, new_table_vec@.last(),
                        );
                        assert(new_table_vec@ =~= old_vec.push(new_table_vec@.last()));
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
                    // Establish spec_parahashtablesteph_wf for the empty new table.
                    assert forall |kk: Key, j: int|
                        0 <= j < new_table.table@.len()
                        && j != (new_table.spec_hash@)(kk) as int
                            % (new_table.current_size as int)
                        implies !#[trigger] new_table.table@[j]
                            .spec_entry_to_map().dom().contains(kk)
                    by {}
                    assert(new_table@ =~= Map::<Key, Value>::empty());
                    assert(spec_seq_pairs_to_map::<Key, Value>(
                        pairs@.subrange(0, 0int)
                    ) =~= Map::<Key, Value>::empty());
                }

                // Phase 3: reinsert all collected pairs.
                let mut m: usize = 0;
                while m < pairs.len()
                    invariant
                        m <= pairs@.len(),
                        new_size > 0,
                        new_table.current_size == new_size,
                        new_table.table@.len() == new_size as int,
                        new_table.num_elements <= m,
                        Self::spec_parahashtablesteph_wf(&new_table),
                        new_table.spec_hash == table.spec_hash,
                        new_table@ =~= spec_seq_pairs_to_map::<Key, Value>(
                            pairs@.subrange(0, m as int)
                        ),
                        obeys_feq_clone::<Key>(),
                        obeys_feq_clone::<Value>(),
                    decreases pairs.len() - m,
                {
                    let key = clone_elem(&pairs[m].0);
                    let value = clone_elem(&pairs[m].1);
                    Self::insert(&mut new_table, key, value);
                    proof {
                        assert(
                            pairs@.subrange(0, (m + 1) as int).drop_last()
                            =~= pairs@.subrange(0, m as int)
                        );
                        assert(
                            pairs@.subrange(0, (m + 1) as int).last()
                            == pairs@[m as int]
                        );
                    }
                    m = m + 1;
                }

                proof {
                    assert(pairs@.subrange(0, pairs@.len() as int) =~= pairs@);
                    // new_table@ =~= spec_seq_pairs_to_map(pairs@) =~= pairs_map =~= table@.
                    assert(new_table@ =~= table@);
                }

                new_table
            }
        }

        impl<Key: StT, Value: StT, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>
            ChainedHashTable<Key, Value, ChainList<Key, Value>, Metrics, H>
            for StructChainedHashTableStEph
        {
            /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to stored hash function.
            fn hash_index(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics, H>, key: &Key) -> (index: usize) {
                call_hash_fn(&table.hash_fn, key, table.current_size, table.spec_hash)
            }
        }

    //		Section 12a. derive impls in verus!


        #[cfg(verus_keep_ghost)]
        impl<Key: PartialEq, Value: PartialEq> PartialEqSpecImpl for Node<Key, Value> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
        }


        impl<Key: Clone, Value: Clone> Clone for Node<Key, Value> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned == *self
                decreases self
            {
                let cloned = Node {
                    key: self.key.clone(),
                    value: self.value.clone(),
                    next: match &self.next {
                        None => None,
                        Some(b) => Some(Box::new((**b).clone())),
                    },
                };
                proof { assume(cloned == *self); }
                cloned
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::PartialEq for Node<Key, Value> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (*self == *other)
                decreases self, other
            {
                let equal = self.key == other.key
                    && self.value == other.value
                    && match (&self.next, &other.next) {
                        (None, None) => true,
                        (Some(a), Some(b)) => (**a) == (**b),
                        _ => false,
                    };
                proof { assume(equal == (*self == *other)); }
                equal
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::Eq for Node<Key, Value> {}

    //		Section 12b. derive impls in verus!


        #[cfg(verus_keep_ghost)]
        impl<Key: PartialEq, Value: PartialEq> PartialEqSpecImpl for ChainList<Key, Value> {
            open spec fn obeys_eq_spec() -> bool { true }
            open spec fn eq_spec(&self, other: &Self) -> bool { *self == *other }
        }

        impl<Key, Value> Default for ChainList<Key, Value> {
            fn default() -> Self { ChainList { head: None } }
        }

        impl<Key: Clone, Value: Clone> Clone for ChainList<Key, Value> {
            fn clone(&self) -> (cloned: Self)
                ensures cloned == *self
            {
                let cloned = ChainList {
                    head: match &self.head {
                        None => None,
                        Some(b) => Some(Box::new((**b).clone())),
                    },
                };
                proof { assume(cloned == *self); }
                cloned
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::PartialEq for ChainList<Key, Value> {
            fn eq(&self, other: &Self) -> (equal: bool)
                ensures equal == (*self == *other)
            {
                let equal = match (&self.head, &other.head) {
                    (None, None) => true,
                    (Some(a), Some(b)) => (**a) == (**b),
                    _ => false,
                };
                proof { assume(equal == (*self == *other)); }
                equal
            }
        }

        impl<Key: PartialEq, Value: PartialEq> core::cmp::Eq for ChainList<Key, Value> {}
    }

    //		Section 14a. derive impls outside verus!


    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for Node<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node")
                .field("key", &self.key)
                .field("value", &self.value)
                .field("next", &self.next)
                .finish()
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for Node<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for ChainList<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ChainList")
                .field("head", &self.head)
                .finish()
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for ChainList<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    //		Section 14c. derive impls outside verus!

    impl std::fmt::Debug for StructChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StructChainedHashTableStEph")
        }
    }

    impl std::fmt::Display for StructChainedHashTableStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StructChainedHashTableStEph")
        }
    }
}
