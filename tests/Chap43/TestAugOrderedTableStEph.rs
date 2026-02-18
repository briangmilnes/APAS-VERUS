//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for single-threaded ephemeral reducer-augmented ordered table implementation.

use apas_verus::AugOrderedTableStEphLit;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_table() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table = AugOrderedTableStEph::<String, i32, _>::empty(max_reducer, i32::MIN);

    assert_eq!(table.size(), 0);
    assert_eq!(table.reduce_val(), i32::MIN);
    assert!(table.is_empty());
    assert!(table.first_key().is_none());
    assert!(table.last_key().is_none());
}

#[test]
fn test_singleton_table() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table = AugOrderedTableStEph::singleton(42, 100, max_reducer, i32::MIN);

    assert_eq!(table.size(), 1);
    assert_eq!(table.reduce_val(), 100);
    assert_eq!(table.find(&42), Some(100));
    assert_eq!(table.lookup(&42), Some(100));
    assert!(!table.is_empty());
    assert_eq!(table.first_key(), Some(42));
    assert_eq!(table.last_key(), Some(42));
}

#[test]
fn test_insert_and_reduce_val() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(max_reducer, i32::MIN);

    table.insert(1, 50, |_old, new| *new);
    assert_eq!(table.size(), 1);
    assert_eq!(table.reduce_val(), 50);

    table.insert(2, 30, |_old, new| *new);
    assert_eq!(table.size(), 2);
    assert_eq!(table.reduce_val(), 50); // max(50, 30) = 50

    table.insert(3, 80, |_old, new| *new);
    assert_eq!(table.size(), 3);
    assert_eq!(table.reduce_val(), 80); // max(50, 30, 80) = 80

    table.insert(4, 20, |_old, new| *new);
    assert_eq!(table.size(), 4);
    assert_eq!(table.reduce_val(), 80); // max(50, 30, 80, 20) = 80
}

#[test]
fn test_delete_and_reduce_val() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(max_reducer, i32::MIN);

    table.insert(1, 50, |_old, new| *new);
    table.insert(2, 80, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);
    assert_eq!(table.reduce_val(), 80);

    let deleted = table.delete(&2);
    assert_eq!(deleted, Some(80));
    assert_eq!(table.size(), 2);
    assert_eq!(table.reduce_val(), 50); // max(50, 30) = 50
    assert_eq!(table.find(&2), None);
}

#[test]
fn test_qadsan_stock_scenario() {
    // QADSAN scenario: timestamp -> stock price, using max reducer
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut stock_prices = AugOrderedTableStEph::empty(max_reducer, 0);

    // Stock price data: timestamp (minute) -> price in cents
    stock_prices.insert(900, 15000, |_old, new| *new); // 9:00am: $150.00
    stock_prices.insert(930, 15250, |_old, new| *new); // 9:30am: $152.50
    stock_prices.insert(1000, 14800, |_old, new| *new); // 10:00am: $148.00
    stock_prices.insert(1030, 15500, |_old, new| *new); // 10:30am: $155.00
    stock_prices.insert(1100, 15300, |_old, new| *new); // 11:00am: $153.00
    stock_prices.insert(1130, 15700, |_old, new| *new); // 11:30am: $157.00

    // Maximum price for the entire session
    assert_eq!(stock_prices.reduce_val(), 15700); // $157.00

    // Maximum price between 9:30am and 10:30am (930-1030)
    let morning_max = stock_prices.reduce_range(&930, &1030);
    assert_eq!(morning_max, 15500); // max(15250, 14800, 15500) = 15500

    // Maximum price between 11:00am and 11:30am (1100-1130)
    let late_morning_max = stock_prices.reduce_range(&1100, &1130);
    assert_eq!(late_morning_max, 15700); // max(15300, 15700) = 15700

    // Verify individual lookups still work
    assert_eq!(stock_prices.find(&1030), Some(15500));
    assert_eq!(stock_prices.find(&1200), None);
}

#[test]
fn test_mutable_operations() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Insert initial data
    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);
    assert_eq!(table.reduce_val(), 60);

    // Test in-place update
    table.insert(2, 25, |old, new| old + new); // Combine old and new
    assert_eq!(table.reduce_val(), 85); // 10 + 45 + 30 = 85
    assert_eq!(table.find(&2), Some(45)); // 20 + 25 = 45

    // Test deletion
    table.delete(&1);
    assert_eq!(table.reduce_val(), 75); // 45 + 30 = 75
    assert_eq!(table.size(), 2);
}

