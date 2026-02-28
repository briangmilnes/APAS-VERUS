//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for Chapter 36 quicksort: St vs Mt, all three pivot strategies.
//! Uses shuffled data to avoid O(n) recursion depth on first-element pivot.

use std::time::Instant;

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap36::QuickSortStEph::QuickSortStEph::*;
use apas_verus::Chap36::QuickSortMtEph::QuickSortMtEph::*;

/// Pseudo-shuffle using a simple LCG to avoid pathological pivot choices.
fn make_shuffled(n: usize) -> Vec<i64> {
    let mut v: Vec<i64> = (0..n as i64).collect();
    let mut rng: u64 = 12345;
    for i in (1..n).rev() {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (rng >> 33) as usize % (i + 1);
        v.swap(i, j);
    }
    v
}

fn bench<F: FnOnce()>(label: &str, f: F) -> std::time::Duration {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("  {label:30} {elapsed:>10.2?}");
    elapsed
}

#[test]
fn bench_quicksort_st_vs_mt() {
    for &n in &[100, 500, 2000] {
        println!("n = {n}");
        let data = make_shuffled(n);

        bench("St first", || {
            let mut a = ArraySeqStEphS::from_vec(data.clone());
            ArraySeqStEphS::quick_sort_first(&mut a);
        });
        bench("St median3", || {
            let mut a = ArraySeqStEphS::from_vec(data.clone());
            ArraySeqStEphS::quick_sort_median3(&mut a);
        });
        bench("St random", || {
            let mut a = ArraySeqStEphS::from_vec(data.clone());
            ArraySeqStEphS::quick_sort_random(&mut a);
        });
        bench("Mt first", || {
            let mut a = ArraySeqMtEphS::from_vec(data.clone());
            ArraySeqMtEphS::quick_sort_first(&mut a);
        });
        bench("Mt median3", || {
            let mut a = ArraySeqMtEphS::from_vec(data.clone());
            ArraySeqMtEphS::quick_sort_median3(&mut a);
        });
        bench("Mt random", || {
            let mut a = ArraySeqMtEphS::from_vec(data.clone());
            ArraySeqMtEphS::quick_sort_random(&mut a);
        });
        println!();
    }
}
