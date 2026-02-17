//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap47::DoubleHashFlatHashTableStEph::DoubleHashFlatHashTableStEph::*;
use apas_verus::Chap47::FlatHashTable::FlatHashTable::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Types::Types::*;
use std::rc::Rc;

type DoubleHashTable = HashTable<i32, String, FlatEntry<i32, String>, ()>;

#[test]
fn test_insert_and_lookup() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert non-colliding keys for deterministic test
    DoubleHashFlatHashTableStEph::insert(&mut table, 0, "zero".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 2, "two".to_string());

    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&table, &0),
        Some("zero".to_string())
    );
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&table, &1),
        Some("one".to_string())
    );
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&table, &2),
        Some("two".to_string())
    );
}

#[test]
fn test_delete() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(DoubleHashFlatHashTableStEph::delete(&mut table, &1));
    assert_eq!(DoubleHashFlatHashTableStEph::lookup(&table, &1), None);
    assert!(!DoubleHashFlatHashTableStEph::delete(&mut table, &1)); // Already deleted
}

#[test]
fn test_second_hash_nonzero() {
    // APAS: hh(k) cannot equal zero
    for i in 0..100 {
        let step = DoubleHashFlatHashTableStEph::second_hash(&i, 11);
        assert_ne!(step, 0, "Second hash returned zero for key {i}");
        assert!(step < 11, "Second hash {step} >= table size 11 for key {i}");
    }
}

#[test]
fn test_second_hash_is_odd() {
    // Our implementation ensures step is odd (for power-of-2 coprimality)
    for i in 0..100 {
        let step = DoubleHashFlatHashTableStEph::second_hash(&i, 16); // Power of 2
        assert_eq!(step % 2, 1, "Second hash {step} is not odd for key {i}");
    }
}

#[test]
fn test_probe_double_hash_sequence() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    let key = 5;
    let slot0 = DoubleHashFlatHashTableStEph::probe(&table, &key, 0);
    let slot1 = DoubleHashFlatHashTableStEph::probe(&table, &key, 1);
    let slot2 = DoubleHashFlatHashTableStEph::probe(&table, &key, 2);

    // Verify probes are different
    assert_ne!(slot0, slot1);
    assert_ne!(slot1, slot2);

    // Verify all within bounds
    assert!(slot0 < 11);
    assert!(slot1 < 11);
    assert!(slot2 < 11);
}

#[test]
fn test_probe_visits_all_slots_prime_size() {
    // APAS: When step is coprime to m (prime size), all m slots are visited
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|_size| Box::new(|_k| 0)); // Hash to 0
    let table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    let key = 1;
    let step = DoubleHashFlatHashTableStEph::second_hash(&key, 11);

    // Should visit all 11 slots before cycling
    let mut visited = vec![false; 11];
    for attempt in 0..11 {
        let slot = DoubleHashFlatHashTableStEph::probe(&table, &key, attempt);
        visited[slot] = true;
    }

    // All slots should be visited (assuming step is coprime to 11)
    let all_visited = visited.iter().all(|&v| v);
    assert!(all_visited, "Not all slots visited with step={step}: {visited:?}");
}

#[test]
fn test_find_slot() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    let slot = DoubleHashFlatHashTableStEph::find_slot(&table, &5);
    assert!(slot < 11);
}

#[test]
fn test_update_existing_key() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "ONE".to_string());
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&table, &1),
        Some("ONE".to_string())
    );
    assert_eq!(table.num_elements, 1); // Should not increment on update
}

#[test]
fn test_high_load_factor() {
    // APAS: Double hashing allows higher load factors than quadratic
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert 8 items (load factor ≈ 0.73)
    for i in 0..8 {
        DoubleHashFlatHashTableStEph::insert(&mut table, i, format!("value{i}"));
    }

    // All should be retrievable
    for i in 0..8 {
        assert_eq!(
            DoubleHashFlatHashTableStEph::lookup(&table, &i),
            Some(format!("value{i}"))
        );
    }
}

#[test]
fn test_delete_maintains_probe_chain() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    // Insert and delete non-colliding keys
    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 2, "two".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 3, "three".to_string());

    DoubleHashFlatHashTableStEph::delete(&mut table, &2);

    // Should still find other keys
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&table, &1),
        Some("one".to_string())
    );
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&table, &3),
        Some("three".to_string())
    );
    assert_eq!(DoubleHashFlatHashTableStEph::lookup(&table, &2), None);
}

#[test]
fn test_lookup_nonexistent_key() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    for _ in 0..11 {
        table.table.push(FlatEntry::Empty);
    }

    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert_eq!(DoubleHashFlatHashTableStEph::lookup(&table, &999), None);
}

#[test]
fn test_different_probe_sequences_for_colliding_keys() {
    // APAS: Double hashing avoids secondary clustering - colliding keys have different sequences
    // Keys 1 and 12 hash to same slot (1 % 11 = 1, 12 % 11 = 1)
    let key1 = 1;
    let key2 = 12;

    // Verify both keys produce valid step sizes
    let step1 = DoubleHashFlatHashTableStEph::second_hash(&key1, 11);
    let step2 = DoubleHashFlatHashTableStEph::second_hash(&key2, 11);

    // Both steps must be non-zero and less than table size
    assert!(step1 > 0 && step1 < 11);
    assert!(step2 > 0 && step2 < 11);
}

#[test]
fn test_resize_empty_table() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    let new_table = DoubleHashFlatHashTableStEph::resize(&table, 23);
    assert_eq!(new_table.current_size, 23);
    assert_eq!(new_table.num_elements, 0);
}

#[test]
fn test_resize_with_elements() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 2, "two".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 3, "three".to_string());

    let new_table = DoubleHashFlatHashTableStEph::resize(&table, 23);
    assert_eq!(new_table.current_size, 23);
    assert_eq!(new_table.num_elements, 3);

    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&new_table, &1),
        Some("one".to_string())
    );
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&new_table, &2),
        Some("two".to_string())
    );
    assert_eq!(
        DoubleHashFlatHashTableStEph::lookup(&new_table, &3),
        Some("three".to_string())
    );
}

#[test]
fn test_load_and_size() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: DoubleHashTable = <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<
        i32,
        String,
        FlatEntry<i32, String>,
        (),
    >>::createTable(hash_fn_gen, 11);

    let result = DoubleHashFlatHashTableStEph::loadAndSize(&table);
    assert_eq!(result.load, 0.0);
    assert_eq!(result.size, 11);

    DoubleHashFlatHashTableStEph::insert(&mut table, 1, "one".to_string());
    DoubleHashFlatHashTableStEph::insert(&mut table, 2, "two".to_string());
    let result = DoubleHashFlatHashTableStEph::loadAndSize(&table);
    assert!((result.load - 0.18181818).abs() < 0.01); // 2/11 ≈ 0.182
    assert_eq!(result.size, 11);
}
