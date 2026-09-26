// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 44: Example 44.1 - Tweet Document Collection

pub mod Example44_1 {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap44::DocumentIndex::DocumentIndex::*;
    use crate::DocumentCollectionLit;
    use crate::Types::Types::*;

    verus! {
        /// Placeholder; Example44 uses Box<dyn Fn>, impl Fn return.
        proof fn _example_44_1_verified() {}
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) — builds 5-element sequence via macro
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2), Span O(D^2) — no textbook cost; D = 5 fixed documents; the macro does D sequential ArraySeqStPer appends, each copying the prefix; does not match old analysis: Work Θ(n), Span Θ(n) vs new; append is not O(1)
    pub fn create_tweet_collection() -> DocumentCollection {
        DocumentCollectionLit![
            "jack" => "chess is fun",
            "mary" => "I had fun in dance club today",
            "nick" => "food at the cafeteria sucks",
            "josefa" => "rock climbing was a blast",
            "peter" => "I had fun at the party, food was great"
        ]
    }

    /// Creates the document index for the tweet collection.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n²), Span Θ(n²) — delegates to make_index
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2 + N·(N + D·h^2)), Span same — no textbook cost; N = total tokens, D = documents, h = document-set height; dominated by make_index; does not match old analysis: Work Θ(n²), Span Θ(n²) vs new; make_index is O(N·(N + D·h^2))
    pub fn create_tweet_index() -> DocumentIndex {
        let tweets = create_tweet_collection();
        DocumentIndex::make_index(&tweets)
    }

    /// Example 44.2: Staged computation pattern.
    /// fw : word -> docs = find (makeIndex T)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n²), Span Θ(n²) — builds index then returns closure
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2 + N·(N + D·h^2)), Span same — no textbook cost; dominated by create_tweet_index; does not match old analysis: Work Θ(n²), Span Θ(n²) vs new; make_index is O(N·(N + D·h^2))
    pub fn create_tweet_finder() -> impl Fn(&Word) -> DocumentSet {
        let index = create_tweet_index();
        move |word: &Word| index.find(word)
    }

    /// Demonstrates the example queries from the textbook.
    /// fw is Box<dyn Fn> — not Verus-parseable (dyn with more than one trait).
    pub struct TweetQueryExamples {
        pub index: DocumentIndex,
        pub fw: Box<dyn Fn(&Word) -> DocumentSet>,
    }

    impl Default for TweetQueryExamples {
        fn default() -> Self { Self::new() }
    }

    impl TweetQueryExamples {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n²), Span Θ(n²) — builds index via create_tweet_index
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2 + N·(N + D·h^2)), Span same — no textbook cost; create_tweet_index plus a deep index clone O(w + M); does not match old analysis: Work Θ(n²), Span Θ(n²) vs new; make_index is O(N·(N + D·h^2))
        pub fn new() -> Self {
            let index = create_tweet_index();
            let index_clone = index.clone();
            let fw = Box::new(move |word: &Word| index_clone.find(word));

            TweetQueryExamples { index, fw }
        }

        /// Example query: searching for 'fun' should return {"jack", "mary", "peter"}.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) — single find
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + d), Span O(w + d) — no textbook cost; one DocumentIndex find (linear table scan plus set clone); does not match old analysis: Work Θ(log n), Span Θ(log n) vs new; the table is an unsorted array
        pub fn search_fun(&self) -> DocumentSet { (self.fw)(&"fun".to_string()) }

        /// Example query: searching for 'club' should return {"mary"}.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) — single find
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + d), Span O(w + d) — no textbook cost; one DocumentIndex find (linear table scan plus set clone); does not match old analysis: Work Θ(log n), Span Θ(log n) vs new; the table is an unsorted array
        pub fn search_club(&self) -> DocumentSet { (self.fw)(&"club".to_string()) }

        /// Example query: searching for 'food' should return {"nick", "peter"}.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) — single find
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + d), Span O(w + d) — no textbook cost; one DocumentIndex find (linear table scan plus set clone); does not match old analysis: Work Θ(log n), Span Θ(log n) vs new; the table is an unsorted array
        pub fn search_food(&self) -> DocumentSet { (self.fw)(&"food".to_string()) }

        /// Example query: searching for 'chess' should return {"jack"}.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) — single find
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + d), Span O(w + d) — no textbook cost; one DocumentIndex find (linear table scan plus set clone); does not match old analysis: Work Θ(log n), Span Θ(log n) vs new; the table is an unsorted array
        pub fn search_chess(&self) -> DocumentSet { (self.fw)(&"chess".to_string()) }

        /// Complex query from textbook:
        /// toSeq (queryAnd ((fw 'fun'), queryOr ((fw 'food'), (fw 'chess'))))
        /// Expected result: ⟨'jack', 'peter'⟩
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) — dominated by set operations
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + n h^2), Span O(w + n h^2) — no textbook cost; three linear finds, a union and an intersection (O(n h^2) each per the Chap41 review), and to_seq O(n h); does not match old analysis: Work Θ(m log(1+n/m)), Span same vs new; finds are linear and set operations are O(n h^2)
        pub fn complex_query_fun_and_food_or_chess(&self) -> ArraySeqStPerS<DocumentId> {
            let fun_docs = (self.fw)(&"fun".to_string());
            let food_docs = (self.fw)(&"food".to_string());
            let chess_docs = (self.fw)(&"chess".to_string());

            let food_or_chess = DocumentIndex::query_or(&food_docs, &chess_docs);
            let result = DocumentIndex::query_and(&fun_docs, &food_or_chess);

            DocumentIndex::to_seq(&result)
        }

        /// Complex query from textbook:
        /// size (queryAndNot ((fw 'fun'), (fw 'chess')))
        /// Expected result: 2 (mary and peter).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) — dominated by set difference
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + n h^2), Span O(w + n h^2) — no textbook cost; two linear finds, a difference O(n h^2) per the Chap41 review, and size O(1); does not match old analysis: Work Θ(m log(1+n/m)), Span same vs new; finds are linear and set operations are O(n h^2)
        pub fn count_fun_but_not_chess(&self) -> usize {
            let fun_docs = (self.fw)(&"fun".to_string());
            let chess_docs = (self.fw)(&"chess".to_string());

            let result = DocumentIndex::query_and_not(&fun_docs, &chess_docs);
            DocumentIndex::size(&result)
        }

        /// Additional example: documents with 'food' OR 'fun'.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m))
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + n h^2), Span O(w + n h^2) — no textbook cost; two linear finds and a union O(n h^2) per the Chap41 review; does not match old analysis: Work Θ(m log(1+n/m)), Span same vs new; finds are linear and set operations are O(n h^2)
        pub fn search_food_or_fun(&self) -> DocumentSet {
            let food_docs = (self.fw)(&"food".to_string());
            let fun_docs = (self.fw)(&"fun".to_string());

            DocumentIndex::query_or(&food_docs, &fun_docs)
        }

        /// Additional example: documents with 'party' AND 'food'.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m))
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + n h^2), Span O(w + n h^2) — no textbook cost; two linear finds and an intersection O(n h^2) per the Chap41 review; does not match old analysis: Work Θ(m log(1+n/m)), Span same vs new; finds are linear and set operations are O(n h^2)
        pub fn search_party_and_food(&self) -> DocumentSet {
            let party_docs = (self.fw)(&"party".to_string());
            let food_docs = (self.fw)(&"food".to_string());

            DocumentIndex::query_and(&party_docs, &food_docs)
        }

        /// Get all unique words in the tweet collection.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + M), Span O(w + M) — no textbook cost; delegates to DocumentIndex get_all_words, whose collect deep-clones every document set (M = Σ set sizes); does not match old analysis: Work Θ(n), Span Θ(n) vs new
        pub fn get_all_words(&self) -> ArraySeqStPerS<Word> { self.index.get_all_words() }

        /// Get word count statistics.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1)
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(1), Span O(1) — no textbook cost
        pub fn get_word_count(&self) -> usize { self.index.word_count() }

        /// Demonstrate query builder pattern.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work dominated by 4 finds + 3 set operations
        /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(w + n h^2), Span O(w + n h^2) — no textbook cost; delegates to QueryBuilder complex_query
        pub fn query_builder_example(&self) -> DocumentSet {
            let builder = QueryBuilder::new(&self.index);

            // Complex query: (fun AND party) OR (chess AND NOT food)
            builder.complex_query(
                &"fun".to_string(),
                &"party".to_string(),
                &"chess".to_string(),
                &"food".to_string(),
            )
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) — to_seq + sort
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(n h(T) + n lg n), Span O(n h(T) + n lg n) — no textbook cost; DocumentIndex to_seq O(n h(T)), a copy loop, and a sequential sort; does not match old analysis: Work Θ(n log n), Span Θ(n log n) vs new; to_seq is O(n h(T)) per the Chap41 review
    pub fn doc_set_to_sorted_vec(docs: &DocumentSet) -> Vec<DocumentId> {
        let seq = DocumentIndex::to_seq(docs);
        let mut result = Vec::new();

        for i in 0..seq.length() {
            let doc_id = seq.nth(i);
            result.push(doc_id.clone());
        }

        result.sort();
        result
    }

    /// Verify the expected results from the textbook examples.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n²), Span Θ(n²) — builds index, runs queries, compares results
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2 + N·(N + D·h^2)), Span same — no textbook cost; dominated by TweetQueryExamples new (index construction); does not match old analysis: Work Θ(n²), Span Θ(n²) vs new; make_index is O(N·(N + D·h^2))
    pub fn verify_textbook_examples() -> bool {
        let examples = TweetQueryExamples::new();

        // Test 1: searching for 'fun' should return {"jack", "mary", "peter"}
        let fun_results = doc_set_to_sorted_vec(&examples.search_fun());
        let expected_fun = vec!["jack".to_string(), "mary".to_string(), "peter".to_string()];
        if fun_results != expected_fun {
            return false;
        }

        // Test 2: searching for 'club' should return {"mary"}
        let club_results = doc_set_to_sorted_vec(&examples.search_club());
        let expected_club = vec!["mary".to_string()];
        if club_results != expected_club {
            return false;
        }

        // Test 3: complex query should return ⟨'jack', 'peter'⟩
        let complex_results = examples.complex_query_fun_and_food_or_chess();
        let mut complex_vec = Vec::new();
        for i in 0..complex_results.length() {
            let doc_id = complex_results.nth(i);
            complex_vec.push(doc_id.clone());
        }
        complex_vec.sort();
        let expected_complex = vec!["jack".to_string(), "peter".to_string()];
        if complex_vec != expected_complex {
            return false;
        }

        // Test 4: count query should return 2
        let count_result = examples.count_fun_but_not_chess();
        if count_result != 2 {
            return false;
        }

        true
    }

    /// Performance demonstration: compare indexed search vs brute force.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n²), Span Θ(n²) — dominated by index construction
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2 + N·(N + D·h^2)), Span same — no textbook cost; dominated by create_tweet_index; does not match old analysis: Work Θ(n²), Span Θ(n²) vs new; make_index is O(N·(N + D·h^2))
    pub fn performance_comparison_demo() -> (usize, usize) {
        let tweets = create_tweet_collection();
        let _index = create_tweet_index();

        // Indexed search work: O(log n) for find
        let indexed_work = 1; // Represents O(log n) complexity

        // Brute force work: O(n * m) where n is documents, m is average content length
        let brute_force_work = tweets.length(); // Represents O(n) complexity

        (indexed_work, brute_force_work)
    }

    // 14. derive impls outside verus!

    impl Debug for TweetQueryExamples {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "TweetQueryExamples")
        }
    }

    impl Display for TweetQueryExamples {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "TweetQueryExamples")
        }
    }

    /// Demonstrate the tokenization process.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m), Span Θ(m) — delegates to tokens()
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(m), Span O(m) — no textbook cost; m = sample length, one tokens call
    pub fn tokenization_demo() -> ArraySeqStPerS<Word> {
        let sample_content = "I had fun in dance club today!";
        tokens(&sample_content.to_string())
    }

    /// Show index statistics for the tweet collection.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n²), Span Θ(n²) — builds index + iterates documents
    /// - Alg Analysis: Code review (Claude Opus 5.5, 2026-09-26): Work O(D^2 + N·(N + D·h^2)), Span same — no textbook cost; dominated by create_tweet_index, plus a tokens pass over every document; does not match old analysis: Work Θ(n²), Span Θ(n²) vs new; make_index is O(N·(N + D·h^2))
    pub fn index_statistics() -> (usize, usize, usize) {
        let tweets = create_tweet_collection();
        let index = create_tweet_index();

        let document_count = tweets.length();
        let unique_word_count = index.word_count();

        // Calculate total words across all documents
        let mut total_words = 0;
        for i in 0..tweets.length() {
            let doc = tweets.nth(i);
            let word_tokens = tokens(&doc.1);
            total_words += word_tokens.length();
        }

        (document_count, unique_word_count, total_words)
    }
}
