/*!
Async usage examples for Lodash-RS.

This example demonstrates the async functionality of the Lodash-RS library.
*/

#[cfg(feature = "async")]
use lodash_rs::prelude::*;
#[cfg(feature = "async")]
use tokio;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    println!("=== Lodash-RS Async Usage Examples ===\n");

    // Basic async operations
    basic_async_operations().await;
    
    // Async chain operations
    async_chain_operations().await;
    
    // Async query operations
    async_query_operations().await;
    
    // Async transform operations
    async_transform_operations().await;
}

#[cfg(feature = "async")]
async fn basic_async_operations() {
    println!("1. Basic Async Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Async map operation
    let doubled = map_async(&numbers, |x| async move { x * 2 }).await;
    println!("  mapAsync([1,2,3,4,5], x => x * 2) = {:?}", doubled);
    
    // Async filter operation
    let evens = filter_async(&numbers, |x| async move { x % 2 == 0 }).await;
    println!("  filterAsync([1,2,3,4,5], x => x % 2 === 0) = {:?}", evens);
    
    // Async reduce operation
    let sum = reduce_async(&numbers, |acc, x| async move { acc + x }, 0).await;
    println!("  reduceAsync([1,2,3,4,5], (acc, x) => acc + x, 0) = {}", sum);
    
    // Async forEach operation
    print!("  forEachAsync([1,2,3,4,5], x => print!(\"{} \", x)) = ");
    for_each_async(&numbers, |x| async move { print!("{} ", x) }).await;
    println!();
    
    println!();
}

#[cfg(feature = "async")]
async fn async_chain_operations() {
    println!("2. Async Chain Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Simple async chain
    let result = chain_async(&numbers)
        .filter_async(|x| async move { x % 2 == 0 })
        .map_async(|x| async move { x * 3 })
        .await;
    println!("  chainAsync([1,2,3,4,5,6,7,8,9,10]).filterAsync(x => x % 2 === 0).mapAsync(x => x * 3) = {:?}", result);
    
    // Complex async chain
    let result = chain_async(&numbers)
        .filter_async(|x| async move { x % 2 == 0 })
        .map_async(|x| async move { x * 3 })
        .take(3)
        .await;
    println!("  chainAsync([1,2,3,4,5,6,7,8,9,10]).filterAsync(x => x % 2 === 0).mapAsync(x => x * 3).take(3) = {:?}", result);
    
    println!();
}

#[cfg(feature = "async")]
async fn async_query_operations() {
    println!("3. Async Query Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Async find operation
    let first_even = find_async(&numbers, |x| async move { x % 2 == 0 }).await;
    println!("  findAsync([1,2,3,4,5,6,7,8,9,10], x => x % 2 === 0) = {:?}", first_even);
    
    // Async every operation
    let all_positive = every_async(&numbers, |x| async move { *x > 0 }).await;
    println!("  everyAsync([1,2,3,4,5,6,7,8,9,10], x => x > 0) = {}", all_positive);
    
    // Async some operation
    let has_even = some_async(&numbers, |x| async move { x % 2 == 0 }).await;
    println!("  someAsync([1,2,3,4,5,6,7,8,9,10], x => x % 2 === 0) = {}", has_even);
    
    println!();
}

#[cfg(feature = "async")]
async fn async_transform_operations() {
    println!("4. Async Transform Operations:");
    
    let users = vec![
        ("john", 30, "engineer"),
        ("jane", 25, "designer"),
        ("bob", 35, "manager"),
        ("alice", 28, "engineer"),
        ("charlie", 32, "designer"),
    ];
    
    // Async invoke operation
    let names = invoke_async(&users, |(name, _, _)| async move { name.to_uppercase() }).await;
    println!("  invokeAsync(users, user => user.name.toUpperCase()) = {:?}", names);
    
    println!();
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async features are not enabled. Please run with --features async");
}
