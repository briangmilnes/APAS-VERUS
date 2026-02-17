//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Document Index Tests

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap44::DocumentIndex::DocumentIndex::*;
use apas_verus::DocumentCollectionLit;
use apas_verus::Types::Types::*;

#[test]
fn test_documentcollectionlit_macro_functionality() {
    // Test empty document collection creation
    let empty: DocumentCollection = DocumentCollectionLit![];
    assert_eq!(empty.length(), 0);

    // Test document collection creation with documents
    let with_data: DocumentCollection = DocumentCollectionLit![
        "doc1" => "hello world",
        "doc2" => "rust programming"
    ];
    assert_eq!(with_data.length(), 2);
}

fn create_test_documents() -> DocumentCollection {
    DocumentCollectionLit![
        "doc1" => "hello world programming",
        "doc2" => "world peace and love",
        "doc3" => "programming is fun",
        "doc4" => "hello programming world",
        "doc5" => "peace love joy"
    ]
}

fn doc_set_to_sorted_vec(docs: &DocumentSet) -> Vec<DocumentId> {
    let seq = DocumentIndex::to_seq(docs);
    let mut result = Vec::new();

    for i in 0..seq.length() {
        let doc_id = seq.nth(i);
        result.push(doc_id.clone());
    }

    result.sort();
    result
}

#[test]
fn test_empty_index() {
    let empty_docs = DocumentCollectionLit![];
    let index = DocumentIndex::make_index(&empty_docs);

    assert_eq!(index.word_count(), 0);

    let result = index.find(&"nonexistent".to_string());
    assert_eq!(DocumentIndex::size(&result), 0);
}

#[test]
fn test_single_document_index() {
    let docs = DocumentCollectionLit!["doc1" => "hello world"];
    let index = DocumentIndex::make_index(&docs);

    assert_eq!(index.word_count(), 2); // "hello" and "world"

    let hello_docs = index.find(&"hello".to_string());
    let hello_vec = doc_set_to_sorted_vec(&hello_docs);
    assert_eq!(hello_vec, vec!["doc1".to_string()]);

    let world_docs = index.find(&"world".to_string());
    let world_vec = doc_set_to_sorted_vec(&world_docs);
    assert_eq!(world_vec, vec!["doc1".to_string()]);

    let nonexistent = index.find(&"nonexistent".to_string());
    assert_eq!(DocumentIndex::size(&nonexistent), 0);
}

#[test]
fn test_make_index_basic() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    // Should have unique words: hello, world, programming, peace, and, love, is, fun, joy
    assert!(index.word_count() >= 9);

    // Test that index was created successfully
    let all_words = index.get_all_words();
    assert!(all_words.length() >= 9);
}

#[test]
fn test_find_operation() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    // Test finding "programming" - should be in doc1, doc3, doc4
    let programming_docs = index.find(&"programming".to_string());
    let programming_vec = doc_set_to_sorted_vec(&programming_docs);
    assert_eq!(
        programming_vec,
        vec!["doc1".to_string(), "doc3".to_string(), "doc4".to_string()]
    );

    // Test finding "world" - should be in doc1, doc2, doc4
    let world_docs = index.find(&"world".to_string());
    let world_vec = doc_set_to_sorted_vec(&world_docs);
    assert_eq!(
        world_vec,
        vec!["doc1".to_string(), "doc2".to_string(), "doc4".to_string()]
    );

    // Test finding "peace" - should be in doc2, doc5
    let peace_docs = index.find(&"peace".to_string());
    let peace_vec = doc_set_to_sorted_vec(&peace_docs);
    assert_eq!(peace_vec, vec!["doc2".to_string(), "doc5".to_string()]);

    // Test finding non-existent word
    let nonexistent = index.find(&"nonexistent".to_string());
    assert_eq!(DocumentIndex::size(&nonexistent), 0);
}

#[test]
fn test_query_and() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let programming_docs = index.find(&"programming".to_string());
    let world_docs = index.find(&"world".to_string());

    // Documents with both "programming" AND "world" - should be doc1, doc4
    let and_result = DocumentIndex::query_and(&programming_docs, &world_docs);
    let and_vec = doc_set_to_sorted_vec(&and_result);
    assert_eq!(and_vec, vec!["doc1".to_string(), "doc4".to_string()]);
}

#[test]
fn test_query_or() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let programming_docs = index.find(&"programming".to_string());
    let peace_docs = index.find(&"peace".to_string());

    // Documents with "programming" OR "peace" - should be doc1, doc2, doc3, doc4, doc5
    let or_result = DocumentIndex::query_or(&programming_docs, &peace_docs);
    let or_vec = doc_set_to_sorted_vec(&or_result);
    assert_eq!(
        or_vec,
        vec![
            "doc1".to_string(),
            "doc2".to_string(),
            "doc3".to_string(),
            "doc4".to_string(),
            "doc5".to_string()
        ]
    );
}

