//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 Example 42.1 demonstrating table operations.

pub mod Example42_1 {

    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use vstd::prelude::*;

    verus! {
        /// Placeholder to satisfy verasification; demo code uses Vec/sort_by.
        proof fn _example_42_1_verified() {}
    }
    use crate::Chap42::TableMtEph::TableMtEph::*;
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;
    use crate::*;
    pub type T = usize;

    pub trait Example42_1Trait {
        /// Example 42.1: Basic table operations demonstration
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn example_42_1();

        /// Demonstrate table operations with different implementations
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn demonstrate_table_operations();
    }

    /// Example 42.1: Basic table operations demonstration
    pub fn example_42_1() {
        println!("=== Chapter 42 Example 42.1: Table Operations ===");

        // Create tables using different implementations
        let table_per = TableStPerLit![1 => "Alice".to_string(), 2 => "Bob".to_string(), 3 => "Carol".to_string()];
        let mut table_eph = TableStEphLit![1 => "Alice".to_string(), 2 => "Bob".to_string(), 3 => "Carol".to_string()];
        let mut table_mt = TableMtEphLit![1 => "Alice".to_string(), 2 => "Bob".to_string(), 3 => "Carol".to_string()];

        println!("Initial tables created with 3 entries each");
        println!("Persistent table size: {}", table_per.size());
        println!("Ephemeral table size: {}", table_eph.size());
        println!("Multi-threaded table size: {}", table_mt.size());

        // Demonstrate find operation
        println!("\n--- Find Operations ---");
        println!("Find key 2 in persistent table: {:?}", table_per.find(&2));
        println!("Find key 4 in persistent table: {:?}", table_per.find(&4));

        // Demonstrate insert operation (persistent vs ephemeral)
        println!("\n--- Insert Operations ---");
        let table_per_new = table_per.insert(4, "Dave".to_string(), |_old, new| new.clone());
        println!(
            "After persistent insert - original size: {}, new size: {}",
            table_per.size(),
            table_per_new.size()
        );

        table_eph.insert(4, "Dave".to_string(), |_old, new| new.clone());
        println!("After ephemeral insert - table size: {}", table_eph.size());

        table_mt.insert(4, "Dave".to_string(), |_old, new| new.clone());
        println!("After multi-threaded insert - table size: {}", table_mt.size());

        // Demonstrate domain operation
        println!("\n--- Domain Operations ---");
        let domain_per = table_per_new.domain();
        let domain_eph = table_eph.domain();
        let domain_mt = table_mt.domain();

        println!("Persistent table domain size: {}", domain_per.size());
        println!("Ephemeral table domain size: {}", domain_eph.size());
        println!("Multi-threaded table domain size: {}", domain_mt.size());

        // Demonstrate map operation
        println!("\n--- Map Operations ---");
        let table_per_mapped = table_per_new.map(|name| name.to_uppercase());
        println!(
            "Persistent table after map (original unchanged): {:?}",
            table_per_new.find(&1)
        );
        println!(
            "Persistent table after map (new table): {:?}",
            table_per_mapped.find(&1)
        );

        table_eph.map(|name| name.to_uppercase());
        println!(
            "Ephemeral table after map (modified in place): {:?}",
            table_eph.find(&1)
        );

        table_mt.map(|name| name.to_uppercase());
        println!(
            "Multi-threaded table after map (modified in place): {:?}",
            table_mt.find(&1)
        );

        // Demonstrate filter operation
        println!("\n--- Filter Operations ---");
        let table_per_filtered = table_per_new.filter(|k, _v| *k <= 2);
        println!(
            "Persistent table after filter (keys <= 2): size = {}",
            table_per_filtered.size()
        );

        table_eph.filter(|k, _v| *k <= 2);
        println!("Ephemeral table after filter (keys <= 2): size = {}", table_eph.size());

        table_mt.filter(|k, _v| *k <= 2);
        println!(
            "Multi-threaded table after filter (keys <= 2): size = {}",
            table_mt.size()
        );

        // Demonstrate tabulate operation
        println!("\n--- Tabulate Operations ---");
        let mut keys = ArraySetStEph::empty();
        keys.insert(10);
        keys.insert(20);
        keys.insert(30);

        let table_per_tab = TableStPer::tabulate(|k| k * k, &keys);
        let table_eph_tab = TableStEph::tabulate(|k| k * k, &keys);
        let table_mt_tab = TableMtEph::tabulate(|k| k * k, &keys);

        println!("Tabulated tables (f(k) = k²):");
        println!("  Persistent: key 20 -> {:?}", table_per_tab.find(&20));
        println!("  Ephemeral: key 20 -> {:?}", table_eph_tab.find(&20));
        println!("  Multi-threaded: key 20 -> {:?}", table_mt_tab.find(&20));

        // Demonstrate set operations
        println!("\n--- Set Operations ---");
        let table1 = TableStPerLit![1 => "A".to_string(), 2 => "B".to_string()];
        let table2 = TableStPerLit![2 => "X".to_string(), 3 => "Y".to_string()];

        let intersection = table1.intersection(&table2, |v1, v2| format!("{v1}+{v2}"));
        let union = table1.union(&table2, |v1, v2| format!("{v1}+{v2}"));
        let difference = table1.difference(&table2);

        println!(
            "Table1 ∩ Table2: size = {}, key 2 -> {:?}",
            intersection.size(),
            intersection.find(&2)
        );
        println!("Table1 ∪ Table2: size = {}", union.size());
        println!("Table1 - Table2: size = {}", difference.size());

        println!("\n=== Example 42.1 Complete ===");
    }

    /// Demonstrate performance characteristics of different table implementations
    pub fn performance_comparison() {
        println!("\n=== Performance Comparison ===");

        let size = 1000;
        println!("Building tables with {size} entries...");

        // Build persistent table
        let start = std::time::Instant::now();
        let mut table_per = TableStPer::empty();
        for i in 0..size {
            table_per = table_per.insert(i, format!("value_{i}"), |_old, new| new.clone());
        }
        let per_time = start.elapsed();
        println!("Persistent table construction: {per_time:?}");

        // Build ephemeral table
        let start = std::time::Instant::now();
        let mut table_eph = TableStEph::empty();
        for i in 0..size {
            table_eph.insert(i, format!("value_{i}"), |_old, new| new.clone());
        }
        let eph_time = start.elapsed();
        println!("Ephemeral table construction: {eph_time:?}");

        // Build multi-threaded table
        let start = std::time::Instant::now();
        let mut table_mt = TableMtEph::empty();
        for i in 0..size {
            table_mt.insert(i, format!("value_{i}"), |_old, new| new.clone());
        }
        let mt_time = start.elapsed();
        println!("Multi-threaded table construction: {mt_time:?}");

        // Test map operation performance
        println!("\nMap operation performance:");

        let start = std::time::Instant::now();
        let _mapped_per = table_per.map(|s| s.to_uppercase());
        let per_map_time = start.elapsed();
        println!("Persistent map: {per_map_time:?}");

        let start = std::time::Instant::now();
        table_eph.map(|s| s.to_uppercase());
        let eph_map_time = start.elapsed();
        println!("Ephemeral map: {eph_map_time:?}");

        let start = std::time::Instant::now();
        table_mt.map(|s| s.to_uppercase());
        let mt_map_time = start.elapsed();
        println!("Multi-threaded map: {mt_map_time:?}");
    }
}
