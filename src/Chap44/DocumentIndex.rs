//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 44: Document Indexing and Searching implementation.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 6a. spec fns
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 8b. traits
//	Section 9b. impls
//	Section 12a. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod DocumentIndex {

    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
    use crate::vstdplus::strings::strings::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_clone};

    pub type Word = String;
    pub type DocumentId = String;
    pub type Contents = String;
    pub type DocumentSet = AVLTreeSetStPer<DocumentId>;

    /// Document collection type - sequence of (id, contents) pairs.
    pub type DocumentCollection = ArraySeqStPerS<Pair<DocumentId, Contents>>;

    verus! 
{

    //		Section 4a. type definitions


    /// Document Index structure implementing Data Type 44.1.
    pub struct DocumentIndex {
        pub word_to_docs: TableStPer<Word, DocumentSet>,
    }

    //		Section 6a. spec fns


    /// Well-formedness predicate for DocumentIndex.
    /// Requires the table to be well-formed and all stored DocumentSets
    /// to be well-formed with bounded size (each set < usize::MAX/2 entries).
    pub open spec fn spec_documentindex_wf(di: &DocumentIndex) -> bool {
        &&& di.word_to_docs.spec_tablestper_wf()
        &&& forall|k: Seq<char>|
                #[trigger] di.word_to_docs@.contains_key(k) ==> {
                    let ds = di.word_to_docs.spec_stored_value(k);
                    &&& ds.spec_avltreesetstper_wf()
                    &&& ds@.len() <= usize::MAX as nat / 2
                }
    }

    //		Section 8a. traits


    /// Trait defining the Document Index ADT (Data Type 44.1).
    pub trait DocumentIndexTrait: Sized {
        /// Well-formedness spec.
        spec fn spec_documentindex_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch44 Alg 44.2): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — ACCEPTED DIFFERENCE: sequential loops, APAS Span O(lg^2 n)
        fn make_index(docs: &DocumentCollection) -> (di: Self)
            requires
                docs.spec_len() <= usize::MAX as nat / 2,
                obeys_view_eq::<Word>(),
                obeys_feq_full::<Pair<Word, DocumentSet>>(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
            ensures di.spec_documentindex_wf();

        /// - Alg Analysis: APAS (Ch44 Alg 44.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find(&self, word: &Word) -> (found: DocumentSet)
            requires
                self.spec_documentindex_wf(),
                obeys_view_eq::<Word>(),
                obeys_feq_full::<DocumentSet>(),
            ensures
                found.spec_avltreesetstper_wf(),
                found@.len() <= usize::MAX as nat / 2,
        ;

        /// - Alg Analysis: APAS (Ch44 Alg 44.3): Work O(m * lg(1+n/m)), Span O(lg n + lg m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — ACCEPTED DIFFERENCE: sequential split-join
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (combined: DocumentSet)
            requires
                docs_a.spec_avltreesetstper_wf(),
                docs_b.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
            ensures
                combined@ == docs_a@.intersect(docs_b@),
                combined.spec_avltreesetstper_wf(),
        ;

        /// - Alg Analysis: APAS (Ch44 Alg 44.3): Work O(m * lg(1+n/m)), Span O(lg n + lg m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — ACCEPTED DIFFERENCE: sequential split-join
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (combined: DocumentSet)
            requires
                docs_a.spec_avltreesetstper_wf(),
                docs_b.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
                docs_a@.len() + docs_b@.len() < usize::MAX as nat,
            ensures
                combined@ == docs_a@.union(docs_b@),
                combined.spec_avltreesetstper_wf(),
        ;

        /// - Alg Analysis: APAS (Ch44 Alg 44.3): Work O(m * lg(1+n/m)), Span O(lg n + lg m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(m·lg(1+n/m)) — ACCEPTED DIFFERENCE: sequential split-join
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (remaining: DocumentSet)
            requires
                docs_a.spec_avltreesetstper_wf(),
                docs_b.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
            ensures
                remaining@ == docs_a@.difference(docs_b@),
                remaining.spec_avltreesetstper_wf(),
        ;

        /// - Alg Analysis: APAS (Ch44 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees with APAS
        fn size(docs: &DocumentSet) -> (count: usize)
            requires docs.spec_avltreesetstper_wf()
            ensures count == docs@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential iteration over AVL tree sequence
        fn to_seq(docs: &DocumentSet) -> (seq: ArraySeqStPerS<DocumentId>)
            requires docs.spec_avltreesetstper_wf()
            ensures seq.spec_arrayseqstper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (di: Self)
            ensures di.spec_documentindex_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — collects table keys into sequence
        fn get_all_words(&self) -> (words: ArraySeqStPerS<Word>)
            requires self.spec_documentindex_wf()
            ensures words.spec_arrayseqstper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to Table.size
        fn word_count(&self) -> (count: usize)
            requires self.spec_documentindex_wf();
    }

    //		Section 9a. impls


    impl DocumentIndexTrait for DocumentIndex {
        open spec fn spec_documentindex_wf(&self) -> bool {
            spec_documentindex_wf(self)
        }

        /// Algorithm 44.2: Make Index — table-based insert.
        /// Iterate docs, iterate words per doc, insert each word into the table.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(D * W * lg n), Span O(D * W * lg n) — nested loops: D docs × W words/doc × O(lg n) table insert; St sequential.
        fn make_index(docs: &DocumentCollection) -> (di: Self) {
            // Veracity: NEEDED proof block (speed hint)
            proof {
// Veracity: UNNEEDED assert                 assert(Pair_feq_trigger::<Word, DocumentSet>());
            }
            let mut table = TableStPer::<Word, DocumentSet>::empty();
            let ghost mut gds: Set<Seq<char>> = Set::empty();
            let mut i: usize = 0;
            while i < docs.length()
                invariant
                    i <= docs.spec_len(),
                    docs.spec_len() <= usize::MAX as nat / 2,
                    table.spec_tablestper_wf(),
                    obeys_view_eq::<Word>(),
                    obeys_feq_full::<Pair<Word, DocumentSet>>(),
                    vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                    view_ord_consistent::<DocumentId>(),
                    gds.finite(),
                    gds.len() <= i as nat,
                    forall|k: Seq<char>| #[trigger] table@.contains_key(k) ==> {
                        let ds = table.spec_stored_value(k);
                        &&& ds.spec_avltreesetstper_wf()
                        &&& ds@.subset_of(gds)
                    },
                decreases docs.spec_len() - i,
            {
                let doc = docs.nth(i);
                let doc_id: DocumentId = doc.0.clone();
                // Veracity: NEEDED proof block
                let words = tokens(&doc.1);
                proof {
                    let ghost old_gds = gds;
                    gds = gds.insert(doc_id@);
                    vstd::set::axiom_set_insert_finite(old_gds, doc_id@);
                    vstd::set::axiom_set_insert_len(old_gds, doc_id@);
                    // Veracity: NEEDED assert
                    assert forall|k: Seq<char>| #[trigger] table@.contains_key(k) implies {
                        let ds = table.spec_stored_value(k);
                        ds.spec_avltreesetstper_wf() && ds@.subset_of(gds)
                    } by {
                        let ds = table.spec_stored_value(k);
// Veracity: UNNEEDED assert                         assert(ds@.subset_of(old_gds));
                        // Veracity: NEEDED assert (speed hint)
                        assert forall|v: Seq<char>| #[trigger] ds@.contains(v) implies gds.contains(v) by {
                            // Veracity: NEEDED assert (speed hint)
                            assert(old_gds.contains(v));
                        };
                    };
                }
                let ghost gds_snap = gds;
                // Veracity: NEEDED proof block
                let ghost doc_id_view: Seq<char> = doc_id@;
                // Veracity: NEEDED assert (speed hint)
                proof { assert(Pair_feq_trigger::<Word, DocumentSet>()); }
                let mut j: usize = 0;
                while j < words.length()
                    invariant
                        j <= words.spec_len(),
                        words.spec_arrayseqstper_wf(),
                        i < docs.spec_len(),
                        docs.spec_len() <= usize::MAX as nat / 2,
                        table.spec_tablestper_wf(),
                        obeys_view_eq::<Word>(),
                        obeys_feq_full::<Pair<Word, DocumentSet>>(),
                        vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                        view_ord_consistent::<DocumentId>(),
                        gds == gds_snap,
                        gds.finite(),
                        gds.len() <= (i + 1) as nat,
                        gds.contains(doc_id_view),
                        doc_id@ == doc_id_view,
                        forall|k: Seq<char>| #[trigger] table@.contains_key(k) ==> {
                            let ds = table.spec_stored_value(k);
                            &&& ds.spec_avltreesetstper_wf()
                            &&& ds@.subset_of(gds)
                        },
                    decreases words.spec_len() - j,
                {
                    let word: &Word = words.nth(j);
                    // Veracity: NEEDED proof block
                    let singleton = AVLTreeSetStPer::singleton(doc_id.clone());
                    let new_set: DocumentSet = match table.find_ref(word) {
                        Some(existing) => {
                            proof {
                                vstd::set_lib::lemma_len_subset(existing@, gds);
                                // existing@.len() <= gds.len() <= i + 1 <= usize::MAX/2
                                // Veracity: NEEDED assert (speed hint)
                                assert(existing@.len() <= gds.len());
// Veracity: UNNEEDED assert                                 assert(gds.len() <= (i + 1) as nat);
// Veracity: UNNEEDED assert                                 assert((i + 1) as nat <= docs.spec_len());
                                // Veracity: NEEDED assert (speed hint)
                                assert(existing@.len() <= usize::MAX as nat / 2);
                                // singleton: {doc_id@}, len 1
// Veracity: UNNEEDED assert                                 assert(singleton@ == Set::<<DocumentId as View>::V>::empty().insert(doc_id@));
                                vstd::set::axiom_set_insert_len(
                                    Set::<<DocumentId as View>::V>::empty(), doc_id@);
                                // Veracity: NEEDED assert (speed hint)
                                assert(singleton@.len() == 1);
                                // Veracity: NEEDED assert (speed hint)
                                assert(existing@.len() + singleton@.len() <= usize::MAX as nat / 2 + 1);
// Veracity: UNNEEDED assert                                 assert(usize::MAX as nat / 2 + 1 < usize::MAX as nat);
                            }
                            existing.union(&singleton)
                        },
                        None => singleton,
                    };
                    let word_owned: Word = word.clone();
                    let ghost word_v: Seq<char> = word_owned@;
                    let ghost old_table = table;
                    let ghost new_set_view = new_set@;
                    table = table.insert(word_owned, new_set,
                        // Veracity: NEEDED proof block
                        |_old: &DocumentSet, nd: &DocumentSet| -> (r: DocumentSet)
                            ensures r@ == nd@
                        { nd.clone() }
                    );
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|k: Seq<char>| #[trigger] table@.contains_key(k)
                            implies {
                                let ds = table.spec_stored_value(k);
                                ds.spec_avltreesetstper_wf() && ds@.subset_of(gds)
                            }
                        by {
                            if k == word_v {
                                let ds = table.spec_stored_value(k);
                                // In both found/not-found cases, ds@ == new_set_view.
                                if !old_table@.contains_key(word_v) {
// Veracity: UNNEEDED assert                                     assert(ds@ == new_set_view);
                                } else {
                                    // Existential witness gives ds@ == new_set_view.
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(table@[word_v] == new_set_view);
                                    table.lemma_spec_stored_value_view(k);
                                    // Veracity: NEEDED assert (speed hint)
                                    assert(ds@ == new_set_view);
                                }
                                // Subset: new_set@ ⊆ gds.
                                // Veracity: NEEDED assert
                                assert forall|v: Seq<char>| new_set_view.contains(v)
                                    implies gds.contains(v)
                                by {
                                    // new_set@ ⊆ existing@ ∪ {doc_id@} ⊆ gds.
                                };
// Veracity: UNNEEDED assert                                 assert(ds@.subset_of(gds));
                                // Wf: ds@.finite() and ds@.len() < usize::MAX.
                                // Veracity: NEEDED assert (speed hint)
                                assert(ds@.finite());
// Veracity: UNNEEDED assert                                 assert(ds@.len() < usize::MAX as nat);
                            } else {
                                // Veracity: NEEDED assert (speed hint)
                                assert(old_table@.contains_key(k));
                                let ds = table.spec_stored_value(k);
                                let old_ds = old_table.spec_stored_value(k);
                                old_table.lemma_spec_stored_value_view(k);
                                table.lemma_spec_stored_value_view(k);
                                // ds@ == table@[k] == old_table@[k] == old_ds@.
                                assert(old_ds.spec_avltreesetstper_wf());
// Veracity: UNNEEDED assert                                 assert(old_ds@.subset_of(gds));
                                // Veracity: NEEDED assert (speed hint)
                                assert(ds@.subset_of(gds));
                                // Veracity: NEEDED assert (speed hint)
                                assert(ds@.finite());
// Veracity: UNNEEDED assert                                 assert(ds@.len() < usize::MAX as nat);
                            }
                        };
                    // Veracity: NEEDED proof block
                    }
                    j += 1;
                }
                i += 1;
            }
            proof {
                // Veracity: NEEDED assert
                assert forall|k: Seq<char>| #[trigger] table@.contains_key(k) implies {
                    let ds = table.spec_stored_value(k);
                    ds.spec_avltreesetstper_wf() && ds@.len() <= usize::MAX as nat / 2
                } by {
                    let ds = table.spec_stored_value(k);
                    vstd::set_lib::lemma_len_subset(ds@, gds);
                };
            }
            DocumentIndex { word_to_docs: table }
        }

        /// Algorithm 44.3: find function - simple table lookup.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — BST table lookup + clone; St sequential.
        fn find(&self, word: &Word) -> (found: DocumentSet) {
            match self.word_to_docs.find_ref(word) {
                Some(doc_set_ref) => {
                    doc_set_ref.clone_wf()
                },
                None => AVLTreeSetStPer::empty(),
            }
        }

        /// Algorithm 44.3: queryAnd - set intersection.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — AVL tree set intersection; St sequential.
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (combined: DocumentSet) {
            docs_a.intersection(docs_b)
        }

        /// Algorithm 44.3: queryOr - set union.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — AVL tree set union; St sequential.
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (combined: DocumentSet) {
            docs_a.union(docs_b)
        }

        /// Algorithm 44.3: queryAndNot - set difference.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — AVL tree set difference; St sequential.
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (remaining: DocumentSet) {
            docs_a.difference(docs_b)
        }

        /// Algorithm 44.3: size function.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — AVL tree cached size.
        fn size(docs: &DocumentSet) -> (count: usize) {
            docs.size()
        }

        /// Algorithm 44.3: toSeq function.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — in-order traversal collecting n elements; St sequential.
        fn to_seq(docs: &DocumentSet) -> (seq: ArraySeqStPerS<DocumentId>) {
            let avl_seq = docs.to_seq();
            let len = avl_seq.length();
            let mut result: Vec<DocumentId> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    avl_seq.spec_avltreeseqstper_wf(),
                    len == avl_seq.spec_seq().len(),
                    i <= len,
                    result@.len() == i as int,
                decreases len - i,
            {
                let doc_id = avl_seq.nth(i);
                result.push(doc_id.clone());
                i += 1;
            }
            ArraySeqStPerS::from_vec(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty table allocation.
        fn empty() -> (di: Self) {
            DocumentIndex {
                word_to_docs: TableStPer::empty(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — collect all table entries then extract keys; St sequential.
        fn get_all_words(&self) -> (words: ArraySeqStPerS<Word>) {
            let entries = self.word_to_docs.collect();
            let len = entries.length();
            let mut result: Vec<Word> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    len == entries.spec_len(),
                    i <= len,
                    result@.len() == i as int,
                decreases len - i,
            {
                let entry = entries.nth(i);
                result.push(entry.0.clone());
                i += 1;
            }
            ArraySeqStPerS::from_vec(result)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — table size query.
        fn word_count(&self) -> (count: usize) {
            self.word_to_docs.size()
        }
    }

    //		Section 4b. type definitions


    /// Complex query builder for chaining operations.
    pub struct QueryBuilder<'a> {
        pub index: &'a DocumentIndex,
    }

    //		Section 8b. traits


    pub trait QueryBuilderTrait<'a>: Sized {
        /// Spec: whether the underlying index is well-formed.
        spec fn spec_index_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new(index: &'a DocumentIndex) -> (qb: Self)
            requires spec_documentindex_wf(index)
            ensures qb.spec_index_wf();

        /// - Alg Analysis: APAS (Ch44 Alg 44.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find(&self, word: &Word) -> (found: DocumentSet)
            requires
                self.spec_index_wf(),
                obeys_view_eq::<Word>(),
                obeys_feq_full::<DocumentSet>(),
            ensures
                found.spec_avltreesetstper_wf(),
                found@.len() <= usize::MAX as nat / 2,
        ;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(1 + n/m)), Span O(m log(1 + n/m))
        fn and(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (combined: DocumentSet)
            requires
                docs_a.spec_avltreesetstper_wf(),
                docs_b.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
            ensures
                combined@ == docs_a@.intersect(docs_b@),
                combined.spec_avltreesetstper_wf(),
        ;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(1 + n/m)), Span O(m log(1 + n/m))
        fn or(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (combined: DocumentSet)
            requires
                docs_a.spec_avltreesetstper_wf(),
                docs_b.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
                docs_a@.len() + docs_b@.len() < usize::MAX as nat,
            ensures
                combined@ == docs_a@.union(docs_b@),
                combined.spec_avltreesetstper_wf(),
        ;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(1 + n/m)), Span O(m log(1 + n/m))
        fn and_not(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (remaining: DocumentSet)
            requires
                docs_a.spec_avltreesetstper_wf(),
                docs_b.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
            ensures
                remaining@ == docs_a@.difference(docs_b@),
                remaining.spec_avltreesetstper_wf(),
        ;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work dominated by 4 finds + 3 set operations
        fn complex_query(&self, word1: &Word, word2: &Word, word3: &Word, word4: &Word) -> (found: DocumentSet)
            requires
                self.spec_index_wf(),
                obeys_view_eq::<Word>(),
                obeys_feq_full::<DocumentSet>(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
        ;
    }

    //		Section 9b. impls


    impl<'a> QueryBuilderTrait<'a> for QueryBuilder<'a> {
        open spec fn spec_index_wf(&self) -> bool {
            spec_documentindex_wf(self.index)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — reference copy.
        fn new(index: &'a DocumentIndex) -> (qb: Self) {
            QueryBuilder { index }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — delegates to DocumentIndex::find.
        fn find(&self, word: &Word) -> (found: DocumentSet) {
            self.index.find(word)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — delegates to query_and.
        fn and(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (combined: DocumentSet) {
            DocumentIndex::query_and(&docs_a, &docs_b)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — delegates to query_or.
        fn or(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (combined: DocumentSet) {
            DocumentIndex::query_or(&docs_a, &docs_b)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — delegates to query_and_not.
        fn and_not(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (remaining: DocumentSet) {
            DocumentIndex::query_and_not(&docs_a, &docs_b)
        }

        /// Complex query: (word1 AND word2) OR (word3 AND NOT word4).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — 4 finds + 3 set ops; St sequential.
        fn complex_query(&self, word1: &Word, word2: &Word, word3: &Word, word4: &Word) -> (found: DocumentSet) {
            let set1 = self.find(word1);
            let set2 = self.find(word2);
            // Veracity: NEEDED proof block
            let set3 = self.find(word3);
            let set4 = self.find(word4);

            let left_side = self.and(set1, set2);
            let right_side = self.and_not(set3, set4);

            proof {
                vstd::set_lib::lemma_len_intersect::<Seq<char>>(set1@, set2@);
                vstd::set_lib::lemma_len_difference::<Seq<char>>(set3@, set4@);
            }

            self.or(left_side, right_side)
        }
    }

    // 10. free functions

    /// Tokenization: splits content into lowercase ASCII words.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — sequential character iteration
    // veracity: no_requires
    pub fn tokens(content: &Contents) -> (words: ArraySeqStPerS<Word>)
        ensures words.spec_arrayseqstper_wf()
    {
        let mut result: Vec<Word> = Vec::new();
        let mut current_word = String::new();
        let content_str: &str = content.as_str();

        let mut chars = content_str.chars();
        loop
            invariant true,
            decreases chars@.1.len() - chars@.0,
        {
            let ch_opt = chars.next();
            match ch_opt {
                None => break,
                Some(ch) => {
                    if char_is_ascii_alphabetic(ch) {
                        let lc = char_to_ascii_lowercase(ch);
                        string_push(&mut current_word, lc);
                    } else if !string_is_empty(&current_word) {
                        result.push(current_word);
                        current_word = String::new();
                    }
                },
            }
        }

        if !string_is_empty(&current_word) {
            result.push(current_word);
        }

        ArraySeqStPerS::from_vec(result)
    }

    //		Section 12a. derive impls in verus!
// Veracity: NEEDED proof block (speed hint)


    impl Clone for DocumentIndex {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = DocumentIndex { word_to_docs: self.word_to_docs.clone() };
            proof { assume(cloned == *self); }
            cloned
        }
    }

    impl core::cmp::PartialEq for DocumentIndex {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> bool {
            self.word_to_docs == other.word_to_docs
        }
    }

    impl core::cmp::Eq for DocumentIndex {}

    } // verus!

    //		Section 13. macros


    /// Macro for creating document collections.
    #[macro_export]
    macro_rules! DocumentCollectionLit {
        ($($id:expr => $content:expr),* $(,)?) => {{
            type PairSS = $crate::Types::Types::Pair<String, String>;
            type SeqT = $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS<PairSS>;
            #[allow(unused_mut)]
            let mut docs = <SeqT as $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait<PairSS>>::empty();
            $(
                let pair = $crate::Types::Types::Pair($id.to_string(), $content.to_string());
                let single_seq = <SeqT as $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait<PairSS>>::singleton(pair);
                docs = <SeqT as $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait<PairSS>>::append(&docs, &single_seq);
            )*
            docs
        }};
        () => {{
            type PairSS = $crate::Types::Types::Pair<String, String>;
            type SeqT = $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS<PairSS>;
            <SeqT as $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait<PairSS>>::empty()
        }};
    }

    //		Section 14. derive impls outside verus!

    /// Convenience function for staged computation pattern (Example 44.2).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — closure capture only
    pub fn create_finder(index: &DocumentIndex) -> impl Fn(&Word) -> DocumentSet + '_ {
        move |word: &Word| index.find(word)
    }

    //		Section 14a. derive impls outside verus!

    impl Display for DocumentIndex {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "DocumentIndex(words: {}, total_mappings: {})",
                self.word_count(),
                self.word_to_docs.size()
            )
        }
    }

    impl Debug for DocumentIndex {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DocumentIndex")
                .field("word_count", &self.word_count())
                .field("word_to_docs", &self.word_to_docs)
                .finish()
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<'a> Debug for QueryBuilder<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "QueryBuilder({:?})", self.index)
        }
    }

    impl<'a> Display for QueryBuilder<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "QueryBuilder({})", self.index)
        }
    }
}