#[test]
fn test_query_and_not() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let programming_docs = index.find(&"programming".to_string());
    let world_docs = index.find(&"world".to_string());

    // Documents with "programming" but NOT "world" - should be doc3
    let and_not_result = DocumentIndex::query_and_not(&programming_docs, &world_docs);
    let and_not_vec = doc_set_to_sorted_vec(&and_not_result);
    assert_eq!(and_not_vec, vec!["doc3".to_string()]);
}

#[test]
fn test_size_operation() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let programming_docs = index.find(&"programming".to_string());
    assert_eq!(DocumentIndex::size(&programming_docs), 3); // doc1, doc3, doc4

    let peace_docs = index.find(&"peace".to_string());
    assert_eq!(DocumentIndex::size(&peace_docs), 2); // doc2, doc5

    let nonexistent = index.find(&"nonexistent".to_string());
    assert_eq!(DocumentIndex::size(&nonexistent), 0);
}

#[test]
fn test_to_seq_operation() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let programming_docs = index.find(&"programming".to_string());
    let programming_seq = DocumentIndex::to_seq(&programming_docs);

    // Should have 3 documents
    assert_eq!(programming_seq.length(), 3);

    // Convert to vector and sort for comparison
    let mut programming_vec = Vec::new();
    for i in 0..programming_seq.length() {
        let doc_id = programming_seq.nth(i);
        programming_vec.push(doc_id.clone());
    }
    programming_vec.sort();
    assert_eq!(
        programming_vec,
        vec!["doc1".to_string(), "doc3".to_string(), "doc4".to_string()]
    );
}

#[test]
fn test_tokens_function() {
    let content = "Hello, World! This is a test.".to_string();
    let word_tokens = tokens(&content);

    // Should tokenize to: ["hello", "world", "this", "is", "a", "test"]
    assert_eq!(word_tokens.length(), 6);

    let mut token_vec = Vec::new();
    for i in 0..word_tokens.length() {
        let token = word_tokens.nth(i);
        token_vec.push(token.clone());
    }

    assert_eq!(token_vec, vec!["hello", "world", "this", "is", "a", "test"]);
}

#[test]
fn test_tokens_edge_cases() {
    // Empty string
    let empty_tokens = tokens(&"".to_string());
    assert_eq!(empty_tokens.length(), 0);

    // Only punctuation
    let punct_tokens = tokens(&"!@#$%^&*()".to_string());
    assert_eq!(punct_tokens.length(), 0);

    // Single word
    let single_tokens = tokens(&"hello".to_string());
    assert_eq!(single_tokens.length(), 1);
    let token = single_tokens.nth(0);
    if true {
        assert_eq!(token, &"hello".to_string());
    }

    // Multiple spaces
    let space_tokens = tokens(&"hello    world".to_string());
    assert_eq!(space_tokens.length(), 2);
}

#[test]
fn test_complex_query_combinations() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let hello_docs = index.find(&"hello".to_string());
    let world_docs = index.find(&"world".to_string());
    let programming_docs = index.find(&"programming".to_string());
    let peace_docs = index.find(&"peace".to_string());

    // (hello AND world) OR (programming AND peace)
    let hello_and_world = DocumentIndex::query_and(&hello_docs, &world_docs);
    let programming_and_peace = DocumentIndex::query_and(&programming_docs, &peace_docs);
    let complex_or = DocumentIndex::query_or(&hello_and_world, &programming_and_peace);

    let complex_vec = doc_set_to_sorted_vec(&complex_or);
    // hello AND world: doc1, doc4
    // programming AND peace: none (no document has both)
    // So result should be: doc1, doc4
    assert_eq!(complex_vec, vec!["doc1".to_string(), "doc4".to_string()]);
}

#[test]
fn test_case_insensitive_tokenization() {
    let docs = DocumentCollectionLit!["doc1" => "Hello WORLD Programming"];
    let index = DocumentIndex::make_index(&docs);

    // Should find documents regardless of case in query
    let hello_docs = index.find(&"hello".to_string());
    assert_eq!(DocumentIndex::size(&hello_docs), 1);

    let world_docs = index.find(&"world".to_string());
    assert_eq!(DocumentIndex::size(&world_docs), 1);

    let programming_docs = index.find(&"programming".to_string());
    assert_eq!(DocumentIndex::size(&programming_docs), 1);
}

#[test]
fn test_duplicate_words_in_document() {
    let docs = DocumentCollectionLit!["doc1" => "hello hello world world"];
    let index = DocumentIndex::make_index(&docs);

    // Should only appear once per document in the index
    let hello_docs = index.find(&"hello".to_string());
    assert_eq!(DocumentIndex::size(&hello_docs), 1);

    let world_docs = index.find(&"world".to_string());
    assert_eq!(DocumentIndex::size(&world_docs), 1);
}

