//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStPer.

pub mod LinkedListStPer {

    use std::collections::HashSet;
    use std::fmt::{Debug, Display, Formatter};

    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct NodeP<T: StT> {
        pub value: T,
        pub next: Option<Box<NodeP<T>>>,
    }

    #[derive(Clone)]
    pub struct LinkedListStPerS<T: StT> {
        head: Option<Box<NodeP<T>>>,
        len: N,
    }

    pub trait LinkedListStPerTrait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn new(length: N, init_value: T)                                            -> Self
        where
            T: Clone;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                                                  -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn singleton(item: T)                                                       -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - cached length
        fn length(&self)                                                            -> N;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index), Parallelism Θ(1) - sequential traversal
        fn nth(&self, index: N)                                                     -> &T;
        /// APAS: Work Θ(start+length), Span Θ(start+length)
        /// claude-4-sonet: Work Θ(start+length), Span Θ(start+length), Parallelism Θ(1) - sequential copy
        fn subseq_copy(&self, start: N, length: N)                                  -> Self;
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T>;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U>;
        /// APAS: Work Θ(|a|+|b|), Span Θ(|a|+|b|)
        /// claude-4-sonet: Work Θ(|a|+|b|), Span Θ(|a|+|b|), Parallelism Θ(1) - sequential
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>)                 -> Self;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index), Parallelism Θ(1) - sequential traversal
        fn select(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>, index: N)       -> Option<T>;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn filter<F: PredSt<T>>(a: &LinkedListStPerS<T>, pred: &F)                  -> Self;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>)                     -> Self;
        /// APAS: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|)
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential with HashSet
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>)  -> Self;
        /// APAS: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|)
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential, overwrites on conflict
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential fold
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential prefix computation
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(
            a: &LinkedListStPerS<T>,
            f: &F,
            x: A,
        ) -> (LinkedListStPerS<A>, A);
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|log|a|), Span Θ(|a|log|a|), Parallelism Θ(1) - sequential divide-and-conquer (no parallelism)
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T;
        /// APAS: Work Θ(|a|²), Span Θ(|a|²)
        /// claude-4-sonet: Work Θ(|a|²), Span Θ(|a|²), Parallelism Θ(1) - naive scan calling reduce repeatedly
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T);
        /// APAS: Work Θ(Σ|ss[i]|), Span Θ(Σ|ss[i]|)
        /// claude-4-sonet: Work Θ(Σ|ss[i]|), Span Θ(Σ|ss[i]|), Parallelism Θ(1) - sequential
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>)                      -> Self;
        /// APAS: Work Θ(|a|²), Span Θ(|a|²)
        /// claude-4-sonet: Work Θ(|a|²), Span Θ(|a|²), Parallelism Θ(1) - sequential with linear search
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStPerS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStPerS<Pair<A, LinkedListStPerS<Bv>>>;
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn from_vec(elts: Vec<T>)                                                   -> Self;
    }

    // Module-level function for node navigation
    fn node_at<T: StT>(list: &LinkedListStPerS<T>, index: N) -> Option<&NodeP<T>> {
        if index >= list.len {
            return None;
        }
        let mut current = list.head.as_deref();
        let mut i = 0usize;
        while let Some(node) = current {
            if i == index {
                return Some(node);
            }
            current = node.next.as_deref();
            i += 1;
        }
        None
    }

    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>
        where
            T: Clone,
        {
            Self::from_vec(vec![init_value; length])
        }

        fn empty() -> LinkedListStPerS<T> { LinkedListStPerS { head: None, len: 0 } }
        fn singleton(item: T) -> LinkedListStPerS<T> { Self::from_vec(vec![item]) }
        fn length(&self) -> N { self.len }
        fn nth(&self, index: N) -> &T {
            node_at(self, index)
                .map(|node| &node.value)
                .expect("Index out of bounds")
        }
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T> {
            if length == 0 || start >= self.len {
                return Self::empty();
            }
            let mut current = self.head.as_deref();
            let mut skipped = 0usize;
            while skipped < start {
                match current {
                    | Some(node) => {
                        current = node.next.as_deref();
                        skipped += 1;
                    }
                    | None => return Self::empty(),
                }
            }
            let mut out = Vec::<T>::with_capacity(length);
            let mut taken = 0usize;
            while taken < length {
                match current {
                    | Some(node) => {
                        out.push(node.value.clone());
                        current = node.next.as_deref();
                        taken += 1;
                    }
                    | None => break,
                }
            }
            Self::from_vec(out)
        }

        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T> {
            let mut values = Vec::<T>::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            LinkedListStPerS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U> {
            let mut values = Vec::<U>::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            LinkedListStPerS::from_vec(values)
        }

        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {
            let mut values = Vec::<T>::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            LinkedListStPerS::from_vec(values)
        }

        fn select(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>, index: N) -> Option<T> {
            // Select from concatenated sequences: if index < |a| then a[index] else b[index - |a|]
            if index < a.length() {
                Some(a.nth(index).clone())
            } else {
                let b_index = index - a.length();
                if b_index < b.length() {
                    Some(b.nth(b_index).clone())
                } else {
                    None
                }
            }
        }

        fn filter<F: PredSt<T>>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T> {
            let mut kept = Vec::<T>::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) {
                    kept.push(value.clone());
                }
            }
            LinkedListStPerS::from_vec(kept)
        }

        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T> {
            let mut values = Vec::<T>::with_capacity(a.length());
            for i in 0..a.length() {
                let current = a.nth(i).clone();
                if i == index {
                    values.push(item.clone());
                } else {
                    values.push(current);
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T> {
            let mut values = (0..a.length()).map(|i| a.nth(i).clone()).collect::<Vec<T>>();
            let mut seen = std::collections::HashSet::new();
            for k in 0..updates.length() {
                let Pair(idx, val) = updates.nth(k).clone();
                if idx < values.len() && !seen.contains(&idx) {
                    values[idx] = val;
                    seen.insert(idx);
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T> {
            let mut values = (0..a.length()).map(|i| a.nth(i).clone()).collect::<Vec<T>>();
            for k in 0..updates.length() {
                let Pair(idx, val) = updates.nth(k).clone();
                if idx < values.len() {
                    values[idx] = val;
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(
            a: &LinkedListStPerS<T>,
            f: &F,
            x: A,
        ) -> (LinkedListStPerS<A>, A) {
            let mut acc = x.clone();
            let mut prefixes = Vec::<A>::with_capacity(a.length());
            for i in 0..a.length() {
                prefixes.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (LinkedListStPerS::from_vec(prefixes), acc)
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T {
            let len = a.length();
            if len == 0 {
                return id;
            }
            if len == 1 {
                return a.nth(0).clone();
            }
            let mid = len / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, len - mid);
            let l = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T) {
            let len = a.length();
            if len == 0 {
                return (Self::empty(), id);
            }
            let mut prefixes = Vec::<T>::with_capacity(len);
            for i in 0..len {
                let prefix = a.subseq_copy(0, i);
                let red = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(&prefix, f, id.clone());
                prefixes.push(red);
            }
            let total = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(a, f, id);
            (LinkedListStPerS::from_vec(prefixes), total)
        }

        fn from_vec(elts: Vec<T>) -> Self {
            let mut head: Option<Box<NodeP<T>>> = None;
            let mut len = 0usize;
            for value in elts.into_iter().rev() {
                head = Some(Box::new(NodeP { value, next: head }));
                len += 1;
            }
            LinkedListStPerS { head, len }
        }

        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {
            let mut values = Vec::<T>::new();
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStPerS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStPerS<Pair<A, LinkedListStPerS<Bv>>> {
            let mut groups = Vec::<Pair<A, Vec<Bv>>>::new();
            for i in 0..a.length() {
                let Pair(k, v) = a.nth(i).clone();
                if let Some(Pair(_, existing)) = groups.iter_mut().find(|Pair(gk, _)| cmp(&k, gk) == O::Equal) {
                    existing.push(v);
                } else {
                    groups.push(Pair(k, vec![v]));
                }
            }
            let pairs = groups
                .into_iter()
                .map(|Pair(k, vs)| Pair(k, LinkedListStPerS::from_vec(vs))).collect::<Vec<Pair<A, LinkedListStPerS<Bv>>>>();
            LinkedListStPerS::from_vec(pairs)
        }
    }

    impl<T: StT> Display for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            let mut current = self.head.as_deref();
            while let Some(node) = current {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", node.value)?;
                current = node.next.as_deref();
            }
            write!(f, "]")
        }
    }

    impl<T: StT> Debug for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            let mut current = self.head.as_deref();
            while let Some(node) = current {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", node.value)?;
                current = node.next.as_deref();
            }
            write!(f, "]")
        }
    }

    impl<T: StT> PartialEq for LinkedListStPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.len != other.len {
                return false;
            }
            let mut left = self.head.as_deref();
            let mut right = other.head.as_deref();
            while let (Some(a), Some(b)) = (left, right) {
                if a.value != b.value {
                    return false;
                }
                left = a.next.as_deref();
                right = b.next.as_deref();
            }
            true
        }
    }

    impl<T: StT> Eq for LinkedListStPerS<T> {}

    #[macro_export]
    macro_rules! LinkedListStPerSLit {
        () => { $crate::Chap18::LinkedListStPer::LinkedListStPer::LinkedListStPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::LinkedListStPer::LinkedListStPer::LinkedListStPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::LinkedListStPer::LinkedListStPer::LinkedListStPerS::from_vec(vec![$($x),*]) };
    }
}
