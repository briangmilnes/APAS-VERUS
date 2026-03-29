//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Document Indexing and Searching implementation.

// Table of Contents
// 1. module
// 2. imports
// 4a. type definitions — struct DocumentIndex
// 6a. spec fns — struct DocumentIndex
// 8a. traits — struct DocumentIndex
// 9a. impls — struct DocumentIndex
// 4b. type definitions — struct QueryBuilder
// 8b. traits — struct QueryBuilder
// 9b. impls — struct QueryBuilder
// 10. free functions
// 12a. derive impls in verus! — struct DocumentIndex
// 13. macros
// 14a. derive impls outside verus! — struct DocumentIndex

pub mod DocumentIndex {

    // 2. imports

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

    verus! {

    // 4a. type definitions — struct DocumentIndex

    /// Document Index structure implementing Data Type 44.1.
    pub struct DocumentIndex {
        pub word_to_docs: TableStPer<Word, DocumentSet>,
    }

    // 6a. spec fns — struct DocumentIndex

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

    // 8a. traits — struct DocumentIndex

    /// Trait defining the Document Index ADT (Data Type 44.1).
    pub trait DocumentIndexTrait: Sized {
        /// Well-formedness spec.
        spec fn spec_documentindex_wf(&self) -> bool;

        /// - APAS: Work O(n log n), Span O(log^2 n)
        /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — sequential nested loops over all_pairs; no Table.collect sort used
        fn make_index(docs: &DocumentCollection) -> (di: Self)
            ensures di.spec_documentindex_wf();

        /// - APAS: Work O(log n), Span O(log n)
        /// - Claude-Opus-4.6: Work O(log n), Span O(log n) — agrees with APAS; delegates to Table.find
        fn find(&self, word: &Word) -> (found: DocumentSet)
            requires
                self.spec_documentindex_wf(),
                obeys_view_eq::<Word>(),
                obeys_feq_full::<DocumentSet>(),
            ensures
                found.spec_avltreesetstper_wf(),
                found@.len() <= usize::MAX as nat / 2,
        ;

        /// - APAS: Work O(m log(1 + n/m)), Span O(log n + log m)
        /// - Claude-Opus-4.6: Work O(m log(1 + n/m)), Span O(m log(1 + n/m)) — delegates to AVLTreeSetStPer.intersection (sequential)
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

        /// - APAS: Work O(m log(1 + n/m)), Span O(log n + log m)
        /// - Claude-Opus-4.6: Work O(m log(1 + n/m)), Span O(m log(1 + n/m)) — delegates to AVLTreeSetStPer.union (sequential)
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

        /// - APAS: Work O(m log(1 + n/m)), Span O(log n + log m)
        /// - Claude-Opus-4.6: Work O(m log(1 + n/m)), Span O(m log(1 + n/m)) — delegates to AVLTreeSetStPer.difference (sequential)
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

        /// - APAS: Work O(1), Span O(1)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — agrees with APAS
        fn size(docs: &DocumentSet) -> (count: usize)
            requires docs.spec_avltreesetstper_wf()
            ensures count == docs@.len();

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — sequential iteration over AVL tree sequence
        fn to_seq(docs: &DocumentSet) -> (seq: ArraySeqStPerS<DocumentId>)
            requires docs.spec_avltreesetstper_wf()
            ensures seq.spec_arrayseqstper_wf();

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1)
        fn empty() -> (di: Self)
            ensures di.spec_documentindex_wf();

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — collects table keys into sequence
        fn get_all_words(&self) -> (words: ArraySeqStPerS<Word>)
            requires self.spec_documentindex_wf()
            ensures words.spec_arrayseqstper_wf();

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — delegates to Table.size
        fn word_count(&self) -> (count: usize)
            requires self.spec_documentindex_wf();
    }

    // 9a. impls — struct DocumentIndex

    impl DocumentIndexTrait for DocumentIndex {
        open spec fn spec_documentindex_wf(&self) -> bool {
            spec_documentindex_wf(self)
        }

        /// Algorithm 44.2: Make Index.
        /// Sort-based grouping: O(n log n) instead of O(n^2) quadratic rescan.
        #[verifier::external_body]
        fn make_index(docs: &DocumentCollection) -> (di: Self) {
            let mut pairs_vec: Vec<(Word, DocumentId)> = Vec::new();

            for i in 0..docs.length() {
                let doc = docs.nth(i);
                let doc_id = doc.0.clone();
                let content = &doc.1;
                let word_tokens = tokens(content);

                for j in 0..word_tokens.length() {
                    let word = word_tokens.nth(j).clone();
                    pairs_vec.push((word, doc_id.clone()));
                }
            }

            pairs_vec.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

            let mut final_table = TableStPer::empty();
            let mut i = 0;

            while i < pairs_vec.len() {
                let word = pairs_vec[i].0.clone();
                let mut doc_ids = Vec::new();

                while i < pairs_vec.len() && pairs_vec[i].0 == word {
                    doc_ids.push(pairs_vec[i].1.clone());
                    i += 1;
                }

                let avl_seq = AVLTreeSeqStPerS::from_vec(doc_ids);
                let doc_set = AVLTreeSetStPer::from_seq(avl_seq);
                final_table = final_table.insert(word, doc_set, |_old, new| new.clone());
            }

            DocumentIndex {
                word_to_docs: final_table,
            }
        }

