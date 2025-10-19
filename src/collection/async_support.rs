/*!
Async support for collection methods.

This module provides async versions of all collection methods, enabling
non-blocking operations on large datasets.
*/

#[cfg(feature = "async")]
use futures::future::{join_all, Future};
#[cfg(feature = "async")]
use crate::collection::Collection;
#[cfg(feature = "async")]
use crate::utils::{LodashError, Result, AsyncPredicate, AsyncMapper, AsyncReducer};

#[cfg(feature = "async")]
/// Async version of `map`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::map_async;
/// 
/// # async fn example() {
/// let numbers = vec![1, 2, 3, 4, 5];
/// let doubled = map_async(&numbers, |x| async move { x * 2 }).await;
/// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
/// # }
/// ```
pub async fn map_async<T, U, F, Fut>(collection: &[T], iteratee: F) -> Vec<U>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = U>,
{
    let futures = collection.iter().map(|item| iteratee(item));
    join_all(futures).await
}

#[cfg(feature = "async")]
/// Async version of `filter`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::filter_async;
/// 
/// # async fn example() {
/// let numbers = vec![1, 2, 3, 4, 5];
/// let evens = filter_async(&numbers, |x| async move { x % 2 == 0 }).await;
/// assert_eq!(evens, vec![2, 4]);
/// # }
/// ```
pub async fn filter_async<T, F, Fut>(collection: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> Fut,
    Fut: Future<Output = bool>,
{
    let mut results = Vec::new();
    let futures = collection.iter().map(|item| (item, predicate(item)));
    let predicate_results = join_all(futures.map(|(item, fut)| async move { (item, fut.await) })).await;
    
    for (item, should_include) in predicate_results {
        if should_include {
            results.push(item.clone());
        }
    }
    
    results
}

#[cfg(feature = "async")]
/// Async version of `reduce`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::reduce_async;
/// 
/// # async fn example() {
/// let numbers = vec![1, 2, 3, 4, 5];
/// let sum = reduce_async(&numbers, |acc, x| async move { acc + x }, 0).await;
/// assert_eq!(sum, 15);
/// # }
/// ```
pub async fn reduce_async<T, U, F, Fut>(collection: &[T], iteratee: F, initial: U) -> U
where
    F: Fn(U, &T) -> Fut,
    Fut: Future<Output = U>,
{
    let mut acc = initial;
    for item in collection {
        acc = iteratee(acc, item).await;
    }
    acc
}

#[cfg(feature = "async")]
/// Async version of `for_each`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::for_each_async;
/// 
/// # async fn example() {
/// let numbers = vec![1, 2, 3, 4, 5];
/// let mut sum = 0;
/// for_each_async(&numbers, |x| async move { sum += x }).await;
/// assert_eq!(sum, 15);
/// # }
/// ```
pub async fn for_each_async<T, F, Fut>(collection: &[T], iteratee: F)
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = ()>,
{
    let futures = collection.iter().map(|item| iteratee(item));
    join_all(futures).await;
}

#[cfg(feature = "async")]
/// Async version of `find`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::find_async;
/// 
/// # async fn example() {
/// let numbers = vec![1, 2, 3, 4, 5];
/// let first_even = find_async(&numbers, |x| async move { x % 2 == 0 }).await;
/// assert_eq!(first_even, Some(&2));
/// # }
/// ```
pub async fn find_async<T, F, Fut>(collection: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = bool>,
{
    for item in collection {
        if predicate(item).await {
            return Some(item);
        }
    }
    None
}

#[cfg(feature = "async")]
/// Async version of `every`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::every_async;
/// 
/// # async fn example() {
/// let numbers = vec![2, 4, 6, 8];
/// let all_even = every_async(&numbers, |x| async move { x % 2 == 0 }).await;
/// assert!(all_even);
/// # }
/// ```
pub async fn every_async<T, F, Fut>(collection: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = bool>,
{
    for item in collection {
        if !predicate(item).await {
            return false;
        }
    }
    true
}

#[cfg(feature = "async")]
/// Async version of `some`.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::async_support::some_async;
/// 
/// # async fn example() {
/// let numbers = vec![1, 3, 5, 7];
/// let has_even = some_async(&numbers, |x| async move { x % 2 == 0 }).await;
/// assert!(!has_even);
/// # }
/// ```
pub async fn some_async<T, F, Fut>(collection: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = bool>,
{
    for item in collection {
        if predicate(item).await {
            return true;
        }
    }
    false
}

#[cfg(feature = "async")]
/// Collection methods that work on the `Collection` type.
impl<T> Collection<T> {
    /// Async version of `map`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let doubled = collection.map_async(|x| async move { x * 2 }).await;
    /// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    /// # }
    /// ```
    pub async fn map_async<U, F, Fut>(&self, iteratee: F) -> Vec<U>
    where
        F: Fn(&T) -> Fut,
        Fut: Future<Output = U>,
    {
        map_async(&self.data, iteratee).await
    }

    /// Async version of `filter`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let evens = collection.filter_async(|x| async move { x % 2 == 0 }).await;
    /// assert_eq!(evens, vec![2, 4]);
    /// # }
    /// ```
    pub async fn filter_async<F, Fut>(&self, predicate: F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> Fut,
        Fut: Future<Output = bool>,
    {
        filter_async(&self.data, predicate).await
    }

    /// Async version of `reduce`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let sum = collection.reduce_async(|acc, x| async move { acc + x }, 0).await;
    /// assert_eq!(sum, 15);
    /// # }
    /// ```
    pub async fn reduce_async<U, F, Fut>(&self, iteratee: F, initial: U) -> U
    where
        F: Fn(U, &T) -> Fut,
        Fut: Future<Output = U>,
    {
        reduce_async(&self.data, iteratee, initial).await
    }

    /// Async version of `for_each`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let mut sum = 0;
    /// collection.for_each_async(|x| async move { sum += x }).await;
    /// assert_eq!(sum, 15);
    /// # }
    /// ```
    pub async fn for_each_async<F, Fut>(&self, iteratee: F)
    where
        F: Fn(&T) -> Fut,
        Fut: Future<Output = ()>,
    {
        for_each_async(&self.data, iteratee).await
    }

    /// Async version of `find`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let first_even = collection.find_async(|x| async move { x % 2 == 0 }).await;
    /// assert_eq!(first_even, Some(&2));
    /// # }
    /// ```
    pub async fn find_async<F, Fut>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> Fut,
        Fut: Future<Output = bool>,
    {
        find_async(&self.data, predicate).await
    }

    /// Async version of `every`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![2, 4, 6, 8]);
    /// let all_even = collection.every_async(|x| async move { x % 2 == 0 }).await;
    /// assert!(all_even);
    /// # }
    /// ```
    pub async fn every_async<F, Fut>(&self, predicate: F) -> bool
    where
        F: Fn(&T) -> Fut,
        Fut: Future<Output = bool>,
    {
        every_async(&self.data, predicate).await
    }

    /// Async version of `some`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// # async fn example() {
    /// let collection = Collection::new(vec![1, 3, 5, 7]);
    /// let has_even = collection.some_async(|x| async move { x % 2 == 0 }).await;
    /// assert!(!has_even);
    /// # }
    /// ```
    pub async fn some_async<F, Fut>(&self, predicate: F) -> bool
    where
        F: Fn(&T) -> Fut,
        Fut: Future<Output = bool>,
    {
        some_async(&self.data, predicate).await
    }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_map_async() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled = map_async(&numbers, |x| async move { x * 2 }).await;
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_filter_async() {
        let numbers = vec![1, 2, 3, 4, 5];
        let evens = filter_async(&numbers, |x| async move { x % 2 == 0 }).await;
        assert_eq!(evens, vec![2, 4]);
    }

    #[tokio::test]
    async fn test_reduce_async() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = reduce_async(&numbers, |acc, x| async move { acc + x }, 0).await;
        assert_eq!(sum, 15);
    }

    #[tokio::test]
    async fn test_for_each_async() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        for_each_async(&numbers, |x| async move { sum += x }).await;
        assert_eq!(sum, 15);
    }

    #[tokio::test]
    async fn test_find_async() {
        let numbers = vec![1, 2, 3, 4, 5];
        let first_even = find_async(&numbers, |x| async move { x % 2 == 0 }).await;
        assert_eq!(first_even, Some(&2));
    }

    #[tokio::test]
    async fn test_every_async() {
        let numbers = vec![2, 4, 6, 8];
        let all_even = every_async(&numbers, |x| async move { x % 2 == 0 }).await;
        assert!(all_even);
    }

    #[tokio::test]
    async fn test_some_async() {
        let numbers = vec![1, 3, 5, 7];
        let has_even = some_async(&numbers, |x| async move { x % 2 == 0 }).await;
        assert!(!has_even);
    }

    #[tokio::test]
    async fn test_collection_map_async() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let doubled = collection.map_async(|x| async move { x * 2 }).await;
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_collection_filter_async() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let evens = collection.filter_async(|x| async move { x % 2 == 0 }).await;
        assert_eq!(evens, vec![2, 4]);
    }

    #[tokio::test]
    async fn test_collection_reduce_async() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let sum = collection.reduce_async(|acc, x| async move { acc + x }, 0).await;
        assert_eq!(sum, 15);
    }

    #[tokio::test]
    async fn test_collection_for_each_async() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let mut sum = 0;
        collection.for_each_async(|x| async move { sum += x }).await;
        assert_eq!(sum, 15);
    }

    #[tokio::test]
    async fn test_collection_find_async() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let first_even = collection.find_async(|x| async move { x % 2 == 0 }).await;
        assert_eq!(first_even, Some(&2));
    }

    #[tokio::test]
    async fn test_collection_every_async() {
        let collection = Collection::new(vec![2, 4, 6, 8]);
        let all_even = collection.every_async(|x| async move { x % 2 == 0 }).await;
        assert!(all_even);
    }

    #[tokio::test]
    async fn test_collection_some_async() {
        let collection = Collection::new(vec![1, 3, 5, 7]);
        let has_even = collection.some_async(|x| async move { x % 2 == 0 }).await;
        assert!(!has_even);
    }

    #[tokio::test]
    async fn test_empty_collection_async() {
        let empty: Vec<i32> = vec![];
        let doubled = map_async(&empty, |x| async move { x * 2 }).await;
        assert!(doubled.is_empty());

        let evens = filter_async(&empty, |x| async move { x % 2 == 0 }).await;
        assert!(evens.is_empty());

        let sum = reduce_async(&empty, |acc, x| async move { acc + x }, 0).await;
        assert_eq!(sum, 0);

        let first_even = find_async(&empty, |x| async move { x % 2 == 0 }).await;
        assert_eq!(first_even, None);

        let all_even = every_async(&empty, |x| async move { x % 2 == 0 }).await;
        assert!(all_even); // vacuous truth

        let has_even = some_async(&empty, |x| async move { x % 2 == 0 }).await;
        assert!(!has_even); // vacuous false
    }
}
