/*!
Transform methods for Lodash-RS.

This module provides transform methods like `group_by`, `key_by`, `sort_by`, etc.
These methods are used to reorganize and transform collections.
*/

use crate::collection::Collection;
use crate::utils::{LodashError, Result, ToKey, ToComparable};
use std::collections::HashMap;

/// Create an object composed of keys generated from the results of running
/// each element of collection through iteratee. The order of grouped values
/// is determined by the order they occur in collection.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::transform::group_by;
/// use std::collections::HashMap;
/// 
/// let numbers = vec![6.1, 4.2, 6.3];
/// let grouped = group_by(&numbers, |x| (*x as f64).floor() as i32);
/// assert_eq!(grouped.get(&6), Some(&vec![6.1, 6.3]));
/// assert_eq!(grouped.get(&4), Some(&vec![4.2]));
/// ```
pub fn group_by<T, K, F>(collection: &[T], iteratee: F) -> HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    T: Clone,
    F: Fn(&T) -> K,
{
    let mut groups = HashMap::new();
    for item in collection {
        let key = iteratee(item);
        groups.entry(key).or_insert_with(Vec::new).push(item.clone());
    }
    groups
}

/// Create an object composed of keys generated from the results of running
/// each element of collection through iteratee. The corresponding value of
/// each key is the last element responsible for generating the key.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::transform::key_by;
/// use std::collections::HashMap;
/// 
/// let users = vec![
///     ("john", 30),
///     ("jane", 25),
///     ("bob", 35),
/// ];
/// let keyed = key_by(&users, |(name, _)| name.to_string());
/// assert_eq!(keyed.get("john"), Some(&("john", 30)));
/// ```
pub fn key_by<T, K, F>(collection: &[T], iteratee: F) -> HashMap<K, T>
where
    K: std::hash::Hash + Eq,
    T: Clone,
    F: Fn(&T) -> K,
{
    let mut keyed = HashMap::new();
    for item in collection {
        let key = iteratee(item);
        keyed.insert(key, item.clone());
    }
    keyed
}

/// Invoke the method at path of each element in collection.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::transform::invoke;
/// 
/// let strings = vec!["hello", "world"];
/// let uppercased = invoke(&strings, |s| s.to_uppercase());
/// assert_eq!(uppercased, vec!["HELLO", "WORLD"]);
/// ```
pub fn invoke<T, U, F>(collection: &[T], method: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    collection.iter().map(method).collect()
}

/// Create an array of elements, sorted in ascending order by the results of
/// running each element in collection through iteratee.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::transform::sort_by;
/// 
/// let users = vec![
///     ("john", 30),
///     ("jane", 25),
///     ("bob", 35),
/// ];
/// let sorted = sort_by(&users, |(_, age)| *age);
/// assert_eq!(sorted[0], ("jane", 25));
/// assert_eq!(sorted[1], ("john", 30));
/// assert_eq!(sorted[2], ("bob", 35));
/// ```
pub fn sort_by<T, K, F>(collection: &[T], iteratee: F) -> Vec<T>
where
    T: Clone,
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut sorted = collection.to_vec();
    sorted.sort_by_key(iteratee);
    sorted
}

/// This method is like `sort_by` except that it allows specifying the sort
/// orders of the iteratees to sort by.
/// 
/// # Examples
/// 
/// ```
/// use rust_lodash::collection::transform::order_by;
/// 
/// let users = vec![
///     ("john", 30, "engineer"),
///     ("jane", 25, "designer"),
///     ("bob", 30, "manager"),
/// ];
/// let sorted = order_by(&users, |(_, age, _)| *age, false);
/// assert_eq!(sorted[0], ("john", 30, "engineer"));
/// assert_eq!(sorted[1], ("bob", 30, "manager"));
/// assert_eq!(sorted[2], ("jane", 25, "designer"));
/// ```
pub fn order_by<T, K, F>(collection: &[T], iteratee: F, ascending: bool) -> Vec<T>
where
    T: Clone,
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut sorted = collection.to_vec();
    if ascending {
        sorted.sort_by_key(iteratee);
    } else {
        sorted.sort_by(|a, b| {
            let key_a = iteratee(a);
            let key_b = iteratee(b);
            key_b.cmp(&key_a)
        });
    }
    sorted
}

