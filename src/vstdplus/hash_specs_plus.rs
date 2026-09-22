// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Trusted specifications for the `std::collections::HashSet` and `HashMap`
//! methods that vstd 0.2026.09.13 (`std_specs/hash.rs`) leaves unspecified:
//! `HashSet::clone`, `HashSet::eq` and `HashMap::eq`. Each is an
//! `assume_specification`, vstd's form of a trusted specification. They are
//! the only trusted items added in r209 (`docs/HashSpecsMigration.md` §3.2,
//! items 1 and 2) and each mirrors an upstream proposal:
//!
//! - `HashSet::clone` follows vstd's `HashMap::clone` (`std_specs/hash.rs:578`):
//!   the result relates to the source element-wise through `cloned`, and the
//!   cardinalities agree. Because vstd defines `cloned(a, b)` as
//!   `strictly_cloned(a, b) || a == b` (`pervasive.rs:426`), this statement
//!   is equivalent to `other@ == this@`; `lemma_hash_set_clone_eq` derives
//!   that equality from it with no hypothesis on the key type.
//! - `HashSet::eq` and `HashMap::eq` state `r == (a@ == b@)` under the
//!   hypotheses vstd's hash specifications carry (`obeys_key_model::<K>()`,
//!   `builds_valid_hashers::<S>()`), plus `laws_eq::obeys_eq::<K>()` as the
//!   proposal in `docs/HashSpecsMigration.md` §3.2 item 2 asks, and, for a
//!   map's values, `laws_eq::obeys_concrete_eq::<V>()`: Rust compares values
//!   with `V::eq`, and the view's `Map` equality is structural, so the
//!   value type's `eq_spec` must be `==`, which `obeys_eq` alone does not
//!   say.
//!
//! Experiments: `src/experiments/vstd_hash_set_clone_plus.rs`,
//! `src/experiments/vstd_hash_eq_plus.rs`.
//!
//! The module also holds two proven (untrusted) projections and their lemmas:
//! `key_view` maps a `HashMap`'s raw view `Map<K, V>` to the `Map<K::V, V>` that
//! `HashMapWithViewPlus` used as its view, and `set_key_view` maps a
//! `HashSet`'s raw view `Set<K>` to `Set<K::V>`. Chap62–65 state their
//! partition, spanning-tree and union-find specifications over these
//! projections; `group_key_view_lemmas` bridges vstd's raw postconditions to
//! them under view injectivity (`obeys_feq_view_injective`).

//  Table of Contents
//  1. module
//  2. imports
//  6. spec fns
//  7. proof fns/broadcast groups
//  9. impls

//  1. module
pub mod hash_specs_plus {

