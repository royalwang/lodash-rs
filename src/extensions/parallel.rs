/*!
Parallel processing extensions for Lodash-RS.

This module provides parallel versions of collection methods using the rayon crate
for automatic parallelization of operations on large datasets.
*/

#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "parallel")]
use crate::collection::Collection;
#[cfg(feature = "parallel")]
use crate::utils::{LodashError, Result};

#[cfg(feature = "parallel")]
/// Parallel version of `map`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::map_parallel;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let doubled = map_parallel(&numbers, |x| x * 2);
/// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
/// ```
pub fn map_parallel<T, U, F>(collection: &[T], iteratee: F) -> Vec<U>
where
    T: Send + Sync,
    U: Send,
    F: Fn(&T) -> U + Sync,
{
    collection.par_iter().map(iteratee).collect()
}

#[cfg(feature = "parallel")]
/// Parallel version of `filter`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::filter_parallel;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let evens = filter_parallel(&numbers, |x| x % 2 == 0);
/// assert_eq!(evens, vec![2, 4]);
/// ```
pub fn filter_parallel<T, F>(collection: &[T], predicate: F) -> Vec<T>
where
    T: Clone + Send + Sync,
    F: Fn(&T) -> bool + Sync,
{
    collection.par_iter()
        .filter(|item| predicate(item))
        .cloned()
        .collect()
}

#[cfg(feature = "parallel")]
/// Parallel version of `reduce`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::reduce_parallel;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let sum = reduce_parallel(&numbers, |acc, x| acc + x, 0);
/// assert_eq!(sum, 15);
/// ```
pub fn reduce_parallel<T, U, F>(collection: &[T], iteratee: F, initial: U) -> U
where
    T: Send + Sync,
    U: Send + Sync,
    F: Fn(U, &T) -> U + Sync,
{
    collection.par_iter().fold(|| initial, |acc, item| iteratee(acc, item)).reduce(|| initial, |a, b| a)
}

#[cfg(feature = "parallel")]
/// Parallel version of `for_each`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::for_each_parallel;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let mut sum = 0;
/// for_each_parallel(&numbers, |x| sum += x);
/// assert_eq!(sum, 15);
/// ```
pub fn for_each_parallel<T, F>(collection: &[T], iteratee: F)
where
    T: Send + Sync,
    F: Fn(&T) + Sync,
{
    collection.par_iter().for_each(iteratee);
}

#[cfg(feature = "parallel")]
/// Parallel version of `find`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::find_parallel;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let first_even = find_parallel(&numbers, |x| x % 2 == 0);
/// assert_eq!(first_even, Some(&2));
/// ```
pub fn find_parallel<T, F>(collection: &[T], predicate: F) -> Option<&T>
where
    T: Send + Sync,
    F: Fn(&T) -> bool + Sync,
{
    collection.par_iter().find_any(|item| predicate(item))
}

#[cfg(feature = "parallel")]
/// Parallel version of `every`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::every_parallel;
/// 
/// let numbers = vec![2, 4, 6, 8];
/// let all_even = every_parallel(&numbers, |x| x % 2 == 0);
/// assert!(all_even);
/// ```
pub fn every_parallel<T, F>(collection: &[T], predicate: F) -> bool
where
    T: Send + Sync,
    F: Fn(&T) -> bool + Sync,
{
    collection.par_iter().all(predicate)
}

#[cfg(feature = "parallel")]
/// Parallel version of `some`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::extensions::parallel::some_parallel;
/// 
/// let numbers = vec![1, 3, 5, 7];
/// let has_even = some_parallel(&numbers, |x| x % 2 == 0);
/// assert!(!has_even);
/// ```
pub fn some_parallel<T, F>(collection: &[T], predicate: F) -> bool
where
    T: Send + Sync,
    F: Fn(&T) -> bool + Sync,
{
    collection.par_iter().any(predicate)
}

#[cfg(feature = "parallel")]
/// Collection methods that work on the `Collection` type.
impl<T> Collection<T> {
    /// Parallel version of `map`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let doubled = collection.map_parallel(|x| x * 2);
    /// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    /// ```
    pub fn map_parallel<U, F>(&self, iteratee: F) -> Vec<U>
    where
        T: Send + Sync,
        U: Send,
        F: Fn(&T) -> U + Sync,
    {
        map_parallel(&self.data, iteratee)
    }

    /// Parallel version of `filter`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let evens = collection.filter_parallel(|x| x % 2 == 0);
    /// assert_eq!(evens, vec![2, 4]);
    /// ```
    pub fn filter_parallel<F>(&self, predicate: F) -> Vec<T>
    where
        T: Clone + Send + Sync,
        F: Fn(&T) -> bool + Sync,
    {
        filter_parallel(&self.data, predicate)
    }

