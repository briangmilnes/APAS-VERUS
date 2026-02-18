//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Single-threaded): Quicksort with three pivot strategies over `ArraySeqStEph`.

pub mod Chapter36St {

    use rand::*;
    use rand::RngExt;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    pub type T<T> = ArraySeqStEphS<T>;

    pub trait Chapter36StTrait<T: StT + Ord> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn pivot_st_first(&self, lo: N, hi: N)   -> T;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn pivot_st_random(&self, lo: N, hi: N)  -> T;
        /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(n log n) expected / Θ(n²) worst
        /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span = Work — sequential, no parallelism.
        fn quick_sort_st_first(&mut self);
        /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(n log n) expected / Θ(n²) worst
        /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span = Work — sequential, no parallelism.
        fn quick_sort_st_median3(&mut self);
        /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(n log n) expected / Θ(n²) worst
        /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span = Work — sequential, no parallelism.
        fn quick_sort_st_random(&mut self);
    }

    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {
        fn pivot_st_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {
            let mid = lo + (hi - lo) / 2;
            let x0 = self.nth(lo).clone();
            let xm = self.nth(mid).clone();
            let xl = self.nth(hi - 1).clone();
            if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                xm
            } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                x0
            } else {
                xl
            }
        }
        fn pivot_st_random(&self, lo: N, hi: N) -> T {
            let mut r = rng();
            let idx = r.random_range(lo..hi);
            self.nth(idx).clone()
        }

        fn quick_sort_st_first(&mut self) {
            /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span = Work
            /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span = Work — sequential inner sort with first-element pivot.
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {
                if hi <= lo + 1 {
                    return;
                }
                let pivot = a.nth(lo).clone();
                let mut lt = lo;
                let mut i = lo;
                let mut gt = hi;
                while i < gt {
                    let xi = a.nth(i).clone();
                    if xi < pivot {
                        if lt != i {
                            let xlt = a.nth(lt).clone();
                            let _ = a.set(lt, xi);
                            let _ = a.set(i, xlt);
                        }
                        lt += 1;
                        i += 1;
                    } else if xi > pivot {
                        gt -= 1;
                        let xgt = a.nth(gt).clone();
                        let _ = a.set(i, xgt);
                        let _ = a.set(gt, xi);
                    } else {
                        i += 1;
                    }
                }
                sort(a, lo, lt);
                sort(a, gt, hi);
            }
            let n = self.length();
            sort(self, 0, n);
        }

        fn quick_sort_st_median3(&mut self) {
            /// - APAS: Work Θ(1), Span Θ(1)
            /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {
                let mid = lo + (hi - lo) / 2;
                let x0 = a.nth(lo).clone();
                let xm = a.nth(mid).clone();
                let xl = a.nth(hi - 1).clone();
                if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                    xm
                } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                    x0
                } else {
                    xl
                }
            }
            /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span = Work
            /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span = Work — sequential inner sort with median-of-3 pivot.
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {
                if hi <= lo + 1 {
                    return;
                }
                let pivot = median3(a, lo, hi);
                let mut lt = lo;
                let mut i = lo;
                let mut gt = hi;
                while i < gt {
                    let xi = a.nth(i).clone();
                    if xi < pivot {
                        if lt != i {
                            let xlt = a.nth(lt).clone();
                            let _ = a.set(lt, xi);
                            let _ = a.set(i, xlt);
                        }
                        lt += 1;
                        i += 1;
                    } else if xi > pivot {
                        gt -= 1;
                        let xgt = a.nth(gt).clone();
                        let _ = a.set(i, xgt);
                        let _ = a.set(gt, xi);
                    } else {
                        i += 1;
                    }
                }
                sort(a, lo, lt);
                sort(a, gt, hi);
            }
            let n = self.length();
            sort(self, 0, n);
        }

        fn quick_sort_st_random(&mut self) {
            /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span = Work
            /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span = Work — sequential inner sort with random pivot.
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {
                if hi <= lo + 1 {
                    return;
                }
                let mut r = rng();
                let idx = r.random_range(lo..hi);
                let pivot = a.nth(idx).clone();
                let mut lt = lo;
                let mut i = lo;
                let mut gt = hi;
                while i < gt {
                    let xi = a.nth(i).clone();
                    if xi < pivot {
                        if lt != i {
                            let xlt = a.nth(lt).clone();
                            let _ = a.set(lt, xi);
                            let _ = a.set(i, xlt);
                        }
                        lt += 1;
                        i += 1;
                    } else if xi > pivot {
                        gt -= 1;
                        let xgt = a.nth(gt).clone();
                        let _ = a.set(i, xgt);
                        let _ = a.set(gt, xi);
                    } else {
                        i += 1;
                    }
                }
                sort(a, lo, lt);
                sort(a, gt, hi);
            }
            let n = self.length();
            sort(self, 0, n);
        }
    }
}
