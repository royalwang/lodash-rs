/*!
Chain module for Lodash-RS.

This module provides the fluent method chaining system that allows
composing multiple operations in a readable and efficient way.
*/

pub mod builder;
pub mod executor;

use crate::collection::Collection;
use crate::utils::{LodashError, Result};

/// Create a chain wrapper that enables method chaining.
/// 
/// # Examples
/// 
/// ```
/// use lodash_rs::chain::chain;
/// 
/// let result = chain(&[1, 2, 3, 4, 5])
///     .filter(|x| x % 2 == 0)
///     .map(|x| x * 3)
///     .collect::<Vec<_>>();
/// assert_eq!(result, vec![6, 12]);
/// ```
pub fn chain<T>(data: &[T]) -> Chain<T>
where
    T: Clone,
{
    Chain::new(data)
}

/// Create an async chain wrapper that enables async method chaining.
/// 
/// # Examples
/// 
/// ```
/// use lodash_rs::chain::chain_async;
/// 
/// # async fn example() {
/// let result = chain_async(&[1, 2, 3, 4, 5])
///     .filter_async(|x| async move { x % 2 == 0 })
///     .map_async(|x| async move { x * 3 })
///     .await;
/// assert_eq!(result, vec![6, 12]);
/// # }
/// ```
#[cfg(feature = "async")]
pub fn chain_async<T>(data: &[T]) -> AsyncChain<T>
where
    T: Clone,
{
    AsyncChain::new(data)
}

/// Chain wrapper for synchronous operations.
/// 
/// This struct provides a fluent interface for chaining collection operations.
/// Operations are lazily evaluated and only executed when `collect()` or `value()` is called.
pub struct Chain<T> {
    /// The underlying data
    data: Vec<T>,
    /// Operations to be applied
    operations: Vec<Operation<T>>,
}

/// Async chain wrapper for asynchronous operations.
#[cfg(feature = "async")]
pub struct AsyncChain<T> {
    /// The underlying data
    data: Vec<T>,
    /// Async operations to be applied
    operations: Vec<AsyncOperation<T>>,
}

/// Represents a synchronous operation in the chain.
pub enum Operation<T> {
    /// Map operation
    Map(Box<dyn Fn(&T) -> T + Send + Sync>),
    /// Filter operation
    Filter(Box<dyn Fn(&T) -> bool + Send + Sync>),
    /// Take operation
    Take(usize),
    /// Skip operation
    Skip(usize),
    /// Reverse operation
    Reverse,
}

/// Represents an asynchronous operation in the chain.
#[cfg(feature = "async")]
pub enum AsyncOperation<T> {
    /// Async map operation
    MapAsync(Box<dyn Fn(&T) -> std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + Sync>> + Send + Sync>),
    /// Async filter operation
    FilterAsync(Box<dyn Fn(&T) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send + Sync>> + Send + Sync>),
    /// Take operation
    Take(usize),
    /// Skip operation
    Skip(usize),
    /// Reverse operation
    Reverse,
}

