//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.2: 3D Points using ArraySeqPer via flatten of nested tabulates.

pub mod Algorithm21_2 {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use vstd::prelude::*;

    verus! {

    pub type T = N;

    /// Algorithm 21.2 (3D Points) using ArraySeqPer: flatten of nested tabulates.
    /// Comprehension form: 〈(x,y,z): 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1〉
    /// Generates n³ points total.
    /// Work: Θ(n³), Span: Θ(lg n)
    pub fn points3d_tab_flat(n: N) -> (result: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
        requires
            n as int * n as int * n as int <= usize::MAX as int,
        ensures
            result.seq@.len() == n as int * n as int * n as int,
    {
        if n == 0 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }

        // Build outer array: for each x in 0..n, build a flattened 2D slice.
        let mut outer_rows: Vec<ArraySeqStPerS<Pair<N, Pair<N, N>>>> = Vec::with_capacity(n);
        let mut x: usize = 0;

        proof {
            // n³ <= MAX implies n² <= MAX and n+1 <= MAX (since n >= 1).
            // lemma_mul_increases(x, y) proves y <= x * y when x > 0 and y > 0.
            
            // First, connect (n * n) * n == n * (n * n) via associativity.
            vstd::arithmetic::mul::lemma_mul_is_associative(n as int, n as int, n as int);
            // Now: n * n * n == n * (n * n)
            
            vstd::arithmetic::mul::lemma_mul_increases(n as int, n as int * n as int);
            // Now: n * n <= n * (n * n) == n * n * n <= MAX
            assert(n as int * n as int <= usize::MAX as int);
            
            vstd::arithmetic::mul::lemma_mul_increases(n as int, n as int);
            // Now: n <= n * n <= n * n * n <= MAX
            assert(n as int <= usize::MAX as int);
            
            // n + 1 <= MAX: If n == MAX, then n² >= 2n-1 > MAX (for MAX >= 3), contradiction.
            // Since n >= 1 and n² <= MAX, we have n < MAX, so n + 1 <= MAX.
            if n as int == usize::MAX as int {
                // n * n >= n (from lemma_mul_increases), and n * n <= MAX.
                // But n == MAX implies n * n >= MAX, so n * n == MAX.
                // But for n >= 2, n * n > n, contradiction unless MAX == 1.
                // And MAX >= 2^32 - 1 >> 1.
                assert(n as int * n as int >= n as int);
                assert(n as int * n as int > usize::MAX as int) by {
                    vstd::arithmetic::mul::lemma_mul_strictly_increases(n as int, n as int);
                }
            }
        }

        while x < n
            invariant
                x <= n,
                n > 0,
                outer_rows@.len() == x as int,
                forall|i: int| #![auto] 0 <= i < x as int ==> outer_rows@[i].seq@.len() == n * n,
                n as int * n as int * n as int <= usize::MAX as int,
                n as int * n as int <= usize::MAX as int,
                n + 1 <= usize::MAX,
            decreases n - x
        {
            let row = make_2d_slice(x, n);
            outer_rows.push(row);
            x = x + 1;
        }

        let outer = ArraySeqStPerS::<ArraySeqStPerS<Pair<N, Pair<N, N>>>> { seq: outer_rows };

        proof {
            lemma_sum_lens_uniform(outer.seq@, n as int, (n * n) as int);
            // sum_lens == n * (n*n) == n³
            vstd::arithmetic::mul::lemma_mul_is_associative(n as int, n as int, n as int);
        }

        flatten(&outer)
    }

    // Creates a 2D slice: all (x, y, z) for fixed x, with y in 1..=n, z in 2..=n+1.
    // Returns n*n pairs.
    fn make_2d_slice(x: usize, n: usize) -> (result: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
        requires
            n > 0,
            n as int * n as int <= usize::MAX as int,
            n + 1 <= usize::MAX,
        ensures
            result.seq@.len() == n * n,
    {
        // Build middle array: for each y in 0..n (representing y+1 in 1..=n).
        let mut mid_rows: Vec<ArraySeqStPerS<Pair<N, Pair<N, N>>>> = Vec::with_capacity(n);
        let mut y: usize = 0;

        while y < n
            invariant
                y <= n,
                n > 0,
                mid_rows@.len() == y as int,
                forall|i: int| #![auto] 0 <= i < y as int ==> mid_rows@[i].seq@.len() == n,
                n as int * n as int <= usize::MAX as int,
                n + 1 <= usize::MAX,
            decreases n - y
        {
            let row = make_z_row(x, y, n);
            mid_rows.push(row);
            y = y + 1;
        }

        let mid = ArraySeqStPerS::<ArraySeqStPerS<Pair<N, Pair<N, N>>>> { seq: mid_rows };

        proof {
            lemma_sum_lens_uniform(mid.seq@, n as int, n as int);
        }

        flatten(&mid)
    }

    // Creates a row of z values: (x, y+1, z) for z in 2..=n+1.
    // Returns n pairs.
    fn make_z_row(x: usize, y: usize, n: usize) -> (result: ArraySeqStPerS<Pair<N, Pair<N, N>>>)
        requires
            n > 0,
            y < n,
            n + 1 <= usize::MAX,
        ensures
            result.seq@.len() == n
    {
        ArraySeqStPerS::<Pair<N, Pair<N, N>>>::tabulate(
            &(|z_idx: usize| -> (p: Pair<N, Pair<N, N>>)
                requires
                    z_idx < n,
                    y < n,
                    n + 1 <= usize::MAX,
            {
                Pair(x, Pair(y + 1, z_idx + 2))
            }),
            n,
        )
    }

    // Proves that if all inner sequences have the same length m, then sum_lens equals k * m.
    proof fn lemma_sum_lens_uniform<T>(ss: Seq<ArraySeqStPerS<T>>, k: int, m: int)
        requires
            k >= 0,
            k <= ss.len(),
            forall|i: int| #![auto] 0 <= i < k ==> ss[i].seq@.len() == m,
        ensures
            sum_lens(ss, k) == k * m,
        decreases k,
    {
        if k == 0 {
        } else {
            lemma_sum_lens_uniform(ss, k - 1, m);
            // Direct calls needed for performance.
            vstd::arithmetic::mul::lemma_mul_is_distributive_add_other_way(m, k - 1, 1);
            vstd::arithmetic::mul::lemma_mul_basics(m);
        }
    }

    } // verus!
}
