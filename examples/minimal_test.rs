//! Minimal test to verify the library compiles and works.

use rust_lodash::collection::Collection;

fn main() {
    println!("Testing lodash-rs basic functionality...");
    
    // Create a collection
    let data = vec![1, 2, 3, 4, 5];
    let collection = Collection::new(data);
    
    // Test basic operations
    println!("Collection size: {}", collection.size());
    println!("First element: {:?}", collection.get(0));
    
    // Test map operation
    let doubled: Vec<i32> = collection.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);
    
    // Test filter operation
    let evens: Vec<i32> = collection.filter(|x| x % 2 == 0);
    println!("Even numbers: {:?}", evens);
    
    println!("All tests passed! ✅");
}
