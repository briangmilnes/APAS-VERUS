//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).
//!
//! Note: Uses unconditional parallelism with ParaPair! for divide-and-conquer operations (map, reduce).

pub mod ArraySeqMtEph {

    use std::collections::HashSet;
    use std::fmt::{Display, Formatter};
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::ParaPair;
    use crate::Types::Types::*;

    #[derive(Debug)]
    pub struct ArraySeqMtEphS<T: StTInMtT> {
        data: Mutex<Box<[T]>>,
    }

    // Base trait: Methods that are not redefined in Chap19
    pub trait ArraySeqMtEphBaseTrait<T: StTInMtT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn new(length: N, init_value: T)                      -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - locks mutex
        fn set(&self, index: N, item: T)                      -> Result<(), &'static str>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - locks mutex
        fn length(&self)                                      -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        fn nth_cloned(&self, index: N)                        -> T;
        /// APAS: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N)                               -> Option<T>;
        /// APAS: Work Θ(len), Span Θ(1)
        /// claude-4-sonet: Work Θ(len), Span Θ(len), Parallelism Θ(1) - sequential copy, locks mutex
        fn subseq_copy(&self, start: N, length: N)            -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - in-place, locks mutex
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut Self;
        /// APAS: Work Θ(|a|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|²), Span Θ(|a|²), Parallelism Θ(1) - sequential with linear search
        fn collect<K: StTInMtT, V: StTInMtT>(
            a: &ArraySeqMtEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqMtEphS<Pair<K, ArraySeqMtEphS<V>>>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential prefix sum
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T);
        // Additional methods
        fn from_vec(values: Vec<T>)                           -> Self;
        fn iter_cloned(&self)                                 -> Vec<T>;
        fn to_vec(&self)                                      -> Vec<T>;
    }

    // Redefinable trait: Methods that Chap19 might redefine
    pub trait ArraySeqMtEphRedefinableTrait<T: StTInMtT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                                              -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn singleton(item: T)                                                   -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(log|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(log|a|), Parallelism Θ(|a|/log|a|) - parallel via ParaPair! divide-and-conquer
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static;
        /// APAS: Work Θ(|a|+|b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|b|), Span Θ(|a|+|b|), Parallelism Θ(1) - sequential
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>)                 -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn filter<F: PredMt<T>>(a: &ArraySeqMtEphS<T>, pred: &F)                -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - locks mutex
        fn isEmpty(&self)                                                       -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - locks mutex
        fn isSingleton(&self)                                                   -> B;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential fold
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(log|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential (Chap18), parallel (Chap19)
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
        where
            T: Send + 'static;
        /// APAS: Work Θ(Σ|ss[i]|), Span Θ(1)
        /// claude-4-sonet: Work Θ(Σ|ss[i]|), Span Θ(Σ|ss[i]|), Parallelism Θ(1) - sequential (Chap18), parallel (Chap19)
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>)                      -> Self;
        /// APAS: Work Θ(|a|+|updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential (Chap18), parallel (Chap19)
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>)  -> Self;
        /// APAS: Work Θ(|a|+|updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential (Chap18), parallel (Chap19)
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> Self;
    }

    impl<T: StTInMtT> ArraySeqMtEphBaseTrait<T> for ArraySeqMtEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphS::from_vec(vec![init_value; length]) }

        fn set(&self, index: N, item: T) -> Result<(), &'static str> {
            let mut guard = self.data.lock().unwrap();
            if index < guard.len() {
                guard[index] = item;
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        }

        fn length(&self) -> N {
            let guard = self.data.lock().unwrap();
            guard.len()
        }

        fn nth_cloned(&self, index: N) -> T {
            let guard = self.data.lock().unwrap();
            guard[index].clone()
        }

        fn nth(&self, index: N) -> Option<T> {
            let guard = self.data.lock().unwrap();
            if index < guard.len() {
                Some(guard[index].clone())
            } else {
                None
            }
        }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {
            let guard = self.data.lock().unwrap();
            let n = guard.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            let values: Vec<T> = guard[s..e].to_vec();
            ArraySeqMtEphS::from_vec(values)
        }

        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {
            let _ = a.set(index, item);
            a
        }

        fn collect<K: StTInMtT, V: StTInMtT>(
            a: &ArraySeqMtEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqMtEphS<Pair<K, ArraySeqMtEphS<V>>> {
            if a.length() == 0 {
                return ArraySeqMtEphS::from_vec(vec![]);
            }
            let mut groups = Vec::<Pair<K, ArraySeqMtEphS<V>>>::new();
            for i in 0..a.length() {
                let Pair(key, value) = a.nth_cloned(i);
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(&key, &group.0) == O::Equal {
                        let mut values = Vec::<V>::with_capacity(group.1.length() + 1);
                        for j in 0..group.1.length() {
                            values.push(group.1.nth_cloned(j));
                        }
                        values.push(value.clone());
                        group.1 = ArraySeqMtEphS::from_vec(values);
                        found_group = true;
                        break;
                    }
                }
                if !found_group {
                    groups.push(Pair(key.clone(), ArraySeqMtEphS::from_vec(vec![value])));
                }
            }
            ArraySeqMtEphS::from_vec(groups)
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T) {
            let mut acc = id.clone();
            let mut values = Vec::<T>::with_capacity(a.length());
            for i in 0..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
                values.push(acc.clone());
            }
            (ArraySeqMtEphS::from_vec(values), acc)
        }

        fn from_vec(values: Vec<T>) -> Self {
            ArraySeqMtEphS {
                data: Mutex::new(values.into_boxed_slice()),
            }
        }

        fn iter_cloned(&self) -> Vec<T> {
            let guard = self.data.lock().unwrap();
            guard.iter().cloned().collect()
        }

        fn to_vec(&self) -> Vec<T> {
            let guard = self.data.lock().unwrap();
            guard.iter().cloned().collect()
        }
    }

    impl<T: StTInMtT> ArraySeqMtEphRedefinableTrait<T> for ArraySeqMtEphS<T> {
        fn empty() -> ArraySeqMtEphS<T> {
            ArraySeqMtEphS {
                data: Mutex::new(Vec::new().into_boxed_slice()),
            }
        }

        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphS::from_vec(vec![item]) }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {
            let mut values = Vec::<T>::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static,
        {
            let n = a.length();
            if n == 0 {
                return ArraySeqMtEphS::from_vec(Vec::new());
            }
            if n == 1 {
                let val = f(&a.nth_cloned(0));
                return ArraySeqMtEphS::from_vec(vec![val]);
            }

            // Unconditionally parallel using ParaPair!
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);
            let f_clone = f.clone();

            let Pair(left_result, right_result) = ParaPair!(
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::map(&left, f_clone),
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::map(&right, f)
            );

            <ArraySeqMtEphS<U> as ArraySeqMtEphRedefinableTrait<U>>::append(&left_result, &right_result)
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            let na = a.length();
            let nb = b.length();
            let mut values = Vec::<T>::with_capacity(na + nb);
            for i in 0..na {
                values.push(a.nth_cloned(i));
            }
            for j in 0..nb {
                values.push(b.nth_cloned(j));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn filter<F: PredMt<T>>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEphS<T> {
            let mut kept = Vec::<T>::new();
            let n = a.length();
            for i in 0..n {
                let value = a.nth_cloned(i);
                if pred(&value) {
                    kept.push(value);
                }
            }
            ArraySeqMtEphS::from_vec(kept)
        }

        fn isEmpty(&self) -> B { self.length() == 0 }

        fn isSingleton(&self) -> B { self.length() == 1 }

        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
        where
            T: Send + 'static,
        {
            // Chap18 base implementation: sequential reduce
            // Chap19 will redefine this with parallel divide-and-conquer
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, &a.nth_cloned(i));
            }
            acc
        }

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {
            let mut values = Vec::<T>::new();
            for i in 0..ss.length() {
                let inner = ss.nth_cloned(i);
                for j in 0..inner.length() {
                    values.push(inner.nth_cloned(j));
                }
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            let out = a.clone();
            let mut seen = HashSet::<N>::new();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth_cloned(i);
                if seen.insert(idx) {
                    let _ = out.set(idx, val);
                }
            }
            out
        }

        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            let out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth_cloned(i);
                let _ = out.set(idx, val);
            }
            out
        }
    }

    impl<T: StTInMtT> Display for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtEphS[")?;
            let guard = self.data.lock().unwrap();
            for (i, item) in guard.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    impl<T: StTInMtT> Clone for ArraySeqMtEphS<T> {
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }
    }

    impl<T: StTInMtT> PartialEq for ArraySeqMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.to_vec() == other.to_vec() }
    }
    impl<T: StTInMtT> Eq for ArraySeqMtEphS<T> {}

    #[macro_export]
    macro_rules! ArraySeqMtEphSLit {
        () => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$($x),*]) };
    }
}

