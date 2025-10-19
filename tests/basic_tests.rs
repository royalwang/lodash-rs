//! Basic tests for lodash-rs core functionality

use lodash_rs::collection::Collection;
use lodash_rs::chain::chain;

#[test]
fn test_collection_creation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    assert_eq!(collection.size(), 5);
}

#[test]
fn test_map_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    let doubled: Vec<i32> = collection.map(|x| x * 2);
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_filter_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    let evens: Vec<i32> = collection.filter(|x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4]);
}

#[test]
fn test_reduce_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    let sum: i32 = collection.reduce(|acc, x| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_find_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    let found = collection.find(|x| *x > 3);
    assert_eq!(found, Some(&4));
}

#[test]
fn test_every_operation() {
    let data = vec![2, 4, 6, 8, 10];
    let collection = Collection::new(data);
    let all_even = collection.every(|x| x % 2 == 0);
    assert!(all_even);
}

#[test]
fn test_some_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    let has_even = collection.some(|x| x % 2 == 0);
    assert!(has_even);
}

#[test]
fn test_includes_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    assert!(collection.includes(&3));
    assert!(!collection.includes(&6));
}

#[test]
fn test_size_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    assert_eq!(collection.size(), 5);
}

#[test]
fn test_chain_operation() {
    let data = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = chain(&data)
        .map(|x| x * 2)
        .filter(|x| *x > 5)
        .collect();
    assert_eq!(result, vec![6, 8, 10]);
}
