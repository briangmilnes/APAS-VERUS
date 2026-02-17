//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap47::FlatHashTable::FlatHashTable::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Chap47::QuadProbFlatHashTableStEph::QuadProbFlatHashTableStEph::*;
use apas_verus::Types::Types::*;
use std::rc::Rc;

type QuadTable = HashTable<i32, String, FlatEntry<i32, String>, ()>;

#[test]
fn test_insert_and_lookup() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    QuadProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 12, "twelve".to_string()); // Collides with 1

    assert_eq!(QuadProbFlatHashTableStEph::lookup(&table, &1), Some("one".to_string()));
    assert_eq!(
        QuadProbFlatHashTableStEph::lookup(&table, &12),
        Some("twelve".to_string())
    );
}

#[test]
fn test_delete() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    QuadProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(QuadProbFlatHashTableStEph::delete(&mut table, &1));
    assert_eq!(QuadProbFlatHashTableStEph::lookup(&table, &1), None);
    assert!(!QuadProbFlatHashTableStEph::delete(&mut table, &1)); // Already deleted
}

#[test]
fn test_probe_quadratic_sequence() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    // Verify quadratic probing: (hash + i²) mod m
    let key = 5;
    let slot0 = QuadProbFlatHashTableStEph::probe(&table, &key, 0);
    let slot1 = QuadProbFlatHashTableStEph::probe(&table, &key, 1);
    let slot2 = QuadProbFlatHashTableStEph::probe(&table, &key, 2);
    let slot3 = QuadProbFlatHashTableStEph::probe(&table, &key, 3);

    // hash(5) = 5, so:
    // slot0 = (5 + 0²) % 11 = 5
    // slot1 = (5 + 1²) % 11 = 6
    // slot2 = (5 + 4) % 11 = 9
    // slot3 = (5 + 9) % 11 = 3
    assert_eq!(slot0, 5);
    assert_eq!(slot1, 6);
    assert_eq!(slot2, 9);
    assert_eq!(slot3, 3);
}

#[test]
fn test_find_slot() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    let slot = QuadProbFlatHashTableStEph::find_slot(&table, &5);
    assert!(slot < 11);
}

#[test]
fn test_update_existing_key() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    QuadProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 1, "ONE".to_string());
    assert_eq!(QuadProbFlatHashTableStEph::lookup(&table, &1), Some("ONE".to_string()));
    assert_eq!(table.num_elements, 1); // Should not increment on update
}

#[test]
fn test_max_attempts_ceiling_m_over_2() {
    // APAS Lemma 47.1: Only first ⌈m/2⌉ probes are guaranteed distinct
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|_size| Box::new(|_k| 0)); // All keys hash to 0
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    // Fill first 6 slots (⌈11/2⌉ = 6) with colliding keys
    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert 6 keys (all hash to 0, probe quadratically)
    for i in 0..6 {
        QuadProbFlatHashTableStEph::insert(&mut table, i, format!("value{i}"));
    }

    // Verify all 6 were inserted
    for i in 0..6 {
        assert!(QuadProbFlatHashTableStEph::lookup(&table, &i).is_some());
    }

    // 7th insertion should still work (table not completely full)
    QuadProbFlatHashTableStEph::insert(&mut table, 99, "extra".to_string());
}

#[test]
fn test_lookup_stops_at_max_attempts() {
    // Test that lookup doesn't loop forever, stops at ⌈m/2⌉ attempts
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    // Fill table with many entries
    for i in 0..8 {
        QuadProbFlatHashTableStEph::insert(&mut table, i, format!("value{i}"));
    }

    // Looking up non-existent key should return None, not hang
    assert_eq!(QuadProbFlatHashTableStEph::lookup(&table, &999), None);
}

#[test]
fn test_delete_maintains_probe_chain() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert three colliding keys
    QuadProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 12, "twelve".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 23, "twentythree".to_string());

    // Delete middle one
    QuadProbFlatHashTableStEph::delete(&mut table, &12);

    // Should still be able to find the third key
    assert_eq!(
        QuadProbFlatHashTableStEph::lookup(&table, &23),
        Some("twentythree".to_string())
    );
}

#[test]
fn test_prime_size_guarantees() {
    // APAS: If m is prime and table at least half empty, quadratic probing finds empty slot
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size)); // 13 is prime
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            13,
        );

    for _ in 0..13 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert up to ⌈13/2⌉ = 7 items - should always succeed
    for i in 0..7 {
        QuadProbFlatHashTableStEph::insert(&mut table, i, format!("value{i}"));
        assert_eq!(
            QuadProbFlatHashTableStEph::lookup(&table, &i),
            Some(format!("value{i}"))
        );
    }
}

#[test]
fn test_resize_empty_table() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    let new_table = QuadProbFlatHashTableStEph::resize(&table, 23);
    assert_eq!(new_table.current_size, 23);
    assert_eq!(new_table.num_elements, 0);
}

#[test]
fn test_resize_with_elements() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    QuadProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 2, "two".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 3, "three".to_string());

    let new_table = QuadProbFlatHashTableStEph::resize(&table, 23);
    assert_eq!(new_table.current_size, 23);
    assert_eq!(new_table.num_elements, 3);

    assert_eq!(
        QuadProbFlatHashTableStEph::lookup(&new_table, &1),
        Some("one".to_string())
    );
    assert_eq!(
        QuadProbFlatHashTableStEph::lookup(&new_table, &2),
        Some("two".to_string())
    );
    assert_eq!(
        QuadProbFlatHashTableStEph::lookup(&new_table, &3),
        Some("three".to_string())
    );
}

#[test]
fn test_load_and_size() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: QuadTable =
        <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<i32, String, FlatEntry<i32, String>, ()>>::createTable(
            hash_fn_gen,
            11,
        );

    let result = QuadProbFlatHashTableStEph::loadAndSize(&table);
    assert_eq!(result.load, 0.0);
    assert_eq!(result.size, 11);

    QuadProbFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    QuadProbFlatHashTableStEph::insert(&mut table, 2, "two".to_string());
    let result = QuadProbFlatHashTableStEph::loadAndSize(&table);
    assert!((result.load - 0.18181818).abs() < 0.01); // 2/11 ≈ 0.182
    assert_eq!(result.size, 11);
}
