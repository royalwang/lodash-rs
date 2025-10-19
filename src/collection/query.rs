/*!
Query methods for Lodash-RS.

This module provides query methods like `find`, `includes`, `every`, `some`, etc.
These methods are used to search and test elements in collections.
*/

use crate::collection::Collection;
use crate::utils::{LodashError, Result, Predicate};

/// Iterate over elements of collection, returning the first element
/// the predicate returns truthy for.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::find;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let first_even = find(&numbers, |x| x % 2 == 0);
/// assert_eq!(first_even, Some(&2));
/// 
/// let not_found = find(&numbers, |x| *x > 10);
/// assert_eq!(not_found, None);
/// ```
pub fn find<T, F>(collection: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    collection.iter().find(|item| predicate(item))
}

/// This method is like `find` except that it iterates over elements of
/// collection from right to left.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::find_last;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let last_even = find_last(&numbers, |x| x % 2 == 0);
/// assert_eq!(last_even, Some(&4));
/// ```
pub fn find_last<T, F>(collection: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    collection.iter().rev().find(|item| predicate(item))
}

/// Check if value is in collection.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::includes;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert!(includes(&numbers, &3));
/// assert!(!includes(&numbers, &6));
/// ```
pub fn includes<T>(collection: &[T], value: &T) -> bool
where
    T: PartialEq,
{
    collection.contains(value)
}

/// Check if predicate returns truthy for all elements of collection.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::every;
/// 
/// let numbers = vec![2, 4, 6, 8];
/// assert!(every(&numbers, |x| x % 2 == 0));
/// 
/// let mixed = vec![2, 4, 5, 8];
/// assert!(!every(&mixed, |x| x % 2 == 0));
/// ```
pub fn every<T, F>(collection: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    collection.iter().all(predicate)
}

/// Check if predicate returns truthy for any element of collection.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::some;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert!(some(&numbers, |x| x % 2 == 0));
/// 
/// let odds = vec![1, 3, 5, 7];
/// assert!(!some(&odds, |x| x % 2 == 0));
/// ```
pub fn some<T, F>(collection: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    collection.iter().any(predicate)
}

