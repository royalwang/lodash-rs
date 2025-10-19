//! Chain builder module for constructing complex operation chains.

use crate::chain::{Chain, AsyncChain, Operation, AsyncOperation};

/// Builder for creating complex operation chains.
pub struct ChainBuilder<T> {
    data: Vec<T>,
    operations: Vec<Operation<T>>,
}

impl<T> ChainBuilder<T> {
    /// Create a new chain builder.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            operations: Vec::new(),
        }
    }

    /// Add a map operation to the chain.
    pub fn map<F>(mut self, mapper: F) -> Self
    where
        F: Fn(&T) -> T + Send + Sync + 'static,
    {
        self.operations.push(Operation::Map(Box::new(mapper)));
        self
    }

    /// Add a filter operation to the chain.
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.operations.push(Operation::Filter(Box::new(predicate)));
        self
    }

    /// Build the chain.
    pub fn build(self) -> Chain<T> {
        Chain {
            data: self.data,
            operations: self.operations,
        }
    }
}

/// Builder for creating complex async operation chains.
pub struct AsyncChainBuilder<T> {
    data: Vec<T>,
    operations: Vec<AsyncOperation<T>>,
}

impl<T> AsyncChainBuilder<T> {
    /// Create a new async chain builder.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            operations: Vec::new(),
        }
    }

    /// Add an async map operation to the chain.
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

    /// Add an async filter operation to the chain.
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

    /// Build the async chain.
    pub fn build(self) -> AsyncChain<T> {
        AsyncChain {
            data: self.data,
            operations: self.operations,
        }
    }
}
