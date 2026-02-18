//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Example 44.1 - Tweet Document Collection

pub mod Example44_1 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap44::DocumentIndex::DocumentIndex::*;
    use crate::DocumentCollectionLit;
    use crate::Types::Types::*;

    /// - APAS: N/A — Example scaffolding.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — builds 5-element sequence via macro
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
    /// - APAS: N/A — Example scaffolding (cost dominated by make_index).
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — delegates to make_index
    pub fn create_tweet_index() -> DocumentIndex {
        let tweets = create_tweet_collection();
        DocumentIndex::make_index(&tweets)
    }

    /// Example 44.2: Staged computation pattern.
    /// fw : word -> docs = find (makeIndex T)
    /// - APAS: N/A — Example scaffolding (cost dominated by make_index).
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — builds index then returns closure
    pub fn create_tweet_finder() -> impl Fn(&Word) -> DocumentSet {
        let index = create_tweet_index();
        move |word: &Word| index.find(word)
    }

    /// Demonstrates the example queries from the textbook.
    pub struct TweetQueryExamples {
        index: DocumentIndex,
        fw: Box<dyn Fn(&Word) -> DocumentSet>,
    }

    impl Default for TweetQueryExamples {
        fn default() -> Self { Self::new() }
    }

    impl TweetQueryExamples {
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — builds index via create_tweet_index
        pub fn new() -> Self {
            let index = create_tweet_index();
            let index_clone = index.clone();
            let fw = Box::new(move |word: &Word| index_clone.find(word));

            TweetQueryExamples { index, fw }
        }

        /// Example query: searching for 'fun' should return {"jack", "mary", "peter"}.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) — single find
        pub fn search_fun(&self) -> DocumentSet { (self.fw)(&"fun".to_string()) }

        /// Example query: searching for 'club' should return {"mary"}.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) — single find
        pub fn search_club(&self) -> DocumentSet { (self.fw)(&"club".to_string()) }

        /// Example query: searching for 'food' should return {"nick", "peter"}.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) — single find
        pub fn search_food(&self) -> DocumentSet { (self.fw)(&"food".to_string()) }

        /// Example query: searching for 'chess' should return {"jack"}.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) — single find
        pub fn search_chess(&self) -> DocumentSet { (self.fw)(&"chess".to_string()) }

        /// Complex query from textbook:
        /// toSeq (queryAnd ((fw 'fun'), queryOr ((fw 'food'), (fw 'chess'))))
        /// Expected result: ⟨'jack', 'peter'⟩
        /// - APAS: N/A — Example scaffolding; cost is 3 finds + 2 set operations + toSeq.
        /// - Claude-Opus-4.6: Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) — dominated by set operations
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
        /// - APAS: N/A — Example scaffolding; cost is 2 finds + queryAndNot + size.
        /// - Claude-Opus-4.6: Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m)) — dominated by set difference
        pub fn count_fun_but_not_chess(&self) -> N {
            let fun_docs = (self.fw)(&"fun".to_string());
            let chess_docs = (self.fw)(&"chess".to_string());

            let result = DocumentIndex::query_and_not(&fun_docs, &chess_docs);
            DocumentIndex::size(&result)
        }

        /// Additional example: documents with 'food' OR 'fun'.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m))
        pub fn search_food_or_fun(&self) -> DocumentSet {
            let food_docs = (self.fw)(&"food".to_string());
            let fun_docs = (self.fw)(&"fun".to_string());

            DocumentIndex::query_or(&food_docs, &fun_docs)
        }

        /// Additional example: documents with 'party' AND 'food'.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(m log(1+n/m)), Span Θ(m log(1+n/m))
        pub fn search_party_and_food(&self) -> DocumentSet {
            let party_docs = (self.fw)(&"party".to_string());
            let food_docs = (self.fw)(&"food".to_string());

            DocumentIndex::query_and(&party_docs, &food_docs)
        }

        /// Get all unique words in the tweet collection.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n)
        pub fn get_all_words(&self) -> ArraySeqStPerS<Word> { self.index.get_all_words() }

        /// Get word count statistics.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        pub fn get_word_count(&self) -> N { self.index.word_count() }

        /// Demonstrate query builder pattern.
        /// - APAS: N/A — Example scaffolding.
        /// - Claude-Opus-4.6: Work dominated by 4 finds + 3 set operations
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

    /// - APAS: N/A — Test helper.
    /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) — to_seq + sort
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
    /// - APAS: N/A — Test verification helper.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — builds index, runs queries, compares results
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
    /// - APAS: N/A — Example scaffolding.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — dominated by index construction
    pub fn performance_comparison_demo() -> (N, N) {
        let tweets = create_tweet_collection();
        let _index = create_tweet_index();

        // Indexed search work: O(log n) for find
        let indexed_work = 1; // Represents O(log n) complexity

        // Brute force work: O(n * m) where n is documents, m is average content length
        let brute_force_work = tweets.length(); // Represents O(n) complexity

        (indexed_work, brute_force_work)
    }

    /// Demonstrate the tokenization process.
    /// - APAS: N/A — Example scaffolding.
    /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to tokens()
    pub fn tokenization_demo() -> ArraySeqStPerS<Word> {
        let sample_content = "I had fun in dance club today!";
        tokens(&sample_content.to_string())
    }

    /// Show index statistics for the tweet collection.
    /// - APAS: N/A — Example scaffolding.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — builds index + iterates documents
    pub fn index_statistics() -> (N, N, N) {
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
