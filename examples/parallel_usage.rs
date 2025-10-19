/*!
Parallel usage examples for Lodash-RS.

This example demonstrates the parallel processing functionality of the Lodash-RS library.
*/

#[cfg(feature = "parallel")]
use lodash_rs::prelude::*;
#[cfg(feature = "parallel")]
use std::time::Instant;

#[cfg(feature = "parallel")]
fn main() {
    println!("=== Lodash-RS Parallel Usage Examples ===\n");

    // Basic parallel operations
    basic_parallel_operations();
    
    // Performance comparison
    performance_comparison();
    
    // Parallel chain operations
    parallel_chain_operations();
}

#[cfg(feature = "parallel")]
fn basic_parallel_operations() {
    println!("1. Basic Parallel Operations:");
    
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // Parallel map operation
    let doubled = map_parallel(&numbers, |x| x * 2);
    println!("  mapParallel([1..1000], x => x * 2) = [{} elements]", doubled.len());
    
    // Parallel filter operation
    let evens = filter_parallel(&numbers, |x| x % 2 == 0);
    println!("  filterParallel([1..1000], x => x % 2 === 0) = [{} elements]", evens.len());
    
    // Parallel reduce operation
    let sum = reduce_parallel(&numbers, |acc, x| acc + x, 0);
    println!("  reduceParallel([1..1000], (acc, x) => acc + x, 0) = {}", sum);
    
    // Parallel forEach operation
    let mut count = 0;
    for_each_parallel(&numbers, |_| count += 1);
    println!("  forEachParallel([1..1000], x => count++) = count = {}", count);
    
    // Parallel find operation
    let first_even = find_parallel(&numbers, |x| x % 2 == 0);
    println!("  findParallel([1..1000], x => x % 2 === 0) = {:?}", first_even);
    
    // Parallel every operation
    let all_positive = every_parallel(&numbers, |x| *x > 0);
    println!("  everyParallel([1..1000], x => x > 0) = {}", all_positive);
    
    // Parallel some operation
    let has_even = some_parallel(&numbers, |x| x % 2 == 0);
    println!("  someParallel([1..1000], x => x % 2 === 0) = {}", has_even);
    
    println!();
}

#[cfg(feature = "parallel")]
fn performance_comparison() {
    println!("2. Performance Comparison:");
    
    let numbers: Vec<i32> = (1..=10000).collect();
    
    // Sequential map
    let start = Instant::now();
    let _sequential = map(&numbers, |x| x * x);
    let sequential_time = start.elapsed();
    
    // Parallel map
    let start = Instant::now();
    let _parallel = map_parallel(&numbers, |x| x * x);
    let parallel_time = start.elapsed();
    
    println!("  Sequential map: {:?}", sequential_time);
    println!("  Parallel map: {:?}", parallel_time);
    println!("  Speedup: {:.2}x", sequential_time.as_nanos() as f64 / parallel_time.as_nanos() as f64);
    
    // Sequential filter
    let start = Instant::now();
    let _sequential = filter(&numbers, |x| x % 2 == 0);
    let sequential_time = start.elapsed();
    
    // Parallel filter
    let start = Instant::now();
    let _parallel = filter_parallel(&numbers, |x| x % 2 == 0);
    let parallel_time = start.elapsed();
    
    println!("  Sequential filter: {:?}", sequential_time);
    println!("  Parallel filter: {:?}", parallel_time);
    println!("  Speedup: {:.2}x", sequential_time.as_nanos() as f64 / parallel_time.as_nanos() as f64);
    
    println!();
}

#[cfg(feature = "parallel")]
fn parallel_chain_operations() {
    println!("3. Parallel Chain Operations:");
    
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // Note: Chain operations are not directly parallel in the current implementation
    // but the individual operations within the chain can be parallel
    let result = chain(&numbers)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .take(10)
        .collect::<Vec<_>>();
    println!("  chain([1..1000]).filter(x => x % 2 === 0).map(x => x * 3).take(10) = {:?}", result);
    
    println!();
}

#[cfg(not(feature = "parallel"))]
fn main() {
    println!("Parallel features are not enabled. Please run with --features parallel");
}