/// Collection methods that work on the `Collection` type.
impl<T> Collection<T> {
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
    /// let grouped = collection.group_by(|x| (*x as f64).floor() as i32);
    /// assert_eq!(grouped.get(&6), Some(&vec![6.1, 6.3]));
    /// ```
    pub fn group_by<K, F>(&self, iteratee: F) -> HashMap<K, Vec<T>>
    where
        K: std::hash::Hash + Eq,
        T: Clone,
        F: Fn(&T) -> K,
    {
        group_by(&self.data, iteratee)
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
    /// let collection = Collection::new(vec![
    ///     ("john", 30),
    ///     ("jane", 25),
    ///     ("bob", 35),
    /// ]);
    /// let keyed = collection.key_by(|(name, _)| name.to_string());
    /// assert_eq!(keyed.get("john"), Some(&("john", 30)));
    /// ```
    pub fn key_by<K, F>(&self, iteratee: F) -> HashMap<K, T>
    where
        K: std::hash::Hash + Eq,
        T: Clone,
        F: Fn(&T) -> K,
    {
        key_by(&self.data, iteratee)
    }

    /// Invoke the method at path of each element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec!["hello", "world"]);
    /// let uppercased = collection.invoke(|s| s.to_uppercase());
    /// assert_eq!(uppercased, vec!["HELLO", "WORLD"]);
    /// ```
    pub fn invoke<U, F>(&self, method: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        invoke(&self.data, method)
    }

    /// Create an array of elements, sorted in ascending order by the results of
    /// running each element through iteratee.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![
    ///     ("john", 30),
    ///     ("jane", 25),
    ///     ("bob", 35),
    /// ]);
    /// let sorted = collection.sort_by(|(_, age)| *age);
    /// assert_eq!(sorted[0], ("jane", 25));
    /// ```
    pub fn sort_by<K, F>(&self, iteratee: F) -> Vec<T>
    where
        T: Clone,
        K: Ord,
        F: Fn(&T) -> K,
    {
        sort_by(&self.data, iteratee)
    }

    /// This method is like `sort_by` except that it allows specifying the sort
    /// orders of the iteratees to sort by.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rust_lodash::collection::Collection;
    /// 
    /// let collection = Collection::new(vec![
    ///     ("john", 30, "engineer"),
    ///     ("jane", 25, "designer"),
    ///     ("bob", 30, "manager"),
    /// ]);
    /// let sorted = collection.order_by(|(_, age, _)| *age, false);
    /// assert_eq!(sorted[0], ("john", 30, "engineer"));
    /// ```
    pub fn order_by<K, F>(&self, iteratee: F, ascending: bool) -> Vec<T>
    where
        T: Clone,
        K: Ord,
        F: Fn(&T) -> K,
    {
        order_by(&self.data, iteratee, ascending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_group_by() {
        let numbers = vec![6.1, 4.2, 6.3];
        let grouped = group_by(&numbers, |x| (*x as f64).floor() as i32);
        assert_eq!(grouped.get(&6), Some(&vec![6.1, 6.3]));
        assert_eq!(grouped.get(&4), Some(&vec![4.2]));
    }

    #[test]
    fn test_key_by() {
        let users = vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ];
        let keyed = key_by(&users, |(name, _)| name.to_string());
        assert_eq!(keyed.get("john"), Some(&("john", 30)));
        assert_eq!(keyed.get("jane"), Some(&("jane", 25)));
        assert_eq!(keyed.get("bob"), Some(&("bob", 35)));
    }

    #[test]
    fn test_invoke() {
        let strings = vec!["hello", "world"];
        let uppercased = invoke(&strings, |s| s.to_uppercase());
        assert_eq!(uppercased, vec!["HELLO", "WORLD"]);
    }

    #[test]
    fn test_sort_by() {
        let users = vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ];
        let sorted = sort_by(&users, |(_, age)| *age);
        assert_eq!(sorted[0], ("jane", 25));
        assert_eq!(sorted[1], ("john", 30));
        assert_eq!(sorted[2], ("bob", 35));
    }

    #[test]
    fn test_order_by_ascending() {
        let users = vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ];
        let sorted = order_by(&users, |(_, age)| *age, true);
        assert_eq!(sorted[0], ("jane", 25));
        assert_eq!(sorted[1], ("john", 30));
        assert_eq!(sorted[2], ("bob", 35));
    }

    #[test]
    fn test_order_by_descending() {
        let users = vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ];
        let sorted = order_by(&users, |(_, age)| *age, false);
        assert_eq!(sorted[0], ("bob", 35));
        assert_eq!(sorted[1], ("john", 30));
        assert_eq!(sorted[2], ("jane", 25));
    }

    #[test]
    fn test_collection_group_by() {
        let collection = Collection::new(vec![6.1, 4.2, 6.3]);
        let grouped = collection.group_by(|x| (*x as f64).floor() as i32);
        assert_eq!(grouped.get(&6), Some(&vec![6.1, 6.3]));
    }

    #[test]
    fn test_collection_key_by() {
        let collection = Collection::new(vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ]);
        let keyed = collection.key_by(|(name, _)| name.to_string());
        assert_eq!(keyed.get("john"), Some(&("john", 30)));
    }

    #[test]
    fn test_collection_invoke() {
        let collection = Collection::new(vec!["hello", "world"]);
        let uppercased = collection.invoke(|s| s.to_uppercase());
        assert_eq!(uppercased, vec!["HELLO", "WORLD"]);
    }

    #[test]
    fn test_collection_sort_by() {
        let collection = Collection::new(vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ]);
        let sorted = collection.sort_by(|(_, age)| *age);
        assert_eq!(sorted[0], ("jane", 25));
    }

    #[test]
    fn test_collection_order_by() {
        let collection = Collection::new(vec![
            ("john", 30),
            ("jane", 25),
            ("bob", 35),
        ]);
        let sorted = collection.order_by(|(_, age)| *age, false);
        assert_eq!(sorted[0], ("bob", 35));
    }

    #[test]
    fn test_empty_collection() {
        let empty: Vec<i32> = vec![];
        let grouped = group_by(&empty, |x| x % 2);
        assert!(grouped.is_empty());

        let keyed = key_by(&empty, |x| x.to_string());
        assert!(keyed.is_empty());

        let invoked = invoke(&empty, |x| x * 2);
        assert!(invoked.is_empty());

        let sorted = sort_by(&empty, |x| *x);
        assert!(sorted.is_empty());
    }
}
