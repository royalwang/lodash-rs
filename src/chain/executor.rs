//! Chain executor module for executing operation chains.

use crate::chain::{Chain, Operation};

#[cfg(feature = "async")]
use crate::chain::{AsyncChain, AsyncOperation};

/// Executor for synchronous operation chains.
pub struct ChainExecutor<T> {
    chain: Chain<T>,
}

impl<T> ChainExecutor<T> {
    /// Create a new chain executor.
    #[must_use]
    pub fn new(chain: Chain<T>) -> Self {
        Self { chain }
    }

    /// Execute the chain and return the result.
    #[must_use]
    pub fn execute(self) -> Vec<T> {
        let mut result = self.chain.data;
        
        for operation in self.chain.operations {
            match operation {
                Operation::Map(mapper) => {
                    result = result.into_iter().map(|x| mapper(&x)).collect();
                }
                Operation::Filter(predicate) => {
                    result.retain(|x| predicate(x));
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
}

/// Executor for asynchronous operation chains.
#[cfg(feature = "async")]
pub struct AsyncChainExecutor<T> {
    chain: AsyncChain<T>,
}

#[cfg(feature = "async")]
impl<T> AsyncChainExecutor<T> {
    /// Create a new async chain executor.
    pub fn new(chain: AsyncChain<T>) -> Self {
        Self { chain }
    }

    /// Execute the async chain and return the result.
    pub async fn execute(self) -> Vec<T> {
        let mut result = self.chain.data;
        
        for operation in self.chain.operations {
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
}