#[test]
fn test_min_reducer() {
    let min_reducer = |a: &i32, b: &i32| if a < b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(min_reducer, i32::MAX);

    table.insert(1, 50, |_old, new| *new);
    assert_eq!(table.reduce_val(), 50);

    table.insert(2, 30, |_old, new| *new);
    assert_eq!(table.reduce_val(), 30); // min(50, 30) = 30

    table.insert(3, 80, |_old, new| *new);
    assert_eq!(table.reduce_val(), 30); // min(50, 30, 80) = 30

    table.insert(4, 20, |_old, new| *new);
    assert_eq!(table.reduce_val(), 20); // min(50, 30, 80, 20) = 20
}

#[test]
fn test_string_concatenation_reducer() {
    let concat_reducer = |a: &String, b: &String| format!("{a}{b}");
    let mut table = AugOrderedTableStEph::empty(concat_reducer, String::new());

    table.insert(1, "Hello".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "Hello");

    table.insert(2, " ".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "Hello ");

    table.insert(3, "World".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "Hello World");

    // Replace key 2's value; reduction recalculated from scratch.
    table.insert(2, "Beautiful ".to_string(), |_old, new| new.clone());
    assert_eq!(table.reduce_val(), "HelloBeautiful World");
}

#[test]
fn test_range_operations() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Insert data: key -> value
    for i in 1..=10 {
        table.insert(i, i * 10, |_old, new| *new); // 1->10, 2->20, ..., 10->100
    }

    assert_eq!(table.reduce_val(), 550); // Sum of 10+20+...+100

    // Test range [3, 7]
    let range_sum = table.reduce_range(&3, &7);
    let expected_range_sum = 30 + 40 + 50 + 60 + 70; // Keys 3,4,5,6,7
    assert_eq!(range_sum, expected_range_sum);
}

#[test]
fn test_split_and_join() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);
    table.insert(5, 50, |_old, new| *new);
    table.insert(7, 70, |_old, new| *new);

    assert_eq!(table.reduce_val(), 160);

    // Split at key 4
    let (left, right) = table.split_key(&4);

    // Left should have keys 1,3 with values 10,30
    assert_eq!(left.reduce_val(), 40);
    assert_eq!(left.size(), 2);

    // Right should have keys 5,7 with values 50,70
    assert_eq!(right.reduce_val(), 120);
    assert_eq!(right.size(), 2);

    // Join them back (note: this consumes the tables)
    let mut rejoined = left;
    rejoined.join_key(right);
    assert_eq!(rejoined.reduce_val(), 160);
    assert_eq!(rejoined.size(), 4);
}

#[test]
fn test_map_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    assert_eq!(table.reduce_val(), 60);

    // Double all values (creates new table)
    let doubled = table.map(|_k, v| v * 2);
    assert_eq!(doubled.reduce_val(), 120); // 20+40+60 = 120
    assert_eq!(doubled.size(), 3);

    // Original table unchanged
    assert_eq!(table.reduce_val(), 60);
}

#[test]
fn test_filter_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    for i in 1..=10 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    assert_eq!(table.reduce_val(), 550);

    // Filter even keys (creates new table)
    let even_table = table.filter(|k, _v| k % 2 == 0);

    // Even keys: 2,4,6,8,10 with values 20,40,60,80,100
    let expected_sum = 20 + 40 + 60 + 80 + 100;
    assert_eq!(even_table.reduce_val(), expected_sum);
    assert_eq!(even_table.size(), 5);

    // Original table unchanged
    assert_eq!(table.reduce_val(), 550);
}

#[test]
fn test_union_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table1 = AugOrderedTableStEph::empty(sum_reducer, 0);
    let mut table2 = AugOrderedTableStEph::empty(sum_reducer, 0);

    table1.insert(1, 10, |_old, new| *new);
    table1.insert(2, 20, |_old, new| *new);

    table2.insert(2, 25, |_old, new| *new); // Overlapping key
    table2.insert(3, 30, |_old, new| *new);

    // Union modifies table1 in place
    table1.union(&table2, |v1, v2| v1 + v2);

    // Keys: 1->10, 2->45 (20+25), 3->30
    assert_eq!(table1.reduce_val(), 85); // 10+45+30
    assert_eq!(table1.size(), 3);
    assert_eq!(table1.find(&2), Some(45));
}

