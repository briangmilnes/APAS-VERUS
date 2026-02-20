//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Example 43.1 from the textbook demonstrating ordered set operations.

pub mod Example43_1 {

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use vstd::prelude::*;

    verus! {
        /// Placeholder to satisfy verasification; demo code uses OrderedSetStPerLit.
        proof fn _example_43_1_verified() {}
    }
    use crate::Chap43::OrderedSetStPer::OrderedSetStPer::*;
    use crate::OrderedSetStPerLit;
    use crate::Types::Types::*;
    pub type T = N;

    pub trait Example43_1Trait {
        /// Demonstrates Example 43.1 from the textbook with lexicographic ordering
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn run_example43_1();

        /// Demonstrate ordered set operations
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn demonstrate_ordered_operations();
    }

    /// Demonstrates Example 43.1 from the textbook with lexicographic ordering
    pub fn run_example43_1() {
        println!("=== Example 43.1: Ordered Set Operations ===");

        // Create the ordered set A = {'artie', 'burt', 'finn', 'mike', 'rachel', 'sam', 'tina'}
        let set_a: OrderedSetStPer<String> = OrderedSetStPerLit![
            "artie".to_string(),
            "burt".to_string(),
            "finn".to_string(),
            "mike".to_string(),
            "rachel".to_string(),
            "sam".to_string(),
            "tina".to_string()
        ];

        print!("Set A: [");
        let seq = set_a.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("]");

        // first A → 'artie'
        match set_a.first() {
            | Some(first) => println!("first(A) = '{first}'"),
            | None => println!("first(A) = ⊥"),
        }

        // next(A, 'quinn') → 'rachel'
        match set_a.next(&"quinn".to_string()) {
            | Some(next_elem) => println!("next(A, 'quinn') = '{next_elem}'"),
            | None => println!("next(A, 'quinn') = ⊥"),
        }

        // next(A, 'mike') → 'rachel'
        match set_a.next(&"mike".to_string()) {
            | Some(next_elem) => println!("next(A, 'mike') = '{next_elem}'"),
            | None => println!("next(A, 'mike') = ⊥"),
        }

        // getRange A ('burt', 'mike') → {'burt', 'finn', 'mike'}
        let range_set = set_a.get_range(&"burt".to_string(), &"mike".to_string());
        print!("getRange(A, 'burt', 'mike') = [");
        let seq = range_set.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("]");

        // rank(A, 'rachel') → 4
        let rachel_rank = set_a.rank(&"rachel".to_string());
        println!("rank(A, 'rachel') = {rachel_rank}");

        // rank(A, 'quinn') → 4 (elements less than 'quinn')
        let quinn_rank = set_a.rank(&"quinn".to_string());
        println!("rank(A, 'quinn') = {quinn_rank}");

        // select(A, 5) → 'sam'
        match set_a.select(5) {
            | Some(selected) => println!("select(A, 5) = '{selected}'"),
            | None => println!("select(A, 5) = ⊥"),
        }

        // splitRank(A, 3) → ({'artie', 'burt', 'finn'}, {'mike', 'rachel', 'sam', 'tina'})
        let (left_set, right_set) = set_a.split_rank(3);
        println!("splitRank(A, 3) = (");
        print!("  left: [");
        let seq = left_set.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("],");
        print!("  right: [");
        let seq = right_set.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("]");
        println!(")");

        // Additional demonstrations of other ordering operations
        println!("\n=== Additional Ordering Operations ===");

        // last A → 'tina'
        match set_a.last() {
            | Some(last) => println!("last(A) = '{last}'"),
            | None => println!("last(A) = ⊥"),
        }

        // previous(A, 'rachel') → 'mike'
        match set_a.previous(&"rachel".to_string()) {
            | Some(prev) => println!("previous(A, 'rachel') = '{prev}'"),
            | None => println!("previous(A, 'rachel') = ⊥"),
        }

        // split(A, 'mike') → ({'artie', 'burt', 'finn'}, true, {'rachel', 'sam', 'tina'})
        let (left_split, found, right_split) = set_a.split(&"mike".to_string());
        println!("split(A, 'mike') = (");
        print!("  left: [");
        let seq = left_split.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("],");
        println!("  found: {found},");
        print!("  right: [");
        let seq = right_split.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("]");
        println!(")");

        // Demonstrate join operation
        let joined_set = OrderedSetStPer::join(&left_split, &right_split);
        print!("join(left, right) = [");
        let seq = joined_set.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("\"{}\"", seq.nth(i));
        }
        println!("]");

        println!("\n=== Example 43.1 Complete ===");
    }

    /// Demonstrates ordering operations with integer sets for additional clarity
    pub fn run_integer_example() {
        println!("\n=== Integer Ordered Set Example ===");

        // Create an ordered set of integers
        let int_set: OrderedSetStPer<i32> = OrderedSetStPerLit![1, 3, 5, 7, 9, 11, 13];

        print!("Integer Set: [");
        let seq = int_set.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", seq.nth(i));
        }
        println!("]");

        // Demonstrate all ordering operations
        println!("first() = {:?}", int_set.first());
        println!("last() = {:?}", int_set.last());
        println!("previous(7) = {:?}", int_set.previous(&7));
        println!("next(7) = {:?}", int_set.next(&7));
        println!("rank(7) = {}", int_set.rank(&7));
        println!("select(3) = {:?}", int_set.select(3));

        let range = int_set.get_range(&3, &9);
        print!("getRange(3, 9) = [");
        let seq = range.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", seq.nth(i));
        }
        println!("]");

        let (left, right) = int_set.split_rank(4);
        print!("splitRank(4) = ([");
        let seq = left.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", seq.nth(i));
        }
        print!("], [");
        let seq = right.to_seq();
        for i in 0..seq.length() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", seq.nth(i));
        }
        println!("])");

        println!("=== Integer Example Complete ===");
    }
}
