/*!
Collection operation methods for Lodash-RS.

This module provides collection operation methods like `shuffle`, `sample`, `size`, etc.
These methods are used to manipulate and analyze collections.
*/

use crate::collection::Collection;
use crate::utils::{LodashError, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Get the size of collection.
/// 
/// # Examples
/// 
/// ```
/// use lodash_rs::collection::operation::size;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert_eq!(size(&numbers), 5);
/// 
/// let empty: Vec<i32> = vec![];
/// assert_eq!(size(&empty), 0);
/// ```
pub fn size<T>(collection: &[T]) -> usize {
    collection.len()
}

/// Creates an array of shuffled values.
/// 
/// # Examples
/// 
/// ```
/// use lodash_rs::collection::operation::shuffle;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let shuffled = shuffle(&numbers);
/// assert_eq!(shuffled.len(), 5);
/// // Note: The order will be different, but all elements will be present
/// ```
pub fn shuffle<T>(collection: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut shuffled = collection.to_vec();
    shuffled.shuffle(&mut thread_rng());
    shuffled
}

/// Gets a random element from collection.
/// 
/// # Examples
/// 
/// ```
/// use lodash_rs::collection::operation::sample;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let random = sample(&numbers);
/// assert!(numbers.contains(random.unwrap()));
/// ```
pub fn sample<T>(collection: &[T]) -> Option<&T> {
    if collection.is_empty() {
        return None;
    }
    collection.choose(&mut thread_rng())
}

/// Gets n random elements at unique keys from collection up to the size of collection.
/// 
/// # Examples
/// 
/// ```
/// use lodash_rs::collection::operation::sample_size;
/// 
/// let numbers = vec![1, 2, 3, 4, 5];
/// let samples = sample_size(&numbers, 3);
/// assert_eq!(samples.len(), 3);
/// // All samples will be unique elements from the original collection
/// ```
pub fn sample_size<T>(collection: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    if collection.is_empty() || n == 0 {
        return Vec::new();
    }
    
    let mut rng = thread_rng();
    let mut samples = collection.to_vec();
    samples.shuffle(&mut rng);
    samples.truncate(n);
    samples
}

/// Collection methods that work on the `Collection` type.
impl<T> Collection<T> {
    /// Get the size of the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(collection.size(), 5);
    /// ```
    pub fn size(&self) -> usize {
        size(&self.data)
    }

    /// Creates an array of shuffled values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let shuffled = collection.shuffle();
    /// assert_eq!(shuffled.len(), 5);
    /// ```
    pub fn shuffle(&self) -> Vec<T>
    where
        T: Clone,
    {
        shuffle(&self.data)
    }

    /// Gets a random element from the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let random = collection.sample();
    /// assert!(collection.data().contains(random.unwrap()));
    /// ```
    pub fn sample(&self) -> Option<&T> {
        sample(&self.data)
    }

    /// Gets n random elements at unique keys from the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lodash_rs::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3, 4, 5]);
    /// let samples = collection.sample_size(3);
    /// assert_eq!(samples.len(), 3);
    /// ```
    pub fn sample_size(&self, n: usize) -> Vec<T>
    where
        T: Clone,
    {
        sample_size(&self.data, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(size(&numbers), 5);

        let empty: Vec<i32> = vec![];
        assert_eq!(size(&empty), 0);
    }

    #[test]
    fn test_shuffle() {
        let numbers = vec![1, 2, 3, 4, 5];
        let shuffled = shuffle(&numbers);
        assert_eq!(shuffled.len(), 5);
        
        // Check that all elements are present (order may be different)
        for num in &numbers {
            assert!(shuffled.contains(num));
        }
    }

    #[test]
    fn test_sample() {
        let numbers = vec![1, 2, 3, 4, 5];
        let random = sample(&numbers);
        assert!(random.is_some());
        assert!(numbers.contains(random.unwrap()));

        let empty: Vec<i32> = vec![];
        assert_eq!(sample(&empty), None);
    }

    #[test]
    fn test_sample_size() {
        let numbers = vec![1, 2, 3, 4, 5];
        let samples = sample_size(&numbers, 3);
        assert_eq!(samples.len(), 3);
        
        // Check that all samples are from the original collection
        for sample in &samples {
            assert!(numbers.contains(sample));
        }

        // Test with n larger than collection size
        let samples_large = sample_size(&numbers, 10);
        assert_eq!(samples_large.len(), 5);

        // Test with n = 0
        let samples_zero = sample_size(&numbers, 0);
        assert!(samples_zero.is_empty());

        // Test with empty collection
        let empty: Vec<i32> = vec![];
        let samples_empty = sample_size(&empty, 3);
        assert!(samples_empty.is_empty());
    }

    #[test]
    fn test_collection_size() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(collection.size(), 5);
    }

    #[test]
    fn test_collection_shuffle() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let shuffled = collection.shuffle();
        assert_eq!(shuffled.len(), 5);
        
        // Check that all elements are present
        for num in collection.data() {
            assert!(shuffled.contains(num));
        }
    }

    #[test]
    fn test_collection_sample() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let random = collection.sample();
        assert!(random.is_some());
        assert!(collection.data().contains(random.unwrap()));

        let empty: Collection<i32> = Collection::empty();
        assert_eq!(empty.sample(), None);
    }

    #[test]
    fn test_collection_sample_size() {
        let collection = Collection::new(vec![1, 2, 3, 4, 5]);
        let samples = collection.sample_size(3);
        assert_eq!(samples.len(), 3);
        
        // Check that all samples are from the original collection
        for sample in &samples {
            assert!(collection.data().contains(sample));
        }
    }

    #[test]
    fn test_empty_collection_operations() {
        let empty: Vec<i32> = vec![];
        assert_eq!(size(&empty), 0);
        assert!(shuffle(&empty).is_empty());
        assert_eq!(sample(&empty), None);
        assert!(sample_size(&empty, 3).is_empty());
    }
}
