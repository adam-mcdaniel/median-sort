//! This crate implements median of medians quicksort.
use rand::prelude::*;
use std::ops::Range;

/// Sorts the given slice using insertion sort.
pub fn insertion_sort<T>(arr: &mut [T]) where T: Ord + Copy {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn median<T>(arr: &mut [T]) -> T where T: Ord + Copy {
    insertion_sort(arr);
    arr[arr.len() / 2]
}

pub fn immut_median<T>(arr: &[T]) -> T where T: Ord + Copy {
    let mut temp = arr.to_vec();
    median(&mut temp)
}

pub fn median_of_medians<T>(arr: &mut [T], r: usize) -> T where T: Ord + Copy {
    if arr.len() <= r {
        return immut_median(arr);
    }

    let mut medians = Vec::new();
    for chunk in arr.chunks_mut(r) {
        let median = immut_median(chunk);
        medians.push(median);
    }

    median_of_medians(&mut medians, r)
}

pub fn quicksort<T>(arr: &mut [T], r: usize, cutoff: usize) where T: Ord + Copy {
    if arr.len() <= 1 {
        return;
    }
    if arr.len() <= cutoff {
        insertion_sort(arr);
        return;
    }
    
    let pivot = median_of_medians(arr, r);

    let (mut left, mut right): (Vec<_>, Vec<_>) = arr.iter().copied().partition(|x| *x < pivot);
    right = right.into_iter().filter(|x| *x != pivot).collect();
    let pivots: Vec<_> = arr.iter().filter(|x| *x == &pivot).copied().collect();

    quicksort(&mut left, r, cutoff);
    quicksort(&mut right, r, cutoff);

    arr.copy_from_slice(&[left, pivots, right].concat());
}

pub fn random_array(n: usize, range: Range<i32>) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(range.clone())).collect()
}
