//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Multi-threaded Slice): Quicksort over `ArraySeqMtEphSlice` without extra copies.

pub mod Chapter36MtEphSlice {

    use std::thread;

    use rand::*;
    use rand::RngExt;

    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Types::Types::*;
    pub type T<T> = ArraySeqMtEphSliceS<T>;

    pub trait Chapter36MtSliceTrait<T: StTInMtT + Ord> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - constant time pivot selection
        fn pivot_mt_first(&self, lo: N, hi: N)   -> T;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - constant time median-of-3
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - constant time random selection
        fn pivot_mt_random(&self, lo: N, hi: N)  -> T;
        /// APAS: Work Θ(n log n) expected, Θ(n²) worst, Span Θ(log² n) expected, Θ(n) worst
        /// claude-4-sonet: Work Θ(n log n) expected, Θ(n²) worst, Span Θ(log² n) expected, Θ(n) worst, Parallelism Θ(n/log n) expected - parallel divide-and-conquer with unconditional thread spawning (slice-based)
        fn quick_sort_mt_first(&self);
        /// APAS: Work Θ(n log n) expected, Θ(n²) worst, Span Θ(log² n) expected, Θ(n) worst
        /// claude-4-sonet: Work Θ(n log n) expected, Θ(n²) worst, Span Θ(log² n) expected, Θ(n) worst, Parallelism Θ(n/log n) expected - parallel divide-and-conquer with median-of-3 pivot (slice-based)
        fn quick_sort_mt_median3(&self);
        /// APAS: Work Θ(n log n) expected, Θ(n²) worst, Span Θ(log² n) expected, Θ(n) worst
        /// claude-4-sonet: Work Θ(n log n) expected, Θ(n²) worst, Span Θ(log² n) expected, Θ(n) worst, Parallelism Θ(n/log n) expected - parallel divide-and-conquer with random pivot (slice-based)
        fn quick_sort_mt_random(&self);
    }

    impl<T: StTInMtT + Ord + 'static> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth(lo).clone() }

        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {
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

        fn pivot_mt_random(&self, lo: N, hi: N) -> T {
            let mut r = rng();
            let idx = r.random_range(lo..hi);
            self.nth(idx).clone()
        }

        fn quick_sort_mt_first(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
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

        fn quick_sort_mt_random(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                fn sort<T: StTInMtT + Ord>(data: &mut [T]) {
                    let len = data.len();
                    if len <= 1 {
                        return;
                    }
                    let mut r = rng();
                    let idx = r.random_range(0..len);
                    let pivot = data[idx].clone();
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