impl<T> Chain<T>
where
    T: Clone,
{
    /// Create a new chain with the given data.
    pub fn new(data: &[T]) -> Self {
        Self {
            data: data.to_vec(),
            operations: Vec::new(),
        }
    }

    /// Apply a map operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3])
    ///     .map(|x| x * 2)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(result, vec![2, 4, 6]);
    /// ```
    pub fn map<F>(mut self, mapper: F) -> Self
    where
        F: Fn(&T) -> T + Send + Sync + 'static,
    {
        self.operations.push(Operation::Map(Box::new(mapper)));
        self
    }

    /// Apply a filter operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3, 4, 5])
    ///     .filter(|x| x % 2 == 0)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(result, vec![2, 4]);
    /// ```
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.operations.push(Operation::Filter(Box::new(predicate)));
        self
    }

    /// Take the first n elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3, 4, 5])
    ///     .take(3)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(result, vec![1, 2, 3]);
    /// ```
    pub fn take(mut self, n: usize) -> Self {
        self.operations.push(Operation::Take(n));
        self
    }

    /// Skip the first n elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3, 4, 5])
    ///     .skip(2)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(result, vec![3, 4, 5]);
    /// ```
    pub fn skip(mut self, n: usize) -> Self {
        self.operations.push(Operation::Skip(n));
        self
    }

    /// Reverse the order of elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3])
    ///     .reverse()
    ///     .collect::<Vec<_>>();
    /// assert_eq!(result, vec![3, 2, 1]);
    /// ```
    pub fn reverse(mut self) -> Self {
        self.operations.push(Operation::Reverse);
        self
    }

    /// Collect the results into a vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3])
    ///     .map(|x| x * 2)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(result, vec![2, 4, 6]);
    /// ```
    pub fn collect<U>(self) -> Vec<U>
    where
        T: Into<U>,
    {
        self.value().into_iter().map(|x| x.into()).collect()
    }

    /// Get the final value after applying all operations.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let result = chain(&[1, 2, 3])
    ///     .map(|x| x * 2)
    ///     .value();
    /// assert_eq!(result, vec![2, 4, 6]);
    /// ```
    pub fn value(self) -> Vec<T> {
        let mut result = self.data;
        
        for operation in self.operations {
            match operation {
                Operation::Map(mapper) => {
                    result = result.into_iter().map(|x| mapper(&x)).collect();
                }
                Operation::Filter(predicate) => {
                    result = result.into_iter().filter(|x| predicate(x)).collect();
                }
                Operation::Take(n) => {
                    result = result.into_iter().take(n).collect();
                }
                Operation::Skip(n) => {
                    result = result.into_iter().skip(n).collect();
                }
                Operation::Reverse => {
                    result.reverse();
                }
            }
        }
        
        result
    }

    /// Convert to a Collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain;
    /// 
    /// let collection = chain(&[1, 2, 3])
    ///     .map(|x| x * 2)
    ///     .into_collection();
    /// assert_eq!(collection.data(), &vec![2, 4, 6]);
    /// ```
    pub fn into_collection(self) -> Collection<T> {
        Collection::new(self.value())
    }
}

