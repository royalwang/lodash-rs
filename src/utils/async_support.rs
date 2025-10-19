/*!
Async support utilities for Lodash-RS.

This module provides async/await support for all collection operations,
enabling non-blocking operations on large datasets.
*/

#[cfg(feature = "async")]
use futures::future::{join_all, Future};
#[cfg(feature = "async")]
use crate::utils::{LodashError, Result};

#[cfg(feature = "async")]
/// Trait for async predicate functions.
pub trait AsyncPredicate<T> {
    /// Apply the async predicate to the given value.
    type Output: Future<Output = bool>;
    
    /// Apply the predicate to the given value.
    fn apply(&self, value: &T) -> Self::Output;
}

#[cfg(feature = "async")]
impl<T, F, Fut> AsyncPredicate<T> for F
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = bool>,
{
    type Output = Fut;
    
    fn apply(&self, value: &T) -> Self::Output {
        self(value)
    }
}

#[cfg(feature = "async")]
/// Trait for async mapper functions.
pub trait AsyncMapper<T, U> {
    /// Apply the async mapper to the given value.
    type Output: Future<Output = U>;
    
    /// Apply the mapper to the given value.
    fn apply(&self, value: &T) -> Self::Output;
}

#[cfg(feature = "async")]
impl<T, U, F, Fut> AsyncMapper<T, U> for F
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = U>,
{
    type Output = Fut;
    
    fn apply(&self, value: &T) -> Self::Output {
        self(value)
    }
}

#[cfg(feature = "async")]
/// Trait for async reducer functions.
pub trait AsyncReducer<T, U> {
    /// Apply the async reducer to the accumulator and value.
    type Output: Future<Output = U>;
    
    /// Apply the reducer to the accumulator and value.
    fn apply(&self, acc: U, value: &T) -> Self::Output;
}

#[cfg(feature = "async")]
impl<T, U, F, Fut> AsyncReducer<T, U> for F
where
    F: Fn(U, &T) -> Fut,
    Fut: Future<Output = U>,
{
    type Output = Fut;
    
    fn apply(&self, acc: U, value: &T) -> Self::Output {
        self(acc, value)
    }
}

#[cfg(feature = "async")]
/// Utility function to execute async operations in parallel.
pub async fn execute_parallel<T, F, Fut, R>(
    items: &[T],
    operation: F,
) -> Result<Vec<R>>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = R>,
{
    let futures = items.iter().map(|item| operation(item));
    let results = join_all(futures).await;
    Ok(results)
}

#[cfg(feature = "async")]
/// Utility function to execute async operations sequentially.
pub async fn execute_sequential<T, F, Fut, R>(
    items: &[T],
    operation: F,
) -> Result<Vec<R>>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = R>,
{
    let mut results = Vec::with_capacity(items.len());
    
    for item in items {
        let result = operation(item).await;
        results.push(result);
    }
    
    Ok(results)
}

#[cfg(feature = "async")]
/// Utility function to execute async operations with concurrency limit.
pub async fn execute_with_concurrency<T, F, Fut, R>(
    items: &[T],
    operation: F,
    concurrency: usize,
) -> Result<Vec<R>>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = R>,
{
    if concurrency == 0 {
        return Err(LodashError::invalid_input("Concurrency must be greater than 0"));
    }
    
    let mut results = Vec::with_capacity(items.len());
    let mut chunks = items.chunks(concurrency);
    
    while let Some(chunk) = chunks.next() {
        let futures = chunk.iter().map(|item| operation(item));
        let chunk_results = join_all(futures).await;
        results.extend(chunk_results);
    }
    
    Ok(results)
}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_async_predicate() {
        let pred = |x: &i32| async move { *x > 5 };
        assert!(pred.apply(&6).await);
        assert!(!pred.apply(&4).await);
    }

    #[tokio::test]
    async fn test_async_mapper() {
        let mapper = |x: &i32| async move { x * 2 };
        assert_eq!(mapper.apply(&3).await, 6);
    }

    #[tokio::test]
    async fn test_async_reducer() {
        let reducer = |acc: i32, x: &i32| async move { acc + x };
        assert_eq!(reducer.apply(5, &3).await, 8);
    }

    #[tokio::test]
    async fn test_execute_parallel() {
        let items = vec![1, 2, 3, 4, 5];
        let operation = |x: &i32| async move { x * 2 };
        
        let results = execute_parallel(&items, operation).await.unwrap();
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_execute_sequential() {
        let items = vec![1, 2, 3, 4, 5];
        let operation = |x: &i32| async move { x * 2 };
        
        let results = execute_sequential(&items, operation).await.unwrap();
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_execute_with_concurrency() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let operation = |x: &i32| async move { x * 2 };
        
        let results = execute_with_concurrency(&items, operation, 3).await.unwrap();
        assert_eq!(results, vec![2, 4, 6, 8, 10, 12, 14, 16]);
    }

    #[tokio::test]
    async fn test_execute_with_concurrency_zero() {
        let items = vec![1, 2, 3];
        let operation = |x: &i32| async move { x * 2 };
        
        let result = execute_with_concurrency(&items, operation, 0).await;
        assert!(result.is_err());
    }
}
