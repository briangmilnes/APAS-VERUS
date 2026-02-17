//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Leftist Heap (Data Structure 45.3)

pub mod LeftistHeapPQ {

    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Types::Types::*;

    #[derive(PartialEq, Clone, Debug)]
    pub enum LeftistHeapNode<T: StT + Ord> {
        Leaf,
        Node {
            key: T,
            left: Box<LeftistHeapNode<T>>,
            right: Box<LeftistHeapNode<T>>,
            rank: N, // Distance to nearest leaf (for leftist property)
        },
    }

    /// Priority Queue implemented using Leftist Heap
    /// Data Type 45.1: Meldable Priority Queue with efficient O(log n) meld
    #[derive(PartialEq, Clone, Debug)]
    pub struct LeftistHeapPQ<T: StT + Ord> {
        root: LeftistHeapNode<T>,
    }

    /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
    pub trait LeftistHeapPQTrait<T: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                       -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(element: T)         -> Self;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// Returns the minimum element (root of heap), or None if empty
        fn find_min(&self)               -> Option<&T>;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Inserts element by creating singleton and melding
        fn insert(&self, element: T)     -> Self;

        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        /// Removes root and melds left and right subtrees
        fn delete_min(&self)             -> (Self, Option<T>)
        where
            Self: Sized;

        /// Claude Work: Θ(log m + log n), Span: Θ(log m + log n)
        /// KEY ADVANTAGE: Efficient meld following right spines
        fn meld(&self, other: &Self)     -> Self;

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Creates heap from sequence using reduce with meld
        fn from_seq(seq: &[T])           -> Self;

