//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod SSSPResultStEphI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct SSSPResultStEphI64 {
        pub distances: ArraySeqStEphS<i64>,
        pub predecessors: ArraySeqStEphS<usize>,
        pub source: usize,
    }

    // 8. traits

    pub trait SSSPResultStEphI64Trait: Sized {
        fn new(n: usize, source: usize) -> (result: Self)
            requires source < n;

        fn get_distance(&self, v: usize) -> (dist: i64);

        fn set_distance(&mut self, v: usize, dist: i64);

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(&mut self, v: usize, pred: usize);

        fn is_reachable(&self, v: usize) -> (b: bool);

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl SSSPResultStEphI64Trait for SSSPResultStEphI64 {
        #[verifier::external_body]
        fn new(n: usize, source: usize) -> (result: Self)
            ensures
                result.distances@.len() == n as int,
                result.predecessors@.len() == n as int,
                result.source == source,
        {
            let mut dist_seq = ArraySeqStEphS::<i64>::new(n, UNREACHABLE);
            let _ = dist_seq.set(source, 0i64);
            let pred_seq = ArraySeqStEphS::<usize>::new(n, NO_PREDECESSOR);
            SSSPResultStEphI64 {
                distances: dist_seq,
                predecessors: pred_seq,
                source,
            }
        }

        fn get_distance(&self, v: usize) -> (dist: i64) {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        #[verifier::external_body]
        fn set_distance(&mut self, v: usize, dist: i64)
            ensures
                v < old(self).distances@.len() ==> self.distances@ == old(self).distances@.update(v as int, dist),
                v >= old(self).distances@.len() ==> self.distances@ == old(self).distances@,
                self.predecessors@ == old(self).predecessors@,
                self.source == old(self).source,
        {
            if v < self.distances.length() {
                let _ = self.distances.set(v, dist);
            }
        }

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        #[verifier::external_body]
        fn set_predecessor(&mut self, v: usize, pred: usize)
            ensures
                v < old(self).predecessors@.len() ==> self.predecessors@ == old(self).predecessors@.update(v as int, pred),
                v >= old(self).predecessors@.len() ==> self.predecessors@ == old(self).predecessors@,
                self.distances@ == old(self).distances@,
                self.source == old(self).source,
        {
            if v < self.predecessors.length() {
                let _ = self.predecessors.set(v, pred);
            }
        }

        fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v) != UNREACHABLE
        }

        #[verifier::external_body]
        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
            if !self.is_reachable(v) {
                return None;
            }
            let mut path = Vec::new();
            let mut current = v;
            path.push(current);
            while current != self.source {
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR { return None; }
                path.push(pred);
                current = pred;
            }
            path.reverse();
            Some(ArraySeqStPerS::from_vec(path))
        }
    }

    } // verus!
}
