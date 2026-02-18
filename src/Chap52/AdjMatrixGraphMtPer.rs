//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded).
//! PARALLEL complement operation.

pub mod AdjMatrixGraphMtPer {

    use std::thread;

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtPer {
        pub matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        pub n: N,
    }

    // 5. view impls

    impl View for AdjMatrixGraphMtPer {
        type V = Seq<Seq<bool>>;
        open spec fn view(&self) -> Self::V {
            self.matrix@
        }
    }

    // 8. traits

    pub trait AdjMatrixGraphMtPerTrait: Sized {
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

    // 9. impls

    impl AdjMatrixGraphMtPerTrait for AdjMatrixGraphMtPer {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential creation of n×n false matrix.
        #[verifier::external_body]
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

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — stored field.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.n }

        /// - APAS: Work Θ(n²), Span Θ(1) [Cost Spec 52.6, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(lg n) — parallel divide-and-conquer over rows and columns.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            count_edges_parallel(&self.matrix)
        }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — parallel divide-and-conquer over columns.
        #[verifier::external_body]
        fn out_neighbors(&self, u: N) -> ArraySeqMtPerS<N> {
            if u >= self.n {
                return ArraySeqMtPerS::empty();
            }
            let row = self.matrix.nth(u);
            collect_neighbors_parallel(row, 0, self.n)
        }

        /// - APAS: Work Θ(n), Span Θ(lg n) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — parallel divide-and-conquer; agrees with APAS.
        #[verifier::external_body]
        fn out_degree(&self, u: N) -> N {
            if u >= self.n {
                return 0;
            }
            let row = self.matrix.nth(u);
            count_row_parallel(row)
        }

        /// - APAS: Work Θ(n²), Span Θ(1) [Exercise 52.6]
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(lg n) — parallel divide-and-conquer over rows and columns.
        #[verifier::external_body]
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

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(lg n) — parallel divide-and-conquer over rows.
    #[verifier::external_body]
    fn count_edges_parallel(matrix: &ArraySeqMtPerS<ArraySeqMtPerS<bool>>) -> (result: N) {
        let n = matrix.length();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return count_row_parallel(matrix.nth(0));
        }

        let mid = n / 2;
        let left_matrix = matrix.subseq_copy(0, mid);
        let right_matrix = matrix.subseq_copy(mid, n - mid);

        let left_handle = thread::spawn(move || count_edges_parallel(&left_matrix));
        let right_count = count_edges_parallel(&right_matrix);
        let left_count = left_handle.join().unwrap();

        left_count + right_count
    }

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — parallel divide-and-conquer over columns.
    #[verifier::external_body]
    fn count_row_parallel(row: &ArraySeqMtPerS<bool>) -> (result: N) {
        let n = row.length();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return if *row.nth(0) { 1 } else { 0 };
        }

        let mid = n / 2;
        let left_row = row.subseq_copy(0, mid);
        let right_row = row.subseq_copy(mid, n - mid);

        let left_handle = thread::spawn(move || count_row_parallel(&left_row));
        let right_count = count_row_parallel(&right_row);
        let left_count = left_handle.join().unwrap();

        left_count + right_count
    }

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — parallel divide-and-conquer + append.
    #[verifier::external_body]
    fn collect_neighbors_parallel(
        row: &ArraySeqMtPerS<bool>,
        start: N,
        end: N,
    ) -> (result: ArraySeqMtPerS<N>) {
        if start >= end {
            return ArraySeqMtPerS::empty();
        }
        if end - start == 1 {
            return if *row.nth(start) {
                ArraySeqMtPerS::from_vec(vec![start])
            } else {
                ArraySeqMtPerS::empty()
            };
        }

        let mid = (start + end) / 2;
        let row_clone = row.clone();

        let left_handle =
            thread::spawn(move || collect_neighbors_parallel(&row_clone, start, mid));
        let right_result = collect_neighbors_parallel(row, mid, end);
        let left_result = left_handle.join().unwrap();

        ArraySeqMtPerS::append(&left_result, &right_result)
    }

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(lg n) — parallel divide-and-conquer over rows and columns.
    #[verifier::external_body]
    fn complement_matrix_parallel(
        matrix: &ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        n: N,
    ) -> (result: ArraySeqMtPerS<ArraySeqMtPerS<bool>>) {
        complement_rows_parallel(matrix, 0, n, n)
    }

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(k × n), Span Θ(lg k × n) — parallel over row range k.
    #[verifier::external_body]
    fn complement_rows_parallel(
        matrix: &ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        start_row: N,
        end_row: N,
        n: N,
    ) -> (result: ArraySeqMtPerS<ArraySeqMtPerS<bool>>) {
        if start_row >= end_row {
            return ArraySeqMtPerS::empty();
        }
        if end_row - start_row == 1 {
            let i = start_row;
            let row = matrix.nth(i);
            let comp_row = complement_row_parallel(row, i, n);
            return ArraySeqMtPerS::from_vec(vec![comp_row]);
        }

        let mid = (start_row + end_row) / 2;
        let matrix_clone = matrix.clone();

        let left_handle =
            thread::spawn(move || complement_rows_parallel(&matrix_clone, start_row, mid, n));
        let right_result = complement_rows_parallel(matrix, mid, end_row, n);
        let left_result = left_handle.join().unwrap();

        ArraySeqMtPerS::append(&left_result, &right_result)
    }

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(lg n) — delegates to parallel column complementing.
    #[verifier::external_body]
    fn complement_row_parallel(row: &ArraySeqMtPerS<bool>, row_idx: N, n: N) -> (result: ArraySeqMtPerS<bool>) {
        complement_columns_parallel(row, row_idx, 0, n)
    }

    /// - APAS: N/A — parallel helper not in cost table.
    /// - Claude-Opus-4.6: Work Θ(k), Span Θ(lg k) — parallel divide-and-conquer over column range k.
    #[verifier::external_body]
    fn complement_columns_parallel(
        row: &ArraySeqMtPerS<bool>,
        row_idx: N,
        start: N,
        end: N,
    ) -> (result: ArraySeqMtPerS<bool>) {
        if start >= end {
            return ArraySeqMtPerS::empty();
        }
        if end - start == 1 {
            let j = start;
            let val = if row_idx == j {
                false
            } else {
                !*row.nth(j)
            };
            return ArraySeqMtPerS::from_vec(vec![val]);
        }

        let mid = (start + end) / 2;
        let row_clone = row.clone();

        let left_handle =
            thread::spawn(move || complement_columns_parallel(&row_clone, row_idx, start, mid));
        let right_result = complement_columns_parallel(row, row_idx, mid, end);
        let left_result = left_handle.join().unwrap();

        ArraySeqMtPerS::append(&left_result, &right_result)
    }

    } // verus!
}