#[cfg(feature = "async")]
impl<T> AsyncChain<T>
where
    T: Clone,
{
    /// Create a new async chain with the given data.
    pub fn new(data: &[T]) -> Self {
        Self {
            data: data.to_vec(),
            operations: Vec::new(),
        }
    }

    /// Apply an async map operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain_async;
    /// 
    /// # async fn example() {
    /// let result = chain_async(&[1, 2, 3])
    ///     .map_async(|x| async move { x * 2 })
    ///     .await;
    /// assert_eq!(result, vec![2, 4, 6]);
    /// # }
    /// ```
    pub fn map_async<F, Fut>(mut self, mapper: F) -> Self
    where
        F: Fn(&T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = T> + Send + Sync + 'static,
    {
        self.operations.push(AsyncOperation::MapAsync(Box::new(move |x| {
            Box::pin(mapper(x))
        })));
        self
    }

    /// Apply an async filter operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain_async;
    /// 
    /// # async fn example() {
    /// let result = chain_async(&[1, 2, 3, 4, 5])
    ///     .filter_async(|x| async move { x % 2 == 0 })
    ///     .await;
    /// assert_eq!(result, vec![2, 4]);
    /// # }
    /// ```
    pub fn filter_async<F, Fut>(mut self, predicate: F) -> Self
    where
        F: Fn(&T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = bool> + Send + Sync + 'static,
    {
        self.operations.push(AsyncOperation::FilterAsync(Box::new(move |x| {
            Box::pin(predicate(x))
        })));
        self
    }

    /// Take the first n elements.
    pub fn take(mut self, n: usize) -> Self {
        self.operations.push(AsyncOperation::Take(n));
        self
    }

    /// Skip the first n elements.
    pub fn skip(mut self, n: usize) -> Self {
        self.operations.push(AsyncOperation::Skip(n));
        self
    }

    /// Reverse the order of elements.
    pub fn reverse(mut self) -> Self {
        self.operations.push(AsyncOperation::Reverse);
        self
    }

    /// Get the final value after applying all async operations.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::chain::chain_async;
    /// 
    /// # async fn example() {
    /// let result = chain_async(&[1, 2, 3])
    ///     .map_async(|x| async move { x * 2 })
    ///     .await;
    /// assert_eq!(result, vec![2, 4, 6]);
    /// # }
    /// ```
    pub async fn r#await(self) -> Vec<T> {
        let mut result = self.data;
        
        for operation in self.operations {
            match operation {
                AsyncOperation::MapAsync(mapper) => {
                    let mut new_result = Vec::new();
                    for item in result {
                        let mapped = mapper(&item).await;
                        new_result.push(mapped);
                    }
                    result = new_result;
                }
                AsyncOperation::FilterAsync(predicate) => {
                    let mut new_result = Vec::new();
                    for item in result {
                        if predicate(&item).await {
                            new_result.push(item);
                        }
                    }
                    result = new_result;
                }
                AsyncOperation::Take(n) => {
                    result = result.into_iter().take(n).collect();
                }
                AsyncOperation::Skip(n) => {
                    result = result.into_iter().skip(n).collect();
                }
                AsyncOperation::Reverse => {
                    result.reverse();
                }
            }
        }
        
        result
    }

    /// Convert to a Collection.
    pub async fn into_collection(self) -> Collection<T> {
        Collection::new(self.r#await().await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_map() {
        let result = chain(&[1, 2, 3])
            .map(|x| x * 2)
            .collect::<Vec<_>>();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_chain_filter() {
        let result = chain(&[1, 2, 3, 4, 5])
            .filter(|x| x % 2 == 0)
            .collect::<Vec<_>>();
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_chain_take() {
        let result = chain(&[1, 2, 3, 4, 5])
            .take(3)
            .collect::<Vec<_>>();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_chain_skip() {
        let result = chain(&[1, 2, 3, 4, 5])
            .skip(2)
            .collect::<Vec<_>>();
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_chain_reverse() {
        let result = chain(&[1, 2, 3])
            .reverse()
            .collect::<Vec<_>>();
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_chain_complex() {
        let result = chain(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
            .filter(|x| x % 2 == 0)
            .map(|x| x * 3)
            .take(3)
            .collect::<Vec<_>>();
        assert_eq!(result, vec![6, 12, 18]);
    }

    #[test]
    fn test_chain_value() {
        let result = chain(&[1, 2, 3])
            .map(|x| x * 2)
            .value();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_chain_into_collection() {
        let collection = chain(&[1, 2, 3])
            .map(|x| x * 2)
            .into_collection();
        assert_eq!(collection.data(), &vec![2, 4, 6]);
    }

    #[test]
    fn test_chain_empty() {
        let empty: Vec<i32> = vec![];
        let result = chain(&empty)
            .map(|x| x * 2)
            .collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_chain_async_map() {
        let result = chain_async(&[1, 2, 3])
            .map_async(|x| async move { x * 2 })
            .await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_chain_async_filter() {
        let result = chain_async(&[1, 2, 3, 4, 5])
            .filter_async(|x| async move { x % 2 == 0 })
            .await;
        assert_eq!(result, vec![2, 4]);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_chain_async_complex() {
        let result = chain_async(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
            .filter_async(|x| async move { x % 2 == 0 })
            .map_async(|x| async move { x * 3 })
            .take(3)
            .await;
        assert_eq!(result, vec![6, 12, 18]);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_chain_async_into_collection() {
        let collection = chain_async(&[1, 2, 3])
            .map_async(|x| async move { x * 2 })
            .into_collection()
            .await;
        assert_eq!(collection.data(), &vec![2, 4, 6]);
    }
}
