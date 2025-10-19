/*!
Basic usage examples for Lodash-RS.

This example demonstrates the core functionality of the Lodash-RS library.
*/

use lodash_rs::prelude::*;

fn main() {
    println!("=== Lodash-RS Basic Usage Examples ===\n");

    // Basic collection operations
    basic_operations();
    
    // Chain operations
    chain_operations();
    
    // Query operations
    query_operations();
    
    // Transform operations
    transform_operations();
    
    // Collection operations
    collection_operations();
}

fn basic_operations() {
    println!("1. Basic Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Map operation
    let doubled = map(&numbers, |x| x * 2);
    println!("  map([1,2,3,4,5], x => x * 2) = {:?}", doubled);
    
    // Filter operation
    let evens = filter(&numbers, |x| x % 2 == 0);
    println!("  filter([1,2,3,4,5], x => x % 2 === 0) = {:?}", evens);
    
    // Reduce operation
    let sum = reduce(&numbers, |acc, x| acc + x, 0);
    println!("  reduce([1,2,3,4,5], (acc, x) => acc + x, 0) = {}", sum);
    
    // ForEach operation
    print!("  forEach([1,2,3,4,5], x => print!(\"{} \", x)) = ");
    for_each(&numbers, |x| print!("{} ", x));
    println!();
    
    println!();
}

fn chain_operations() {
    println!("2. Chain Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Simple chain
    let result = chain(&numbers)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .collect::<Vec<_>>();
    println!("  chain([1,2,3,4,5,6,7,8,9,10]).filter(x => x % 2 === 0).map(x => x * 3) = {:?}", result);
    
    // Complex chain
    let result = chain(&numbers)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .take(3)
        .collect::<Vec<_>>();
    println!("  chain([1,2,3,4,5,6,7,8,9,10]).filter(x => x % 2 === 0).map(x => x * 3).take(3) = {:?}", result);
    
    // Chain with reverse
    let result = chain(&numbers)
        .take(5)
        .reverse()
        .collect::<Vec<_>>();
    println!("  chain([1,2,3,4,5,6,7,8,9,10]).take(5).reverse() = {:?}", result);
    
    println!();
}

fn query_operations() {
    println!("3. Query Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Find operations
    let first_even = find(&numbers, |x| x % 2 == 0);
    println!("  find([1,2,3,4,5,6,7,8,9,10], x => x % 2 === 0) = {:?}", first_even);
    
    let last_even = find_last(&numbers, |x| x % 2 == 0);
    println!("  findLast([1,2,3,4,5,6,7,8,9,10], x => x % 2 === 0) = {:?}", last_even);
    
    // Includes operation
    let includes_5 = includes(&numbers, &5);
    println!("  includes([1,2,3,4,5,6,7,8,9,10], 5) = {}", includes_5);
    
    // Every operation
    let all_positive = every(&numbers, |x| *x > 0);
    println!("  every([1,2,3,4,5,6,7,8,9,10], x => x > 0) = {}", all_positive);
    
    // Some operation
    let has_even = some(&numbers, |x| x % 2 == 0);
    println!("  some([1,2,3,4,5,6,7,8,9,10], x => x % 2 === 0) = {}", has_even);
    
    // Count by operation
    let counts = count_by(&numbers, |x| x % 3);
    println!("  countBy([1,2,3,4,5,6,7,8,9,10], x => x % 3) = {:?}", counts);
    
    // Partition operation
    let (evens, odds) = partition(&numbers, |x| x % 2 == 0);
    println!("  partition([1,2,3,4,5,6,7,8,9,10], x => x % 2 === 0) = ({:?}, {:?})", evens, odds);
    
    println!();
}

fn transform_operations() {
    println!("4. Transform Operations:");
    
    let users = vec![
        ("john", 30, "engineer"),
        ("jane", 25, "designer"),
        ("bob", 35, "manager"),
        ("alice", 28, "engineer"),
        ("charlie", 32, "designer"),
    ];
    
    // Group by operation
    let grouped_by_role = group_by(&users, |(_, _, role)| role.to_string());
    println!("  groupBy(users, user => user.role) = {:?}", grouped_by_role);
    
    // Key by operation
    let keyed_by_name = key_by(&users, |(name, _, _)| name.to_string());
    println!("  keyBy(users, user => user.name) = {:?}", keyed_by_name);
    
    // Sort by operation
    let sorted_by_age = sort_by(&users, |(_, age, _)| *age);
    println!("  sortBy(users, user => user.age) = {:?}", sorted_by_age);
    
    // Order by operation (descending)
    let ordered_by_age = order_by(&users, |(_, age, _)| *age, false);
    println!("  orderBy(users, user => user.age, false) = {:?}", ordered_by_age);
    
    // Invoke operation
    let names = invoke(&users, |(name, _, _)| name.to_uppercase());
    println!("  invoke(users, user => user.name.toUpperCase()) = {:?}", names);
    
    println!();
}

fn collection_operations() {
    println!("5. Collection Operations:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Size operation
    let size = size(&numbers);
    println!("  size([1,2,3,4,5,6,7,8,9,10]) = {}", size);
    
    // Shuffle operation
    let shuffled = shuffle(&numbers);
    println!("  shuffle([1,2,3,4,5,6,7,8,9,10]) = {:?}", shuffled);
    
    // Sample operation
    let sample = sample(&numbers);
    println!("  sample([1,2,3,4,5,6,7,8,9,10]) = {:?}", sample);
    
    // Sample size operation
    let samples = sample_size(&numbers, 3);
    println!("  sampleSize([1,2,3,4,5,6,7,8,9,10], 3) = {:?}", samples);
    
    println!();
}
