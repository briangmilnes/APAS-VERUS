//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared spec functions and proof lemmas for the Table modules (StEph, StPer, MtEph).
//! All definitions are generic over `(KV, VV)` and operate on `Seq<(KV, VV)>` / `Map<KV, VV>`.

//  Table of Contents
//	Section 1. module
//	Section 6. spec fns
//	Section 7. proof fns

//		Section 1. module

pub mod TableSpecsAndLemmas {

    use vstd::prelude::*;

    verus! {

broadcast use {
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    //		Section 6. spec fns


    // Converts a sequence of (key, value) pairs to a Map.
    // Later entries win on duplicate keys (irrelevant when keys are unique).
    pub open spec fn spec_entries_to_map<KV, VV>(entries: Seq<(KV, VV)>) -> Map<KV, VV>
        decreases entries.len()
    {
        if entries.len() == 0 {
            Map::empty()
        } else {
            let last = entries.last();
            spec_entries_to_map(entries.drop_last()).insert(last.0, last.1)
        }
    }


    // Keys in the entry sequence are unique.
    pub open spec fn spec_keys_no_dups<KV, VV>(entries: Seq<(KV, VV)>) -> bool {
        forall|i: int, j: int|
            0 <= i < j < entries.len() ==> (#[trigger] entries[i]).0 != (#[trigger] entries[j]).0
    }

    //		Section 7. proof fns


    pub proof fn lemma_entries_to_map_finite<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries).dom().finite()
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_finite::<KV, VV>(entries.drop_last());
        }
    }

