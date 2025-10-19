/*!
Collection module for Lodash-RS.

This module contains the core collection types and all collection methods
organized by functionality.
*/

pub mod iteration;
pub mod query;
pub mod transform;
pub mod operation;
pub mod async_support;

use crate::utils::{LodashError, Result};

/// Core collection type that wraps a vector of items.
/// 
/// This is the main type used throughout Lodash-RS for collection operations.
/// It provides a type-safe wrapper around `Vec<T>` with additional functionality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection<T> {
    /// The underlying data
    data: Vec<T>,
}

impl<T> Collection<T> {
    /// Create a new collection from an iterator.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert_eq!(collection.len(), 3);
    /// ```
    pub fn new(data: impl IntoIterator<Item = T>) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }

    /// Create a new empty collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection: Collection<i32> = Collection::empty();
    /// assert!(collection.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }

    /// Get the length of the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert_eq!(collection.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the collection is empty.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection: Collection<i32> = Collection::empty();
    /// assert!(collection.is_empty());
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert!(!collection.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a reference to the underlying data.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert_eq!(collection.data(), &vec![1, 2, 3]);
    /// ```
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Get a mutable reference to the underlying data.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let mut collection = Collection::new(vec![1, 2, 3]);
    /// collection.data_mut().push(4);
    /// assert_eq!(collection.data(), &vec![1, 2, 3, 4]);
    /// ```
    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Get an item at the specified index.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert_eq!(collection.get(1), Some(&2));
    /// assert_eq!(collection.get(5), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get a mutable item at the specified index.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let mut collection = Collection::new(vec![1, 2, 3]);
    /// if let Some(item) = collection.get_mut(1) {
    ///     *item = 42;
    /// }
    /// assert_eq!(collection.get(1), Some(&42));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Get the first item in the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert_eq!(collection.first(), Some(&1));
    /// 
    /// let empty: Collection<i32> = Collection::empty();
    /// assert_eq!(empty.first(), None);
    /// ```
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    /// Get the last item in the collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// assert_eq!(collection.last(), Some(&3));
    /// 
    /// let empty: Collection<i32> = Collection::empty();
    /// assert_eq!(empty.last(), None);
    /// ```
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    /// Convert the collection to a vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// let vec = collection.into_vec();
    /// assert_eq!(vec, vec![1, 2, 3]);
    /// ```
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Convert the collection to an iterator.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// let sum: i32 = collection.into_iter().sum();
    /// assert_eq!(sum, 6);
    /// ```
    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.data.into_iter()
    }

    /// Get an iterator over references to the items.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![1, 2, 3]);
    /// let sum: i32 = collection.iter().sum();
    /// assert_eq!(sum, 6);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Get a mutable iterator over references to the items.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let mut collection = Collection::new(vec![1, 2, 3]);
    /// for item in collection.iter_mut() {
    ///     *item *= 2;
    /// }
    /// assert_eq!(collection.data(), &vec![2, 4, 6]);
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T> From<Vec<T>> for Collection<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> From<Collection<T>> for Vec<T> {
    fn from(collection: Collection<T>) -> Self {
        collection.data
    }
}

impl<T> std::ops::Index<usize> for Collection<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Collection<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Collection<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new(vec![1, 2, 3]);
        assert_eq!(collection.len(), 3);
        assert!(!collection.is_empty());
    }

    #[test]
    fn test_empty_collection() {
        let collection: Collection<i32> = Collection::empty();
        assert_eq!(collection.len(), 0);
        assert!(collection.is_empty());
    }

    #[test]
    fn test_get() {
        let collection = Collection::new(vec![1, 2, 3]);
        assert_eq!(collection.get(1), Some(&2));
        assert_eq!(collection.get(5), None);
    }

    #[test]
    fn test_first_last() {
        let collection = Collection::new(vec![1, 2, 3]);
        assert_eq!(collection.first(), Some(&1));
        assert_eq!(collection.last(), Some(&3));
    }

    #[test]
    fn test_indexing() {
        let collection = Collection::new(vec![1, 2, 3]);
        assert_eq!(collection[1], 2);
    }

    #[test]
    fn test_into_iter() {
        let collection = Collection::new(vec![1, 2, 3]);
        let sum: i32 = collection.into_iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_iter() {
        let collection = Collection::new(vec![1, 2, 3]);
        let sum: i32 = collection.iter().sum();
        assert_eq!(sum, 6);
    }
}