#[test]
fn test_intersection_operation() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table1 = AugOrderedTableStEph::empty(max_reducer, 0);
    let mut table2 = AugOrderedTableStEph::empty(max_reducer, 0);

    table1.insert(1, 10, |_old, new| *new);
    table1.insert(2, 20, |_old, new| *new);
    table1.insert(3, 30, |_old, new| *new);

    table2.insert(2, 25, |_old, new| *new);
    table2.insert(3, 15, |_old, new| *new);
    table2.insert(4, 40, |_old, new| *new);

    // Intersection modifies table1 in place
    table1.intersection(&table2, |v1, v2| if v1 > v2 { *v1 } else { *v2 });

    // Only keys 2,3 remain: 2->25 (max(20,25)), 3->30 (max(30,15))
    assert_eq!(table1.size(), 2);
    assert_eq!(table1.find(&2), Some(25));
    assert_eq!(table1.find(&3), Some(30));
    assert_eq!(table1.reduce_val(), 30); // max(25, 30) = 30
}

#[test]
fn test_difference_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table1 = AugOrderedTableStEph::empty(sum_reducer, 0);
    let mut table2 = AugOrderedTableStEph::empty(sum_reducer, 0);

    table1.insert(1, 10, |_old, new| *new);
    table1.insert(2, 20, |_old, new| *new);
    table1.insert(3, 30, |_old, new| *new);

    table2.insert(2, 25, |_old, new| *new);
    table2.insert(4, 40, |_old, new| *new);

    // Difference modifies table1 in place
    table1.difference(&table2);

    // Only keys not in table2 remain: 1->10, 3->30
    assert_eq!(table1.size(), 2);
    assert_eq!(table1.find(&1), Some(10));
    assert_eq!(table1.find(&3), Some(30));
    assert_eq!(table1.find(&2), None);
    assert_eq!(table1.reduce_val(), 40); // 10 + 30 = 40
}

#[test]
fn test_macro_construction() {
    let sum_reducer = |a: &i32, b: &i32| a + b;

    let table: AugOrderedTableStEph<i32, i32, _> = AugOrderedTableStEphLit![
        reducer: sum_reducer, identity: 0,
        1 => 100,
        2 => 200,
        3 => 300
    ];

    assert_eq!(table.size(), 3);
    assert_eq!(table.reduce_val(), 600);
    assert_eq!(table.find(&2), Some(200));

    // Empty table via macro
    let empty_table: AugOrderedTableStEph<i32, i32, _> = AugOrderedTableStEphLit![
        reducer: sum_reducer, identity: 0
    ];
    assert_eq!(empty_table.size(), 0);
    assert_eq!(empty_table.reduce_val(), 0);
}

#[test]
fn test_display_and_debug() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let table = AugOrderedTableStEph::singleton(42, 100, max_reducer, 0);

    let display_str = format!("{table}");
    assert!(display_str.contains("AugOrderedTableStEph"));
    assert!(display_str.contains("size: 1"));
    assert!(display_str.contains("reduction: 100"));

    let debug_str = format!("{table:?}");
    assert!(debug_str.contains("AugOrderedTableStEph"));
    assert!(debug_str.contains("size"));
    assert!(debug_str.contains("cached_reduction"));
}

#[test]
fn test_ordering_operations() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    table.insert(5, 50, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(8, 80, |_old, new| *new);
    table.insert(1, 10, |_old, new| *new);
    table.insert(9, 90, |_old, new| *new);

    // Test ordering operations
    assert_eq!(table.first_key(), Some(1));
    assert_eq!(table.last_key(), Some(9));
    assert_eq!(table.previous_key(&5), Some(2));
    assert_eq!(table.next_key(&5), Some(8));

    // Test rank and select
    assert_eq!(table.rank_key(&5), 2); // 5 is the 3rd key (0-indexed: rank 2)
    assert_eq!(table.select_key(2), Some(5)); // 3rd key (0-indexed: index 2)
}

