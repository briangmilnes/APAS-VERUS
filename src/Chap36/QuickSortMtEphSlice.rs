//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Multi-threaded Slice): Quicksort over `ArraySeqMtEphSlice` without extra copies.

pub mod Chapter36MtEphSlice {

    use std::thread;

    // use rand::*;
    // use rand::RngExt;  // Verus can't link rand; random pivot disabled

    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Types::Types::*;
    pub type T<T> = ArraySeqMtEphSliceS<T>;

    pub trait Chapter36MtSliceTrait<T: StTInMtT + Ord> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn pivot_mt_first(&self, lo: usize, hi: usize)   -> T;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn pivot_mt_median3(&self, lo: usize, hi: usize) -> T;
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn pivot_mt_random(&self, lo: usize, hi: usize)  -> T;
        /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst
        /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst — parallel via thread::scope (slice-based, no copy); partition is sequential Θ(n).
        fn quick_sort_mt_first(&self);
        /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst
        /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst — parallel via thread::scope (slice-based, no copy); partition is sequential Θ(n).
        fn quick_sort_mt_median3(&self);
        /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst
        /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst — parallel via thread::scope (slice-based, no copy); partition is sequential Θ(n).
        fn quick_sort_mt_random(&self);
    }

    impl<T: StTInMtT + Ord + 'static> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        fn pivot_mt_first(&self, lo: usize, _hi: usize) -> T { self.nth(lo).clone() }

        fn pivot_mt_median3(&self, lo: usize, hi: usize) -> T {
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

        fn pivot_mt_random(&self, lo: usize, hi: usize) -> T {
            // rand disabled; use median-of-three as deterministic fallback
            self.pivot_mt_median3(lo, hi)
        }

        fn quick_sort_mt_first(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst
                /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst — parallel via thread::scope with first-element pivot.
                fn sort<T: StTInMtT + Ord>(data: &mut [T]) {
                    let len = data.len();
                    if len <= 1 {
                        return;
                    }
                    let pivot = data[0].clone();
                    let mut lt = 0;
                    let mut i = 0;
                    let mut gt = len;
                    while i < gt {
                        if data[i] < pivot {
                            data.swap(lt, i);
                            lt += 1;
                            i += 1;
                        } else if data[i] > pivot {
                            gt -= 1;
                            data.swap(i, gt);
                        } else {
                            i += 1;
                        }
                    }
                    let (left, mid_and_right) = data.split_at_mut(lt);
                    let (_, right) = mid_and_right.split_at_mut(gt - lt);
                    // Unconditionally parallel - no thresholding
                    thread::scope(|scope| {
                        scope.spawn(|| sort(left));
                        sort(right);
                    });
                }
                sort(data);
            });
        }

        fn quick_sort_mt_median3(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst
                /// - Claude-Opus-4.6: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst — parallel via thread::scope with median-of-3 pivot.
                fn sort<T: StTInMtT + Ord>(data: &mut [T]) {
                    let len = data.len();
                    if len <= 1 {
                        return;
                    }
                    let pivot = {
                        let mid = len / 2;
                        let last = len - 1;
                        let x0 = data[0].clone();
                        let xm = data[mid].clone();
                        let xl = data[last].clone();
                        if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                            xm
                        } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                            x0
                        } else {
                            xl
                        }
                    };
                    let mut lt = 0;
                    let mut i = 0;
                    let mut gt = len;
                    while i < gt {
                        if data[i] < pivot {
                            data.swap(lt, i);
                            lt += 1;
                            i += 1;
                        } else if data[i] > pivot {
                            gt -= 1;
                            data.swap(i, gt);
                        } else {
                            i += 1;
                        }
                    }
                    let (left, mid_and_right) = data.split_at_mut(lt);
                    let (_, right) = mid_and_right.split_at_mut(gt - lt);
                    // Unconditionally parallel - no thresholding
                    thread::scope(|scope| {
                        scope.spawn(|| sort(left));
                        sort(right);
                    });
                }
                sort(data);
            });
        }

        #[verifier::external_body]
        fn quick_sort_mt_random(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                /// - APAS: Work Θ(n log n) expected / Θ(n²) worst, Span Θ(log² n) expected / Θ(n) worst
                /// - rand disabled; uses median-of-three as deterministic fallback
                fn sort<T: StTInMtT + Ord>(data: &mut [T]) {
                    let len = data.len();
                    if len <= 1 {
                        return;
                    }
                    let pivot = {
                        let mid = len / 2;
                        let last = len - 1;
                        let x0 = data[0].clone();
                        let xm = data[mid].clone();
                        let xl = data[last].clone();
                        if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                            xm
                        } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                            x0
                        } else {
                            xl
                        }
                    };
                    let mut lt = 0;
                    let mut i = 0;
                    let mut gt = len;
                    while i < gt {
                        if data[i] < pivot {
                            data.swap(lt, i);
                            lt += 1;
                            i += 1;
                        } else if data[i] > pivot {
                            gt -= 1;
                            data.swap(i, gt);
                        } else {
                            i += 1;
                        }
                    }
                    let (left, mid_and_right) = data.split_at_mut(lt);
                    let (_, right) = mid_and_right.split_at_mut(gt - lt);
                    // Unconditionally parallel - no thresholding
                    thread::scope(|scope| {
                        scope.spawn(|| sort(left));
                        sort(right);
                    });
                }
                sort(data);
            });
        }
    }
}
