//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (ephemeral, multi-threaded).

pub mod AdjMatrixGraphMtEph {

    use std::sync::Arc;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtEph {
        matrix: ArraySeqMtEphS<ArraySeqMtEphS<bool>>,
        n: N,
    }

    pub trait AdjMatrixGraphMtEphTrait {
        /// claude-4-sonet: Work Θ(n²), Span Θ(1)
        fn new(n: N)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)         -> N;
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        fn num_edges(&self)            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn out_neighbors(&self, u: N)  -> ArraySeqMtEphS<N>;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn out_degree(&self, u: N)     -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1) with locking
        fn set_edge(&mut self, u: N, v: N, exists: B);
        /// claude-4-sonet: Work Θ(n²), Span Θ(1)
        fn complement(&self)           -> Self;
    }

    impl AdjMatrixGraphMtEphTrait for AdjMatrixGraphMtEph {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential creation of n×n false matrix.
        fn new(n: N) -> Self {
            let false_row = ArraySeqMtEphS::from_vec(vec![false; n]);
            let mut matrix_rows = Vec::with_capacity(n);
            for _ in 0..n {
                matrix_rows.push(false_row.clone());
            }
            AdjMatrixGraphMtEph {
                matrix: ArraySeqMtEphS::from_vec(matrix_rows),
                n,
            }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — stored field.
        fn num_vertices(&self) -> N { self.n }

        /// - APAS: Work Θ(n²), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential double loop; span not parallel despite Mt type.
        fn num_edges(&self) -> N {
            let mut count = 0;
            for i in 0..self.n {
                let row = self.matrix.nth(i).clone();
                for j in 0..self.n {
                    if row.nth(j).clone() {
                        count += 1;
                    }
                }
            }
            count
        }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential row scan; span not parallel despite Mt type.
        fn out_neighbors(&self, u: N) -> ArraySeqMtEphS<N> {
            if u >= self.n {
                return ArraySeqMtEphS::empty();
            }
            let row = self.matrix.nth(u).clone();
            let mut neighbors = Vec::new();
            for v in 0..self.n {
                if row.nth(v).clone() {
                    neighbors.push(v);
                }
            }
            ArraySeqMtEphS::from_vec(neighbors)
        }

        /// - APAS: Work Θ(n), Span Θ(lg n) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential row count; span not logarithmic despite Mt type.
        fn out_degree(&self, u: N) -> N {
            if u >= self.n {
                return 0;
            }
            let row = self.matrix.nth(u).clone();
            let mut count = 0;
            for v in 0..self.n {
                if row.nth(v).clone() {
                    count += 1;
                }
            }
            count
        }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.6]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — ephemeral in-place array set; better than APAS persistent bound.
        fn set_edge(&mut self, u: N, v: N, exists: B) {
            if u >= self.n || v >= self.n {
                return;
            }
            let mut row = self.matrix.nth(u).clone();
            let _ = row.set(v, exists);
            let _ = self.matrix.set(u, row);
        }

        /// - APAS: Work Θ(n²), Span Θ(1) [Exercise 52.6]
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — sequential double loop; span not parallel despite Mt type.
        fn complement(&self) -> Self {
            let mut new_matrix_vec = Vec::with_capacity(self.n);
            for i in 0..self.n {
                let row = self.matrix.nth(i).clone();
                let mut new_row_vec = Vec::with_capacity(self.n);
                for j in 0..self.n {
                    if i == j {
                        new_row_vec.push(false);
                    } else {
                        new_row_vec.push(!*row.nth(j));
                    }
                }
                new_matrix_vec.push(ArraySeqMtEphS::from_vec(new_row_vec));
            }
            AdjMatrixGraphMtEph {
                matrix: ArraySeqMtEphS::from_vec(new_matrix_vec),
                n: self.n,
            }
        }
    }
}
