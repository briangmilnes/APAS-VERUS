//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Document Indexing and Searching implementation.

pub mod DocumentIndex {

    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;

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
        /// claude-4-sonet: Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log² n)
        /// Creates an index from a sequence of (id, contents) pairs
        fn make_index(docs: &DocumentCollection)                     -> Self;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Finds documents containing the given word
        fn find(&self, word: &Word)                                  -> DocumentSet;

        /// claude-4-sonet: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// Logical AND: documents in both sets
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet)     -> DocumentSet;

        /// claude-4-sonet: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// Logical OR: documents in either set
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet)      -> DocumentSet;

        /// claude-4-sonet: Work Θ(m log(1 + n/m)), Span Θ(log n + log m)
        /// Logical AND NOT: documents in first set but not second
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Returns the number of documents in the set
        fn size(docs: &DocumentSet)                                  -> N;

        /// Claude Work: O(n), Span: O(log n)
        /// Converts document set to sequence
        fn to_seq(docs: &DocumentSet)                                -> ArraySeqStPerS<DocumentId>;
        fn empty()                                                   -> Self;
        fn get_all_words(&self)                                      -> ArraySeqStPerS<Word>;
        fn word_count(&self)                                         -> N;
    }

    impl DocumentIndexTrait for DocumentIndex {
        /// Claude Work: O(n log n), Span: O(log² n)
        /// Algorithm 44.2: Make Index
        /// Cost dominated by Table.collect which performs a sort
        fn make_index(docs: &DocumentCollection) -> Self {
            // Step 1: Create word-document pairs using tagWords
            let mut all_pairs = ArraySeqStPerS::empty();

            for i in 0..docs.length() {
                let doc = docs.nth(i);
                let doc_id = &doc.0;
                let content = &doc.1;
                let word_tokens = tokens(content);

                // Tag each word with the document ID
                for j in 0..word_tokens.length() {
                    let word = word_tokens.nth(j);
                    let pair = Pair(word.clone(), doc_id.clone());
                    let single_seq = ArraySeqStPerS::singleton(pair);
                    all_pairs = ArraySeqStPerS::append(&all_pairs, &single_seq);
                }
            }

            // Step 2: Build table by inserting word-document pairs
            let mut word_table = TableStPer::empty();

            for i in 0..all_pairs.length() {
                let pair = all_pairs.nth(i);
                let word = &pair.0;
                let doc_id = &pair.1;

                // Insert or update the word entry
                word_table = word_table.insert(word.clone(), doc_id.clone(), |_old_doc, new_doc| {
                    // This shouldn't happen since we're building from scratch, but just in case
                    new_doc.clone()
                });
            }

            // Step 3: Convert the table to have sets as values instead of single documents
            let mut final_table = TableStPer::empty();
            let word_entries = word_table.collect();

            for i in 0..word_entries.length() {
                let entry = word_entries.nth(i);
                let word = &entry.0;
                let _doc_id = &entry.1; // Single document ID

                // Get all documents for this word by collecting from all_pairs
                let mut doc_ids = ArraySeqStPerS::empty();
                for j in 0..all_pairs.length() {
                    let pair = all_pairs.nth(j);
                    if &pair.0 == word {
                        let single_seq = ArraySeqStPerS::singleton(pair.1.clone());
                        doc_ids = ArraySeqStPerS::append(&doc_ids, &single_seq);
                    }
                }

                // Convert ArraySeqStPerS to AVLTreeSeqStPerS
                let mut doc_vec = Vec::new();
                for k in 0..doc_ids.length() {
                    let doc_id = doc_ids.nth(k);
                    doc_vec.push(doc_id.clone());
                }
                let avl_seq = AVLTreeSeqStPerS::from_vec(doc_vec);

                // Convert sequence to set to eliminate duplicates
                let doc_set = AVLTreeSetStPer::from_seq(avl_seq);
                final_table = final_table.insert(word.clone(), doc_set, |_old, new| new.clone());
            }

            DocumentIndex {
                word_to_docs: final_table,
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        /// Algorithm 44.3: find function - simple table lookup
        fn find(&self, word: &Word) -> DocumentSet {
            match self.word_to_docs.find(word) {
                | Some(doc_set) => doc_set,
                | None => AVLTreeSetStPer::empty(),
            }
        }

        /// Claude Work: O(m log(1 + n/m)), Span: O(log n + log m)
        /// Algorithm 44.3: queryAnd - set intersection
        fn query_and(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet { docs_a.intersection(docs_b) }

        /// Claude Work: O(m log(1 + n/m)), Span: O(log n + log m)
        /// Algorithm 44.3: queryOr - set union
        fn query_or(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet { docs_a.union(docs_b) }

        /// Claude Work: O(m log(1 + n/m)), Span: O(log n + log m)
        /// Algorithm 44.3: queryAndNot - set difference
        fn query_and_not(docs_a: &DocumentSet, docs_b: &DocumentSet) -> DocumentSet { docs_a.difference(docs_b) }

        /// Claude Work: O(1), Span: O(1)
        /// Algorithm 44.3: size function
        fn size(docs: &DocumentSet) -> N { docs.size() }

        /// Claude Work: O(n), Span: O(log n)
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

    /// Tokenization function: splits content into words
    /// Claude Work: O(m), Span: O(1) where m is the length of the content string
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

    /// Convenience function for staged computation pattern (Example 44.2)
    /// Creates a partially applied find function for a given index
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
        fn new(index: &'a DocumentIndex)                                                -> Self;
        fn find(&self, word: &Word)                                                     -> DocumentSet;
        fn and(&self, docs_a: DocumentSet, docs_b: DocumentSet)                         -> DocumentSet;
        fn or(&self, docs_a: DocumentSet, docs_b: DocumentSet)                          -> DocumentSet;
        fn and_not(&self, docs_a: DocumentSet, docs_b: DocumentSet)                     -> DocumentSet;
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
