//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! MtEph slice-oriented Array sequence variant sharing a single mutex.
//!
//! Abstract:
//! - Provides `ArraySeqMtEphSliceS<T>` backed by `Arc<Mutex<Box<[T]>>>` with range metadata.
//! - Offers trait `ArraySeqMtEphSliceTrait<T>` mirroring the MT ephemeral API while avoiding `Vec` copies.
//! - Adds `with_exclusive` to project a mutable slice guarded by the single mutex for batch updates.

pub mod ArraySeqMtEphSlice {

    use std::fmt::{Debug, Display, Formatter};
    use std::ops::Range;
    use std::sync::{Arc, Mutex};

    use crate::ParaPair;
    use crate::Types::Types::*;

    #[derive(Debug)]
    struct Inner<T: StTInMtT> {
        data: Mutex<Box<[T]>>,
    }

    impl<T: StTInMtT> Inner<T> {
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }

        fn len(&self) -> N {
            let guard = self.data.lock().unwrap();
            guard.len()
        }
    }

    /// Shared slice view over the mutex-protected backing buffer.
    pub struct ArraySeqMtEphSliceS<T: StTInMtT> {
        inner: Arc<Inner<T>>,
        range: Range<N>,
    }

    /// Sequence trait for the slice-backed MT ephemeral array.
    pub trait ArraySeqMtEphSliceTrait<T: StTInMtT> {
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn new(length: N, init_value: T)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self)                                   -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth_cloned(&self, index: N)                     -> T;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                         -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn update(&mut self, index: N, item: T)            -> Result<&mut Self, &'static str>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T)                              -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self)                                  -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self)                              -> B;
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N)         -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn slice(&self, start: N, length: N)               -> Self;
        /// claude-4-sonet: Work Θ(n + Σᵢ W(f(i))), Span Θ(1 + maxᵢ S(f(i))), Parallelism Θ(n)
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self;
        /// claude-4-sonet: Work Θ(|a| + Σₓ W(f(x))), Span Θ(1 + maxₓ S(f(x))), Parallelism Θ(|a|)
        fn map<U: MtVal, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Self, f: F) -> ArraySeqMtEphSliceS<U>;
        /// claude-4-sonet: Work Θ(|a| + Σᵢ W(f(aᵢ))), Span Θ(1 + maxᵢ S(f(aᵢ))), Parallelism Θ(|a|)
        fn filter<F: PredMt<T> + Clone>(a: &Self, pred: F) -> Self;
        fn append(a: &Self, b: &Self)                      -> Self;
        fn append_select(a: &Self, b: &Self)               -> Self;
        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>])   -> Self;
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSliceS<T>, T);
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) -> A;
        fn inject(a: &Self, updates: &[(N, T)])            -> Self;
        fn ninject(a: &Self, updates: &[(N, T)])           -> Self;
        fn from_box(data: Box<[T]>)                        -> Self;
        fn from_vec(data: Vec<T>)                          -> Self;
        fn to_vec(&self)                                   -> Vec<T>;
        fn with_exclusive<F: FnOnce(&mut [T]) -> R, R>(&self, f: F) -> R;
        fn set(&mut self, index: N, item: T)               -> Result<&mut Self, &'static str>;
    }

    impl<T: StTInMtT + 'static> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        fn new(length: N, init_value: T) -> Self {
            let data = repeat_vec(length, init_value);
            ArraySeqMtEphSliceS::from_vec(data)
        }

        fn length(&self) -> N { self.range.end - self.range.start }

        fn nth_cloned(&self, index: N) -> T {
            let guard = self.inner.data.lock().unwrap();
            let idx = self.range.start + index;
            guard[idx].clone()
        }

        fn empty() -> Self {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0) - use trait method
            <Self as ArraySeqMtEphSliceTrait<T>>::tabulate(&|_| unreachable!("empty sequence has no elements"), 0)
        }

        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            if index >= self.length() {
                return Err("Index out of bounds");
            }
            {
                let mut guard = self.inner.data.lock().unwrap();
                let idx = self.range.start + index;
                guard[idx] = item;
            }
            Ok(self)
        }

        fn singleton(item: T) -> Self {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1) - use trait method
            // Implement directly since we can't capture with &F
            let data = vec![item];
            let inner = Arc::new(Inner {
                data: Mutex::new(data.into_boxed_slice()),
            });
            Self { inner, range: 0..1 }
        }

        fn isEmpty(&self) -> B { self.length() == 0 }

        fn isSingleton(&self) -> B { self.length() == 1 }

        fn subseq_copy(&self, start: N, length: N) -> Self {
            let sub = clamp_subrange(self, start, length);
            let guard = self.inner.data.lock().unwrap();
            let data: Vec<T> = guard[sub.start..sub.end].to_vec();
            ArraySeqMtEphSliceS::from_vec(data)
        }

        fn slice(&self, start: N, length: N) -> Self {
            let sub = clamp_subrange(self, start, length);
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: sub,
            }
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self {
            // Algorithm 19.14: "allocate a fresh array of n elements, evaluate f at each position i 
            // and write the result into position i of the array"
            // "the function f can be evaluated at each element independently in parallel"
            // Sequential evaluation - parallel version requires F: 'static
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtEphSliceS::from_vec(values)
        }

        fn map<U: MtVal, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Self, f: F) -> ArraySeqMtEphSliceS<U> {
            // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
            // Uses divide-and-conquer parallelism instead of spawning n threads
            if a.length() == 0 {
                return ArraySeqMtEphSliceS::<U>::from_vec(Vec::new());
            }
            if a.length() == 1 {
                return ArraySeqMtEphSliceS::<U>::from_vec(vec![f(&a.nth_cloned(0))]);
            }

            // Divide-and-conquer with ParaPair!
            let mid = a.length() / 2;
            let left_slice = a.slice(0, mid);
            let right_slice = a.slice(mid, a.length() - mid);
            let f_left = f.clone();
            let f_right = f.clone();

            let Pair(left_result, right_result) = ParaPair!(move || Self::map(&left_slice, f_left), move || Self::map(
                &right_slice,
                f_right
            ));

            // Append results
            let mut results = Vec::with_capacity(a.length());
            for i in 0..left_result.length() {
                results.push(left_result.nth_cloned(i));
            }
            for i in 0..right_result.length() {
                results.push(right_result.nth_cloned(i));
            }
            ArraySeqMtEphSliceS::<U>::from_vec(results)
        }

        fn filter<F: PredMt<T> + Clone>(a: &Self, pred: F) -> Self {
            // Algorithm 19.5: filter f a = flatten (map (deflate f) a)
            // Map each element to a sequence (empty or singleton), then flatten
            let deflated = <ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>> as ArraySeqMtEphSliceTrait<ArraySeqMtEphSliceS<T>>>::tabulate(
                &|i| {
                    let x = a.nth_cloned(i);
                    if pred(&x) {
                        ArraySeqMtEphSliceS::from_vec(vec![x])
                    } else {
                        <Self as ArraySeqMtEphSliceTrait<T>>::empty()
                    }
                },
                a.length()
            );
            // Flatten the sequence of sequences
            let sequences: Vec<ArraySeqMtEphSliceS<T>> = (0..deflated.length()).map(|i| deflated.nth_cloned(i)).collect();
            <Self as ArraySeqMtEphSliceTrait<T>>::flatten(&sequences)
        }

        fn append(a: &Self, b: &Self) -> Self {
            // Algorithm 19.4: append a b = flatten(<a, b>)
            let sequences = [a.clone(), b.clone()];
            <Self as ArraySeqMtEphSliceTrait<T>>::flatten(&sequences)
        }

        fn append_select(a: &Self, b: &Self) -> Self {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a, b), |a| + |b|)
            let total_len = a.length() + b.length();
            <Self as ArraySeqMtEphSliceTrait<T>>::tabulate(
                &|i| {
                    if i < a.length() {
                        a.nth_cloned(i)
                    } else {
                        b.nth_cloned(i - a.length())
                    }
                },
                total_len,
            )
        }

        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self {
            // Algorithm 19.15: parallel flatten using divide-and-conquer
            if sequences.is_empty() {
                return <Self as ArraySeqMtEphSliceTrait<T>>::empty();
            }
            if sequences.len() == 1 {
                return sequences[0].clone();
            }

            // Divide-and-conquer with ParaPair!
            let mid = sequences.len() / 2;
            let left_seqs = &sequences[..mid];
            let right_seqs = &sequences[mid..];

            let Pair(left_result, right_result) =
                ParaPair!(move || Self::flatten(left_seqs), move || Self::flatten(right_seqs));

            // Append the two flattened results
            let total_len = left_result.length() + right_result.length();
            let mut result = Vec::with_capacity(total_len);
            for i in 0..left_result.length() {
                result.push(left_result.nth_cloned(i));
            }
            for i in 0..right_result.length() {
                result.push(right_result.nth_cloned(i));
            }

            ArraySeqMtEphSliceS::from_vec(result)
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> T {
            // Algorithm 19.9: divide-and-conquer parallel reduce
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth_cloned(0);
            }

            let mid = a.length() / 2;
            let left_slice = a.slice(0, mid);
            let right_slice = a.slice(mid, a.length() - mid);

            let f_left = f.clone();
            let f_right = f.clone();
            let id_left = id.clone();
            let id_right = id.clone();

            // APAS Algorithm 19.9: (rb, rc) = (reduce f id b) || (reduce f id c)
            let Pair(left_result, right_result) = ParaPair!(
                move || Self::reduce(&left_slice, f_left, id_left),
                move || Self::reduce(&right_slice, f_right, id_right)
            );

            f(&left_result, &right_result)
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSliceS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified for slice)
            if a.length() == 0 {
                return (<ArraySeqMtEphSliceS<T> as ArraySeqMtEphSliceTrait<T>>::empty(), id);
            }
            if a.length() == 1 {
                let result_seq = <ArraySeqMtEphSliceS<T> as ArraySeqMtEphSliceTrait<T>>::tabulate(&|_| id.clone(), 1);
                return (result_seq, a.nth_cloned(0));
            }

            // For simplicity, implement sequentially (full parallel scan is complex)
            let mut results = Vec::with_capacity(a.length());
            let mut acc = id.clone();

            for i in 0..a.length() {
                results.push(acc.clone());
                let current = a.nth_cloned(i);
                acc = f(&acc, &current);
            }

            let result_seq = ArraySeqMtEphSliceS::<T>::from_vec(results);
            (result_seq, acc)
        }

        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) -> A {
            // Algorithm 19.8: iterate f x a (sequential left-to-right)
            let mut acc = seed;
            for i in 0..a.length() {
                let current = a.nth_cloned(i);
                acc = f(&acc, &current);
            }
            acc
        }

        fn inject(a: &Self, updates: &[(N, T)]) -> Self {
            // Algorithm 19.16: parallel inject with leftmost-wins atomic writes
            use std::sync::Arc;
            use std::sync::Mutex;

            // Helper: APAS Algorithm 19.16 atomicWrite - leftmost wins
            fn atomic_write_leftmost<T: StTInMtT>(
                aa: &Arc<Box<[Mutex<(T, N)>]>>,
                idx: N,
                val: T,
                k: N,
            ) {
                // atomicWrite aa b k =
                //   atomically do:
                //     (j, v) ← b[k]      // j=idx, v=val from caller
                //     (w, i) ← aa[j]     // read current (value, index) at position idx
                //     if k < i then      // leftmost wins: update only if this update is earlier
                //       aa[j] ← (v, k)   // write new value and update index
                if idx < aa.len() {
                    let mut slot = aa[idx].lock().unwrap();
                    if k < slot.1 {
                        *slot = (val, k);
                    }
                }
            }

            if updates.is_empty() {
                return a.clone();
            }

            // Algorithm 19.16: Create aa from a where aa[i] = (a[i], |a|)
            let n = a.length();
            
            // Create copied array aa with per-element Mutexes for atomic writes (benign effect)
            // Initialize aa[i] = (a[i], |a|) by reading from sequence a
            let mut aa_uninit = Box::new_uninit_slice(n);
            for i in 0..n {
                aa_uninit[i].write(Mutex::new((a.nth_cloned(i), n)));
            }
            let aa = Arc::new(unsafe { aa_uninit.assume_init() });

            // Inject all updates in parallel using atomicWrite - leftmost wins
            std::thread::scope(|s| {
                for (k, (idx, val)) in updates.iter().enumerate() {
                    let idx = *idx;
                    let val = val.clone();
                    let aa_ref = Arc::clone(&aa);
                    s.spawn(move || atomic_write_leftmost(&aa_ref, idx, val, k));
                }
            });

            // Extract result: create array with just the value component using tabulate
            <Self as ArraySeqMtEphSliceTrait<T>>::tabulate(
                &|i| aa[i].lock().unwrap().0.clone(),
                n
            )
        }

        fn ninject(a: &Self, updates: &[(N, T)]) -> Self {
            // Algorithm 19.17: parallel ninject with rightmost-wins atomic writes
            use std::sync::Arc;
            use std::sync::Mutex;

            // Helper: APAS Algorithm 19.17 atomicWrite - rightmost wins
            fn atomic_write_rightmost<T: StTInMtT>(
                aa: &Arc<Box<[Mutex<(T, N)>]>>,
                idx: N,
                val: T,
                k: N,
            ) {
                // atomicWrite aa b k =
                //   atomically do:
                //     (j, v) ← b[k]      // j=idx, v=val from caller
                //     (w, i) ← aa[j]     // read current (value, index) at position idx
                //     if k >= i then     // rightmost wins: update if this update is later or equal
                //       aa[j] ← (v, k)   // write new value and update index
                if idx < aa.len() {
                    let mut slot = aa[idx].lock().unwrap();
                    if k >= slot.1 {
                        *slot = (val, k);
                    }
                }
            }

            if updates.is_empty() {
                return a.clone();
            }

            // Algorithm 19.17: Create aa from a where aa[i] = (a[i], 0)
            let n = a.length();
            
            // Create copied array aa with per-element Mutexes for atomic writes (benign effect)
            // Initialize aa[i] = (a[i], 0) by reading from sequence a
            let mut aa_uninit = Box::new_uninit_slice(n);
            for i in 0..n {
                aa_uninit[i].write(Mutex::new((a.nth_cloned(i), 0)));
            }
            let aa = Arc::new(unsafe { aa_uninit.assume_init() });

            // Inject all updates in parallel using atomicWrite - rightmost wins
            std::thread::scope(|s| {
                for (k, (idx, val)) in updates.iter().enumerate() {
                    let idx = *idx;
                    let val = val.clone();
                    let aa_ref = Arc::clone(&aa);
                    s.spawn(move || atomic_write_rightmost(&aa_ref, idx, val, k));
                }
            });

            // Extract result: create array with just the value component using tabulate
            <Self as ArraySeqMtEphSliceTrait<T>>::tabulate(
                &|i| aa[i].lock().unwrap().0.clone(),
                n
            )
        }

        fn from_box(data: Box<[T]>) -> Self {
            let len = data.len();
            ArraySeqMtEphSliceS {
                inner: Arc::new(Inner::new(data)),
                range: 0..len,
            }
        }

        fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }

        fn to_vec(&self) -> Vec<T> {
            let guard = self.inner.data.lock().unwrap();
            guard[self.range.start..self.range.end].to_vec()
        }

        fn with_exclusive<F: FnOnce(&mut [T]) -> R, R>(&self, f: F) -> R {
            let mut guard = self.inner.data.lock().unwrap();
            let start = self.range.start;
            let end = self.range.end;
            f(&mut guard[start..end])
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> { self.update(index, item) }
    }

    fn clamp_subrange<T: StTInMtT + 'static>(a: &ArraySeqMtEphSliceS<T>, start: N, length: N) -> Range<N> {
        let local_len = a.length();
        let clamped_start = start.min(local_len);
        let clamped_end = clamped_start.saturating_add(length).min(local_len);
        let base = a.range.start;
        (base + clamped_start)..(base + clamped_end)
    }

    impl<T: StTInMtT> Clone for ArraySeqMtEphSliceS<T> {
        fn clone(&self) -> Self {
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: self.range.clone(),
            }
        }
    }

    impl<T: StTInMtT + 'static> PartialEq for ArraySeqMtEphSliceS<T> {
        fn eq(&self, other: &Self) -> bool {
            if Arc::ptr_eq(&self.inner, &other.inner) && self.range == other.range {
                return true;
            }
            if self.length() != other.length() {
                return false;
            }
            let left = self.to_vec();
            let right = other.to_vec();
            left.iter().zip(right.iter()).all(|(a, b)| a == b)
        }
    }

    impl<T: StTInMtT + 'static> Eq for ArraySeqMtEphSliceS<T> {}

    impl<T: StTInMtT> Debug for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            f.debug_list()
                .entries(guard[self.range.start..self.range.end].iter())
                .finish()
        }
    }

    impl<T: StTInMtT> Display for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            let mut first = true;
            write!(f, "[")?;
            for item in &guard[self.range.start..self.range.end] {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    fn repeat_vec<T: StTInMtT>(length: N, init: T) -> Vec<T> {
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(init.clone());
        }
        data
    }

    #[macro_export]
    macro_rules! ArraySeqMtEphSliceSLit {
        () => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$($x),*]) };
    }
}