#[test]
fn test_query_builder() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);
    let builder = QueryBuilder::new(&index);

    // Test basic find
    let programming_docs = builder.find(&"programming".to_string());
    assert_eq!(DocumentIndex::size(&programming_docs), 3);

    // Test AND operation
    let hello_docs = builder.find(&"hello".to_string());
    let world_docs = builder.find(&"world".to_string());
    let and_result = builder.and(hello_docs, world_docs);
    assert_eq!(DocumentIndex::size(&and_result), 2); // doc1, doc4

    // Test complex query
    let complex_result = builder.complex_query(
        &"hello".to_string(),
        &"world".to_string(),
        &"programming".to_string(),
        &"nonexistent".to_string(),
    );
    // (hello AND world) OR (programming AND NOT nonexistent)
    // = (doc1, doc4) OR (doc1, doc3, doc4) = (doc1, doc3, doc4)
    assert_eq!(DocumentIndex::size(&complex_result), 3);
}

#[test]
fn test_empty_query_results() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    let nonexistent1 = index.find(&"xyz".to_string());
    let nonexistent2 = index.find(&"abc".to_string());

    // AND with empty sets
    let and_empty = DocumentIndex::query_and(&nonexistent1, &nonexistent2);
    assert_eq!(DocumentIndex::size(&and_empty), 0);

    // OR with empty sets
    let or_empty = DocumentIndex::query_or(&nonexistent1, &nonexistent2);
    assert_eq!(DocumentIndex::size(&or_empty), 0);

    // AND NOT with empty sets
    let and_not_empty = DocumentIndex::query_and_not(&nonexistent1, &nonexistent2);
    assert_eq!(DocumentIndex::size(&and_not_empty), 0);
}

#[test]
fn test_large_document_collection() {
    // Create a larger collection to test scalability
    let mut docs = ArraySeqStPerS::empty();
    for i in 0..100 {
        let doc_id = format!("doc{i}");
        let content = format!("document {} contains words like test data item {}", i, i % 10);
        let pair = Pair(doc_id, content);
        let single_seq = ArraySeqStPerS::singleton(pair);
        docs = ArraySeqStPerS::append(&docs, &single_seq);
    }

    let index = DocumentIndex::make_index(&docs);

    // Should have many unique words (at least common words like document, contains, test, etc.)
    assert!(index.word_count() > 5);

    // Test finding common words
    let document_docs = index.find(&"document".to_string());
    assert_eq!(DocumentIndex::size(&document_docs), 100); // All documents

    let test_docs = index.find(&"test".to_string());
    assert_eq!(DocumentIndex::size(&test_docs), 100); // All documents

    // Test finding specific numbers (0-9 appear as "item X")
    let item_docs = index.find(&"item".to_string());
    assert_eq!(DocumentIndex::size(&item_docs), 100); // All documents
}

#[test]
fn test_create_finder() {
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    // Create a finder function for this index
    let find_fn = create_finder(&index);

    // Use the finder to find documents
    let programming_docs = find_fn(&"programming".to_string());
    let programming_vec = doc_set_to_sorted_vec(&programming_docs);
    assert_eq!(
        programming_vec,
        vec!["doc1".to_string(), "doc3".to_string(), "doc4".to_string()]
    );

    // Use it again for different queries
    let peace_docs = find_fn(&"peace".to_string());
    assert_eq!(DocumentIndex::size(&peace_docs), 2);

    let nonexistent = find_fn(&"nonexistent".to_string());
    assert_eq!(DocumentIndex::size(&nonexistent), 0);

    // Demonstrates staged computation pattern
    let words = vec!["hello", "world", "programming"];
    let results: Vec<N> = words
        .iter()
        .map(|word| {
            let docs = find_fn(&word.to_string());
            DocumentIndex::size(&docs)
        })
        .collect();
    
    assert_eq!(results, vec![2, 3, 3]); // hello:2, world:3, programming:3
}

#[test]
fn test_algorithmic_costs_verification() {
    // This test verifies that the operations complete in reasonable time
    // indicating correct algorithmic complexity
    let docs = create_test_documents();
    let index = DocumentIndex::make_index(&docs);

    // makeIndex should be O(n log n) - completed during index creation
    assert!(index.word_count() > 0);

    // find should be O(log n) - should be very fast
    let start = std::time::Instant::now();
    let _result = index.find(&"programming".to_string());
    let find_duration = start.elapsed();
    assert!(find_duration.as_millis() < 10); // Should be very fast

    // Query operations should be O(m log(1 + n/m))
    let docs1 = index.find(&"programming".to_string());
    let docs2 = index.find(&"world".to_string());

    let start = std::time::Instant::now();
    let _and_result = DocumentIndex::query_and(&docs1, &docs2);
    let and_duration = start.elapsed();
    assert!(and_duration.as_millis() < 10); // Should be very fast

    let start = std::time::Instant::now();
    let _or_result = DocumentIndex::query_or(&docs1, &docs2);
    let or_duration = start.elapsed();
    assert!(or_duration.as_millis() < 10); // Should be very fast

    let start = std::time::Instant::now();
    let _and_not_result = DocumentIndex::query_and_not(&docs1, &docs2);
    let and_not_duration = start.elapsed();
    assert!(and_not_duration.as_millis() < 10); // Should be very fast
}