/// Create an object composed of keys generated from the results of running
/// each element of collection through iteratee. The corresponding value of
/// each key is the number of times the key was returned by iteratee.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::count_by;
/// use std::collections::HashMap;
/// 
/// let numbers = vec![6.1, 4.2, 6.3];
/// let counts = count_by(&numbers, |x| (*x as f64).floor() as i32);
/// assert_eq!(counts.get(&6), Some(&2));
/// assert_eq!(counts.get(&4), Some(&1));
/// ```
pub fn count_by<T, K, F>(collection: &[T], iteratee: F) -> std::collections::HashMap<K, usize>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut counts = std::collections::HashMap::new();
    for item in collection {
        let key = iteratee(item);
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

/// Create an array of elements split into two groups, the first of which
/// contains elements the predicate returns truthy for, while the second
/// contains elements the predicate returns falsy for.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::query::partition;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let (evens, odds) = partition(&numbers, |x| x % 2 == 0);
/// assert_eq!(evens, vec![2, 4]);
/// assert_eq!(odds, vec![1, 3, 5]);
/// ```
pub fn partition<T, F>(collection: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut truthy = Vec::new();
    let mut falsy = Vec::new();
    
    for item in collection {
        if predicate(item) {
            truthy.push(item.clone());
        } else {
            falsy.push(item.clone());
        }
    }
    
    (truthy, falsy)
}

/// Collection methods that work on the `Collection` type.
impl<T> Collection<T> {
    /// Iterate over elements, returning the first element
    /// the predicate returns truthy for.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let first_even = collection.find(|x| x % 2 == 0);
    /// assert_eq!(first_even, Some(&2));
    /// ```
    pub fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        find(&self.data, predicate)
    }

    /// This method is like `find` except that it iterates from right to left.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let last_even = collection.find_last(|x| x % 2 == 0);
    /// assert_eq!(last_even, Some(&4));
    /// ```
    pub fn find_last<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        find_last(&self.data, predicate)
    }

    /// Check if value is in the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// assert!(collection.includes(&3));
    /// assert!(!collection.includes(&6));
    /// ```
    pub fn includes(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        includes(&self.data, value)
    }

    /// Check if predicate returns truthy for all elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![2, 4, 6, 8]);
    /// assert!(collection.every(|x| x % 2 == 0));
    /// ```
    pub fn every<F>(&self, predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        every(&self.data, predicate)
    }

    /// Check if predicate returns truthy for any element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// assert!(collection.some(|x| x % 2 == 0));
    /// ```
    pub fn some<F>(&self, predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        some(&self.data, predicate)
    }

    /// Create an object composed of keys generated from the results of running
    /// each element through iteratee.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// use std::collections::HashMap;
    /// 
    /// let collection = Collection::new(vec![6.1, 4.2, 6.3]);
    /// let counts = collection.count_by(|x| (*x as f64).floor() as i32);
    /// assert_eq!(counts.get(&6), Some(&2));
    /// ```
    pub fn count_by<K, F>(&self, iteratee: F) -> std::collections::HashMap<K, usize>
    where
        K: std::hash::Hash + Eq,
        F: Fn(&T) -> K,
    {
        count_by(&self.data, iteratee)
    }

    /// Create an array of elements split into two groups.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let (evens, odds) = collection.partition(|x| x % 2 == 0);
    /// assert_eq!(evens, vec![2, 4]);
    /// assert_eq!(odds, vec![1, 3, 5]);
    /// ```
    pub fn partition<F>(&self, predicate: F) -> (Vec<T>, Vec<T>)
    where
        T: Clone,
        F: Fn(&T) -> bool,
    {
        partition(&self.data, predicate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_find() {
        let numbers = vec![1, 2, 3, 4, 5];
        let first_even = find(&numbers, |x| x % 2 == 0);
        assert_eq!(first_even, Some(&2));

        let not_found = find(&numbers, |x| *x > 10);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_find_last() {
        let numbers = vec![1, 2, 3, 4, 5];
        let last_even = find_last(&numbers, |x| x % 2 == 0);
        assert_eq!(last_even, Some(&4));
    }

    #[test]
    fn test_includes() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(includes(&numbers, &3));
        assert!(!includes(&numbers, &6));
    }

    #[test]
    fn test_every() {
        let numbers = vec![2, 4, 6, 8];
        assert!(every(&numbers, |x| x % 2 == 0));

        let mixed = vec![2, 4, 5, 8];
        assert!(!every(&mixed, |x| x % 2 == 0));
    }

    #[test]
    fn test_some() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(some(&numbers, |x| x % 2 == 0));

        let odds = vec![1, 3, 5, 7];
        assert!(!some(&odds, |x| x % 2 == 0));
    }

    #[test]
    fn test_count_by() {
        let numbers = vec![6.1, 4.2, 6.3];
        let counts = count_by(&numbers, |x| (*x as f64).floor() as i32);
        assert_eq!(counts.get(&6), Some(&2));
        assert_eq!(counts.get(&4), Some(&1));
    }

    #[test]
    fn test_partition() {
        let numbers = vec![1, 2, 3, 4, 5];
        let (evens, odds) = partition(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_collection_find() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let first_even = collection.find(|x| x % 2 == 0);
        assert_eq!(first_even, Some(&2));
    }

    #[test]
    fn test_collection_find_last() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let last_even = collection.find_last(|x| x % 2 == 0);
        assert_eq!(last_even, Some(&4));
    }

    #[test]
    fn test_collection_includes() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        assert!(collection.includes(&3));
        assert!(!collection.includes(&6));
    }

    #[test]
    fn test_collection_every() {
        let collection = Collection::new(vec![2, 4, 6, 8]);
        assert!(collection.every(|x| x % 2 == 0));
    }

    #[test]
    fn test_collection_some() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        assert!(collection.some(|x| x % 2 == 0));
        
        let odds_collection = Collection::new(vec![1, 3, 5, 7]);
        assert!(!odds_collection.some(|x| x % 2 == 0));
    }

    #[test]
    fn test_collection_count_by() {
        let collection = Collection::new(vec![6.1, 4.2, 6.3]);
        let counts = collection.count_by(|x| (*x as f64).floor() as i32);
        assert_eq!(counts.get(&6), Some(&2));
    }

    #[test]
    fn test_collection_partition() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let (evens, odds) = collection.partition(|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_empty_collection() {
        let empty: Vec<i32> = vec![];
        assert_eq!(find(&empty, |x| x % 2 == 0), None);
        assert!(every(&empty, |x| x % 2 == 0)); // vacuous truth
        assert!(!some(&empty, |x| x % 2 == 0)); // vacuous false
        assert!(!includes(&empty, &1));
    }
}
