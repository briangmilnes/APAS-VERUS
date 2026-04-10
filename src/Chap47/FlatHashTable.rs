//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses open addressing for collision resolution.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod FlatHashTable {

    //		Section 2. imports
    use vstd::prelude::*;
    use crate::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    verus!
{

    //		Section 4. type definitions


    /// Entry type for flat hash tables using open addressing.
    /// Represents the three states a slot can be in.
    #[derive(PartialEq)]
    pub enum FlatEntry<Key, Value> {
        /// Empty slot - never occupied
        Empty,
        /// Occupied slot with key-value pair
        Occupied(Key, Value),
        /// Deleted slot - previously occupied, now available for insertion
        Deleted,
    }

    //		Section 6. spec fns


    /// Whether a flat entry slot contains the given key.
    /// Avoids EntryTrait bounds so wf specs can use it with unconstrained type params.
    pub open spec fn spec_flat_has_key<Key, Value>(entry: FlatEntry<Key, Value>, k: Key) -> bool {
        match entry {
            FlatEntry::Occupied(ek, _) => ek == k,
            _ => false,
        }
    }

    /// Counts the number of Empty entries in a flat hash table sequence.
    pub open spec fn spec_count_empties<Key, Value>(
        table: Seq<FlatEntry<Key, Value>>,
    ) -> int
        decreases table.len(),
    {
        if table.len() == 0 { 0 }
        else if table.last() is Empty { spec_count_empties(table.drop_last()) + 1 }
        else { spec_count_empties(table.drop_last()) }
    }

    //		Section 7. proof fns/broadcast groups


    /// An all-Empty sequence has empties count equal to its length.
    pub proof fn lemma_all_empties_count<Key, Value>(table: Seq<FlatEntry<Key, Value>>)
        requires forall |j: int| 0 <= j < table.len() ==> (#[trigger] table[j]) is Empty,
        ensures spec_count_empties(table) == table.len(),
        decreases table.len(),
    {
        if table.len() > 0 {
            lemma_all_empties_count::<Key, Value>(table.drop_last());
        }
    }

    /// If empties count > 0, there exists an Empty slot.
    pub proof fn lemma_empties_positive_implies_exists_empty<Key, Value>(
        table: Seq<FlatEntry<Key, Value>>,
    )
        requires spec_count_empties(table) > 0,
        ensures exists |j: int| 0 <= j < table.len() && (#[trigger] table[j]) is Empty,
        decreases table.len(),
    {
        if table.last() is Empty {
        } else {
            lemma_empties_positive_implies_exists_empty::<Key, Value>(table.drop_last());
            let j = choose |j: int| 0 <= j < table.drop_last().len()
                && (#[trigger] table.drop_last()[j]) is Empty;
        }
    }

    /// Changing one slot decreases empties by at most 1.
    pub proof fn lemma_one_slot_change_empties<Key, Value>(
        old_table: Seq<FlatEntry<Key, Value>>,
        new_table: Seq<FlatEntry<Key, Value>>,
        s: int,
    )
        requires
            old_table.len() == new_table.len(),
            0 <= s < old_table.len(),
            forall |j: int| 0 <= j < old_table.len() && j != s
                ==> #[trigger] new_table[j] == old_table[j],
        ensures
            spec_count_empties(new_table) >= spec_count_empties(old_table) - 1,
        decreases old_table.len(),
    {
        if old_table.len() == 1 {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(spec_count_empties::<Key, Value>(old_table.drop_last()) == 0);
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(spec_count_empties::<Key, Value>(new_table.drop_last()) == 0);
        } else if s == old_table.len() - 1 {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert(new_table.drop_last() =~= old_table.drop_last());
        } else {
            // Veracity: NEEDED assert
            // Veracity: NEEDED assert
            assert forall |j: int| 0 <= j < old_table.drop_last().len() && j != s
                implies #[trigger] new_table.drop_last()[j] == old_table.drop_last()[j] by {
            }
            lemma_one_slot_change_empties::<Key, Value>(
                old_table.drop_last(), new_table.drop_last(), s);
        }
    }

    /// Modular probe identity: (h + (j - h + m) % m) % m == j for 0 <= h, j < m.
    /// Shared by linear probing and double hashing.
    pub proof fn lemma_probe_mod_identity(h: int, j: int, m: int)
        requires 0 <= h < m, 0 <= j < m, m > 0,
        ensures (h + (j - h + m) % m) % m == j,
    {
        if j >= h {
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(j - h, m);
            vstd::arithmetic::div_mod::lemma_small_mod((j - h) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_small_mod(j as nat, m as nat);
        } else {
            vstd::arithmetic::div_mod::lemma_small_mod((j - h + m) as nat, m as nat);
            vstd::arithmetic::div_mod::lemma_mod_add_multiples_vanish(j, m);
            vstd::arithmetic::div_mod::lemma_small_mod(j as nat, m as nat);
        }
    }

    //		Section 8. traits


    /// Flat Hash Table trait - extends ParaHashTableStEphTrait.
    /// Uses open addressing (linear probing, quadratic probing, double hashing).
    /// Entry type is parametric - can be FlatEntry or any type implementing EntryTrait.
    pub trait FlatHashTable<Key: StT, Value: StT, Entry: EntryTrait<Key, Value>, Metrics: Default, H: Fn(&Key, usize) -> usize + Clone>:
        ParaHashTableStEphTrait<Key, Value, Entry, Metrics, H>
    {
        /// Probes for the next slot in the sequence.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — abstract trait method; cost depends on probing strategy.
        fn probe(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key, attempt: usize) -> (slot: usize)
            requires
                table.current_size > 0,
                spec_hash_fn_valid::<Key, H>(table.spec_hash@),
            ensures
                slot < table.current_size;

        /// Finds the first available slot (Empty or Deleted) for insertion.
        /// - Alg Analysis: APAS (Ch47 ref): Work O(1/(1−α)) expected, Span O(1/(1−α)).
        /// - Alg Analysis: Code review (Claude Opus 4.6): N/A — abstract trait method; cost depends on probing strategy.
        fn find_slot(table: &HashTable<Key, Value, Entry, Metrics, H>, key: &Key) -> (slot: usize)
            requires
                table.current_size > 0,
                table.table@.len() == table.current_size as int,
                spec_hash_fn_valid::<Key, H>(table.spec_hash@),
            ensures
                slot < table.current_size;

    }

    //		Section 9. impls


    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for FlatEntry<Key, Value> {
        open spec fn spec_entry_to_map(&self) -> Map<Key, Value> {
            match *self {
                FlatEntry::Empty => Map::empty(),
                FlatEntry::Deleted => Map::empty(),
                FlatEntry::Occupied(k, v) => Map::empty().insert(k, v),
            }
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-time enum construction.
        fn new() -> (entry: Self)
            ensures entry is Empty,
        { FlatEntry::Empty }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — constant-time enum assignment.
        fn insert(&mut self, key: Key, value: Value)
            ensures
                *self == FlatEntry::<Key, Value>::Occupied(key, value),
                self.spec_entry_to_map()[key] == value,
        { *self = FlatEntry::Occupied(key, value); }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single match + key comparison.
        fn lookup(&self, key: &Key) -> (found: Option<Value>)
            ensures
                self is Empty ==> found is None,
                self is Deleted ==> found is None,
        {
            match self {
                | FlatEntry::Occupied(k, v) => {
                    if *k == *key { Some(v.clone()) } else { None }
                }
                | _ => None,
            }
        }

        /// - Alg Analysis: APAS (Ch47 ref): Work O(1), Span O(1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single match + enum assignment.
        fn delete(&mut self, key: &Key) -> (deleted: bool)
            ensures
                deleted ==> *self is Deleted,
                deleted ==> self.spec_entry_to_map() == Map::<Key, Value>::empty(),
                !deleted ==> *self == *old(self),
        {
            let is_match = if let FlatEntry::Occupied(k, _) = &*self {
                *k == *key
            } else {
                false
            };
            if is_match {
                *self = FlatEntry::Deleted;
                true
            } else {
                false
            }
        }

        /// Delegates to FlatEntry's Clone impl.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — clones a single flat entry.
        fn clone_entry(&self) -> (cloned: Self) {
            self.clone()
        }
    }

    //		Section 12. derive impls in verus!


    impl<Key: Clone, Value: Clone> Clone for FlatEntry<Key, Value> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self,
        {
            let c = match self {
                FlatEntry::Empty => FlatEntry::Empty,
                FlatEntry::Occupied(k, v) => FlatEntry::Occupied(k.clone(), v.clone()),
                FlatEntry::Deleted => FlatEntry::Deleted,
            };
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof { assume(c == *self); } // Clone bridge.
            c
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Debug for FlatEntry<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FlatEntry::Empty => write!(f, "Empty"),
                FlatEntry::Occupied(k, v) => f.debug_tuple("Occupied").field(k).field(v).finish(),
                FlatEntry::Deleted => write!(f, "Deleted"),
            }
        }
    }

    impl<Key: std::fmt::Debug, Value: std::fmt::Debug> std::fmt::Display for FlatEntry<Key, Value> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
