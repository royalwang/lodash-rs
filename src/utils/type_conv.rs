/*!
Type conversion utilities for Lodash-RS.

This module provides type conversion traits and utilities for handling
different data types in a type-safe manner.
*/

use crate::utils::{LodashError, Result};

/// Trait for types that can be converted to a key for grouping operations.
pub trait ToKey {
    /// Convert the value to a key.
    fn to_key(&self) -> String;
}

impl ToKey for String {
    fn to_key(&self) -> String {
        self.clone()
    }
}

impl ToKey for &str {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for i32 {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for i64 {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for u32 {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for u64 {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for f32 {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for f64 {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

impl ToKey for bool {
    fn to_key(&self) -> String {
        self.to_string()
    }
}

/// Trait for types that can be used as predicate functions.
pub trait Predicate<T> {
    /// Apply the predicate to the given value.
    fn apply(&self, value: &T) -> bool;
}

impl<T, F> Predicate<T> for F
where
    F: Fn(&T) -> bool,
{
    fn apply(&self, value: &T) -> bool {
        self(value)
    }
}

/// Trait for types that can be used as mapper functions.
pub trait Mapper<T, U> {
    /// Apply the mapper to the given value.
    fn apply(&self, value: &T) -> U;
}

impl<T, U, F> Mapper<T, U> for F
where
    F: Fn(&T) -> U,
{
    fn apply(&self, value: &T) -> U {
        self(value)
    }
}

/// Trait for types that can be used as reducer functions.
pub trait Reducer<T, U> {
    /// Apply the reducer to the accumulator and value.
    fn apply(&self, acc: U, value: &T) -> U;
}

impl<T, U, F> Reducer<T, U> for F
where
    F: Fn(U, &T) -> U,
{
    fn apply(&self, acc: U, value: &T) -> U {
        self(acc, value)
    }
}

/// Trait for types that can be converted to a comparable value for sorting.
pub trait ToComparable {
    /// Convert to a comparable value.
    type Output: PartialOrd;
    
    /// Convert the value to a comparable type.
    fn to_comparable(&self) -> Self::Output;
}

impl ToComparable for i32 {
    type Output = i32;
    
    fn to_comparable(&self) -> Self::Output {
        *self
    }
}

impl ToComparable for i64 {
    type Output = i64;
    
    fn to_comparable(&self) -> Self::Output {
        *self
    }
}

impl ToComparable for u32 {
    type Output = u32;
    
    fn to_comparable(&self) -> Self::Output {
        *self
    }
}

impl ToComparable for u64 {
    type Output = u64;
    
    fn to_comparable(&self) -> Self::Output {
        *self
    }
}

impl ToComparable for f32 {
    type Output = f32;
    
    fn to_comparable(&self) -> Self::Output {
        *self
    }
}

impl ToComparable for f64 {
    type Output = f64;
    
    fn to_comparable(&self) -> Self::Output {
        *self
    }
}

impl ToComparable for String {
    type Output = String;
    
    fn to_comparable(&self) -> Self::Output {
        self.clone()
    }
}

impl ToComparable for &str {
    type Output = String;
    
    fn to_comparable(&self) -> Self::Output {
        self.to_string()
    }
}

/// Trait for types that can be safely cloned for collection operations.
pub trait SafeClone {
    /// Safely clone the value.
    fn safe_clone(&self) -> Self;
}

impl<T: Clone> SafeClone for T {
    fn safe_clone(&self) -> Self {
        self.clone()
    }
}

/// Trait for types that can be converted to a hash key.
pub trait ToHashKey {
    /// Convert to a hash key.
    fn to_hash_key(&self) -> u64;
}

impl ToHashKey for i32 {
    fn to_hash_key(&self) -> u64 {
        *self as u64
    }
}

impl ToHashKey for i64 {
    fn to_hash_key(&self) -> u64 {
        *self as u64
    }
}

impl ToHashKey for u32 {
    fn to_hash_key(&self) -> u64 {
        *self as u64
    }
}

impl ToHashKey for u64 {
    fn to_hash_key(&self) -> u64 {
        *self
    }
}

impl ToHashKey for String {
    fn to_hash_key(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl ToHashKey for &str {
    fn to_hash_key(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

/// Utility function to safely convert between types.
pub fn safe_convert<T, U>(value: T) -> Result<U>
where
    T: std::fmt::Display,
    U: std::str::FromStr,
    U::Err: std::fmt::Display,
{
    value.to_string().parse::<U>()
        .map_err(|_e| LodashError::type_conversion(
            std::any::type_name::<T>(),
            std::any::type_name::<U>()
        ))
}

/// Utility function to convert a value to a string representation.
pub fn to_string<T>(value: &T) -> String
where
    T: std::fmt::Display,
{
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_key() {
        assert_eq!("hello".to_key(), "hello");
        assert_eq!(42.to_key(), "42");
        assert_eq!(true.to_key(), "true");
    }

    #[test]
    fn test_predicate() {
        let pred = |x: &i32| *x > 5;
        assert!(Predicate::apply(&pred, &6));
        assert!(!Predicate::apply(&pred, &4));
    }

    #[test]
    fn test_mapper() {
        let mapper = |x: &i32| x * 2;
        assert_eq!(mapper.apply(&3), 6);
    }

    #[test]
    fn test_reducer() {
        let reducer = |acc: i32, x: &i32| acc + x;
        assert_eq!(reducer.apply(5, &3), 8);
    }

    #[test]
    fn test_to_comparable() {
        assert_eq!(42.to_comparable(), 42);
        assert_eq!("hello".to_comparable(), "hello");
    }

    #[test]
    fn test_safe_clone() {
        let original = vec![1, 2, 3];
        let cloned = original.safe_clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_to_hash_key() {
        let key1 = 42.to_hash_key();
        let key2 = 42.to_hash_key();
        assert_eq!(key1, key2);
        
        let str_key1 = "hello".to_hash_key();
        let str_key2 = "hello".to_hash_key();
        assert_eq!(str_key1, str_key2);
    }
}
