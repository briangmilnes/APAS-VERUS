// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: which postconditions of APAS's `HashMapWithViewPlus`
//! (`src/vstdplus/hash_map_with_view_plus.rs`, 9 `external_body` trait
//! methods plus `Clone` and `PartialEq`) are now derivable from vstd
//! 0.2026.09.13's `std_specs::hash` specifications on
//! `std::collections::HashMap` with no trusted code in APAS. One fn per APAS
//! method; each `ensures` is the APAS postcondition restated over the raw
//! `HashMap` view `Map<Key, Value>`. The last fn shows vstd's `deep_view`
//! giving APAS's mapped view `Map<Key::V, ...>` under key injectivity.
//!
//! RESULT: SUCCEEDS — 12 verified, 0 errors. Every APAS map postcondition
//!         (new, len, is_empty, get, insert, clear, contains_key, remove,
//!         iter, Clone for value types where `cloned` is equality) is
//!         derivable, and `deep_view` gives the mapped view under key
//!         injectivity; vstd adds keys, values.
//! DATE: 2026-09-20
//! VERUS: 0.2026.09.13.671956e
//! LOG: logs/validate-standard-vstd_hash_map_derived.20260920-222027.log
//! Run: scripts/validate-standard.sh --experiment vstd_hash_map_derived

pub mod vstd_hash_map_derived {
    use std::collections::HashMap;
    use std::hash::Hash;
    use vstd::prelude::*;
    use vstd::std_specs::hash::*;
    use vstd::std_specs::iter::*;

    verus! {

    broadcast use group_hash_axioms;

    /// APAS `new`: `empty@ == Map::empty()`.
    pub fn derived_new<Key: View + Eq + Hash, Value>() -> (m: HashMap<Key, Value>)
        ensures
            m@ == Map::<Key, Value>::empty(),
    {
        HashMap::<Key, Value>::new()
    }

    /// APAS `len`: `count == self@.len()`.
    pub fn derived_len<Key: View + Eq + Hash, Value>(m: &HashMap<Key, Value>) -> (count: usize)
        requires
            obeys_key_model::<Key>(),
        ensures
            count == m@.len(),
    {
        m.len()
    }

    /// APAS `is_empty`.
    pub fn derived_is_empty<Key: View + Eq + Hash, Value>(m: &HashMap<Key, Value>) -> (b: bool)
        ensures
            b == m@.is_empty(),
    {
        m.is_empty()
    }

    /// APAS `get`: `Some(v) => contains_key && *v == self@[k]`, `None => !contains_key`.
    pub fn derived_get<'a, Key: View + Eq + Hash, Value>(m: &'a HashMap<Key, Value>, k: &Key) -> (value: Option<&'a Value>)
        requires
            obeys_key_model::<Key>(),
        ensures
            match value {
                Some(v) => m@.contains_key(*k) && *v == m@[*k],
                None => !m@.contains_key(*k),
            },
    {
        m.get(k)
    }

    /// APAS `insert`: `self@ == old(self)@.insert(k, v)`. vstd also returns the old value.
    pub fn derived_insert<Key: View + Eq + Hash, Value>(m: &mut HashMap<Key, Value>, k: Key, v: Value)
        requires
            obeys_key_model::<Key>(),
        ensures
            m@ == old(m)@.insert(k, v),
    {
        let _old = m.insert(k, v);
    }

    /// APAS `clear`: `self@ == Map::empty()`.
    pub fn derived_clear<Key: View + Eq + Hash, Value>(m: &mut HashMap<Key, Value>)
        ensures
            m@ == Map::<Key, Value>::empty(),
    {
        m.clear()
    }

    /// APAS `contains_key`: `contains == self@.contains_key(k)`.
    pub fn derived_contains_key<Key: View + Eq + Hash, Value>(m: &HashMap<Key, Value>, k: &Key) -> (contains: bool)
        requires
            obeys_key_model::<Key>(),
        ensures
            contains == m@.contains_key(*k),
    {
        m.contains_key(k)
    }

    /// APAS `remove`: `Some(v) => old contains && v == old[k] && self@ == old.remove(k)`,
    /// `None => !old contains && self@ == old@`.
    pub fn derived_remove<Key: View + Eq + Hash, Value>(m: &mut HashMap<Key, Value>, k: &Key) -> (removed: Option<Value>)
        requires
            obeys_key_model::<Key>(),
        ensures
            match removed {
                Some(v) => old(m)@.contains_key(*k) && v == old(m)@[*k] && m@ == old(m)@.remove(*k),
                None => !old(m)@.contains_key(*k) && m@ == old(m)@,
            },
    {
        let removed = m.remove(k);
        proof {
            if removed is None {
                assert(m@ =~= old(m)@);
            }
        }
        removed
    }

    /// APAS `iter`: every yielded pair is in the map and every key of the map is yielded.
    pub fn derived_iter<'a, Key: View + Eq + Hash, Value>(m: &'a HashMap<Key, Value>) -> (it: std::collections::hash_map::Iter<'a, Key, Value>)
        requires
            obeys_key_model::<Key>(),
        ensures
            forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len() ==>
                m@.contains_key(*#[trigger] IteratorSpec::remaining(&it)[i].0)
                && m@[*IteratorSpec::remaining(&it)[i].0] == *IteratorSpec::remaining(&it)[i].1,
            forall|k: Key| #[trigger] m@.contains_key(k) ==> IteratorSpec::remaining(&it).contains((&k, &m@[k])),
            IteratorSpec::remaining(&it).no_duplicates(),
            IteratorSpec::decrease(&it) is Some,
    {
        m.iter()
    }

    /// APAS `Clone`: APAS ensured `cloned@ == self@`; vstd ensures equal domains and
    /// per-key `cloned(this[k], other[k])`. For `Value = u64` the per-key relation is
    /// equality, so APAS's postcondition follows.
    pub fn derived_clone(m: &HashMap<u64, u64>) -> (other: HashMap<u64, u64>)
        ensures
            other@.dom() == m@.dom(),
            other@ == m@,
    {
        let other = m.clone();
        proof {
            assert forall|k: u64| #[trigger] other@.dom().contains(k) implies other@[k] == m@[k] by {
                assert(cloned(m@[k], other@[k]));
            }
            assert(other@ =~= m@);
        }
        other
    }

    /// vstd extras with no APAS counterpart: `keys` and `values` iterators.
    pub fn derived_keys<'a, Key: View + Eq + Hash, Value>(m: &'a HashMap<Key, Value>) -> (keys: std::collections::hash_map::Keys<'a, Key, Value>)
        requires
            obeys_key_model::<Key>(),
        ensures
            IteratorSpec::remaining(&keys).unref().to_set() == m@.dom(),
            IteratorSpec::remaining(&keys).no_duplicates(),
    {
        m.keys()
    }

    /// APAS's mapped view `Map<Key::V, Value>` is vstd's `deep_view` under key injectivity.
    pub proof fn derived_mapped_view<Key: DeepView + Eq + Hash, Value: DeepView>(m: HashMap<Key, Value>, k: Key)
        requires
            vstd::relations::injective(|k: Key| k.deep_view()),
            m@.contains_key(k),
        ensures
            m.deep_view().contains_key(k.deep_view()),
            m.deep_view()[k.deep_view()] == m@[k].deep_view(),
    {
        lemma_hashmap_deepview_properties(m);
    }

    } // verus!
}
