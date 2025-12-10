//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral array sequence (mutable) implementation.

pub mod ArraySeqStEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct ArraySeqStEphS<T: StT> {
        data: Box<[T]>,
    }

    pub type ArrayStEph<T> = ArraySeqStEphS<T>;

    // Base methods - never redefined in later chapters
    pub trait ArraySeqStEphBaseTrait<T: StT> {
        fn new(length: N, init_value: T)                  -> Self;
        fn set(&mut self, index: N, item: T)              -> Result<&mut ArraySeqStEphS<T>, &'static str>;
        fn length(&self)                                  -> N;
        fn nth(&self, index: N)                           -> &T;
        fn subseq(&self, start: N, length: N)             -> Self;
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> Self;
        fn update(&mut self, update: Pair<N, T>)          -> &mut Self;
        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStEphS<Pair<K, ArraySeqStEphS<V>>>;
        fn from_vec(elts: Vec<T>)                         -> Self;
        fn iter(&self)                                    -> Iter<'_, T>;
    }

    // Redefinable methods - may be overridden with better algorithms in later chapters
    pub trait ArraySeqStEphRedefinableTrait<T: StT> {
        fn empty()                                                 -> Self;
        fn singleton(item: T)                                      -> Self;
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T>;
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;
        fn append(&self, b: &ArraySeqStEphS<T>)                    -> Self;
        fn filter<F: PredSt<T>>(&self, pred: &F)                   -> Self;
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T);
        fn isEmpty(&self)                                          -> B;
        fn isSingleton(&self)                                      -> B;
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self;
    }

    impl<T: StT> ArraySeqStEphBaseTrait<T> for ArraySeqStEphS<T> {
        fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }
        fn length(&self) -> N { self.data.len() }
        fn nth(&self, index: N) -> &T { &self.data[index] }

        fn subseq(&self, start: N, length: N) -> Self {
            let total = self.data.len();
            let begin = start.min(total);
            let end = start.saturating_add(length).min(total);
            Self::from_vec(self.data[begin..end].to_vec())
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            if index < self.data.len() {
                self.data[index] = item;
                Ok(self)
            } else {
                Err("Index out of bounds")
            }
        }

        fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {
            let _ = self.set(index, item);
            self
        }

        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {
            let mut values = Vec::<T>::new();
            for i in 0..a.length() {
                let inner = a.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStEphS<Pair<K, ArraySeqStEphS<V>>> {
            let mut groups = Vec::<Pair<K, Vec<V>>>::new();
            'outer: for i in 0..pairs.length() {
                let Pair(key, value) = pairs.nth(i).clone();
                for group in groups.iter_mut() {
                    if cmp(&group.0, &key) == O::Equal {
                        group.1.push(value.clone());
                        continue 'outer;
                    }
                }
                groups.push(Pair(key, vec![value]));
            }
            let collected = groups
                .into_iter()
                .map(|Pair(key, bucket)| Pair(key, ArraySeqStEphS::from_vec(bucket))).collect::<Vec<Pair<K, ArraySeqStEphS<V>>>>();
            ArraySeqStEphS::from_vec(collected)
        }

        fn from_vec(elts: Vec<T>) -> Self {
            Self {
                data: elts.into_boxed_slice(),
            }
        }
        fn iter(&self) -> Iter<'_, T> { self.data.iter() }
    }

    impl<T: StT> ArraySeqStEphRedefinableTrait<T> for ArraySeqStEphS<T> {
        fn empty() -> Self { Self::from_vec(Vec::new()) }
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }

        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T> {
            let mut values = Vec::<T>::with_capacity(length);
            for i in 0..length {
                values.push(f(i));
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {
            let mut values = Vec::<U>::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn append(&self, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            let total = self.length() + b.length();
            let mut values = Vec::<T>::with_capacity(total);
            for i in 0..self.length() {
                values.push(self.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn filter<F: PredSt<T>>(&self, pred: &F) -> ArraySeqStEphS<T> {
            let mut kept = Vec::<T>::new();
            for i in 0..self.length() {
                let value = self.nth(i);
                if pred(value) {
                    kept.push(value.clone());
                }
            }
            ArraySeqStEphS::from_vec(kept)
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A {
            let mut acc = seed;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T) {
            let mut prefixes = Vec::<T>::with_capacity(a.length());
            let mut acc = id.clone();
            prefixes.push(acc.clone());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                if i < a.length() - 1 {
                    prefixes.push(acc.clone());
                }
            }
            (ArraySeqStEphS::from_vec(prefixes), acc)
        }

        fn isEmpty(&self) -> B { self.length() == 0 }
        fn isSingleton(&self) -> B { self.length() == 1 }

        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {
            let mut last_values = HashMap::<N, T>::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i).clone();
                if index < self.data.len() {
                    last_values.insert(index, value);
                }
            }
            for (index, value) in last_values {
                let _ = self.set(index, value);
            }
            self
        }
    }

    impl<T: StT> PartialEq for ArraySeqStEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }
    }

    impl<T: StT> Eq for ArraySeqStEphS<T> {}

    impl<T: StT> Debug for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.iter()).finish() }
    }

    impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    impl<T: StT> IntoIterator for ArraySeqStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter { self.data.into_vec().into_iter() }
    }

    impl<T: StT> Display for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }
}

