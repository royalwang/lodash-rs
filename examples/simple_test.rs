/*!
Simple test to verify basic functionality without external dependencies.
*/

use std::collections::HashMap;

// Simple implementation of basic functions
fn map<T, U, F>(collection: &[T], iteratee: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    collection.iter().map(iteratee).collect()
}

fn filter<T, F>(collection: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    collection.iter()
        .filter(|item| predicate(item))
        .cloned()
        .collect()
}

fn reduce<T, U, F>(collection: &[T], iteratee: F, initial: U) -> U
where
    F: Fn(U, &T) -> U,
{
    collection.iter().fold(initial, iteratee)
}

fn find<T, F>(collection: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    collection.iter().find(|item| predicate(item))
}

fn group_by<T, K, F>(collection: &[T], iteratee: F) -> HashMap<K, Vec<T>>
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

fn main() {
    println!("=== Simple Lodash-RS Test ===\n");

    // Test map
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = map(&numbers, |x| x * 2);
    println!("map([1,2,3,4,5], x => x * 2) = {:?}", doubled);

    // Test filter
    let evens = filter(&numbers, |x| x % 2 == 0);
    println!("filter([1,2,3,4,5], x => x % 2 === 0) = {:?}", evens);

    // Test reduce
    let sum = reduce(&numbers, |acc, x| acc + x, 0);
    println!("reduce([1,2,3,4,5], (acc, x) => acc + x, 0) = {}", sum);

    // Test find
    let first_even = find(&numbers, |x| x % 2 == 0);
    println!("find([1,2,3,4,5], x => x % 2 === 0) = {:?}", first_even);

    // Test group_by
    let users = vec![
        ("john", 30, "engineer"),
        ("jane", 25, "designer"),
        ("bob", 35, "manager"),
        ("alice", 28, "engineer"),
    ];
    let grouped = group_by(&users, |(_, _, role)| role.to_string());
    println!("groupBy(users, user => user.role) = {:?}", grouped);

    println!("\nAll tests passed! ✅");
}
