/*!
Iteration methods for Lodash-RS.

This module provides iteration methods like `each`, `map`, `filter`, `reduce`, etc.
These are the core methods for processing collections.
*/

use crate::collection::Collection;
use crate::utils::{LodashError, Result, Predicate, Mapper, Reducer};

/// Iterate over elements of a collection, executing a function for each element.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::each;
/// 
/// let mut sum = 0;
/// each(&[1, 2, 3], |x| sum += x);
/// assert_eq!(sum, 6);
/// ```
pub fn each<T, F>(collection: &[T], mut iteratee: F)
where
    F: FnMut(&T),
{
    for item in collection {
        iteratee(item);
    }
}

/// Alias for `each`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::for_each;
/// 
/// let mut sum = 0;
/// for_each(&[1, 2, 3], |x| sum += x);
/// assert_eq!(sum, 6);
/// ```
pub fn for_each<T, F>(collection: &[T], iteratee: F)
where
    F: FnMut(&T),
{
    each(collection, iteratee);
}

/// Create an array of values by running each element in collection through iteratee.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::map;
/// 
/// let doubled = map(&[1, 2, 3], |x| x * 2);
/// assert_eq!(doubled, vec![2, 4, 6]);
/// 
/// // Type conversion
/// let strings = map(&[1, 2, 3], |x| x.to_string());
/// assert_eq!(strings, vec!["1", "2", "3"]);
/// ```
pub fn map<T, U, F>(collection: &[T], iteratee: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    collection.iter().map(iteratee).collect()
}

/// Iterate over elements of collection, returning an array of all elements
/// the predicate returns truthy for.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::filter;
/// 
/// let evens = filter(&[1, 2, 3, 4, 5], |x| x % 2 == 0);
/// assert_eq!(evens, vec![2, 4]);
/// ```
pub fn filter<T, F>(collection: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    collection.iter()
        .filter(|item| predicate(item))
        .cloned()
        .collect()
}

/// Reduce collection to a value which is the accumulated result of running
/// each element in collection through iteratee, where each successive
/// invocation is supplied the return value of the previous.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::reduce;
/// 
/// let sum = reduce(&[1, 2, 3, 4], |acc, x| acc + x, 0);
/// assert_eq!(sum, 10);
/// 
/// // String concatenation
/// let result = reduce(&["a", "b", "c"], |acc, x| format!("{}{}", acc, x), String::new());
/// assert_eq!(result, "abc");
/// ```
pub fn reduce<T, U, F>(collection: &[T], iteratee: F, initial: U) -> U
where
    F: Fn(U, &T) -> U,
{
    collection.iter().fold(initial, iteratee)
}

/// This method is like `reduce` except that it iterates over elements of
/// collection from right to left.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::reduce_right;
/// 
/// let result = reduce_right(&["a", "b", "c"], |acc, x| format!("{}{}", acc, x), String::new());
/// assert_eq!(result, "cba");
/// ```
pub fn reduce_right<T, U, F>(collection: &[T], iteratee: F, initial: U) -> U
where
    F: Fn(U, &T) -> U,
{
    collection.iter().rev().fold(initial, iteratee)
}

/// This method is like `each` except that it iterates over elements of
/// collection from right to left.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::iteration::for_each_right;
/// 
/// let mut result = Vec::new();
/// for_each_right(&[1, 2, 3], |x| result.push(*x));
/// assert_eq!(result, vec![3, 2, 1]);
/// ```
pub fn for_each_right<T, F>(collection: &[T], mut iteratee: F)
where
    F: FnMut(&T),
{
    for item in collection.iter().rev() {
        iteratee(item);
    }
}

/// Collection methods that work on the `Collection` type.
impl<T> Collection<T> {
    /// Iterate over elements of the collection, executing a function for each element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// let mut sum = 0;
    /// collection.each(|x| sum += x);
    /// assert_eq!(sum, 6);
    /// ```
    pub fn each<F>(&self, iteratee: F)
    where
        F: FnMut(&T),
    {
        each(&self.data, iteratee);
    }

