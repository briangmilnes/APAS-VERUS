//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Document Indexing and Searching implementation.

pub mod DocumentIndex {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;

    verus! {
        /// Placeholder; DocumentIndex uses Vec, Box<dyn Fn>, chars().
        proof fn _document_index_verified() {}
    }

    pub type Word = String;
    pub type DocumentId = String;
    pub type Contents = String;
    pub type DocumentSet = AVLTreeSetStPer<DocumentId>;

    /// Document collection type - sequence of (id, contents) pairs
    pub type DocumentCollection = ArraySeqStPerS<Pair<DocumentId, Contents>>;

    /// Document Index structure implementing Data Type 44.1
    #[derive(PartialEq, Clone)]
    pub struct DocumentIndex {
        word_to_docs: TableStPer<Word, DocumentSet>,
    }

    /// Trait defining the Document Index ADT (Data Type 44.1)
    pub trait DocumentIndexTrait {
        /// - APAS: Work Θ(n log n), Span Θ(log² n)
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential nested loops over all_pairs; no Table.collect sort used
        fn make_index(docs: &DocumentCollection)                     -> Self;

        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) — agrees with APAS; delegates to Table.find
        fn find(&self, word: &Word)                                  -> DocumentSet;

        /// - APAS: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// - Claude-Opus-4.6: Work Θ(m log(1 + n/m)), Span Θ(m log(1 + n/m)) — delegates to AVLTreeSetStPer.intersection (sequential)
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet)     -> DocumentSet;

        /// - APAS: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// - Claude-Opus-4.6: Work Θ(m log(1 + n/m)), Span Θ(m log(1 + n/m)) — delegates to AVLTreeSetStPer.union (sequential)
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet)      -> DocumentSet;

        /// - APAS: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// - Claude-Opus-4.6: Work Θ(m log(1 + n/m)), Span Θ(m log(1 + n/m)) — delegates to AVLTreeSetStPer.difference (sequential)
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS
        fn size(docs: &DocumentSet)                                  -> N;

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential iteration over AVL tree sequence
        fn to_seq(docs: &DocumentSet)                                -> ArraySeqStPerS<DocumentId>;

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn empty()                                                   -> Self;

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — collects table keys into sequence
        fn get_all_words(&self)                                      -> ArraySeqStPerS<Word>;

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to Table.size
        fn word_count(&self)                                         -> N;
    }

    impl DocumentIndexTrait for DocumentIndex {
        /// Algorithm 44.2: Make Index
        /// - Sort-based grouping: O(n log n) instead of O(n²) quadratic rescan.
        fn make_index(docs: &DocumentCollection) -> Self {
            // Step 1: Create word-document pairs using tagWords (flatten)
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

            // Step 2: Sort by (word, doc_id) for O(n log n) grouping
            pairs_vec.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

            // Step 3: Group consecutive equal keys in one pass (Table.collect)
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

        /// Algorithm 44.3: find function - simple table lookup
        fn find(&self, word: &Word) -> DocumentSet {
            match self.word_to_docs.find(word) {
                | Some(doc_set) => doc_set,
                | None => AVLTreeSetStPer::empty(),
            }
        }

        /// Algorithm 44.3: queryAnd - set intersection
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet { docs_a.intersection(docs_b) }

        /// Algorithm 44.3: queryOr - set union
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet { docs_a.union(docs_b) }

        /// Algorithm 44.3: queryAndNot - set difference
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet { docs_a.difference(docs_b) }

        /// Algorithm 44.3: size function
        fn size(docs: &DocumentSet) -> N { docs.size() }

        /// Algorithm 44.3: toSeq function
        fn to_seq(docs: &DocumentSet) -> ArraySeqStPerS<DocumentId> {
            let avl_seq = docs.to_seq();
            let mut array_seq = ArraySeqStPerS::empty();

            for i in 0..avl_seq.length() {
                let doc_id = avl_seq.nth(i);
                let single_seq = ArraySeqStPerS::singleton(doc_id.clone());
                array_seq = ArraySeqStPerS::append(&array_seq, &single_seq);
            }

            array_seq
        }

        fn empty() -> Self {
            DocumentIndex {
                word_to_docs: TableStPer::empty(),
            }
        }

        fn get_all_words(&self) -> ArraySeqStPerS<Word> {
            let entries = self.word_to_docs.collect();
            let mut words = ArraySeqStPerS::empty();

            for i in 0..entries.length() {
                let entry = entries.nth(i);
                let single_seq = ArraySeqStPerS::singleton(entry.0.clone());
                words = ArraySeqStPerS::append(&words, &single_seq);
            }

            words
        }

        fn word_count(&self) -> N { self.word_to_docs.size() }
    }

    /// Tokenization function: splits content into words.
    /// - APAS: (no cost stated — tokens is a helper assumed O(m) where m = string length)
    /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — sequential character iteration
    pub fn tokens(content: &Contents) -> ArraySeqStPerS<Word> {
        let mut words = ArraySeqStPerS::empty();
        let content_lower = content.to_lowercase();

        // Simple tokenization: split on whitespace and punctuation
        let mut current_word = String::new();

        for ch in content_lower.chars() {
            if ch.is_alphabetic() {
                current_word.push(ch);
            } else if !current_word.is_empty() {
                let single_seq = ArraySeqStPerS::singleton(current_word.clone());
                words = ArraySeqStPerS::append(&words, &single_seq);
                current_word = String::new();
            }
        }

        // Don't forget the last word
        if !current_word.is_empty() {
            let single_seq = ArraySeqStPerS::singleton(current_word);
            words = ArraySeqStPerS::append(&words, &single_seq);
        }

        words
    }

    /// Convenience function for staged computation pattern (Example 44.2).
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — closure capture only
    pub fn create_finder(index: &DocumentIndex) -> impl Fn(&Word) -> DocumentSet + '_ {
        move |word: &Word| index.find(word)
    }

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

    // Macro for creating document collections
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

    /// Complex query builder for chaining operations
    pub struct QueryBuilder<'a> {
        index: &'a DocumentIndex,
    }

    pub trait QueryBuilderTrait<'a> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn new(index: &'a DocumentIndex)                                                -> Self;

        /// - APAS: N/A — delegates to DocumentIndex::find.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n)
        fn find(&self, word: &Word)                                                     -> DocumentSet;

        /// - APAS: N/A — delegates to DocumentIndex::query_and.
        /// - Claude-Opus-4.6: Work Θ(m log(1 + n/m)), Span Θ(m log(1 + n/m))
        fn and(&self, docs_a: DocumentSet, docs_b: DocumentSet)                         -> DocumentSet;

        /// - APAS: N/A — delegates to DocumentIndex::query_or.
        /// - Claude-Opus-4.6: Work Θ(m log(1 + n/m)), Span Θ(m log(1 + n/m))
        fn or(&self, docs_a: DocumentSet, docs_b: DocumentSet)                          -> DocumentSet;

        /// - APAS: N/A — delegates to DocumentIndex::query_and_not.
        /// - Claude-Opus-4.6: Work Θ(m log(1 + n/m)), Span Θ(m log(1 + n/m))
        fn and_not(&self, docs_a: DocumentSet, docs_b: DocumentSet)                     -> DocumentSet;

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work dominated by 4 finds + 3 set operations
        fn complex_query(&self, word1: &Word, word2: &Word, word3: &Word, word4: &Word) -> DocumentSet;
    }

    impl<'a> QueryBuilderTrait<'a> for QueryBuilder<'a> {
        fn new(index: &'a DocumentIndex) -> Self { QueryBuilder { index } }

        fn find(&self, word: &Word) -> DocumentSet { self.index.find(word) }

        fn and(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> DocumentSet {
            DocumentIndex::query_and(&docs_a, &docs_b)
        }

        fn or(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> DocumentSet {
            DocumentIndex::query_or(&docs_a, &docs_b)
        }

        fn and_not(&self, docs_a: DocumentSet, docs_b: DocumentSet) -> DocumentSet {
            DocumentIndex::query_and_not(&docs_a, &docs_b)
        }

        /// Complex query: (word1 AND word2) OR (word3 AND NOT word4)
        fn complex_query(&self, word1: &Word, word2: &Word, word3: &Word, word4: &Word) -> DocumentSet {
            let set1 = self.find(word1);
            let set2 = self.find(word2);
            let set3 = self.find(word3);
            let set4 = self.find(word4);

            let left_side = self.and(set1, set2);
            let right_side = self.and_not(set3, set4);

            self.or(left_side, right_side)
        }
    }
}
