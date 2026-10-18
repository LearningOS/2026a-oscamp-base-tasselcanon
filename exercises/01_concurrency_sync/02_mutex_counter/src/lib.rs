//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::env::consts::ARCH;
use std::sync::{Arc, Mutex};
use std::thread::{self, ScopedJoinHandle};

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    let data = Arc::new(Mutex::new(0));
    thread::scope(|s| {
        let mut thread_pool = vec![];
        for _ in 0..n_threads {
            let data = Arc::clone(&data);
            let handle = s.spawn(move || {
                *data.lock().unwrap() += count_per_thread;
            });
            thread_pool.push(handle);
        }
        thread_pool.into_iter().map(|thread| thread.join().unwrap());
    });
    let x = *data.lock().unwrap();
    x
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    let data: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    thread::scope(|s| {
        let mut thread_pool = Vec::new();
        for index in 0..n_threads {
            let data = Arc::clone(&data);
            let handle = s.spawn(move || {
                data.lock().unwrap().push(index);
            });
            thread_pool.push(handle);
        }
        thread_pool.into_iter().map(|thread| thread.join().unwrap());
    });
    let mut res = data.lock().unwrap().clone();
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
