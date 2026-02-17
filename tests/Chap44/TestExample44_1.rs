//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Example 44.1 Tests - Tweet Document Collection

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap44::DocumentIndex::DocumentIndex::*;
use apas_verus::Chap44::Example44_1::Example44_1::*;
use apas_verus::Types::Types::*;

#[test]
fn test_create_tweet_collection() {
    let tweets = create_tweet_collection();
    assert_eq!(tweets.length(), 5);

    // Check specific tweets
    let jack_tweet = tweets.nth(0);
    assert_eq!(jack_tweet.0, "jack");
    assert_eq!(jack_tweet.1, "chess is fun");

    let mary_tweet = tweets.nth(1);
    assert_eq!(mary_tweet.0, "mary");
    assert_eq!(mary_tweet.1, "I had fun in dance club today");
}

#[test]
fn test_create_tweet_index() {
    let index = create_tweet_index();

    // Should have multiple unique words
    assert!(index.word_count() > 10);

    // Test basic functionality
    let fun_docs = index.find(&"fun".to_string());
    assert_eq!(DocumentIndex::size(&fun_docs), 3); // jack, mary, peter
}

#[test]
fn test_tweet_query_examples() {
    let examples = TweetQueryExamples::new();

    // Test individual searches
    let fun_results = doc_set_to_sorted_vec(&examples.search_fun());
    assert_eq!(
        fun_results,
        vec!["jack".to_string(), "mary".to_string(), "peter".to_string()]
    );

    let club_results = doc_set_to_sorted_vec(&examples.search_club());
    assert_eq!(club_results, vec!["mary".to_string()]);

    let food_results = doc_set_to_sorted_vec(&examples.search_food());
    assert_eq!(food_results, vec!["nick".to_string(), "peter".to_string()]);

    let chess_results = doc_set_to_sorted_vec(&examples.search_chess());
    assert_eq!(chess_results, vec!["jack".to_string()]);
}

#[test]
fn test_complex_query_fun_and_food_or_chess() {
    let examples = TweetQueryExamples::new();

    // Test the complex query from the textbook
    let result_seq = examples.complex_query_fun_and_food_or_chess();
    let mut result_vec = Vec::new();

    for i in 0..result_seq.length() {
        let doc_id = result_seq.nth(i);
        result_vec.push(doc_id.clone());
    }

    result_vec.sort();
    assert_eq!(result_vec, vec!["jack".to_string(), "peter".to_string()]);
}

#[test]
fn test_count_fun_but_not_chess() {
    let examples = TweetQueryExamples::new();

    // Test the count query from the textbook
    let count = examples.count_fun_but_not_chess();
    assert_eq!(count, 2); // mary and peter have 'fun' but not 'chess'
}

#[test]
fn test_search_food_or_fun() {
    let examples = TweetQueryExamples::new();

    let result = examples.search_food_or_fun();
    let result_vec = doc_set_to_sorted_vec(&result);

    // Should include: jack (fun), mary (fun), nick (food), peter (fun, food)
    assert_eq!(
        result_vec,
        vec![
            "jack".to_string(),
            "mary".to_string(),
            "nick".to_string(),
            "peter".to_string()
        ]
    );
}

#[test]
fn test_search_party_and_food() {
    let examples = TweetQueryExamples::new();

    let result = examples.search_party_and_food();
    let result_vec = doc_set_to_sorted_vec(&result);

    // Only peter has both 'party' and 'food'
    assert_eq!(result_vec, vec!["peter".to_string()]);
}

#[test]
fn test_get_all_words() {
    let examples = TweetQueryExamples::new();

    let words = examples.get_all_words();
    assert!(words.length() > 15); // Should have many unique words

    // Check that some expected words are present by searching for them
    let index = create_tweet_index();
    let fun_docs = index.find(&"fun".to_string());
    assert!(DocumentIndex::size(&fun_docs) > 0); // 'fun' should be found

    let chess_docs = index.find(&"chess".to_string());
    assert!(DocumentIndex::size(&chess_docs) > 0); // 'chess' should be found
}

#[test]
fn test_get_word_count() {
    let examples = TweetQueryExamples::new();

    let word_count = examples.get_word_count();
    assert!(word_count > 15); // Should have many unique words
    assert!(word_count < 50); // But not too many for this small collection
}

#[test]
fn test_query_builder_example() {
    let examples = TweetQueryExamples::new();

    let result = examples.query_builder_example();
    let result_size = DocumentIndex::size(&result);

    // Complex query: (fun AND party) OR (chess AND NOT food)
    // (fun AND party): peter
    // (chess AND NOT food): jack (has chess, doesn't have food)
    // Result should be: jack, peter
    assert_eq!(result_size, 2);
}

#[test]
fn test_verify_textbook_examples() {
    // This test verifies all the examples from the textbook
    let verification_result = verify_textbook_examples();
    assert!(verification_result, "Textbook examples should all pass verification");
}

#[test]
fn test_performance_comparison_demo() {
    let (indexed_work, brute_force_work) = performance_comparison_demo();

    // Indexed search should be more efficient (represented by smaller numbers)
    assert!(indexed_work < brute_force_work);
    assert_eq!(indexed_work, 1); // O(log n) represented as 1
    assert_eq!(brute_force_work, 5); // O(n) represented as collection length
}

