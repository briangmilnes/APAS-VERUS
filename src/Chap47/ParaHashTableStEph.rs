//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parametric Nested Hash Table - Sequential Ephemeral (Chapter 47, Section 1.1).
//! A parametric implementation of hash tables using nested tables.
//! Work: insert O(1), lookup O(1), delete O(1) expected with constant load factor.
//! Span: O(1) (sequential).

pub mod ParaHashTableStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions (inside verus!: LoadAndSize, HashTable)
    // 6. spec fns (inside verus!: spec_hashtable_wf, spec_seq_pairs_to_map, spec_table_to_map)
    // 8. traits (inside verus!: EntryTrait, ParaHashTableStEphTrait)
    // 9. impls (inside verus!: View for HashTable)
    // 13. derive impls outside verus!

    // 2. imports
    use std::fmt::Display;
    use std::marker::PhantomData;

    use vstd::prelude::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    #[derive(Clone, Copy, PartialEq)]
    pub struct LoadAndSize {
        pub load: usize,
        pub size: usize,
    }

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

    // 6. spec fns

    pub open spec fn spec_hashtable_wf<Key, Value, Entry: EntryTrait<Key, Value>, Metrics, H>(table: &HashTable<Key, Value, Entry, Metrics, H>) -> bool {
        table.table@.len() == table.current_size as int
        && table.current_size > 0
        && forall |k: Key, j: int| 0 <= j < table.table@.len()
            && j != (table.spec_hash@)(k) as int % table.current_size as int
            ==> !#[trigger] table.table@[j].spec_entry_to_map().dom().contains(k)
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

    // 7. proof fns

    /// All-empty entries produce an empty map when composed by spec_table_to_map.
    proof fn lemma_table_to_map_push_empty<Key, Value, Entry: EntryTrait<Key, Value>>(
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
        assert(table.push(entry).last() == entry);
        // spec_table_to_map(table.push(entry))
        //   = spec_table_to_map(table).union_prefer_right(entry.spec_entry_to_map())
        //   = Map::empty().union_prefer_right(Map::empty())
        //   = Map::empty()
        assert(Map::<Key, Value>::empty().union_prefer_right(Map::<Key, Value>::empty()) =~= Map::<Key, Value>::empty());
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
            assert(updated.last() == new_entry);
            assert(spec_table_to_map(updated) ==
                spec_table_to_map(table.drop_last()).union_prefer_right(
                    new_entry.spec_entry_to_map()));
            assert(spec_table_to_map(updated).dom() =~=
                spec_table_to_map(table.drop_last()).dom().union(
                    new_entry.spec_entry_to_map().dom()));
        } else {
            // Updated element is before last: recurse on drop_last.
            assert(updated.drop_last() == table.drop_last().update(index, new_entry));
            assert(updated.last() == table.last());
            lemma_table_to_map_update_contains(table.drop_last(), index, new_entry, key);
            assert(spec_table_to_map(updated) ==
                spec_table_to_map(table.drop_last().update(index, new_entry)).union_prefer_right(
                    table.last().spec_entry_to_map()));
            assert(spec_table_to_map(updated).dom() =~=
                spec_table_to_map(table.drop_last().update(index, new_entry)).dom().union(
                    table.last().spec_entry_to_map().dom()));
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
            assert forall |j: int| 0 <= j < table.drop_last().len()
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
                assert(table.drop_last()[j] == table[j]);
            }
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
            assert(spec_table_to_map(table).dom() =~=
                spec_table_to_map(table.drop_last()).dom().union(
                    table.last().spec_entry_to_map().dom()));
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
            assert(updated.last() == new_entry);
            // key is not in any entry in drop_last (all have j != index).
            assert forall |j: int| 0 <= j < table.drop_last().len()
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
                assert(table.drop_last()[j] == table[j]);
            }
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
            // rest.union_prefer_right(old_map.insert(key, value))
            //   =~= rest.union_prefer_right(old_map).insert(key, value)
            // when key not in rest.
            assert(spec_table_to_map(updated) =~= spec_table_to_map(table).insert(key, value));
        } else {
            assert(updated.drop_last() =~= table.drop_last().update(index, new_entry));
            assert(updated.last() == table.last());
            // Precondition for recursive call: key not in entries j != index of drop_last.
            assert forall |j: int| 0 <= j < table.drop_last().len() && j != index
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
                assert(table.drop_last()[j] == table[j]);
            }
            // Entry at index in drop_last matches table.
            assert(table.drop_last()[index] == table[index]);
            lemma_table_to_map_update_insert::<Key, Value, Entry>(
                table.drop_last(), index, new_entry, key, value);
            // key not in last entry's map.
            assert(!table.last().spec_entry_to_map().dom().contains(key));
            // rest.insert(key, value).union_prefer_right(last_map)
            //   =~= rest.union_prefer_right(last_map).insert(key, value)
            // when key not in last_map.
            assert(spec_table_to_map(updated) =~= spec_table_to_map(table).insert(key, value));
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
            assert(updated.last() == new_entry);
            assert forall |j: int| 0 <= j < table.drop_last().len()
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
                assert(table.drop_last()[j] == table[j]);
            }
            lemma_table_to_map_not_contains::<Key, Value, Entry>(table.drop_last(), key);
            assert(spec_table_to_map(updated) =~= spec_table_to_map(table).remove(key));
        } else {
            assert(updated.drop_last() =~= table.drop_last().update(index, new_entry));
            assert(updated.last() == table.last());
            assert forall |j: int| 0 <= j < table.drop_last().len() && j != index
                implies !#[trigger] table.drop_last()[j].spec_entry_to_map().dom().contains(key) by {
                assert(table.drop_last()[j] == table[j]);
            }
            assert(table.drop_last()[index] == table[index]);
            lemma_table_to_map_update_remove::<Key, Value, Entry>(
                table.drop_last(), index, new_entry, key);
            assert(!table.last().spec_entry_to_map().dom().contains(key));
            assert(spec_table_to_map(updated) =~= spec_table_to_map(table).remove(key));
        }
    }

    // 7a. helpers

    /// Calls the hash function and returns a bucket index.
    /// External_body because Verus cannot reason about opaque Fn closures.
    #[verifier::external_body]
    pub fn call_hash_fn<Key, H: Fn(&Key, usize) -> usize>(hash_fn: &H, key: &Key, table_size: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (index: usize)
        requires table_size > 0,
        ensures
            index < table_size,
            index as nat == (spec_hash@)(*key) % (table_size as nat),
    {
        (hash_fn)(key, table_size)
    }

    /// Linear probe: (hash(key) + attempt) % table_size.
    pub fn linear_probe<Key, H: Fn(&Key, usize) -> usize>(hash_fn: &H, key: &Key, table_size: usize, attempt: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (slot: usize)
        requires table_size > 0,
        ensures slot < table_size,
    {
        let h = call_hash_fn(hash_fn, key, table_size, spec_hash);
        (h.wrapping_add(attempt)) % table_size
    }

    /// Quadratic probe: (hash(key) + attempt + attempt^2) % table_size.
    pub fn quadratic_probe<Key, H: Fn(&Key, usize) -> usize>(hash_fn: &H, key: &Key, table_size: usize, attempt: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (slot: usize)
        requires table_size > 0,
        ensures slot < table_size,
    {
        let h = call_hash_fn(hash_fn, key, table_size, spec_hash);
        (h.wrapping_add(attempt).wrapping_add(attempt.wrapping_mul(attempt))) % table_size
    }

    /// Computes a second hash value for double hashing.
    /// External_body because Verus cannot reason about std::hash types.
    #[verifier::external_body]
    pub fn compute_second_hash<Key: std::hash::Hash>(key: &Key, table_size: usize) -> (step: usize)
        requires table_size > 0,
        ensures step >= 1,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        if table_size <= 1 {
            return 1;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let base = (table_size - 1) as u64;
        let mut step = ((hash % base) + 1) as usize;
        if step % 2 == 0 && step < table_size - 1 {
            step += 1;
        }
        step
    }

    /// Double hash probe: (hash(key) + attempt * second_hash(key, table_size)) % table_size.
    pub fn double_hash_probe<Key: std::hash::Hash, H: Fn(&Key, usize) -> usize>(hash_fn: &H, key: &Key, table_size: usize, attempt: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (slot: usize)
        requires table_size > 0,
        ensures slot < table_size,
    {
        let h = call_hash_fn(hash_fn, key, table_size, spec_hash);
        let h2 = compute_second_hash(key, table_size);
        (h.wrapping_add(attempt.wrapping_mul(h2))) % table_size
    }

    // 8. traits

    /// Trait for parametric nested hash tables.
    /// Entry type must implement this trait to define how Key and Value are stored.
    pub trait EntryTrait<Key, Value> : Sized {
        /// Abstract map view of this entry's key-value content.
        spec fn spec_entry_to_map(&self) -> Map<Key, Value>;
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn new() -> (entry: Self)
            ensures entry.spec_entry_to_map() == Map::<Key, Value>::empty();
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn insert(&mut self, key: Key, value: Value)
            ensures self.spec_entry_to_map().dom().contains(key);
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn lookup(&self, key: &Key) -> (found: Option<Value>);
        /// - APAS: N/A — inner table interface, cost depends on implementation.
        /// - Claude-Opus-4.6: N/A — abstract trait method.
        fn delete(&mut self, key: &Key) -> (deleted: bool)
            ensures !deleted ==> self.spec_entry_to_map() == old(self).spec_entry_to_map();
        /// Element-wise clone that avoids Verus tuple-Clone limitation.
        fn clone_entry(&self) -> (cloned: Self);
    }

    // 9. impls

    impl<Key, Value, Entry: EntryTrait<Key, Value>, Metrics, H> View for HashTable<Key, Value, Entry, Metrics, H> {
        type V = Map<Key, Value>;
        open spec fn view(&self) -> Map<Key, Value> {
            spec_table_to_map(self.table@)
        }
    }

    /// Trait for parametric nested hash tables.
    pub trait ParaHashTableStEphTrait<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone> {
        /// Creates an empty hash table with the given initial size.
        /// Takes a hash function that maps (&Key, table_size) to a bucket index.
        /// - APAS: Work O(m), Span O(m) where m is initial size.
        /// - Claude-Opus-4.6: Work O(m), Span O(m) — agrees with APAS; iterates m times to create entries.
        fn createTable(hash_fn: H, initial_size: usize, spec_hash: Ghost<spec_fn(Key) -> nat>) -> (table: HashTable<Key, Value, Entry, Metrics, H>)
            requires
                initial_size > 0,
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
                    assert(table_vec@ == old_view.push(table_vec@.last()));
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
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn insert(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: Key, value: Value)
            requires
                spec_hashtable_wf(old(table)),
                old(table).num_elements < usize::MAX,
            ensures
                table@ == old(table)@.insert(key, value),
                spec_hashtable_wf(table),
                table.spec_hash == old(table).spec_hash,
                table.current_size == old(table).current_size,
                table.num_elements <= old(table).num_elements + 1;

        /// Looks up a key in the hash table, returning its value if found.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn lookup(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (found: Option<Value>)
            requires
                spec_hashtable_wf(table),
            ensures
                table@.dom().contains(*key) ==> found == Some(table@[*key]),
                !table@.dom().contains(*key) ==> found is None;

        /// Deletes a key from the hash table if it exists.
        /// - APAS: Work O(1) expected, Span O(1).
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn delete(table: &mut HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (deleted: bool)
            requires
                spec_hashtable_wf(old(table)),
            ensures
                deleted == old(table)@.dom().contains(*key),
                table@ == old(table)@.remove(*key),
                spec_hashtable_wf(table),
                table.spec_hash == old(table).spec_hash,
                table.current_size == old(table).current_size;

        /// Accessor for metrics field.
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — field access.
        fn metrics(table: &HashTable<Key, Value, Entry, Metrics, H>) -> (m: &Metrics)
            requires true,
            ensures m == &table.metrics,
        { &table.metrics }

        /// Returns the load (number of elements) and size (table capacity).
        /// Load factor α = load/size.
        /// - APAS: Work O(1), Span O(1).
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — agrees with APAS; field reads only.
        fn loadAndSize(table: &HashTable<Key, Value, Entry, Metrics, H>) -> (load_and_size: LoadAndSize)
            requires true,
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
        /// - APAS: Work O(n + m + m'), Span O(n + m + m') where n is number of elements,
        ///   m is old size, m' is new size.
        /// - Claude-Opus-4.6: N/A — abstract trait method; cost depends on implementation.
        fn resize(table: &HashTable<Key, Value, Entry, Metrics, H>, new_size: usize) -> (resized: HashTable<Key, Value, Entry, Metrics, H>)
            requires
                new_size > 0,
                spec_hashtable_wf(table),
            ensures
                resized@ == table@,
                resized.current_size == new_size,
                resized.table@.len() == new_size as int,
                spec_hashtable_wf(&resized),
                resized.spec_hash == table.spec_hash;
    }

    } // verus!

    // 13. derive impls outside verus!

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