    /// Parallel version of `reduce`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let sum = collection.reduce_parallel(|acc, x| acc + x, 0);
    /// assert_eq!(sum, 15);
    /// ```
    pub fn reduce_parallel<U, F>(&self, iteratee: F, initial: U) -> U
    where
        T: Send + Sync,
        U: Send + Sync,
        F: Fn(U, &T) -> U + Sync,
    {
        reduce_parallel(&self.data, iteratee, initial)
    }

    /// Parallel version of `for_each`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let mut sum = 0;
    /// collection.for_each_parallel(|x| sum += x);
    /// assert_eq!(sum, 15);
    /// ```
    pub fn for_each_parallel<F>(&self, iteratee: F)
    where
        T: Send + Sync,
        F: Fn(&T) + Sync,
    {
        for_each_parallel(&self.data, iteratee)
    }

    /// Parallel version of `find`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let first_even = collection.find_parallel(|x| x % 2 == 0);
    /// assert_eq!(first_even, Some(&2));
    /// ```
    pub fn find_parallel<F>(&self, predicate: F) -> Option<&T>
    where
        T: Send + Sync,
        F: Fn(&T) -> bool + Sync,
    {
        find_parallel(&self.data, predicate)
    }

    /// Parallel version of `every`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![2, 4, 6, 8]);
    /// let all_even = collection.every_parallel(|x| x % 2 == 0);
    /// assert!(all_even);
    /// ```
    pub fn every_parallel<F>(&self, predicate: F) -> bool
    where
        T: Send + Sync,
        F: Fn(&T) -> bool + Sync,
    {
        every_parallel(&self.data, predicate)
    }

    /// Parallel version of `some`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 3, 5, 7]);
    /// let has_even = collection.some_parallel(|x| x % 2 == 0);
    /// assert!(!has_even);
    /// ```
    pub fn some_parallel<F>(&self, predicate: F) -> bool
    where
        T: Send + Sync,
        F: Fn(&T) -> bool + Sync,
    {
        some_parallel(&self.data, predicate)
    }
}

#[cfg(test)]
#[cfg(feature = "parallel")]
mod tests {
    use super::*;

    #[test]
    fn test_map_parallel() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled = map_parallel(&numbers, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_filter_parallel() {
        let numbers = vec![1, 2, 3, 4, 5];
        let evens = filter_parallel(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_reduce_parallel() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = reduce_parallel(&numbers, |acc, x| acc + x, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_for_each_parallel() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        for_each_parallel(&numbers, |x| sum += x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_find_parallel() {
        let numbers = vec![1, 2, 3, 4, 5];
        let first_even = find_parallel(&numbers, |x| x % 2 == 0);
        assert_eq!(first_even, Some(&2));
    }

    #[test]
    fn test_every_parallel() {
        let numbers = vec![2, 4, 6, 8];
        let all_even = every_parallel(&numbers, |x| x % 2 == 0);
        assert!(all_even);
    }

    #[test]
    fn test_some_parallel() {
        let numbers = vec![1, 3, 5, 7];
        let has_even = some_parallel(&numbers, |x| x % 2 == 0);
        assert!(!has_even);
    }

    #[test]
    fn test_collection_map_parallel() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let doubled = collection.map_parallel(|x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_collection_filter_parallel() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let evens = collection.filter_parallel(|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_collection_reduce_parallel() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let sum = collection.reduce_parallel(|acc, x| acc + x, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_collection_for_each_parallel() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let mut sum = 0;
        collection.for_each_parallel(|x| sum += x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_collection_find_parallel() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let first_even = collection.find_parallel(|x| x % 2 == 0);
        assert_eq!(first_even, Some(&2));
    }

    #[test]
    fn test_collection_every_parallel() {
        let collection = Collection::new(vec![2, 4, 6, 8]);
        let all_even = collection.every_parallel(|x| x % 2 == 0);
        assert!(all_even);
    }

    #[test]
    fn test_collection_some_parallel() {
        let collection = Collection::new(vec![1, 3, 5, 7]);
        let has_even = collection.some_parallel(|x| x % 2 == 0);
        assert!(!has_even);
    }

    #[test]
    fn test_empty_collection_parallel() {
        let empty: Vec<i32> = vec![];
        let doubled = map_parallel(&empty, |x| x * 2);
        assert!(doubled.is_empty());

        let evens = filter_parallel(&empty, |x| x % 2 == 0);
        assert!(evens.is_empty());

        let sum = reduce_parallel(&empty, |acc, x| acc + x, 0);
        assert_eq!(sum, 0);

        let first_even = find_parallel(&empty, |x| x % 2 == 0);
        assert_eq!(first_even, None);

        let all_even = every_parallel(&empty, |x| x % 2 == 0);
        assert!(all_even); // vacuous truth

        let has_even = some_parallel(&empty, |x| x % 2 == 0);
        assert!(!has_even); // vacuous false
    }
}