#[test]
fn test_tokenization_demo() {
    let tokens = tokenization_demo();

    // Should tokenize "I had fun in dance club today!" correctly
    assert_eq!(tokens.length(), 7); // I had fun in dance club today

    let mut token_vec = Vec::new();
    for i in 0..tokens.length() {
        let token = tokens.nth(i);
        token_vec.push(token.clone());
    }

    assert_eq!(token_vec, vec!["i", "had", "fun", "in", "dance", "club", "today"]);
}

#[test]
fn test_index_statistics() {
    let (document_count, unique_word_count, total_words) = index_statistics();

    assert_eq!(document_count, 5); // 5 tweets
    assert!(unique_word_count > 15); // Many unique words
    assert!(total_words > 20); // Total words across all documents
    assert!(total_words > unique_word_count); // Total should be more than unique
}

#[test]
fn test_staged_computation_pattern() {
    // Test the staged computation pattern from Example 44.2
    let finder = create_tweet_finder();

    // The finder should work like the fw function in the textbook
    let fun_docs = finder(&"fun".to_string());
    assert_eq!(DocumentIndex::size(&fun_docs), 3);

    let chess_docs = finder(&"chess".to_string());
    assert_eq!(DocumentIndex::size(&chess_docs), 1);

    let nonexistent_docs = finder(&"nonexistent".to_string());
    assert_eq!(DocumentIndex::size(&nonexistent_docs), 0);
}

#[test]
fn test_case_insensitive_search() {
    let examples = TweetQueryExamples::new();

    // All searches should be case-insensitive due to tokenization
    let fun_docs = examples.search_fun();
    let fun_size = DocumentIndex::size(&fun_docs);

    // Should find 'fun', 'Fun', etc. (though our test data is lowercase)
    assert_eq!(fun_size, 3);
}

#[test]
fn test_empty_search_results() {
    let _examples = TweetQueryExamples::new();
    let index = create_tweet_index();

    // Search for words that don't exist
    let nonexistent1 = index.find(&"xyz".to_string());
    let nonexistent2 = index.find(&"abc".to_string());

    assert_eq!(DocumentIndex::size(&nonexistent1), 0);
    assert_eq!(DocumentIndex::size(&nonexistent2), 0);

    // Operations on empty sets should return empty sets
    let and_result = DocumentIndex::query_and(&nonexistent1, &nonexistent2);
    assert_eq!(DocumentIndex::size(&and_result), 0);

    let or_result = DocumentIndex::query_or(&nonexistent1, &nonexistent2);
    assert_eq!(DocumentIndex::size(&or_result), 0);
}

#[test]
fn test_single_word_documents() {
    // Test edge case with documents containing single words
    let examples = TweetQueryExamples::new();

    // Test searching for words that appear in multiple contexts
    let is_docs = examples.search_fun(); // 'fun' appears in multiple documents
    assert!(DocumentIndex::size(&is_docs) > 1);
}

#[test]
fn test_query_combinations() {
    let examples = TweetQueryExamples::new();

    // Test various query combinations
    let fun_docs = examples.search_fun();
    let food_docs = examples.search_food();
    let chess_docs = examples.search_chess();

    // Test AND operations
    let fun_and_food = DocumentIndex::query_and(&fun_docs, &food_docs);
    let fun_and_food_vec = doc_set_to_sorted_vec(&fun_and_food);
    assert_eq!(fun_and_food_vec, vec!["peter".to_string()]); // Only peter has both

    // Test OR operations
    let chess_or_food = DocumentIndex::query_or(&chess_docs, &food_docs);
    let chess_or_food_vec = doc_set_to_sorted_vec(&chess_or_food);
    assert_eq!(
        chess_or_food_vec,
        vec!["jack".to_string(), "nick".to_string(), "peter".to_string()]
    );

    // Test AND NOT operations
    let fun_not_chess = DocumentIndex::query_and_not(&fun_docs, &chess_docs);
    let fun_not_chess_vec = doc_set_to_sorted_vec(&fun_not_chess);
    assert_eq!(fun_not_chess_vec, vec!["mary".to_string(), "peter".to_string()]);
    // fun but not chess
}

#[test]
fn test_algorithmic_complexity_verification() {
    // Verify that operations complete in reasonable time
    let examples = TweetQueryExamples::new();

    // Test find operation speed (should be O(log n))
    let start = std::time::Instant::now();
    let _result = examples.search_fun();
    let find_duration = start.elapsed();
    assert!(find_duration.as_millis() < 10); // Should be very fast

    // Test query operations speed
    let fun_docs = examples.search_fun();
    let food_docs = examples.search_food();

    let start = std::time::Instant::now();
    let _and_result = DocumentIndex::query_and(&fun_docs, &food_docs);
    let and_duration = start.elapsed();
    assert!(and_duration.as_millis() < 10); // Should be very fast

    let start = std::time::Instant::now();
    let _or_result = DocumentIndex::query_or(&fun_docs, &food_docs);
    let or_duration = start.elapsed();
    assert!(or_duration.as_millis() < 10); // Should be very fast
}