        fn size(&self)                   -> N;
        fn is_empty(&self)               -> bool;
        fn extract_all_sorted(&self)     -> Vec<T>;
        fn height(&self)                 -> N;
        fn root_rank(&self)              -> N;
        fn is_valid_leftist_heap(&self)  -> bool;
        fn from_vec(vec: Vec<T>)         -> Self;
        fn to_vec(&self)                 -> Vec<T>;
        fn to_sorted_vec(&self)          -> Vec<T>;
        fn meld_multiple(heaps: &[Self]) -> Self
        where
            Self: Sized;
        fn split(&self, key: &T)         -> (Self, Self)
        where
            Self: Sized;
    }

    pub trait LeftistHeapDemoTrait {
        /// Demonstrate the power of O(log n) meld operations
        /// This would be much slower with other priority queue implementations!
        fn efficient_multi_way_merge<T: StT + Ord>(sequences: Vec<Vec<T>>) -> Vec<T>;
        /// Demonstrate parallel heap construction
        fn parallel_heap_construction<T: StT + Ord>(elements: Vec<T>)      -> LeftistHeapPQ<T>;
    }

    impl<T: StT + Ord> LeftistHeapNode<T> {
        /// Get the rank (distance to nearest leaf) of a node
        fn rank(&self) -> N {
            match self {
                | LeftistHeapNode::Leaf => 0,
                | LeftistHeapNode::Node { rank, .. } => *rank,
            }
        }

        /// Create a new node with correct rank and leftist property
        fn make_node(key: T, left: LeftistHeapNode<T>, right: LeftistHeapNode<T>) -> Self {
            let left_rank = left.rank();
            let right_rank = right.rank();

            // Ensure leftist property: left subtree has >= rank than right subtree
            let (final_left, final_right) = if left_rank >= right_rank {
                (left, right)
            } else {
                (right, left)
            };

            let node_rank = final_right.rank() + 1;

            LeftistHeapNode::Node {
                key,
                left: Box::new(final_left),
                right: Box::new(final_right),
                rank: node_rank,
            }
        }

        /// Core meld operation - follows right spines (Data Structure 45.3 algorithm)
        /// This is the key innovation that makes meld O(log n)!
        fn meld_nodes(a: LeftistHeapNode<T>, b: LeftistHeapNode<T>) -> LeftistHeapNode<T> {
            match (a, b) {
                | (LeftistHeapNode::Leaf, other) => other,
                | (other, LeftistHeapNode::Leaf) => other,
                | (
                    LeftistHeapNode::Node {
                        key: ka,
                        left: la,
                        right: ra,
                        ..
                    },
                    LeftistHeapNode::Node {
                        key: kb,
                        left: lb,
                        right: rb,
                        ..
                    },
                ) => {
                    if ka <= kb {
                        // ka is smaller, so it becomes the root
                        // Meld ra (right subtree of a) with entire b
                        let melded_right = Self::meld_nodes(
                            *ra,
                            LeftistHeapNode::Node {
                                key: kb,
                                left: lb,
                                right: rb,
                                rank: 0, // rank will be recalculated
                            },
                        );
                        Self::make_node(ka, *la, melded_right)
                    } else {
                        // kb is smaller, so it becomes the root
                        // Meld entire a with rb (right subtree of b)
                        let melded_right = Self::meld_nodes(
                            LeftistHeapNode::Node {
                                key: ka,
                                left: la,
                                right: ra,
                                rank: 0, // rank will be recalculated
                            },
                            *rb,
                        );
                        Self::make_node(kb, *lb, melded_right)
                    }
                }
            }
        }

        /// Count total number of nodes in the heap
        fn size(&self) -> N {
            match self {
                | LeftistHeapNode::Leaf => 0,
                | LeftistHeapNode::Node { left, right, .. } => 1 + left.size() + right.size(),
            }
        }

        /// Get height of the heap
        fn height(&self) -> N {
            match self {
                | LeftistHeapNode::Leaf => 0,
                | LeftistHeapNode::Node { left, right, .. } => 1 + left.height().max(right.height()),
            }
        }

        /// Check if leftist property is maintained
        fn is_leftist(&self) -> bool {
            match self {
                | LeftistHeapNode::Leaf => true,
                | LeftistHeapNode::Node { left, right, .. } => {
                    left.rank() >= right.rank() && left.is_leftist() && right.is_leftist()
                }
            }
        }

        /// Check if heap property is maintained
        fn is_heap(&self) -> bool {
            match self {
                | LeftistHeapNode::Leaf => true,
                | LeftistHeapNode::Node { key, left, right, .. } => {
                    let left_ok = match left.as_ref() {
                        | LeftistHeapNode::Leaf => true,
                        | LeftistHeapNode::Node { key: left_key, .. } => key <= left_key,
                    };
                    let right_ok = match right.as_ref() {
                        | LeftistHeapNode::Leaf => true,
                        | LeftistHeapNode::Node { key: right_key, .. } => key <= right_key,
                    };
                    left_ok && right_ok && left.is_heap() && right.is_heap()
                }
            }
        }

        /// Convert to vector for testing (in-order traversal)
        fn to_vec(&self) -> Vec<T> {
            match self {
                | LeftistHeapNode::Leaf => Vec::new(),
                | LeftistHeapNode::Node { key, left, right, .. } => {
                    let mut result = left.to_vec();
                    result.push(key.clone());
                    result.extend(right.to_vec());
                    result
                }
            }
        }
    }

    impl<T: StT + Ord> LeftistHeapPQTrait<T> for LeftistHeapPQ<T> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn empty() -> Self {
            LeftistHeapPQ {
                root: LeftistHeapNode::Leaf,
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn singleton(element: T) -> Self {
            LeftistHeapPQ {
                root: LeftistHeapNode::Node {
                    key: element,
                    left: Box::new(LeftistHeapNode::Leaf),
                    right: Box::new(LeftistHeapNode::Leaf),
                    rank: 1,
                },
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Minimum is always at the root
        fn find_min(&self) -> Option<&T> {
            match &self.root {
                | LeftistHeapNode::Leaf => None,
                | LeftistHeapNode::Node { key, .. } => Some(key),
            }
        }

        /// Claude Work: Θ(log n), Span: Θ(log n)
        /// Create singleton and meld with existing heap
        fn insert(&self, element: T) -> Self {
            let singleton = Self::singleton(element);
            self.meld(&singleton)
        }

        /// Claude Work: Θ(log n), Span: Θ(log n)
        /// Remove root and meld left and right subtrees
        fn delete_min(&self) -> (Self, Option<T>) {
            match &self.root {
                | LeftistHeapNode::Leaf => (self.clone(), None),
                | LeftistHeapNode::Node { key, left, right, .. } => {
                    let min_element = key.clone();
                    let melded_root = LeftistHeapNode::meld_nodes(left.as_ref().clone(), right.as_ref().clone());
                    let new_heap = LeftistHeapPQ { root: melded_root };
                    (new_heap, Some(min_element))
                }
            }
        }

        /// Claude Work: Θ(log m + log n), Span: Θ(log m + log n)
        /// ⭐ THE STAR OPERATION: Efficient meld following right spines!
        /// This is what makes leftist heaps superior for applications requiring frequent melding
        fn meld(&self, other: &Self) -> Self {
            LeftistHeapPQ {
                root: LeftistHeapNode::meld_nodes(self.root.clone(), other.root.clone()),
            }
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Build heap using parallel reduce with meld operations
        fn from_seq(seq: &[T]) -> Self {
            if seq.is_empty() {
                return Self::empty();
            }

            // Create singletons for each element
            let mut heaps = seq.iter().map(|x| Self::singleton(x.clone())).collect::<Vec<Self>>();

            // Reduce using meld operations (can be done in parallel)
            while heaps.len() > 1 {
                let mut next_level = Vec::new();

                // Pair up heaps and meld them
                for chunk in heaps.chunks(2) {
                    if chunk.len() == 2 {
                        next_level.push(chunk[0].meld(&chunk[1]));
                    } else {
                        next_level.push(chunk[0].clone());
                    }
                }

                heaps = next_level;
            }

            heaps.into_iter().next().unwrap_or_else(Self::empty)
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        fn size(&self) -> N { self.root.size() }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn is_empty(&self) -> bool { matches!(self.root, LeftistHeapNode::Leaf) }

        fn extract_all_sorted(&self) -> Vec<T> {
            let mut result = Vec::new();
            let mut current_heap = self.clone();

            while !current_heap.is_empty() {
                let (new_heap, min_element) = current_heap.delete_min();
                if let Some(element) = min_element {
                    result.push(element);
                }
                current_heap = new_heap;
            }

            result
        }

        fn height(&self) -> N { self.root.height() }

        fn root_rank(&self) -> N { self.root.rank() }

        fn is_valid_leftist_heap(&self) -> bool { self.root.is_leftist() && self.root.is_heap() }

        fn from_vec(vec: Vec<T>) -> Self { Self::from_seq(&vec) }

        fn to_vec(&self) -> Vec<T> { self.root.to_vec() }

        fn to_sorted_vec(&self) -> Vec<T> { self.extract_all_sorted() }

        fn meld_multiple(heaps: &[Self]) -> Self {
            if heaps.is_empty() {
                return Self::empty();
            }

            let mut result = heaps[0].clone();
            for heap in &heaps[1..] {
                result = result.meld(heap);
            }
            result
        }

        fn split(&self, value: &T) -> (Self, Self) {
            let mut less_than = Self::empty();
            let mut equal_or_greater = Self::empty();

            let all_elements = self.to_vec();
            for element in all_elements {
                if element < *value {
                    less_than = less_than.insert(element);
                } else {
                    equal_or_greater = equal_or_greater.insert(element);
                }
            }

            (less_than, equal_or_greater)
        }
    }

    impl<T: StT + Ord> Default for LeftistHeapPQ<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> Display for LeftistHeapPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            fn format_node<T: StT + Ord>(node: &LeftistHeapNode<T>, f: &mut Formatter<'_>, depth: usize) -> Result {
                match node {
                    | LeftistHeapNode::Leaf => Ok(()),
                    | LeftistHeapNode::Node { key, left, right, rank } => {
                        let indent = "  ".repeat(depth);
                        writeln!(f, "{indent}{key}(rank:{rank})")?;
                        format_node(left, f, depth + 1)?;
                        format_node(right, f, depth + 1)?;
                        Ok(())
                    }
                }
            }

            writeln!(f, "LeftistHeapPQ:")?;
            format_node(&self.root, f, 0)
        }
    }

    // Macro for creating leftist heap priority queues
    #[macro_export]
    macro_rules! LeftistHeapPQLit {
        () => {
            $crate::Chap45::LeftistHeapPQ::LeftistHeapPQ::LeftistHeapPQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let elements = vec![$($x),*];
            $crate::Chap45::LeftistHeapPQ::LeftistHeapPQ::LeftistHeapPQ::from_seq(&elements)
        }};
    }

    /// Demonstrate the power of O(log n) meld operations
    /// This would be much slower with other priority queue implementations!
    pub fn efficient_multi_way_merge<T: StT + Ord>(sequences: Vec<Vec<T>>) -> Vec<T> {
        // Convert each sorted sequence to a leftist heap
        let heaps = sequences.into_iter().map(|seq| LeftistHeapPQ::from_seq(&seq)).collect::<Vec<LeftistHeapPQ<T>>>();

        // Meld all heaps together efficiently
        let merged_heap = LeftistHeapPQ::meld_multiple(&heaps);

        // Extract all elements in sorted order
        merged_heap.extract_all_sorted()
    }

    /// Demonstrate parallel heap construction
    pub fn parallel_heap_construction<T: StT + Ord>(elements: Vec<T>) -> LeftistHeapPQ<T> {
        // This could be parallelized using the reduce pattern
        LeftistHeapPQ::from_seq(&elements)
    }
}