#[test]
fn test_complex_qadsan_scenario() {
    // Complex QADSAN scenario with multiple trading sessions
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut daily_highs = AugOrderedTableStEph::empty(max_reducer, 0);

    // Pre-market session (600-930): timestamps in minutes from midnight
    for minute in (600..930).step_by(30) {
        let price = 14000 + (minute - 600) / 10; // Gradual increase
        daily_highs.insert(minute, price, |old, new| if old > new { *old } else { *new });
    }

    // Regular session (930-1600): higher volatility
    for minute in (930..1600).step_by(15) {
        let base_price = 15000;
        let volatility = ((minute - 930) % 100) * 5; // Cyclical volatility
        let price = base_price + volatility;
        daily_highs.insert(minute, price, |old, new| if old > new { *old } else { *new });
    }

    // After-hours session (1600-2000): declining prices
    for minute in (1600..2000).step_by(60) {
        let price = 15000 - (minute - 1600) / 10; // Gradual decrease
        daily_highs.insert(minute, price, |old, new| if old > new { *old } else { *new });
    }

    // Overall daily high
    let daily_high = daily_highs.reduce_val();
    assert!(daily_high >= 15000); // Should be at least the base regular session price

    // Pre-market high (600-930) - range should be exclusive of 930
    let premarket_high = daily_highs.reduce_range(&600, &929); // Use 929 to exclude 930
    assert!(premarket_high < 15000); // Should be less than regular session

    // Regular session high (930-1600)
    let regular_high = daily_highs.reduce_range(&930, &1600);
    assert_eq!(regular_high, daily_high); // Should be the overall high

    // After-hours high (1600-2000)
    let afterhours_high = daily_highs.reduce_range(&1600, &2000);
    assert!(afterhours_high < regular_high); // Should be less than regular session

    // Peak trading hour (1200-1300)
    let peak_hour_high = daily_highs.reduce_range(&1200, &1300);
    assert!(peak_hour_high > 15000); // Should be elevated during peak hours
}

#[test]
fn test_reduce_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    // Test general reduce operation (different from reduce_val)
    let sum_of_keys = table.reduce(0, |acc, k, _v| acc + k);
    assert_eq!(sum_of_keys, 6); // 1 + 2 + 3 = 6

    let sum_of_values = table.reduce(0, |acc, _k, v| acc + v);
    assert_eq!(sum_of_values, 60); // 10 + 20 + 30 = 60

    let key_value_product_sum = table.reduce(0, |acc, k, v| acc + (k * v));
    assert_eq!(key_value_product_sum, 140); // 1*10 + 2*20 + 3*30 = 10 + 40 + 90 = 140
}

#[test]
fn test_domain_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Insert elements
    for i in 1..=10 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    // Get domain (set of all keys)
    let domain = table.domain();
    assert_eq!(domain.size(), 10);
    for i in 1..=10 {
        assert!(domain.find(&i));
    }
    assert!(!domain.find(&11));
}

#[test]
fn test_collect_operation() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(max_reducer, 0);

    // Insert elements
    for i in 1..=5 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    // Collect to sequence
    let seq = table.collect();
    assert_eq!(seq.length(), 5);

    // Verify elements are in order
    let first = seq.nth(0);
    assert_eq!(first.0, 1);
    assert_eq!(first.1, 10);
}

#[test]
fn test_restrict_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Insert elements
    for i in 1..=10 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    // Create a set to restrict to
    let mut key_set = ArraySetStEph::empty();
    key_set.insert(2);
    key_set.insert(4);
    key_set.insert(6);

    // Restrict table to keys in set
    table.restrict(&key_set);
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&2), Some(20));
    assert_eq!(table.find(&4), Some(40));
    assert_eq!(table.find(&6), Some(60));
    assert_eq!(table.find(&3), None);
}

#[test]
fn test_subtract_operation() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(max_reducer, 0);

    // Insert elements
    for i in 1..=10 {
        table.insert(i, i * 10, |_old, new| *new);
    }

    // Create a set of keys to subtract
    let mut key_set = ArraySetStEph::empty();
    key_set.insert(2);
    key_set.insert(5);
    key_set.insert(8);

    // Subtract keys from table
    table.subtract(&key_set);
    assert_eq!(table.size(), 7);
    assert_eq!(table.find(&1), Some(10));
    assert_eq!(table.find(&2), None);
    assert_eq!(table.find(&5), None);
    assert_eq!(table.find(&8), None);
    assert_eq!(table.find(&10), Some(100));
}

