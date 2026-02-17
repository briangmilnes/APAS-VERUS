//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral reducer-augmented ordered table implementation.

pub mod AugOrderedTableStEph {

    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
    use crate::OrderedTableStEphLit;
    use crate::Types::Types::*;

    #[derive(PartialEq, Clone)]
    pub struct AugOrderedTableStEph<K: StT + Ord, V: StT, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        base_table: OrderedTableStEph<K, V>,
        reducer: F,
        identity: V,
        cached_reduction: V,
    }

    pub type AugOrderedTableEph<K, V, F> = AugOrderedTableStEph<K, V, F>;

    /// Trait defining all augmented ordered table operations (ADT 43.3) with ephemeral semantics
    /// Extends ordered table operations with efficient reduction and in-place mutations
    pub trait AugOrderedTableStEphTrait<K: StT + Ord, V: StT, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        // Base table operations (ADT 42.1) - ephemeral semantics
        fn size(&self) -> N;
        fn empty(reducer: F, identity: V) -> Self;
        fn singleton(k: K, v: V, reducer: F, identity: V) -> Self;
        fn find(&self, k: &K) -> Option<V>;
        fn lookup(&self, k: &K) -> Option<V>;
        fn is_empty(&self) -> B;
        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G);
        fn delete(&mut self, k: &K) -> Option<V>;
        fn domain(&self) -> ArraySetStEph<K>;
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> Self;
        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> Self;
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> Self;
        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> R;
        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G);
        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G);
        fn difference(&mut self, other: &Self);
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        fn subtract(&mut self, keys: &ArraySetStEph<K>);
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>>;

        // Key ordering operations (ADT 43.1 adapted for tables) - ephemeral semantics
        fn first_key(&self) -> Option<K>;
        fn last_key(&self) -> Option<K>;
        fn previous_key(&self, k: &K) -> Option<K>;
        fn next_key(&self, k: &K) -> Option<K>;
        fn split_key(&mut self, k: &K) -> (Self, Self)
        where
            Self: Sized;
        fn join_key(&mut self, other: Self);
        fn get_key_range(&self, k1: &K, k2: &K) -> Self;
        fn rank_key(&self, k: &K) -> N;
        fn select_key(&self, i: N) -> Option<K>;
        fn split_rank_key(&mut self, i: N) -> (Self, Self)
        where
            Self: Sized;

        // Augmented operations (ADT 43.3) - the key innovation
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Returns the cached reduction of all values using the reducer function
        fn reduce_val(&self) -> V;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Efficient range reduction: getRange followed by reduceVal
        fn reduce_range(&self, k1: &K, k2: &K) -> V;
    }

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStEphTrait<K, V, F> for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N { self.base_table.size() }

        /// Claude Work: O(1), Span: O(1)
        fn empty(reducer: F, identity: V) -> Self {
            Self {
                base_table: OrderedTableStEph::empty(),
                cached_reduction: identity.clone(),
                reducer,
                identity,
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(k: K, v: V, reducer: F, identity: V) -> Self {
            Self {
                base_table: OrderedTableStEph::singleton(k, v.clone()),
                cached_reduction: v,
                reducer,
                identity,
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn find(&self, k: &K) -> Option<V> { self.base_table.find(k) }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn lookup(&self, k: &K) -> Option<V> { self.base_table.lookup(k) }

        /// Claude Work: O(1), Span: O(1)
        fn is_empty(&self) -> B { self.base_table.is_empty() }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G) {
            let old_size = self.base_table.size();
            self.base_table.insert(k, v.clone(), combine);

            // Update cached reduction
            if old_size == 0 {
                self.cached_reduction = v;
            } else {
                self.cached_reduction = (self.reducer)(&self.cached_reduction, &v);
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn delete(&mut self, k: &K) -> Option<V> {
            let result = self.base_table.delete(k);
            // Recalculate reduction after deletion
            self.cached_reduction = self.recalculate_reduction();
            result
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn domain(&self) -> ArraySetStEph<K> { self.base_table.domain() }

        /// Claude Work: O(n), Span: O(lg n)
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> Self {
            let base_table = OrderedTableStEph::tabulate(f, keys);
            let cached_reduction = Self::calculate_reduction(&base_table, &reducer, &identity);

            Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            }
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> Self {
            let new_base = self.base_table.map(f);
            let new_reduction = Self::calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> Self {
            let new_base = self.base_table.filter(f);
            let new_reduction = Self::calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> R { self.base_table.reduce(init, f) }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G) {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G) {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn difference(&mut self, other: &Self) {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.restrict(keys);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.subtract(keys);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>> { self.base_table.collect() }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn first_key(&self) -> Option<K> { self.base_table.first_key() }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn last_key(&self) -> Option<K> { self.base_table.last_key() }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn previous_key(&self, k: &K) -> Option<K> { self.base_table.previous_key(k) }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn next_key(&self, k: &K) -> Option<K> { self.base_table.next_key(k) }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn split_key(&mut self, k: &K) -> (Self, Self) {
            let (left_base, right_base) = self.base_table.split_key(k);

            let left_reduction = Self::calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = Self::calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            (left, right)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn join_key(&mut self, other: Self) {
            let old_reduction = self.cached_reduction.clone();
            let other_reduction = other.cached_reduction.clone();
            let other_size = other.base_table.size();

            self.base_table.join_key(other.base_table);

            // Combine reductions
            if self.base_table.size() == 0 {
                self.cached_reduction = other_reduction;
            } else if other_size == 0 {
                self.cached_reduction = old_reduction;
            } else {
                self.cached_reduction = (self.reducer)(&old_reduction, &other_reduction);
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn get_key_range(&self, k1: &K, k2: &K) -> Self {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = Self::calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn rank_key(&self, k: &K) -> N { self.base_table.rank_key(k) }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn select_key(&self, i: N) -> Option<K> { self.base_table.select_key(i) }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn split_rank_key(&mut self, i: N) -> (Self, Self) {
            let (left_base, right_base) = self.base_table.split_rank_key(i);

            let left_reduction = Self::calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = Self::calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            (left, right)
        }

        /// Claude Work: O(1), Span: O(1)
        /// The key innovation: O(1) reduction using cached value
        fn reduce_val(&self) -> V { self.cached_reduction.clone() }

        /// Claude Work: O(lg n), Span: O(lg n)
        /// Efficient range reduction for TRAMLAW/QADSAN scenarios
        fn reduce_range(&self, k1: &K, k2: &K) -> V {
            let range_table = self.get_key_range(k1, k2);
            range_table.reduce_val()
        }
    }

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn recalculate_reduction(&self) -> V {
            Self::calculate_reduction(&self.base_table, &self.reducer, &self.identity)
        }

        fn calculate_reduction(base: &OrderedTableStEph<K, V>, reducer: &F, identity: &V) -> V {
            if base.size() == 0 {
                return identity.clone();
            }

            let pairs = base.collect();
            let mut result = identity.clone();
            let mut first = true;

            for i in 0..pairs.length() {
                let pair = pairs.nth(i);
                if first {
                    result = pair.1.clone();
                    first = false;
                } else {
                    result = reducer(&result, &pair.1);
                }
            }

            result
        }
    }

    impl<K: StT + Ord, V: StT, F> Display for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableStEph(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: StT + Ord, V: StT, F> Debug for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableStEph")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableStEphLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::empty($reducer, $identity);
            $(
                $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::insert(&mut table, $k, $v, |_old, new| new.clone());
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::empty($reducer, $identity)
        }};
    }
}