    /// Create an array of values by running each element through iteratee.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// let doubled = collection.map(|x| x * 2);
    /// assert_eq!(doubled, vec![2, 4, 6]);
    /// ```
    pub fn map<U, F>(&self, iteratee: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        map(&self.data, iteratee)
    }

    /// Iterate over elements, returning an array of all elements
    /// the predicate returns truthy for.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let evens = collection.filter(|x| x % 2 == 0);
    /// assert_eq!(evens, vec![2, 4]);
    /// ```
    pub fn filter<F>(&self, predicate: F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> bool,
    {
        filter(&self.data, predicate)
    }

    /// Reduce the collection to a value which is the accumulated result
    /// of running each element through iteratee.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4]);
    /// let sum = collection.reduce(|acc, x| acc + x, 0);
    /// assert_eq!(sum, 10);
    /// ```
    pub fn reduce<U, F>(&self, iteratee: F, initial: U) -> U
    where
        F: Fn(U, &T) -> U,
    {
        reduce(&self.data, iteratee, initial)
    }

    /// This method is like `reduce` except that it iterates from right to left.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec!["a", "b", "c"]);
    /// let result = collection.reduce_right(|acc, x| format!("{}{}", acc, x), String::new());
    /// assert_eq!(result, "cba");
    /// ```
    pub fn reduce_right<U, F>(&self, iteratee: F, initial: U) -> U
    where
        F: Fn(U, &T) -> U,
    {
        reduce_right(&self.data, iteratee, initial)
    }

    /// This method is like `each` except that it iterates from right to left.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// let mut result = Vec::new();
    /// collection.for_each_right(|x| result.push(*x));
    /// assert_eq!(result, vec![3, 2, 1]);
    /// ```
    pub fn for_each_right<F>(&self, iteratee: F)
    where
        F: FnMut(&T),
    {
        for_each_right(&self.data, iteratee);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_each() {
        let mut sum = 0;
        each(&[1, 2, 3], |x| sum += x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_for_each() {
        let mut sum = 0;
        for_each(&[1, 2, 3], |x| sum += x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_map() {
        let doubled = map(&[1, 2, 3], |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);

        let strings = map(&[1, 2, 3], |x| x.to_string());
        assert_eq!(strings, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_filter() {
        let evens = filter(&[1, 2, 3, 4, 5], |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_reduce() {
        let sum = reduce(&[1, 2, 3, 4], |acc, x| acc + x, 0);
        assert_eq!(sum, 10);

        let result = reduce(&["a", "b", "c"], |acc, x| format!("{}{}", acc, x), String::new());
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_reduce_right() {
        let result = reduce_right(&["a", "b", "c"], |acc, x| format!("{}{}", acc, x), String::new());
        assert_eq!(result, "cba");
    }

    #[test]
    fn test_for_each_right() {
        let mut result = Vec::new();
        for_each_right(&[1, 2, 3], |x| result.push(*x));
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_collection_each() {
        let collection = Collection::new(vec![1, 2, 3]);
        let mut sum = 0;
        collection.each(|x| sum += x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_collection_map() {
        let collection = Collection::new(vec![1, 2, 3]);
        let doubled = collection.map(|x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_collection_filter() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let evens = collection.filter(|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_collection_reduce() {
        let collection = Collection::new(vec![1, 2, 3, 4]);
        let sum = collection.reduce(|acc, x| acc + x, 0);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_collection_reduce_right() {
        let collection = Collection::new(vec!["a", "b", "c"]);
        let result = collection.reduce_right(|acc, x| format!("{}{}", acc, x), String::new());
        assert_eq!(result, "cba");
    }

    #[test]
    fn test_collection_for_each_right() {
        let collection = Collection::new(vec![1, 2, 3]);
        let mut result = Vec::new();
        collection.for_each_right(|x| result.push(*x));
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = map(&empty, |x| x * 2);
        assert!(result.is_empty());

        let sum = reduce(&empty, |acc, x| acc + x, 0);
        assert_eq!(sum, 0);
    }
}