        /// Algorithm 44.3: find function - simple table lookup.
        fn find(&self, word: &Word) -> (found: DocumentSet) {
            match self.word_to_docs.find_ref(word) {
                Some(doc_set_ref) => {
                    doc_set_ref.clone_wf()
                },
                None => AVLTreeSetStPer::empty(),
            }
        }

        /// Algorithm 44.3: queryAnd - set intersection.
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (combined: DocumentSet) {
            docs_a.intersection(docs_b)
        }

        /// Algorithm 44.3: queryOr - set union.
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (combined: DocumentSet) {
            docs_a.union(docs_b)
        }

        /// Algorithm 44.3: queryAndNot - set difference.
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> (remaining: DocumentSet) {
            docs_a.difference(docs_b)
        }

        /// Algorithm 44.3: size function.
        fn size(docs: &DocumentSet) -> (count: usize) {
            docs.size()
        }

        /// Algorithm 44.3: toSeq function.
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

        fn empty() -> (di: Self) {
            DocumentIndex {
                word_to_docs: TableStPer::empty(),
            }
        }

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

        fn word_count(&self) -> (count: usize) {
            self.word_to_docs.size()
        }
    }

    // 4b. type definitions — struct QueryBuilder

    /// Complex query builder for chaining operations.
    pub struct QueryBuilder<'a> {
        pub index: &'a DocumentIndex,
    }

    // 8b. traits — struct QueryBuilder

    pub trait QueryBuilderTrait<'a>: Sized {
        /// Spec: whether the underlying index is well-formed.
        spec fn spec_index_wf(&self) -> bool;

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1)
        fn new(index: &'a DocumentIndex) -> (qb: Self)
            requires spec_documentindex_wf(index)
            ensures qb.spec_index_wf();

        /// - APAS: N/A — delegates to DocumentIndex::find.
        /// - Claude-Opus-4.6: Work O(log n), Span O(log n)
        fn find(&self, word: &Word) -> (found: DocumentSet)
            requires
                self.spec_index_wf(),
                obeys_view_eq::<Word>(),
                obeys_feq_full::<DocumentSet>(),
            ensures
                found.spec_avltreesetstper_wf(),
                found@.len() <= usize::MAX as nat / 2,
        ;

        /// - APAS: N/A — delegates to DocumentIndex::query_and.
        /// - Claude-Opus-4.6: Work O(m log(1 + n/m)), Span O(m log(1 + n/m))
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

        /// - APAS: N/A — delegates to DocumentIndex::query_or.
        /// - Claude-Opus-4.6: Work O(m log(1 + n/m)), Span O(m log(1 + n/m))
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

        /// - APAS: N/A — delegates to DocumentIndex::query_and_not.
        /// - Claude-Opus-4.6: Work O(m log(1 + n/m)), Span O(m log(1 + n/m))
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

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work dominated by 4 finds + 3 set operations
        fn complex_query(&self, word1: &Word, word2: &Word, word3: &Word, word4: &Word) -> (result: DocumentSet)
            requires
                self.spec_index_wf(),
                obeys_view_eq::<Word>(),
                obeys_feq_full::<DocumentSet>(),
                vstd::laws_cmp::obeys_cmp_spec::<DocumentId>(),
                view_ord_consistent::<DocumentId>(),
        ;
    }

    // 9b. impls — struct QueryBuilder

    impl<'a> QueryBuilderTrait<'a> for QueryBuilder<'a> {
        open spec fn spec_index_wf(&self) -> bool {
            spec_documentindex_wf(self.index)
        }

        fn new(index: &'a DocumentIndex) -> (qb: Self) {
            QueryBuilder { index }
        }

        fn find(&self, word: &Word) -> (found: DocumentSet) {
            self.index.find(word)
        }

        fn and(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (combined: DocumentSet) {
            DocumentIndex::query_and(&docs_a, &docs_b)
        }

        fn or(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (combined: DocumentSet) {
            DocumentIndex::query_or(&docs_a, &docs_b)
        }

        fn and_not(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> (remaining: DocumentSet) {
            DocumentIndex::query_and_not(&docs_a, &docs_b)
        }

        /// Complex query: (word1 AND word2) OR (word3 AND NOT word4).
        fn complex_query(&self, word1: &Word, word2: &Word, word3: &Word, word4: &Word) -> (result: DocumentSet) {
            let set1 = self.find(word1);
            let set2 = self.find(word2);
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
    /// - APAS: (no cost stated — tokens is a helper assumed O(m) where m = string length)
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential character iteration
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

    // 12a. derive impls in verus! — struct DocumentIndex

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

    /// Convenience function for staged computation pattern (Example 44.2).
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1) — closure capture only
    pub fn create_finder(index: &DocumentIndex) -> impl Fn(&Word) -> DocumentSet + '_ {
        move |word: &Word| index.find(word)
    }

    // 13. macros

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

    // 14a. derive impls outside verus! — struct DocumentIndex

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
}