    // If a key is in spec_entries_to_map, it appears in the seq.
    pub proof fn lemma_entries_to_map_key_in_seq<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires spec_entries_to_map(entries).contains_key(k)
        ensures exists|i: int| 0 <= i < entries.len() && (#[trigger] entries[i]).0 == k
        decreases entries.len()
    {
        if entries.len() > 0 {
            let last = entries.last();
            if last.0 == k {
            } else {
                lemma_entries_to_map_key_in_seq::<KV, VV>(entries.drop_last(), k);
                let prefix = entries.drop_last();
                let i = choose|i: int| 0 <= i < prefix.len() && (#[trigger] prefix[i]).0 == k;
            }
        }
    }

    // If entries[idx] has key k, the map contains k.
    pub proof fn lemma_entries_to_map_contains_key<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
        requires 0 <= idx < entries.len()
        ensures spec_entries_to_map(entries).contains_key(entries[idx].0)
        decreases entries.len()
    {
        if entries.len() > 0 {
            if idx == entries.len() - 1 {
            } else {
                lemma_entries_to_map_contains_key::<KV, VV>(entries.drop_last(), idx);
            }
        }
    }

    // When keys are unique, spec_entries_to_map length equals seq length.
    pub proof fn lemma_entries_to_map_len<KV, VV>(entries: Seq<(KV, VV)>)
        requires spec_keys_no_dups(entries)
        ensures spec_entries_to_map(entries).len() == entries.len()
        decreases entries.len()
    {
        if entries.len() > 0 {
            let prefix = entries.drop_last();
            let last = entries.last();
            let last_idx = entries.len() - 1;
            lemma_entries_to_map_len::<KV, VV>(prefix);
            let prefix_map = spec_entries_to_map(prefix);
            // Veracity: NEEDED assert
            assert(!prefix_map.contains_key(last.0)) by {
                if prefix_map.contains_key(last.0) {
                    lemma_entries_to_map_key_in_seq(prefix, last.0);
                    let idx = choose|i: int| 0 <= i < prefix.len() && (#[trigger] prefix[i]).0 == last.0;
                }
            };
            // Veracity: NEEDED assert
            assert(prefix_map.dom().finite()) by {
                lemma_entries_to_map_finite::<KV, VV>(prefix);
            };
        }
    }

    // If no entry has key k, spec_entries_to_map does not contain k.
    pub proof fn lemma_entries_to_map_no_key<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires forall|i: int| 0 <= i < entries.len() ==> (#[trigger] entries[i]).0 != k
        ensures !spec_entries_to_map(entries).contains_key(k)
    {
        if spec_entries_to_map(entries).contains_key(k) {
            lemma_entries_to_map_key_in_seq(entries, k);
        }
    }

    // If entries[idx] = (k, v) and keys are unique, map contains k with value v.
    pub proof fn lemma_entries_to_map_get<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
        requires
            0 <= idx < entries.len(),
            spec_keys_no_dups(entries),
        ensures
            spec_entries_to_map(entries).contains_key(entries[idx].0),
            spec_entries_to_map(entries)[entries[idx].0] == entries[idx].1,
        decreases entries.len(),
    {
        let k = entries[idx].0;
        let v = entries[idx].1;
        if entries.len() > 0 {
            let last = entries.last();
            let prefix = entries.drop_last();
            if idx == entries.len() - 1 {
            } else {
                lemma_entries_to_map_get::<KV, VV>(prefix, idx);
            }
        }
    }

    // If every key in sub appears in sup, sub map domain is subset of sup map domain.
    pub proof fn lemma_entries_to_map_dom_subset<KV, VV>(
        sub: Seq<(KV, VV)>,
        sup: Seq<(KV, VV)>,
    )
        requires forall|i: int| 0 <= i < sub.len() ==>
            exists|j: int| 0 <= j < sup.len() && (#[trigger] sup[j]).0 == (#[trigger] sub[i]).0,
        ensures spec_entries_to_map(sub).dom().subset_of(spec_entries_to_map(sup).dom()),
    {
        // Veracity: NEEDED assert
        assert forall|k: KV| spec_entries_to_map(sub).dom().contains(k)
            implies spec_entries_to_map(sup).dom().contains(k)
        by {
            lemma_entries_to_map_key_in_seq(sub, k);
            let i = choose|i: int| 0 <= i < sub.len() && (#[trigger] sub[i]).0 == k;
            let j = choose|j: int| 0 <= j < sup.len() && (#[trigger] sup[j]).0 == sub[i].0;
            lemma_entries_to_map_contains_key(sup, j);
        };
    }

    // If two sequences have the same keys at each position, their maps have the same domain.
    pub proof fn lemma_entries_to_map_dom_same_keys<KV, VV1, VV2>(
        s1: Seq<(KV, VV1)>,
        s2: Seq<(KV, VV2)>,
    )
        requires
            s1.len() == s2.len(),
            forall|i: int| 0 <= i < s1.len() ==> (#[trigger] s1[i]).0 == (#[trigger] s2[i]).0,
        ensures
            spec_entries_to_map(s1).dom() =~= spec_entries_to_map(s2).dom(),
        decreases s1.len(),
    {
        if s1.len() > 0 {
            lemma_entries_to_map_dom_same_keys::<KV, VV1, VV2>(
                s1.drop_last(), s2.drop_last(),
            );
        }
    }

    // Value preservation for subsequences: if filtered is a subsequence of entries
    // (via strictly increasing sources) that includes all entries with key k, the
    // map values agree on k.
    pub proof fn lemma_entries_to_map_subseq_value<KV, VV>(
        entries: Seq<(KV, VV)>,
        filtered: Seq<(KV, VV)>,
        sources: Seq<int>,
        k: KV,
    )
        requires
            filtered.len() == sources.len(),
            forall|j: int| 0 <= j < sources.len() ==>
                0 <= #[trigger] sources[j] < entries.len()
                && filtered[j] == entries[sources[j]],
            forall|j1: int, j2: int| 0 <= j1 < j2 < sources.len()
                ==> sources[j1] < sources[j2],
            forall|i: int| 0 <= i < entries.len() && (#[trigger] entries[i]).0 == k
                ==> exists|j: int| 0 <= j < sources.len() && sources[j] == i,
            spec_entries_to_map(filtered).contains_key(k),
        ensures
            spec_entries_to_map(entries).contains_key(k),
            spec_entries_to_map(filtered)[k] == spec_entries_to_map(entries)[k],
        decreases entries.len(),
    {
        if entries.len() == 0 {
            if sources.len() > 0 {
                // Veracity: NEEDED assert
                assert(0 <= sources[0] < entries.len());
            }
            return;
        }
        let last = entries.last();
        let n = entries.len() - 1;
        let prefix = entries.drop_last();
        if last.0 == k {
            let j_last = choose|j: int| 0 <= j < sources.len() && sources[j] == n;
            // Veracity: NEEDED assert
            assert(j_last == sources.len() - 1) by {
                if j_last < sources.len() - 1 {
                    // Veracity: NEEDED assert
                    assert(sources[j_last + 1] < entries.len());
                }
            };
        } else {
            let last_kept = exists|j: int| 0 <= j < sources.len() && sources[j] == n;
            if last_kept {
                let j_last = choose|j: int| 0 <= j < sources.len() && sources[j] == n;
                // Veracity: NEEDED assert
                assert(j_last == sources.len() - 1) by {
                    if j_last < sources.len() - 1 {
                        // Veracity: NEEDED assert
                        assert(sources[j_last + 1] < entries.len());
                    }
                };
                let f_prefix = filtered.drop_last();
                let s_prefix = sources.drop_last();
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < s_prefix.len() implies
                    0 <= #[trigger] s_prefix[j] < prefix.len()
                    && f_prefix[j] == prefix[s_prefix[j]]
                by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(s_prefix[j] == sources[j]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(sources[j] < sources[j_last]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(sources[j_last] == n);
                    // Veracity: NEEDED assert (speed hint)
                    assert(sources[j] < n);
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= sources[j] < n);
// Veracity: UNNEEDED assert                     assert(f_prefix[j] == filtered[j]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= sources[j] < entries.len());
// Veracity: UNNEEDED assert                     assert(filtered[j] == entries[sources[j]]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(sources[j] < prefix.len());
// Veracity: UNNEEDED assert                     assert(prefix[sources[j]] == entries[sources[j]]);
                };
                // Veracity: NEEDED assert
                assert forall|j1: int, j2: int| 0 <= j1 < j2 < s_prefix.len()
                    implies s_prefix[j1] < s_prefix[j2]
                by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= j1 < sources.len());
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= j2 < sources.len());
                    // Veracity: NEEDED assert (speed hint)
                    assert(s_prefix[j1] == sources[j1]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(s_prefix[j2] == sources[j2]);
                };
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < prefix.len()
                    && (#[trigger] prefix[i]).0 == k
                    implies exists|j: int| 0 <= j < s_prefix.len() && s_prefix[j] == i
                by {
// Veracity: UNNEEDED assert                     assert(0 <= i < entries.len());
                    // Veracity: NEEDED assert (speed hint)
                    assert(entries[i].0 == k);
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == i;
                    // Veracity: NEEDED assert (speed hint)
                    assert(j < sources.len() - 1) by {
                        if j == sources.len() - 1 {
                            // Veracity: NEEDED assert (speed hint)
                            assert(j == j_last);
// Veracity: UNNEEDED assert                             assert(sources[j] == n);
                            // Veracity: NEEDED assert (speed hint)
                            assert(i == n);
                            // Veracity: NEEDED assert (speed hint)
                            assert(i < prefix.len());
// Veracity: UNNEEDED assert                             assert(n == prefix.len());
                        }
                    };
// Veracity: UNNEEDED assert                     assert(0 <= j < s_prefix.len());
                    // Veracity: NEEDED assert
                    assert(s_prefix[j] == sources[j]);
                };
                lemma_entries_to_map_subseq_value::<KV, VV>(prefix, f_prefix, s_prefix, k);
            } else {
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < prefix.len()
                    && filtered[j] == prefix[sources[j]]
                by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= sources[j] < entries.len());
                    if sources[j] == n {
                    }
                    // Veracity: NEEDED assert (speed hint)
                    assert(sources[j] != n);
// Veracity: UNNEEDED assert                     assert(sources[j] < n);
                    // Veracity: NEEDED assert (speed hint)
                    assert(sources[j] < prefix.len());
                    // Veracity: NEEDED assert (speed hint)
                    assert(prefix[sources[j]] == entries[sources[j]]);
                };
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < prefix.len()
                    && (#[trigger] prefix[i]).0 == k
                    implies exists|j: int| 0 <= j < sources.len() && sources[j] == i
                by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= i < entries.len());
                    // Veracity: NEEDED assert (speed hint)
                    assert(entries[i].0 == k);
                };
                lemma_entries_to_map_subseq_value::<KV, VV>(prefix, filtered, sources, k);
            }
        }
    }

    // If the first n entries have no key k, the map value for k comes from the suffix.
    pub proof fn lemma_entries_to_map_skip_prefix<KV, VV>(
        entries: Seq<(KV, VV)>, n: int, k: KV,
    )
        requires
            0 <= n <= entries.len(),
            forall|i: int| 0 <= i < n ==> (#[trigger] entries[i]).0 != k,
            spec_entries_to_map(entries).contains_key(k),
        ensures
            spec_entries_to_map(entries.subrange(n, entries.len() as int)).contains_key(k),
            spec_entries_to_map(entries.subrange(n, entries.len() as int))[k]
                == spec_entries_to_map(entries)[k],
        decreases entries.len(),
    {
        if entries.len() == 0 {
        } else if n == entries.len() as int {
            lemma_entries_to_map_no_key::<KV, VV>(entries, k);
        } else {
            let suffix = entries.subrange(n, entries.len() as int);
            if entries.last().0 == k {
                // Veracity: NEEDED assert (speed hint)
                assert(suffix.len() > 0) by {
                    lemma_entries_to_map_key_in_seq(entries, k);
                    let idx = choose|idx: int| 0 <= idx < entries.len()
                        && (#[trigger] entries[idx]).0 == k;
// Veracity: UNNEEDED assert                     assert(idx >= n);
                };
// Veracity: UNNEEDED assert                 assert(suffix.last() == entries.last());
            } else {
                let prefix = entries.drop_last();
// Veracity: UNNEEDED assert                 assert(spec_entries_to_map(entries) =~=
// Veracity: UNNEEDED assert                     spec_entries_to_map(prefix).insert(entries.last().0, entries.last().1));
// Veracity: UNNEEDED assert                 assert(spec_entries_to_map(prefix).contains_key(k));
                // Veracity: NEEDED assert (speed hint)
                assert forall|i: int| 0 <= i < n implies (#[trigger] prefix[i]).0 != k by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(i < prefix.len());
// Veracity: UNNEEDED assert                     assert(prefix[i] == entries[i]);
                };
                lemma_entries_to_map_skip_prefix(prefix, n, k);
                // Veracity: NEEDED assert (speed hint)
                assert(suffix.len() > 0) by {
                    lemma_entries_to_map_key_in_seq(entries, k);
                    let idx = choose|idx: int| 0 <= idx < entries.len()
                        && (#[trigger] entries[idx]).0 == k;
// Veracity: UNNEEDED assert                     assert(idx >= n);
// Veracity: UNNEEDED assert                     assert(idx < entries.len() - 1 || entries.last().0 == k);
// Veracity: UNNEEDED assert                     assert(idx < entries.len() - 1);
                };
// Veracity: UNNEEDED assert                 assert(suffix.last() == entries.last());
                // Veracity: NEEDED assert
                assert(suffix.drop_last() =~= prefix.subrange(n, prefix.len() as int));
            }
        }
    }

    // If entries at indices [n, len) don't have key k, the map value comes from the prefix.
    pub proof fn lemma_entries_to_map_ignore_suffix<KV, VV>(
        entries: Seq<(KV, VV)>, n: int, k: KV,
    )
        requires
            0 <= n <= entries.len(),
            forall|i: int| n <= i < entries.len() ==> (#[trigger] entries[i]).0 != k,
            spec_entries_to_map(entries).contains_key(k),
        ensures
            spec_entries_to_map(entries.subrange(0, n)).contains_key(k),
            spec_entries_to_map(entries.subrange(0, n))[k] == spec_entries_to_map(entries)[k],
        decreases entries.len() - n,
    {
        if n == entries.len() as int {
            // Veracity: NEEDED assert
            assert(entries.subrange(0, n) =~= entries);
        } else {
            let last = entries.last();
            let prefix = entries.drop_last();
            // Veracity: NEEDED assert (speed hint)
            assert(last.0 != k);
// Veracity: UNNEEDED assert             assert(spec_entries_to_map(prefix).contains_key(k));
            // Veracity: NEEDED assert
            assert forall|i: int| n <= i < prefix.len()
                implies (#[trigger] prefix[i]).0 != k
            by {
// Veracity: UNNEEDED assert                 assert(prefix[i] == entries[i]);
            };
            lemma_entries_to_map_ignore_suffix(prefix, n, k);
            // Veracity: NEEDED assert
            assert(entries.subrange(0, n) =~= prefix.subrange(0, n));
        }
    }

    // If two sequences have same keys and same values at key-k positions, maps agree at k.
    pub proof fn lemma_entries_to_map_agree_on_key<KV, VV>(
        seq1: Seq<(KV, VV)>, seq2: Seq<(KV, VV)>, k: KV,
    )
        requires
            seq1.len() == seq2.len(),
            forall|i: int| 0 <= i < seq1.len()
                ==> (#[trigger] seq1[i]).0 == (#[trigger] seq2[i]).0,
            forall|i: int| 0 <= i < seq1.len() && (#[trigger] seq1[i]).0 == k
                ==> seq1[i].1 == seq2[i].1,
            spec_entries_to_map(seq1).contains_key(k),
        ensures
            spec_entries_to_map(seq2).contains_key(k),
            spec_entries_to_map(seq1)[k] == spec_entries_to_map(seq2)[k],
        decreases seq1.len(),
    {
        if seq1.len() > 0 {
            let last1 = seq1.last();
            let last2 = seq2.last();
// Veracity: UNNEEDED assert             assert(last1.0 == last2.0);
            if last1.0 == k {
                // Veracity: NEEDED assert (speed hint)
                assert(last1.1 == last2.1);
            } else {
                let prefix1 = seq1.drop_last();
                let prefix2 = seq2.drop_last();
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < prefix1.len()
                    implies (#[trigger] prefix1[i]).0 == (#[trigger] prefix2[i]).0
                by {
// Veracity: UNNEEDED assert                     assert(prefix1[i] == seq1[i]);
// Veracity: UNNEEDED assert                     assert(prefix2[i] == seq2[i]);
                };
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < prefix1.len() && (#[trigger] prefix1[i]).0 == k
                    implies prefix1[i].1 == prefix2[i].1
                by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(prefix1[i] == seq1[i]);
// Veracity: UNNEEDED assert                     assert(prefix2[i] == seq2[i]);
                };
                // Veracity: NEEDED assert (speed hint)
                assert(spec_entries_to_map(prefix1).contains_key(k));
                lemma_entries_to_map_agree_on_key(prefix1, prefix2, k);
            }
        }
    }

    // If original has no duplicate keys and filtered is a strictly-increasing
    // subsequence of original (via sources), then filtered has no duplicate keys.
    pub proof fn lemma_subseq_no_dups<KV, VV>(
        original: Seq<(KV, VV)>,
        filtered: Seq<(KV, VV)>,
        sources: Seq<int>,
    )
        requires
            spec_keys_no_dups(original),
            filtered.len() == sources.len(),
            forall|j: int| 0 <= j < sources.len() ==>
                0 <= #[trigger] sources[j] < original.len()
                && filtered[j].0 == original[sources[j]].0,
            forall|j1: int, j2: int| 0 <= j1 < j2 < sources.len()
                ==> sources[j1] < sources[j2],
        ensures spec_keys_no_dups(filtered),
    {
        // Veracity: NEEDED assert
        assert forall|a: int, b: int|
            0 <= a < b < filtered.len()
            implies (#[trigger] filtered[a]).0 != (#[trigger] filtered[b]).0
        by {
            // Veracity: NEEDED assert
            assert(sources[a] < sources[b]);
            // Veracity: NEEDED assert (speed hint)
            assert(filtered[a].0 == original[sources[a]].0);
// Veracity: UNNEEDED assert             assert(filtered[b].0 == original[sources[b]].0);
        };
    }

    // If filtered is a strictly-increasing subsequence of original (via sources),
    // then every key in spec_entries_to_map(filtered) is also in spec_entries_to_map(original).
    pub proof fn lemma_subseq_dom_forward<KV, VV>(
        original: Seq<(KV, VV)>,
        filtered: Seq<(KV, VV)>,
        sources: Seq<int>,
    )
        requires
            filtered.len() == sources.len(),
            forall|j: int| 0 <= j < sources.len() ==>
                0 <= #[trigger] sources[j] < original.len()
                && filtered[j].0 == original[sources[j]].0,
        ensures
            spec_entries_to_map(filtered).dom().subset_of(spec_entries_to_map(original).dom()),
    {
        // Veracity: NEEDED assert
        assert forall|k: KV| spec_entries_to_map(filtered).dom().contains(k)
            implies spec_entries_to_map(original).dom().contains(k)
        by {
            lemma_entries_to_map_key_in_seq(filtered, k);
            let idx = choose|idx: int| 0 <= idx < filtered.len()
                && (#[trigger] filtered[idx]).0 == k;
            let s = sources[idx];
            lemma_entries_to_map_contains_key(original, s);
        };
    }

    // If original has no duplicate keys and filtered is a strictly-increasing
    // subsequence of original (via sources), then for every key in the filtered
    // map, the value agrees with the original map.
    pub proof fn lemma_subseq_value_agrees<KV, VV>(
        original: Seq<(KV, VV)>,
        filtered: Seq<(KV, VV)>,
        sources: Seq<int>,
    )
        requires
            spec_keys_no_dups(original),
            filtered.len() == sources.len(),
            forall|j: int| 0 <= j < sources.len() ==>
                0 <= #[trigger] sources[j] < original.len()
                && filtered[j] == original[sources[j]],
            forall|j1: int, j2: int| 0 <= j1 < j2 < sources.len()
                ==> sources[j1] < sources[j2],
        ensures
            forall|k: KV| spec_entries_to_map(filtered).contains_key(k) ==> (
                spec_entries_to_map(original).contains_key(k)
                && #[trigger] spec_entries_to_map(filtered)[k]
                    == spec_entries_to_map(original)[k]
            ),
    {
        // First establish dom forward.
        lemma_subseq_dom_forward(original, filtered, sources);
        // Veracity: NEEDED assert
        assert forall|k: KV| spec_entries_to_map(filtered).contains_key(k)
            implies spec_entries_to_map(original).contains_key(k)
                && #[trigger] spec_entries_to_map(filtered)[k]
                    == spec_entries_to_map(original)[k]
        by {
            // filtered[j] == original[sources[j]], and sources is monotone,
            // so all original entries with key k that appear in filtered are
            // exactly those at sources positions. Since original has no dups,
            // there is exactly one entry with key k at sources[j] for some j.
            lemma_entries_to_map_key_in_seq(filtered, k);
            let j = choose|j: int| 0 <= j < filtered.len()
                && (#[trigger] filtered[j]).0 == k;
            let s = sources[j];
            // All original entries with key k are covered by sources.
            // Veracity: NEEDED assert
            assert forall|i: int| 0 <= i < original.len()
                && (#[trigger] original[i]).0 == k
                implies exists|jj: int| 0 <= jj < sources.len() && sources[jj] == i
            by {
                // original has no dups, so i == s is the only entry with key k.
                // Veracity: NEEDED assert (speed hint)
                assert(original[s].0 == k);
                if i != s {
                    if i < s {
                        // Veracity: NEEDED assert (speed hint)
                        assert(original[i].0 != original[s].0);
                    } else {
                        // Veracity: NEEDED assert (speed hint)
                        assert(original[s].0 != original[i].0);
                    }
                }
                // Veracity: NEEDED assert (speed hint)
                assert(i == s);
            };
            lemma_entries_to_map_subseq_value(original, filtered, sources, k);
        };
    }

    } // verus!
} // pub mod TableSpecsAndLemmas
