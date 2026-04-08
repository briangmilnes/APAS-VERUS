//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Parametric Nested Hash Table - Sequential Ephemeral (Chapter 47, Section 1.1).
//! A parametric implementation of hash tables using nested tables.
//! Work: insert O(1), lookup O(1), delete O(1) expected with constant load factor.
//! Span: O(1) (sequential).

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod ParaHashTableStEph {

    //
    // Note: spec_seq_pairs_to_map lives here because both Flat and Chained impls use it.
    // The lemma_seq_pairs_* proofs (chained-family only) live in ChainedHashTable.rs.
    // spec_hashtable_wf lives here because the Para default spec_parahashtablesteph_wf references it;
    // moving it to Chained would create a circular import.

    //		Section 2. imports
    use std::fmt::Display;
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4a. type definitions


    #[derive(Clone, Copy, PartialEq)]
    pub struct LoadAndSize {
        pub load: usize,
        pub size: usize,
    }

    //		Section 4b. type definitions


    /// Parametric nested hash table structure.
    /// Generic `H` is the hash function type: takes (&Key, usize) and returns an index.
    #[verifier::reject_recursive_types(Key)]
    pub struct HashTable<Key, Value, Entry, Metrics, H> {
        pub table: Vec<Entry>,
        pub hash_fn: H,
        pub initial_size: usize,
        pub current_size: usize,
        pub num_elements: usize,
        pub metrics: Metrics,
        pub spec_hash: Ghost<spec_fn(Key) -> nat>,
        pub _phantom: PhantomData<(Key, Value)>,
    }

    //		Section 5b. view impls


    impl<Key, Value, Entry: EntryTrait<Key, Value>, Metrics, H> View for HashTable<Key, Value, Entry, Metrics, H> {
        type V = Map<Key, Value>;
        open spec fn view(&self) -> Map<Key, Value> {
            spec_table_to_map(self.table@)
        }
    }

    //		Section 6b. spec fns


    pub open spec fn spec_hashtable_wf<Key, Value, Entry: EntryTrait<Key, Value>, Metrics, H: Fn(&Key, usize) -> usize>(table: &HashTable<Key, Value, Entry, Metrics, H>) -> bool {
        table.table@.len() == table.current_size as int
        && table.current_size > 0
        && (forall |k: Key, j: int| 0 <= j < table.table@.len()
            && j != (table.spec_hash@)(k) as int % table.current_size as int
            ==> !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k))
        && spec_hash_fn_valid::<Key, H>(table.spec_hash@)
    }

    /// Maps a sequence of key-value pairs to its abstract Map representation.
    pub open spec fn spec_seq_pairs_to_map<Key, Value>(
        pairs: Seq<(Key, Value)>,
    ) -> Map<Key, Value>
        decreases pairs.len(),
    {
        if pairs.len() == 0 {
            Map::empty()
        } else {
            spec_seq_pairs_to_map(pairs.drop_last()).insert(pairs.last().0, pairs.last().1)
        }
    }

    /// Maps a table (sequence of entries) to its abstract Map representation.
    pub open spec fn spec_table_to_map<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
    ) -> Map<Key, Value>
        decreases table.len(),
    {
        if table.len() == 0 {
            Map::empty()
        } else {
            spec_table_to_map(table.drop_last()).union_prefer_right(
                table.last().spec_entry_to_map()
            )
        }
    }

    /// Whether only one slot changed between old and new table sequences.
    /// Used as trigger for existential quantifier in insert ensures.
    pub open spec fn spec_other_slots_preserved<Entry>(
        old_seq: Seq<Entry>, new_seq: Seq<Entry>, s: int,
    ) -> bool {
        &&& 0 <= s < new_seq.len()
        &&& old_seq.len() == new_seq.len()
        &&& forall |j: int| 0 <= j < new_seq.len() && j != s
            ==> #[trigger] new_seq[j] == old_seq[j]
    }

    /// Whether all values of hash function type H produce valid indices matching spec_hash.
    /// Quantified over all H values (not a specific instance) so the property survives Clone.
    pub open spec fn spec_hash_fn_valid<Key, H: Fn(&Key, usize) -> usize>(
        spec_hash: spec_fn(Key) -> nat,
    ) -> bool {
        (forall|h: H, k: &Key, ts: usize| ts > 0 ==> #[trigger] h.requires((k, ts)))
        && (forall|h: H, k: &Key, ts: usize, idx: usize|
            ts > 0 && #[trigger] h.ensures((k, ts), idx)
                ==> idx < ts && idx as nat == spec_hash(*k) % (ts as nat))
    }

    //		Section 7b. proof fns/broadcast groups


    /// All-empty entries produce an empty map when composed by spec_table_to_map.
    pub proof fn lemma_table_to_map_push_empty<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
        entry: Entry,
    )
        requires
            spec_table_to_map(table) == Map::<Key, Value>::empty(),
            entry.spec_entry_to_map() == Map::<Key, Value>::empty(),
        ensures
            spec_table_to_map(table.push(entry)) == Map::<Key, Value>::empty(),
    {
        assert(table.push(entry).drop_last() == table);
        // spec_table_to_map(table.push(entry))
        //   = spec_table_to_map(table).union_prefer_right(entry.spec_entry_to_map())
        //   = Map::empty().union_prefer_right(Map::empty())
        //   = Map::empty()
    }

    /// If the new entry's map contains key, so does spec_table_to_map after the update.
    pub proof fn lemma_table_to_map_update_contains<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
        index: int,
        new_entry: Entry,
        key: Key,
    )
        requires
            0 <= index < table.len(),
            new_entry.spec_entry_to_map().dom().contains(key),
        ensures
            spec_table_to_map(table.update(index, new_entry)).dom().contains(key),
        decreases table.len(),
    {
        let updated = table.update(index, new_entry);
        if index == table.len() - 1 {
            // Updated element is the last: union_prefer_right includes its domain.
            assert(updated.drop_last() == table.drop_last());
        } else {
            // Updated element is before last: recurse on drop_last.
            assert(updated.drop_last() == table.drop_last().update(index, new_entry));
            lemma_table_to_map_update_contains(table.drop_last(), index, new_entry, key);
        }
    }

    /// If key is absent from every entry's map, it is absent from spec_table_to_map.
    pub proof fn lemma_table_to_map_not_contains<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
        key: Key,
    )
        requires
            forall |j: int| 0 <= j < table.len()
                ==> !#[trigger] table[j].spec_entry_to_map().dom().contains(key),
        ensures
            !spec_table_to_map(table).dom().contains(key),
        decreases table.len(),
    {
        if table.len() > 0 {
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
        }
    }

    /// If one entry's map changes from M to M.insert(key, value), and key does not appear
    /// in any other entry's map, then spec_table_to_map gains exactly key→value.
    pub proof fn lemma_table_to_map_update_insert<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
        index: int,
        new_entry: Entry,
        key: Key,
        value: Value,
    )
        requires
            0 <= index < table.len(),
            new_entry.spec_entry_to_map() == table[index].spec_entry_to_map().insert(key, value),
            forall |j: int| 0 <= j < table.len() && j != index
                ==> !#[trigger] table[j].spec_entry_to_map().dom().contains(key),
        ensures
            spec_table_to_map(table.update(index, new_entry))
                == spec_table_to_map(table).insert(key, value),
        decreases table.len(),
    {
        let updated = table.update(index, new_entry);
        if index == table.len() - 1 {
            assert(updated.drop_last() =~= table.drop_last());
            // key is not in any entry in drop_last (all have j != index).
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
            // rest.union_prefer_right(old_map.insert(key, value))
            //   =~= rest.union_prefer_right(old_map).insert(key, value)
            // when key not in rest.
        } else {
            assert(updated.drop_last() =~= table.drop_last().update(index, new_entry));
            // Precondition for recursive call: key not in entries j != index of drop_last.
            assert forall |j: int| 0 <= j < table.drop_last().len() && j != index
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
            }
            // Entry at index in drop_last matches table.
            lemma_table_to_map_update_insert::<Key, Value, Entry>(
                table.drop_last(), index, new_entry, key, value);
            // key not in last entry's map.
            // rest.insert(key, value).union_prefer_right(last_map)
            //   =~= rest.union_prefer_right(last_map).insert(key, value)
            // when key not in last_map.
        }
    }

    /// If one entry's map changes from M to M.remove(key), and key does not appear
    /// in any other entry's map, then spec_table_to_map loses exactly key.
    pub proof fn lemma_table_to_map_update_remove<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
        index: int,
        new_entry: Entry,
        key: Key,
    )
        requires
            0 <= index < table.len(),
            new_entry.spec_entry_to_map() == table[index].spec_entry_to_map().remove(key),
            forall |j: int| 0 <= j < table.len() && j != index
                ==> !#[trigger] table[j].spec_entry_to_map().dom().contains(key),
        ensures
            spec_table_to_map(table.update(index, new_entry))
                == spec_table_to_map(table).remove(key),
        decreases table.len(),
    {
        let updated = table.update(index, new_entry);
        if index == table.len() - 1 {
            assert(updated.drop_last() =~= table.drop_last());
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
        } else {
            assert(updated.drop_last() =~= table.drop_last().update(index, new_entry));
            assert forall |j: int| 0 <= j < table.drop_last().len() && j != index
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
            }
            lemma_table_to_map_update_remove::<Key, Value, Entry>(
                table.drop_last(), index, new_entry, key);
        }
    }

    /// If key appears in exactly one entry's map (at index), then
    /// spec_table_to_map gives that entry's value for the key.
    pub proof fn lemma_table_to_map_unique_entry_value<Key, Value, Entry: EntryTrait<Key, Value>>(
        table: Seq<Entry>,
        index: int,
        key: Key,
    )
        requires
            0 <= index < table.len(),
            table[index].spec_entry_to_map().dom().contains(key),
            forall |j: int| 0 <= j < table.len() && j != index
                ==> !#[trigger] table[j].spec_entry_to_map().dom().contains(key),
        ensures
            spec_table_to_map(table).dom().contains(key),
            spec_table_to_map(table)[key] == table[index].spec_entry_to_map()[key],
        decreases table.len(),
    {
        if index == table.len() - 1 {
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
        } else {
            assert forall |j: int| 0 <= j < table.drop_last().len() && j != index
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
            }
            lemma_table_to_map_unique_entry_value::<Key, Value, Entry>(table.drop_last(), index, key);
        }
    }

    //		Section 8b. traits


    /// Trait for parametric nested hash tables.
    /// Entry type must implement this trait to define how Key and Value are stored.
    pub trait EntryTrait<Key, Value> : Sized {
        /// Abstract map view of this entry's key-value content.
        spec fn spec_entry_to_map(&self) -> Map<Key, Value>;
        /// - Alg Analysis: APAS: N/A — inner table interface, cost depends on implementation.
        /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — abstract trait method.
        fn new() -> (entry: Self)
            ensures entry.spec_entry_to_map() == Map::<Key, Value>::empty();
        /// - Alg Analysis: APAS (Ch47 Def 47.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) expected, Span O(1) expected — matches APAS
        fn insert(&mut self, key: Key, value: Value)
            ensures self.spec_entry_to_map().dom().contains(key);
        /// - Alg Analysis: APAS (Ch47 Def 47.3): Work O(1 + alpha), Span O(1 + alpha)
        /// - Alg Analysis: APAS (Ch47 Alg 47.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) expected, Span O(1) expected — matches APAS
        fn lookup(&self, key: &Key) -> (found: Option<Value>);
        /// - Alg Analysis: APAS (Ch47 Def 47.3): Work O(1 + alpha), Span O(1 + alpha)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1 + α) expected, Span O(1 + α) expected — matches APAS
        fn delete(&mut self, key: &Key) -> (deleted: bool)
            ensures !deleted ==> self.spec_entry_to_map() == old(self).spec_entry_to_map();
        /// Element-wise clone that avoids Verus tuple-Clone limitation.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — copies all chain entries.
        fn clone_entry(&self) -> (cloned: Self);
    }

    /// Trait for parametric nested hash tables.
    pub trait ParaHashTableStEphTrait<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone> {
        /// Per-implementation well-formedness predicate.
        /// Chained hash tables use the default (spec_hashtable_wf: key at hash slot).
        /// Flat/open-addressing hash tables override with probe-chain wf.
        open spec fn spec_parahashtablesteph_wf(table: &HashTable<Key, Value, Entry, Metrics, H>) -> bool {
            spec_hashtable_wf(table)
        }

        /// Whether the table has capacity for an insertion.
        /// Default true for chained tables; flat tables override to require an empty slot.
        open spec fn spec_has_insert_capacity(table: &HashTable<Key, Value, Entry, Metrics, H>) -> bool {
            true
        }

        /// Whether resize to new_size is valid for this implementation.
        /// Default true for chained tables; flat tables override.
        open spec fn spec_resize_ok(table: &HashTable<Key, Value, Entry, Metrics, H>, new_size: usize) -> bool {
            true
        }

        /// Creates an empty hash table with the given initial size.
        /// Takes a hash function that maps (&Key, table_size) to a bucket index.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(m), Span O(m) where m is initial size.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — agrees with APAS; iterates m times to create entries.
        fn createTable(hash_fn: H, initial_size: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (table: HashTable<Key, Value, Entry, Metrics, H>)
            requires
                initial_size > 0,
                spec_hash_fn_valid::<Key, H>(spec_hash@),
            ensures
                table.initial_size == initial_size,
                table.current_size == initial_size,
                table.num_elements == 0,
                table.table@.len() == initial_size as int,
                spec_hashtable_wf(&table),
                table@ == Map::<Key, Value>::empty(),
                table.spec_hash == spec_hash,
        {
            let mut table_vec: Vec<Entry> = Vec::new();
            let mut i: usize = 0;
            while i < initial_size
                invariant
                    i <= initial_size,
                    table_vec@.len() == i as int,
                    spec_table_to_map::<Key, Value, Entry>(table_vec@) == Map::<Key, Value>::empty(),
                    forall |j: int| 0 <= j < table_vec@.len()
                        ==> (#[trigger] table_vec@[j]).spec_entry_to_map() == Map::<Key, Value>::empty(),
                decreases initial_size - i,
            {
                let ghost old_view = table_vec@;
                table_vec.push(Entry::new());
                proof {
                    lemma_table_to_map_push_empty::<Key, Value, Entry>(old_view, table_vec@.last());
                }
                i += 1;
            }
            proof {
                assert forall |k: Key, j: int| 0 <= j < table_vec@.len()
                    && j != (spec_hash@)(k) as int % initial_size as int
                    implies !#[trigger] table_vec@[j].spec_entry_to_map().dom().contains(k) by {}
            }
            HashTable {
                table: table_vec,
                hash_fn,
                initial_size,
                current_size: initial_size,
                num_elements: 0,
                metrics: Metrics::default(),
                spec_hash,
                _phantom: PhantomData,
            }
        }

        /// Inserts a key-value pair into the hash table.
        /// - Alg Analysis: APAS (Ch47 Def 47.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) expected, Span O(1) expected — matches APAS
        fn insert(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: Key, value: Value)
            requires
                Self::spec_parahashtablesteph_wf(old(table)),
                old(table).num_elements < usize::MAX,
                Self::spec_has_insert_capacity(old(table)),
                obeys_feq_clone::<Key>(),
                obeys_feq_clone::<Value>(),
            ensures
                table@ == old(table)@.insert(key, value),
                Self::spec_parahashtablesteph_wf(table),
                table.spec_hash == old(table).spec_hash,
                table.current_size == old(table).current_size,
                table.num_elements <= old(table).num_elements + 1,
                exists |s: int| #[trigger] spec_other_slots_preserved(
                    old(table).table@, table.table@, s);

        /// Looks up a key in the hash table, returning its value if found.
        /// - Alg Analysis: APAS (Ch47 Def 47.3): Work O(1 + alpha), Span O(1 + alpha)
        /// - Alg Analysis: APAS (Ch47 Alg 47.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) expected, Span O(1) expected — matches APAS
        fn lookup(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (found: Option<Value>)
            requires
                Self::spec_parahashtablesteph_wf(table),
                obeys_feq_clone::<Key>(),
                obeys_feq_clone::<Value>(),
            ensures
                table@.dom().contains(*key) ==> found == Some(table@[*key]),
                !table@.dom().contains(*key) ==> found is None;

        /// Deletes a key from the hash table if it exists.
        /// - Alg Analysis: APAS (Ch47 Def 47.3): Work O(1 + alpha), Span O(1 + alpha)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1 + α) expected, Span O(1 + α) expected — matches APAS
        fn delete(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (deleted: bool)
            requires
                Self::spec_parahashtablesteph_wf(old(table)),
                obeys_feq_clone::<Key>(),
                obeys_feq_clone::<Value>(),
            ensures
                deleted == old(table)@.dom().contains(*key),
                table@ == old(table)@.remove(*key),
                Self::spec_parahashtablesteph_wf(table),
                table.spec_hash == old(table).spec_hash,
                table.current_size == old(table).current_size;

        /// Accessor for metrics field.
        /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
        fn metrics(table: &HashTable<Key, Value, Entry, Metrics, H>) -> (m: &Metrics)
            requires Self::spec_parahashtablesteph_wf(table),
            ensures m == &table.metrics,
        { &table.metrics }

        /// Returns the load (number of elements) and size (table capacity).
        /// Load factor α = load/size.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS; field reads only.
        fn loadAndSize(table: &HashTable<Key, Value, Entry, Metrics, H>) -> (load_and_size: LoadAndSize)
            requires Self::spec_parahashtablesteph_wf(table),
            ensures
                load_and_size.size == table.current_size,
                load_and_size.load == table.num_elements,
        {
            LoadAndSize {
                load: table.num_elements,
                size: table.current_size,
            }
        }

        /// Resizes the hash table to a new size and rehashes all entries.
        /// Clones the stored hash function for the new table.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(n + m + m'), Span O(n + m + m') where n is number of elements,
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m + m'), Span O(n + m + m') where n is number of elements, — matches APAS
        ///   m is old size, m' is new size.
        /// - Alg Analysis: APAS (Ch47 ref): N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — abstract trait method; cost depends on implementation.
        fn resize(table: &HashTable<Key, Value, Entry, Metrics, H>, new_size: usize) -> (resized: HashTable<Key, Value, Entry, Metrics, H>)
            requires
                new_size > 0,
                Self::spec_parahashtablesteph_wf(table),
                Self::spec_resize_ok(table, new_size),
                obeys_feq_clone::<Key>(),
                obeys_feq_clone::<Value>(),
            ensures
                resized@ == table@,
                resized.current_size == new_size,
                resized.table@.len() == new_size as int,
                Self::spec_parahashtablesteph_wf(&resized),
                resized.spec_hash == table.spec_hash;
    }

    //		Section 9b. impls


    /// Clone bridge for generic element: requires obeys_feq_clone so axiom_cloned_implies_eq fires.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single element clone.
    pub fn clone_elem<T: Eq + Clone>(x: &T) -> (c: T)
        requires obeys_feq_clone::<T>(),
        ensures c == *x,
    {
        let c = x.clone();
        assert(cloned(*x, c));
        c
    }

    // 7a. helpers

    /// Calls the hash function and returns a bucket index.
    /// Closure specs bridge the exec hash_fn to the ghost spec_hash via spec_hash_fn_valid.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single hash function call.
    pub fn call_hash_fn<Key, H: Fn(&Key, usize) -> usize>(hash_fn: &H, key: &Key, table_size: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (index: usize)
        requires
            table_size > 0,
            spec_hash_fn_valid::<Key, H>(spec_hash@),
        ensures
            index < table_size,
            index as nat == (spec_hash@)(*key) % (table_size as nat),
    {
        (hash_fn)(key, table_size)
    }
    } // verus!

    //		Section 14a. derive impls outside verus!


    impl std::fmt::Debug for LoadAndSize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LoadAndSize")
                .field("load", &self.load)
                .field("size", &self.size)
                .finish()
        }
    }

    impl std::fmt::Display for LoadAndSize {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LoadAndSize(load={}, size={}, α={:.3})", self.load, self.size,
                if self.size == 0 { 0.0 } else { self.load as f64 / self.size as f64 })
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<Key, Value, Entry: std::fmt::Debug, Metrics: std::fmt::Debug, H> std::fmt::Debug
        for HashTable<Key, Value, Entry, Metrics, H>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("HashTable")
                .field("current_size", &self.current_size)
                .field("num_elements", &self.num_elements)
                .field("initial_size", &self.initial_size)
                .finish()
        }
    }

    impl<Key, Value, Entry: std::fmt::Debug, Metrics: std::fmt::Debug, H> std::fmt::Display
        for HashTable<Key, Value, Entry, Metrics, H>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HashTable(size={}, elements={})", self.current_size, self.num_elements)
        }
    }
}
