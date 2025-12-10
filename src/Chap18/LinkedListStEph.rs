//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStEph (ephemeral).

pub mod LinkedListStEph {

    use std::collections::HashSet;
    use std::fmt::{Debug, Display, Formatter};

    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct NodeE<T: StT> {
        pub value: T,
        pub next: Option<Box<NodeE<T>>>,
    }

    #[derive(Clone)]
    pub struct LinkedListStEphS<T: StT> {
        head: Option<Box<NodeE<T>>>,
        len: N,
    }

    pub trait LinkedListStEphTrait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn new(length: N, init_value: T)                             -> Self
        where
            T: Clone;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index), Parallelism Θ(1) - sequential traversal and in-place
        fn set(&mut self, index: N, item: T)                         -> Result<&mut Self, &'static str>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - cached length
        fn length(&self)                                             -> N;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index), Parallelism Θ(1) - sequential traversal
        fn nth(&self, index: N)                                      -> &T;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                                   -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn singleton(item: T)                                        -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U>;
        /// APAS: Work Θ(start+length), Span Θ(start+length)
        /// claude-4-sonet: Work Θ(start+length), Span Θ(start+length), Parallelism Θ(1) - sequential traversal and copy
        fn subseq_copy(&self, start: N, length: N)                   -> Self;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(|a| + |b|), Parallelism Θ(1) - sequential
        fn append(a: &Self, b: &Self)                                -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn filter<F: PredSt<T>>(a: &Self, pred: &F)                  -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn deflate<F: PredSt<T>>(f: &F, x: &T)                       -> Self;
        /// APAS: Work Θ(Σ|ss[i]|), Span Θ(Σ|ss[i]|)
        /// claude-4-sonet: Work Θ(Σ|ss[i]|), Span Θ(Σ|ss[i]|), Parallelism Θ(1) - sequential
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>)       -> Self;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index), Parallelism Θ(1) - in-place, sequential traversal
        fn update(a: &mut Self, item_at: Pair<N, T>)                 -> &mut Self;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(|a| + |updates|), Parallelism Θ(1) - sequential with HashSet
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>)  -> Self;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(|a| + |updates|), Parallelism Θ(1) - sequential, overwrites on conflict
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;
        /// APAS: Work Θ(|a|²), Span Θ(|a|²)
        /// claude-4-sonet: Work Θ(|a|²), Span Θ(|a|²), Parallelism Θ(1) - sequential with linear search
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStEphS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStEphS<Pair<A, LinkedListStEphS<Bv>>>;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential fold
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential prefix computation
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEphS<A>, A);
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        /// claude-4-sonet: Work Θ(|a|log|a|), Span Θ(|a|log|a|), Parallelism Θ(1) - sequential divide-and-conquer (no parallelism)
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T;
        /// APAS: Work Θ(|a|²), Span Θ(|a|²)
        /// claude-4-sonet: Work Θ(|a|²), Span Θ(|a|²), Parallelism Θ(1) - naive scan calling reduce repeatedly
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T);
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn from_vec(elts: Vec<T>)                                    -> Self;
    }

    fn node_at<T: StT>(list: &LinkedListStEphS<T>, index: N) -> Option<&NodeE<T>> {
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

    fn node_at_mut<T: StT>(list: &mut LinkedListStEphS<T>, index: N) -> Option<&mut NodeE<T>> {
        if index >= list.len {
            return None;
        }
        let mut current = list.head.as_deref_mut();
        let mut i = 0usize;
        while let Some(node) = current {
            if i == index {
                return Some(node);
            }
            current = node.next.as_deref_mut();
            i += 1;
        }
        None
    }

    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {
        fn new(length: N, init_value: T) -> Self
        where
            T: Clone,
        {
            LinkedListStEphS::from_vec(vec![init_value; length])
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            match node_at_mut(self, index) {
                | Some(node) => {
                    node.value = item;
                    Ok(self)
                }
                | None => Err("Index out of bounds"),
            }
        }

        fn length(&self) -> N { self.len }

        fn nth(&self, index: N) -> &T {
            node_at(self, index)
                .map(|node| &node.value)
                .expect("Index out of bounds")
        }

        fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }

        fn singleton(item: T) -> Self { LinkedListStEphS::from_vec(vec![item]) }

        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self {
            let mut values = Vec::<T>::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            LinkedListStEphS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U> {
            let mut values = Vec::<U>::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            LinkedListStEphS::from_vec(values)
        }

        fn subseq_copy(&self, start: N, length: N) -> Self {
            if length == 0 || start >= self.len {
                return LinkedListStEphS::empty();
            }
            let mut current = self.head.as_deref();
            let mut skipped = 0usize;
            while skipped < start {
                match current {
                    | Some(node) => {
                        current = node.next.as_deref();
                        skipped += 1;
                    }
                    | None => return LinkedListStEphS::empty(),
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
            LinkedListStEphS::from_vec(out)
        }

        fn append(a: &Self, b: &Self) -> Self {
            let mut values = Vec::<T>::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            LinkedListStEphS::from_vec(values)
        }

        fn filter<F: PredSt<T>>(a: &Self, pred: &F) -> Self {
            let mut kept = Vec::<T>::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) {
                    kept.push(value.clone());
                }
            }
            LinkedListStEphS::from_vec(kept)
        }

        fn deflate<F: PredSt<T>>(f: &F, x: &T) -> Self {
            if f(x) {
                LinkedListStEphS::from_vec(vec![x.clone()])
            } else {
                LinkedListStEphS::empty()
            }
        }

        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {
            let mut values = Vec::<T>::new();
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            LinkedListStEphS::from_vec(values)
        }

        fn update(a: &mut Self, Pair(index, item): Pair<N, T>) -> &mut Self {
            let _ = a.set(index, item);
            a
        }

        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {
            let mut out = a.clone();
            let mut applied = HashSet::<N>::new();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                if applied.insert(idx) {
                    let _ = out.set(idx, val);
                }
            }
            out
        }

        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {
            let mut out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                let _ = out.set(idx, val);
            }
            out
        }

        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStEphS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStEphS<Pair<A, LinkedListStEphS<Bv>>> {
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
                .map(|Pair(k, vs)| Pair(k, LinkedListStEphS::from_vec(vs))).collect::<Vec<Pair<A, LinkedListStEphS<Bv>>>>();
            LinkedListStEphS::from_vec(pairs)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEphS<A>, A) {
            let mut acc = x.clone();
            let mut prefixes = Vec::<A>::with_capacity(a.length());
            for i in 0..a.length() {
                prefixes.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (LinkedListStEphS::from_vec(prefixes), acc)
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T {
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
            let l = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T) {
            let len = a.length();
            if len == 0 {
                return (LinkedListStEphS::empty(), id);
            }
            let mut prefixes = Vec::<T>::with_capacity(len);
            for i in 0..len {
                let prefix = a.subseq_copy(0, i);
                let red = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(&prefix, f, id.clone());
                prefixes.push(red);
            }
            let total = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(a, f, id);
            (LinkedListStEphS::from_vec(prefixes), total)
        }

        fn from_vec(mut elts: Vec<T>) -> Self {
            let len = elts.len();
            let mut head: Option<Box<NodeE<T>>> = None;
            while let Some(value) = elts.pop() {
                head = Some(Box::new(NodeE { value, next: head }));
            }
            LinkedListStEphS { head, len }
        }
    }

    impl<T: StT> Display for LinkedListStEphS<T> {
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

    impl<T: StT> Debug for LinkedListStEphS<T> {
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

    impl<T: StT> PartialEq for LinkedListStEphS<T> {
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

    impl<T: StT> Eq for LinkedListStEphS<T> {}

    #[macro_export]
    macro_rules! LinkedListStEphSLit {
        () => { $crate::Chap18::LinkedListStEph::LinkedListStEph::LinkedListStEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::LinkedListStEph::LinkedListStEph::LinkedListStEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::LinkedListStEph::LinkedListStEph::LinkedListStEphS::from_vec(vec![$($x),*]) };
    }
}
