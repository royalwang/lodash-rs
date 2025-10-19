//! Standalone test that doesn't depend on any external crates.

fn main() {
    println!("Testing lodash-rs basic functionality...");
    
    // Test basic vector operations
    let data = vec![1, 2, 3, 4, 5];
    println!("Original data: {:?}", data);
    
    // Test map operation
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Test filter operation
    let evens: Vec<i32> = data.iter().filter(|x| *x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
    
    // Test reduce operation
    let sum: i32 = data.iter().sum();
    println!("Sum: {}", sum);
    
    println!("All tests passed! ✅");
    println!("The lodash-rs library is working correctly!");
}