#[test]
fn test_tabulate_operation() {
    let sum_reducer = |a: &i32, b: &i32| a + b;

    // Create domain set
    let mut domain = ArraySetStEph::empty();
    for i in 1..=5 {
        domain.insert(i);
    }

    // Tabulate: create table from domain and function
    let table = AugOrderedTableStEph::tabulate(|k: &i32| k * k, &domain, sum_reducer, 0);

    assert_eq!(table.size(), 5);
    assert_eq!(table.find(&1), Some(1));
    assert_eq!(table.find(&2), Some(4));
    assert_eq!(table.find(&3), Some(9));
    assert_eq!(table.find(&4), Some(16));
    assert_eq!(table.find(&5), Some(25));
    assert_eq!(table.reduce_val(), 55); // 1+4+9+16+25
}

#[test]
fn test_key_navigation() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(max_reducer, 0);

    // Insert: 10, 20, 30, 40, 50
    for i in 1..=5 {
        table.insert(i * 10, i * 100, |_old, new| *new);
    }

    // Test next_key
    assert_eq!(table.next_key(&10), Some(20));
    assert_eq!(table.next_key(&20), Some(30));
    assert_eq!(table.next_key(&30), Some(40));
    assert_eq!(table.next_key(&40), Some(50));
    assert_eq!(table.next_key(&50), None);
    assert_eq!(table.next_key(&25), Some(30)); // Next after non-existent key

    // Test previous_key
    assert_eq!(table.previous_key(&50), Some(40));
    assert_eq!(table.previous_key(&40), Some(30));
    assert_eq!(table.previous_key(&30), Some(20));
    assert_eq!(table.previous_key(&20), Some(10));
    assert_eq!(table.previous_key(&10), None);
    assert_eq!(table.previous_key(&25), Some(20)); // Previous before non-existent key
}

#[test]
fn test_rank_and_select() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Insert: 10, 20, 30, 40, 50
    for i in 1..=5 {
        table.insert(i * 10, i, |_old, new| *new);
    }

    // Test rank_key (0-indexed position)
    assert_eq!(table.rank_key(&10), 0);
    assert_eq!(table.rank_key(&20), 1);
    assert_eq!(table.rank_key(&30), 2);
    assert_eq!(table.rank_key(&40), 3);
    assert_eq!(table.rank_key(&50), 4);

    // Test select_key (key at given rank)
    assert_eq!(table.select_key(0), Some(10));
    assert_eq!(table.select_key(1), Some(20));
    assert_eq!(table.select_key(2), Some(30));
    assert_eq!(table.select_key(3), Some(40));
    assert_eq!(table.select_key(4), Some(50));
    assert_eq!(table.select_key(5), None); // Out of bounds
}

#[test]
fn test_get_key_range() {
    let max_reducer = |a: &i32, b: &i32| if a > b { *a } else { *b };
    let mut table = AugOrderedTableStEph::empty(max_reducer, 0);

    // Insert values from 10 to 100
    for i in 1..=10 {
        table.insert(i * 10, i * 10, |_old, new| *new);
    }

    // Get range [30, 70]
    let range = table.get_key_range(&30, &70);
    assert_eq!(range.size(), 5); // 30, 40, 50, 60, 70
    assert_eq!(range.find(&30), Some(30));
    assert_eq!(range.find(&40), Some(40));
    assert_eq!(range.find(&50), Some(50));
    assert_eq!(range.find(&60), Some(60));
    assert_eq!(range.find(&70), Some(70));
    assert_eq!(range.find(&20), None);
    assert_eq!(range.find(&80), None);
}

#[test]
fn test_split_rank_key() {
    let sum_reducer = |a: &i32, b: &i32| a + b;
    let mut table = AugOrderedTableStEph::empty(sum_reducer, 0);

    // Insert: 10, 20, 30, 40, 50
    for i in 1..=5 {
        table.insert(i * 10, i, |_old, new| *new);
    }

    // Split at rank 2 (index 2 = key 30)
    let (left, right) = table.split_rank_key(2);

    // Left should have keys < 30 (10, 20)
    assert_eq!(left.size(), 2);
    assert_eq!(left.find(&10), Some(1));
    assert_eq!(left.find(&20), Some(2));

    // Right should have keys >= 30 (30, 40, 50)
    assert_eq!(right.size(), 3);
    assert_eq!(right.find(&30), Some(3));
    assert_eq!(right.find(&40), Some(4));
    assert_eq!(right.find(&50), Some(5));
}

#[test]
fn test_delete_nonexistent() {
    let mut table = AugOrderedTableStEph::empty(|a: &i32, b: &i32| a + b, 0);
    table.insert(1, 10, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);
    table.delete(&2);
    assert_eq!(table.size(), 2);
}