    //  2. imports
    use std::collections::{HashMap, HashSet};
    use std::hash::{BuildHasher, Hash};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::{obeys_concrete_eq, obeys_eq};
    #[cfg(verus_keep_ghost)]
    use vstd::set_lib::{lemma_map_size, lemma_subset_equality};
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::{builds_valid_hashers, obeys_key_model};
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_reveal_view_injective, obeys_feq_clone, obeys_feq_view_injective};

    verus! {

    broadcast use {
        vstd::set_lib::group_set_lib_default,
    };

    //  6. spec fns

    /// The key-view projection of a hash map's view: the `Map<K::V, V>` that
    /// `HashMapWithViewPlus` used as its view. A view `kv` is a key when some
    /// raw key has that view; its value is the value of such a key, which is
    /// unique under view injectivity.
    pub open spec fn key_view<K: View, V>(m: Map<K, V>) -> Map<K::V, V> {
        Map::new(
            set_key_view(m.dom()),
            |kv: K::V| m[choose|k: K| #[trigger] m.contains_key(k) && k@ == kv],
        )
    }

    /// The view projection of a hash set's view: the `Set<K::V>` that
    /// `HashSetWithViewPlus` used as its view (and that `SetStEph` still uses).
    pub open spec fn set_key_view<K: View>(s: Set<K>) -> Set<K::V> {
        s.map(|k: K| k@)
    }

    /// The relation `HashSet::clone` establishes between source and result:
    /// equal cardinality, and each element of the result is a clone of an
    /// element of the source. It is one opaque predicate rather than an
    /// inline quantifier because every Skolemized `cloned(k0, k)` term
    /// matches `group_feq_axioms`: stated inline, `SetStEph::clone` exceeded
    /// its rlimit with 65,541 instantiations of the quantifier and 118,530 of
    /// each feq axiom (`logs/validate.20260921-082833.log`, an isolate Chap05
    /// run with `--profile`).
    /// `lemma_hash_set_clone_eq` reveals it.
    #[verifier::opaque]
    pub open spec fn spec_hash_set_cloned<K: Clone>(this: Set<K>, other: Set<K>) -> bool {
        &&& other.len() == this.len()
        &&& forall|k: K| #[trigger] other.contains(k)
                <==> exists|k0: K| #[trigger] this.contains(k0) && cloned(k0, k)
    }

    //  7. proof fns/broadcast groups

    /// A key is in the key view exactly when it is in the raw map, and its value
    /// agrees. Both directions need view injectivity: without it a second key
    /// with the same view could stand in for `k`.
    pub broadcast proof fn lemma_key_view_contains<K: Eq + View, V>(m: Map<K, V>, k: K)
        requires
            obeys_feq_view_injective::<K>(),
        ensures
            #![trigger key_view(m), m.contains_key(k)]
            #![trigger key_view(m).contains_key(k@)]
            #![trigger key_view(m)[k@]]
            key_view(m).contains_key(k@) == m.contains_key(k),
            m.contains_key(k) ==> key_view(m)[k@] == m[k],
    {
        lemma_reveal_view_injective::<K>();
        if m.contains_key(k) {
            assert(m.dom().contains(k) && k@ == k@);
            assert(key_view(m).contains_key(k@));
            let k0 = choose|k0: K| #[trigger] m.contains_key(k0) && k0@ == k@;
            assert(k0 == k);
        } else if key_view(m).contains_key(k@) {
            let k0 = choose|k0: K| #[trigger] m.dom().contains(k0) && k@ == k0@;
            assert(k0 == k);
        }
    }

    /// Inserting into the raw map inserts the key's view into the key view.
    pub broadcast proof fn lemma_key_view_insert<K: Eq + View, V>(m: Map<K, V>, k: K, v: V)
        requires
            obeys_feq_view_injective::<K>(),
        ensures
            #[trigger] key_view(m.insert(k, v)) == key_view(m).insert(k@, v),
    {
        lemma_reveal_view_injective::<K>();
        let m2 = m.insert(k, v);
        let lhs = key_view(m2);
        let rhs = key_view(m).insert(k@, v);
        assert forall|kv: K::V| lhs.contains_key(kv) == rhs.contains_key(kv) by {
            if kv == k@ {
                assert(m2.dom().contains(k) && kv == k@);
            } else if lhs.contains_key(kv) {
                let k0 = choose|k0: K| #[trigger] m2.dom().contains(k0) && kv == k0@;
                assert(k0 != k);
                assert(m.dom().contains(k0) && kv == k0@);
            } else if rhs.contains_key(kv) {
                let k0 = choose|k0: K| #[trigger] m.dom().contains(k0) && kv == k0@;
                assert(m2.dom().contains(k0) && kv == k0@);
            }
        };
        assert(lhs.dom() =~= rhs.dom());
        assert forall|kv: K::V| #[trigger] lhs.contains_key(kv) implies lhs[kv] == rhs[kv] by {
            let k0 = choose|k0: K| #[trigger] m2.contains_key(k0) && k0@ == kv;
            assert(m2.contains_key(k0) && k0@ == kv) by {
                let kw = choose|kw: K| #[trigger] m2.dom().contains(kw) && kv == kw@;
                assert(m2.contains_key(kw) && kw@ == kv);
            }
            if kv == k@ {
                assert(k0 == k);
            } else {
                assert(k0 != k);
                assert(m.contains_key(k0) && k0@ == kv);
                let k1 = choose|k1: K| #[trigger] m.contains_key(k1) && k1@ == kv;
                assert(k1 == k0);
            }
        };
        assert(lhs =~= rhs);
    }

    /// The key view of the empty map is empty.
    pub broadcast proof fn lemma_key_view_empty<K: View, V>()
        ensures
            #[trigger] key_view(Map::<K, V>::empty()) == Map::<K::V, V>::empty(),
    {
        broadcast use lemma_set_key_view_empty;
        assert(key_view(Map::<K, V>::empty()).dom() =~= Set::<K::V>::empty());
        assert(key_view(Map::<K, V>::empty()) =~= Map::<K::V, V>::empty());
    }

    /// Every key of the key view is the view of a raw key; this direction needs
    /// no injectivity and is the shape the partition proofs use.
    pub broadcast proof fn lemma_key_view_key<K: View, V>(m: Map<K, V>, kv: K::V)
        ensures
            #[trigger] key_view(m).contains_key(kv)
                <==> exists|k: K| #[trigger] m.contains_key(k) && k@ == kv,
    {
        if key_view(m).contains_key(kv) {
            let k0 = choose|k0: K| #[trigger] m.dom().contains(k0) && kv == k0@;
            assert(m.contains_key(k0) && k0@ == kv);
        }
        if exists|k: K| #[trigger] m.contains_key(k) && k@ == kv {
            let k0 = choose|k0: K| #[trigger] m.contains_key(k0) && k0@ == kv;
            assert(m.dom().contains(k0) && kv == k0@);
            assert(set_key_view(m.dom()).contains(kv));
        }
    }

    /// The key view has the raw map's cardinality under view injectivity.
    pub broadcast proof fn lemma_key_view_len<K: Eq + View, V>(m: Map<K, V>)
        requires
            obeys_feq_view_injective::<K>(),
        ensures
            #![trigger key_view(m).len()]
            #![trigger key_view(m).dom().len()]
            key_view(m).len() == m.len(),
            key_view(m).dom().len() == m.dom().len(),
    {
        lemma_set_key_view_len::<K>(m.dom());
    }

    /// An element is in the set view exactly when it is in the raw set.
    pub broadcast proof fn lemma_set_key_view_contains<K: Eq + View>(s: Set<K>, k: K)
        requires
            obeys_feq_view_injective::<K>(),
        ensures
            #![trigger set_key_view(s), s.contains(k)]
            #![trigger set_key_view(s).contains(k@)]
            set_key_view(s).contains(k@) == s.contains(k),
    {
        lemma_reveal_view_injective::<K>();
        if s.contains(k) {
        } else if set_key_view(s).contains(k@) {
            let k0 = choose|k0: K| #[trigger] s.contains(k0) && k@ == k0@;
            assert(k0 == k);
        }
    }

    /// Inserting into the raw set inserts the element's view into the set view.
    pub broadcast proof fn lemma_set_key_view_insert<K: View>(s: Set<K>, k: K)
        ensures
            #[trigger] set_key_view(s.insert(k)) == set_key_view(s).insert(k@),
    {
        let lhs = set_key_view(s.insert(k));
        let rhs = set_key_view(s).insert(k@);
        assert forall|kv: K::V| lhs.contains(kv) == rhs.contains(kv) by {
            if kv == k@ {
                assert(s.insert(k).contains(k));
            } else if lhs.contains(kv) {
                let k0 = choose|k0: K| #[trigger] s.insert(k).contains(k0) && kv == k0@;
                assert(s.contains(k0));
            } else if rhs.contains(kv) {
                let k0 = choose|k0: K| #[trigger] s.contains(k0) && kv == k0@;
                assert(s.insert(k).contains(k0));
            }
        };
        assert(lhs =~= rhs);
    }

    /// The set view of the empty set is empty.
    pub broadcast proof fn lemma_set_key_view_empty<K: View>()
        ensures
            #[trigger] set_key_view(Set::<K>::empty()) == Set::<K::V>::empty(),
    {
        assert forall|kv: K::V| !set_key_view(Set::<K>::empty()).contains(kv) by {
            if set_key_view(Set::<K>::empty()).contains(kv) {
                let k0 = choose|k0: K| #[trigger] Set::<K>::empty().contains(k0) && kv == k0@;
            }
        };
        assert(set_key_view(Set::<K>::empty()) =~= Set::<K::V>::empty());
    }

    /// The set view has the raw set's cardinality under view injectivity.
    pub broadcast proof fn lemma_set_key_view_len<K: Eq + View>(s: Set<K>)
        requires
            obeys_feq_view_injective::<K>(),
        ensures
            #[trigger] set_key_view(s).len() == s.len(),
    {
        lemma_reveal_view_injective::<K>();
        let f = |k: K| k@;
        assert forall|kv: K::V| #[trigger] set_key_view(s).contains(kv) == s.map(f).contains(kv) by {
            if set_key_view(s).contains(kv) {
                let k0 = choose|k0: K| #[trigger] s.contains(k0) && kv == k0@;
                assert(s.contains(k0) && kv == f(k0));
            } else if s.map(f).contains(kv) {
                let k0 = choose|k0: K| #[trigger] s.contains(k0) && kv == f(k0);
                assert(s.contains(k0) && kv == k0@);
            }
        };
        assert(set_key_view(s) =~= s.map(f));
        assert(s.injective_on(f)) by {
            assert forall|x: K, y: K| s.contains(x) && s.contains(y) && #[trigger] f(x) == #[trigger] f(y) implies x == y by {
                assert(x@ == y@);
            };
        }
        lemma_map_size(s, s.map(f), f);
    }

    pub broadcast group group_key_view_lemmas {
        lemma_key_view_contains,
        lemma_key_view_insert,
        lemma_key_view_empty,
        lemma_key_view_key,
        lemma_key_view_len,
        lemma_set_key_view_contains,
        lemma_set_key_view_insert,
        lemma_set_key_view_empty,
        lemma_set_key_view_len,
    }

    /// vstd's `HashMap::clone` (`std_specs/hash.rs:578`) relates values through
    /// `cloned`; under `obeys_feq_clone::<V>()` (`group_feq_axioms`) that is
    /// equality, so the clone's view equals the source's.
    pub proof fn lemma_hash_map_clone_eq<K, V: Eq + Clone>(this: Map<K, V>, other: Map<K, V>)
        requires
            other.dom() == this.dom(),
            forall|key: K| #[trigger] other.dom().contains(key) ==> cloned(this[key], other[key]),
            obeys_feq_clone::<V>(),
        ensures
            other == this,
    {
        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
        assert forall|key: K| #[trigger] other.dom().contains(key) implies other[key] == this[key] by {
            assert(cloned(this[key], other[key]));
        };
        assert(other =~= this);
    }

    /// `spec_hash_set_cloned(this, other)` implies `other == this`: the
    /// witness `k0 == k` puts every element of `this` into `other`, since
    /// `cloned(k, k)` holds by definition, and equal cardinality then gives
    /// equality by `lemma_subset_equality`.
    pub proof fn lemma_hash_set_clone_eq<K: Clone>(this: Set<K>, other: Set<K>)
        requires
            spec_hash_set_cloned(this, other),
        ensures
            other == this,
    {
        reveal(spec_hash_set_cloned);
        assert forall|k: K| #[trigger] this.contains(k) implies other.contains(k) by {
            assert(this.contains(k) && cloned(k, k));
        }
        lemma_subset_equality(this, other);
    }

    //  9. impls

    /// Proposal: `std_specs/hash.rs`, beside `HashMap::clone` (line 578), with
    /// the same shape: element-wise `cloned` and equal cardinality.
    pub assume_specification<K: Clone, S: Clone, A: std::alloc::Allocator + Clone>[
        <HashSet<K, S, A> as Clone>::clone
    ](this: &HashSet<K, S, A>) -> (other: HashSet<K, S, A>)
        ensures
            spec_hash_set_cloned(this@, other@),
    ;

    /// Proposal: `std_specs/hash.rs`, a `PartialEq::eq` specification for
    /// `HashSet` (`docs/HashSpecsMigration.md` §3.2 item 2; not taken upstream).
    /// Rust's `HashSet::eq` compares lengths and then looks each element of one
    /// set up in the other, so under the key model the result is view equality.
    pub assume_specification<K: Eq + Hash, S: BuildHasher, A: std::alloc::Allocator>[
        <HashSet<K, S, A> as PartialEq<HashSet<K, S, A>>>::eq
    ](a: &HashSet<K, S, A>, b: &HashSet<K, S, A>) -> (r: bool)
        ensures
            obeys_key_model::<K>() && builds_valid_hashers::<S>() && obeys_eq::<K>()
                ==> r == (a@ == b@),
    ;

    /// Proposal: as for `HashSet::eq`, for `HashMap`. Values are compared with
    /// `V::eq`, so the value type must obey concrete equality.
    pub assume_specification<K: Eq + Hash, V: PartialEq, S: BuildHasher, A: std::alloc::Allocator>[
        <HashMap<K, V, S, A> as PartialEq<HashMap<K, V, S, A>>>::eq
    ](a: &HashMap<K, V, S, A>, b: &HashMap<K, V, S, A>) -> (r: bool)
        ensures
            obeys_key_model::<K>() && builds_valid_hashers::<S>() && obeys_eq::<K>()
                && obeys_concrete_eq::<V>()
                ==> r == (a@ == b@),
    ;

    } // verus!
}
