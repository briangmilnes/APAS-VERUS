//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded).
//! PARALLEL complement operation.

pub mod AdjMatrixGraphMtPer {

    use std::sync::Arc;
    use std::thread;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtPer {
        matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        n: N,
    }

    pub trait AdjMatrixGraphMtPerTrait {
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn new(n: N)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)         -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn num_edges(&self)            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn out_neighbors(&self, u: N)  -> ArraySeqMtPerS<N>;
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn out_degree(&self, u: N)     -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(log n), Parallelism Θ(n²/log n)
        fn complement(&self)           -> Self
        where
            bool: 'static;
    }

    impl AdjMatrixGraphMtPerTrait for AdjMatrixGraphMtPer {
        fn new(n: N) -> Self {
            let false_row = ArraySeqMtPerS::from_vec(vec![false; n]);
            let mut matrix_rows = Vec::with_capacity(n);
            for _ in 0..n {
                matrix_rows.push(false_row.clone());
            }
            AdjMatrixGraphMtPer {
                matrix: ArraySeqMtPerS::from_vec(matrix_rows),
                n,
            }
        }

        fn num_vertices(&self) -> N { self.n }

        fn num_edges(&self) -> N {
            // Parallel: divide-and-conquer over rows
            // Work: Θ(n²), Span: Θ(log n)
            count_edges_parallel(&self.matrix)
        }

        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> {
            if u >= self.n {
                return ArraySeqMtPerS::empty();
            }
            // Parallel: divide-and-conquer over columns
            // Work: Θ(n), Span: Θ(log n)
            let row = self.matrix.nth(u);
            collect_neighbors_parallel(row, 0, self.n)
        }

        fn out_degree(&self, u: N) -> N {
            if u >= self.n {
                return 0;
            }
            // Parallel: divide-and-conquer over row
            // Work: Θ(n), Span: Θ(log n)
            let row = self.matrix.nth(u);
            count_row_parallel(row)
        }

        // Exercise 52.6: Parallel complement with Θ(log n) span
        // Work: Θ(n²), Span: Θ(log n) 
        fn complement(&self) -> Self
        where
            bool: 'static,
        {
            let n = self.n;
            let new_matrix = complement_matrix_parallel(&self.matrix, n);
            AdjMatrixGraphMtPer {
                matrix: new_matrix,
                n: self.n,
            }
        }
    }

    // ============================================================================
    // Parallel Helper Functions (using thread::spawn/join, no generic closures)
    // ============================================================================

    /// Count total edges in matrix (parallel divide-and-conquer over rows)
    /// Work: Θ(n²), Span: Θ(log n)
    fn count_edges_parallel(matrix: &ArraySeqMtPerS<ArraySeqMtPerS<bool>>) -> N {
        let n = matrix.length();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            // Base case: count true values in single row
            return count_row_parallel(matrix.nth(0));
        }

        // Divide-and-conquer: split rows in half
        let mid = n / 2;
        let left_matrix = matrix.subseq_copy(0, mid);
        let right_matrix = matrix.subseq_copy(mid, n - mid);

        let left_handle = thread::spawn(move || count_edges_parallel(&left_matrix));
        let right_count = count_edges_parallel(&right_matrix);
        let left_count = left_handle.join().unwrap();

        left_count + right_count
    }

    /// Count true values in a single row (parallel divide-and-conquer)
    /// Work: Θ(n), Span: Θ(log n)
    fn count_row_parallel(row: &ArraySeqMtPerS<bool>) -> N {
        let n = row.length();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return if *row.nth(0) { 1 } else { 0 };
        }

        // Divide columns in half
        let mid = n / 2;
        let left_row = row.subseq_copy(0, mid);
        let right_row = row.subseq_copy(mid, n - mid);

        let left_handle = thread::spawn(move || count_row_parallel(&left_row));
        let right_count = count_row_parallel(&right_row);
        let left_count = left_handle.join().unwrap();

        left_count + right_count
    }

    /// Collect neighbor indices where row[i] == true (parallel)
    /// Work: Θ(n), Span: Θ(log n)
    fn collect_neighbors_parallel(
        row: &ArraySeqMtPerS<bool>,
        start: N,
        end: N,
    ) -> ArraySeqMtPerS<N> {
        if start >= end {
            return ArraySeqMtPerS::empty();
        }
        if end - start == 1 {
            // Base case: single column
            return if *row.nth(start) {
                ArraySeqMtPerS::from_vec(vec![start])
            } else {
                ArraySeqMtPerS::empty()
            };
        }

        // Divide columns in half
        let mid = (start + end) / 2;
        let row_clone = row.clone();

        let left_handle =
            thread::spawn(move || collect_neighbors_parallel(&row_clone, start, mid));
        let right_result = collect_neighbors_parallel(row, mid, end);
        let left_result = left_handle.join().unwrap();

        ArraySeqMtPerS::append(&left_result, &right_result)
    }

    /// Complement entire matrix (parallel over rows and columns)
    /// Work: Θ(n²), Span: Θ(log n)
    fn complement_matrix_parallel(
        matrix: &ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        n: N,
    ) -> ArraySeqMtPerS<ArraySeqMtPerS<bool>> {
        complement_rows_parallel(matrix, 0, n, n)
    }

    /// Complement a range of rows (parallel divide-and-conquer)
    fn complement_rows_parallel(
        matrix: &ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        start_row: N,
        end_row: N,
        n: N,
    ) -> ArraySeqMtPerS<ArraySeqMtPerS<bool>> {
        if start_row >= end_row {
            return ArraySeqMtPerS::empty();
        }
        if end_row - start_row == 1 {
            // Base case: complement single row
            let i = start_row;
            let row = matrix.nth(i);
            let comp_row = complement_row_parallel(row, i, n);
            return ArraySeqMtPerS::from_vec(vec![comp_row]);
        }

        // Divide rows in half
        let mid = (start_row + end_row) / 2;
        let matrix_clone = matrix.clone();

        let left_handle =
            thread::spawn(move || complement_rows_parallel(&matrix_clone, start_row, mid, n));
        let right_result = complement_rows_parallel(matrix, mid, end_row, n);
        let left_result = left_handle.join().unwrap();

        ArraySeqMtPerS::append(&left_result, &right_result)
    }

    /// Complement a single row (parallel over columns)
    fn complement_row_parallel(row: &ArraySeqMtPerS<bool>, row_idx: N, n: N) -> ArraySeqMtPerS<bool> {
        complement_columns_parallel(row, row_idx, 0, n)
    }

    /// Complement a range of columns in a row (parallel divide-and-conquer)
    fn complement_columns_parallel(
        row: &ArraySeqMtPerS<bool>,
        row_idx: N,
        start: N,
        end: N,
    ) -> ArraySeqMtPerS<bool> {
        if start >= end {
            return ArraySeqMtPerS::empty();
        }
        if end - start == 1 {
            // Base case: single column
            let j = start;
            let val = if row_idx == j {
                false // No self-loops
            } else {
                !*row.nth(j) // Complement the edge
            };
            return ArraySeqMtPerS::from_vec(vec![val]);
        }

        // Divide columns in half
        let mid = (start + end) / 2;
        let row_clone = row.clone();

        let left_handle =
            thread::spawn(move || complement_columns_parallel(&row_clone, row_idx, start, mid));
        let right_result = complement_columns_parallel(row, row_idx, mid, end);
        let left_result = left_handle.join().unwrap();

        ArraySeqMtPerS::append(&left_result, &right_result)
    }
}
