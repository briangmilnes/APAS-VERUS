//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices.

pub type T<S> = [S];

pub trait InsertionSortStTrait<T: Ord + Clone> {
    fn insSort(&self, slice: &mut [T]);
}

impl<T: Ord + Clone> InsertionSortStTrait<T> for T {
    fn insSort(&self, slice: &mut [T]) {
        for i in 1..slice.len() {
            let key = slice[i].clone();
            let mut j = i;
            while j > 0 && slice[j - 1] > key {
                slice[j] = slice[j - 1].clone();
                j -= 1;
            }
            slice[j] = key;
        }
    }
}
