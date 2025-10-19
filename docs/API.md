# rust-lodash API Reference

This document provides a comprehensive reference for all rust-lodash APIs.

## Table of Contents

- [Collection Operations](#collection-operations)
  - [Iteration](#iteration)
  - [Query](#query)
  - [Transform](#transform)
  - [Operations](#operations)
- [Chain API](#chain-api)
- [Collection Type](#collection-type)
- [Error Handling](#error-handling)
- [Type Conversion](#type-conversion)

## Collection Operations

### Iteration

#### `map(collection, iteratee)`

Creates an array of values by running each element in collection through iteratee.

```rust
use rust_lodash::map;

let numbers = vec![1, 2, 3, 4, 5];
let doubled = map(&numbers, |x| x * 2);
// Result: [2, 4, 6, 8, 10]
```

**Parameters:**
- `collection: &[T]` - The collection to iterate over
- `iteratee: F` - The function invoked per iteration

**Returns:** `Vec<U>` - The new mapped array

#### `filter(collection, predicate)`

Iterates over elements of collection, returning an array of all elements predicate returns truthy for.

```rust
use rust_lodash::filter;

let numbers = vec![1, 2, 3, 4, 5];
let evens = filter(&numbers, |x| x % 2 == 0);
// Result: [2, 4]
```

**Parameters:**
- `collection: &[T]` - The collection to iterate over
- `predicate: F` - The function invoked per iteration

**Returns:** `Vec<T>` - Returns the new filtered array

#### `reduce(collection, iteratee, initial)`

Reduces collection to a value which is the accumulated result of running each element in collection through iteratee.

```rust
use rust_lodash::reduce;

let numbers = vec![1, 2, 3, 4, 5];
let sum = reduce(&numbers, |acc, x| acc + x, 0);
// Result: 15
```

**Parameters:**
- `collection: &[T]` - The collection to iterate over
- `iteratee: F` - The function invoked per iteration
- `initial: U` - The initial value

**Returns:** `U` - Returns the accumulated value

#### `forEach(collection, iteratee)`

Iterates over elements of collection and invokes iteratee for each element.

```rust
use rust_lodash::forEach;

let numbers = vec![1, 2, 3];
let mut sum = 0;
forEach(&numbers, |x| sum += x);
// sum is now 6
```

**Parameters:**
- `collection: &[T]` - The collection to iterate over
- `iteratee: F` - The function invoked per iteration

**Returns:** `()`

#### `forEachRight(collection, iteratee)`

This method is like `forEach` except that it iterates over elements of collection from right to left.

```rust
use rust_lodash::forEachRight;

let numbers = vec![1, 2, 3];
let mut result = Vec::new();
forEachRight(&numbers, |x| result.push(*x));
// Result: [3, 2, 1]
```

### Query

#### `find(collection, predicate)`

Iterates over elements of collection, returning the first element predicate returns truthy for.

```rust
use rust_lodash::find;

let numbers = vec![1, 2, 3, 4, 5];
let first_even = find(&numbers, |x| x % 2 == 0);
// Result: Some(&2)
```

**Parameters:**
- `collection: &[T]` - The collection to search
- `predicate: F` - The function invoked per iteration

**Returns:** `Option<&T>` - Returns the matched element, else `None`

#### `findLast(collection, predicate)`

This method is like `find` except that it iterates over elements of collection from right to left.

```rust
use rust_lodash::findLast;

let numbers = vec![1, 2, 3, 4, 5];
let last_even = findLast(&numbers, |x| x % 2 == 0);
// Result: Some(&4)
```

#### `includes(collection, value)`

Checks if value is in collection.

```rust
use rust_lodash::includes;

let numbers = vec![1, 2, 3, 4, 5];
let has_three = includes(&numbers, &3);
// Result: true
```

**Parameters:**
- `collection: &[T]` - The collection to inspect
- `value: &T` - The value to search for

**Returns:** `bool` - Returns `true` if value is found, else `false`

#### `every(collection, predicate)`

Checks if predicate returns truthy for all elements of collection.

```rust
use rust_lodash::every;

let numbers = vec![2, 4, 6, 8];
let all_even = every(&numbers, |x| x % 2 == 0);
// Result: true
```

**Parameters:**
- `collection: &[T]` - The collection to iterate over
- `predicate: F` - The function invoked per iteration

**Returns:** `bool` - Returns `true` if all elements pass the predicate check, else `false`

#### `some(collection, predicate)`

Checks if predicate returns truthy for any element of collection.

```rust
use rust_lodash::some;

let numbers = vec![1, 2, 3, 4, 5];
let has_even = some(&numbers, |x| x % 2 == 0);
// Result: true
```

#### `countBy(collection, iteratee)`

Creates an object composed of keys generated from the results of running each element of collection thru iteratee.

```rust
use rust_lodash::countBy;
use std::collections::HashMap;

let numbers = vec![6.1, 4.2, 6.3];
let counts = countBy(&numbers, |x| (*x as f64).floor() as i32);
// Result: {6: 2, 4: 1}
```

**Returns:** `HashMap<K, usize>` - Returns the composed aggregate object

#### `partition(collection, predicate)`

Creates an array of elements split into two groups, the first of which contains elements predicate returns truthy for.

```rust
use rust_lodash::partition;

let numbers = vec![1, 2, 3, 4, 5];
let (evens, odds) = partition(&numbers, |x| x % 2 == 0);
// Result: ([2, 4], [1, 3, 5])
```

**Returns:** `(Vec<T>, Vec<T>)` - Returns the array of grouped elements

### Transform

#### `groupBy(collection, iteratee)`

Creates an object composed of keys generated from the results of running each element of collection thru iteratee.

```rust
use rust_lodash::groupBy;
use std::collections::HashMap;

let users = vec![
    ("john", 30, "engineer"),
    ("jane", 25, "designer"),
    ("bob", 35, "manager"),
];
let grouped = groupBy(&users, |(_, _, role)| role.to_string());
// Result: {"engineer": [("john", 30, "engineer")], "designer": [("jane", 25, "designer")], "manager": [("bob", 35, "manager")]}
```

**Returns:** `HashMap<K, Vec<T>>` - Returns the composed aggregate object

#### `keyBy(collection, iteratee)`

Creates an object composed of keys generated from the results of running each element of collection thru iteratee.

```rust
use rust_lodash::keyBy;
use std::collections::HashMap;

let users = vec![
    ("john", 30, "engineer"),
    ("jane", 25, "designer"),
];
let keyed = keyBy(&users, |(name, _, _)| name.to_string());
// Result: {"john": ("john", 30, "engineer"), "jane": ("jane", 25, "designer")}
```

**Returns:** `HashMap<K, T>` - Returns the composed aggregate object

#### `sortBy(collection, iteratee)`

Creates an array of elements, sorted in ascending order by the results of running each element in a collection thru each iteratee.

```rust
use rust_lodash::sortBy;

let users = vec![
    ("john", 30, "engineer"),
    ("jane", 25, "designer"),
    ("bob", 35, "manager"),
];
let sorted = sortBy(&users, |(_, age, _)| *age);
// Result: [("jane", 25, "designer"), ("john", 30, "engineer"), ("bob", 35, "manager")]
```

**Returns:** `Vec<T>` - Returns the new sorted array

#### `orderBy(collection, iteratee, descending)`

This method is like `sortBy` except that it allows specifying the sort orders of the iteratees to sort by.

```rust
use rust_lodash::orderBy;

let users = vec![
    ("john", 30, "engineer"),
    ("jane", 25, "designer"),
    ("bob", 35, "manager"),
];
let ordered = orderBy(&users, |(_, age, _)| *age, false); // descending
// Result: [("bob", 35, "manager"), ("john", 30, "engineer"), ("jane", 25, "designer")]
```

**Parameters:**
- `descending: bool` - Specify the sort order

#### `invoke(collection, method)`

Invokes the method at path of each element in collection.

```rust
use rust_lodash::invoke;

let names = vec!["john", "jane", "bob"];
let upper_names = invoke(&names, |name| name.to_uppercase());
// Result: ["JOHN", "JANE", "BOB"]
```

### Operations

#### `shuffle(collection)`

Creates an array of shuffled values, using a version of the Fisher-Yates shuffle.

```rust
use rust_lodash::shuffle;

let numbers = vec![1, 2, 3, 4, 5];
let shuffled = shuffle(&numbers);
// Result: [3, 1, 5, 2, 4] (random order)
```

**Returns:** `Vec<T>` - Returns the new shuffled array

#### `sample(collection)`

Gets a random element from collection.

```rust
use rust_lodash::sample;

let numbers = vec![1, 2, 3, 4, 5];
let random = sample(&numbers);
// Result: Some(&3) (random element)
```

**Returns:** `Option<&T>` - Returns the random element

#### `sampleSize(collection, n)`

Gets n random elements at unique keys from collection up to the size of collection.

```rust
use rust_lodash::sampleSize;

let numbers = vec![1, 2, 3, 4, 5];
let samples = sampleSize(&numbers, 3);
// Result: [2, 5, 1] (3 random elements)
```

**Parameters:**
- `n: usize` - The number of elements to sample

**Returns:** `Vec<T>` - Returns the random elements

#### `size(collection)`

Gets the size of collection.

```rust
use rust_lodash::size;

let numbers = vec![1, 2, 3, 4, 5];
let length = size(&numbers);
// Result: 5
```

**Returns:** `usize` - Returns the collection size

## Chain API

The Chain API provides a fluent interface for method chaining.

### Basic Usage

```rust
use rust_lodash::chain;

let result = chain(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    .filter(|x| x % 2 == 0)    // [2, 4, 6, 8, 10]
    .map(|x| x * 3)            // [6, 12, 18, 24, 30]
    .take(3)                   // [6, 12, 18]
    .reverse()                 // [18, 12, 6]
    .collect();                // Vec<i32>
```

### Available Methods

- `map(mapper)` - Transform each element
- `filter(predicate)` - Filter elements by condition
- `take(n)` - Take first n elements
- `skip(n)` - Skip first n elements
- `reverse()` - Reverse the order
- `collect()` - Collect into Vec<T>
- `value()` - Get the final value
- `into_collection()` - Convert to Collection<T>

## Collection Type

The `Collection<T>` type provides an object-oriented interface to collection operations.

### Creating Collections

```rust
use rust_lodash::Collection;

// From vector
let collection = Collection::new(vec![1, 2, 3, 4, 5]);

// Empty collection
let empty = Collection::<i32>::empty();
```

### Available Methods

All collection operations are available as methods on the `Collection` type:

```rust
let collection = Collection::new(vec![1, 2, 3, 4, 5]);

// Iteration
let doubled = collection.map(|x| x * 2);
let evens = collection.filter(|x| x % 2 == 0);
let sum = collection.reduce(|acc, x| acc + x, 0);

// Query
let first_even = collection.find(|x| x % 2 == 0);
let has_three = collection.includes(&3);
let all_positive = collection.every(|x| *x > 0);

// Transform
let grouped = collection.group_by(|x| x % 3);
let sorted = collection.sort_by(|x| *x);

// Operations
let shuffled = collection.shuffle();
let random = collection.sample();
let size = collection.size();
```

## Error Handling

lodash-rs uses custom error types for better error handling.

### LodashError

```rust
use rust_lodash::LodashError;

// Error types
LodashError::InvalidInput { message: String }
LodashError::TypeConversion { from: String, to: String }
LodashError::IndexOutOfBounds { index: usize, size: usize }
LodashError::InvalidPredicate { message: String }
LodashError::EmptyCollection
LodashError::Custom { message: String }
```

### Result Type

```rust
use rust_lodash::Result;

// Most operations return Result<T, LodashError>
let result: Result<Vec<i32>> = some_operation();
match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(error) => println!("Error: {}", error),
}
```

## Type Conversion

lodash-rs provides utilities for safe type conversion.

### Safe Conversion

```rust
use rust_lodash::safe_convert;

let number_str = "42";
let number: Result<i32> = safe_convert(number_str);
// Result: Ok(42)
```

### Type Traits

- `ToKey<T>` - Convert to key type
- `ToComparable<T>` - Convert to comparable type
- `SafeClone<T>` - Safe cloning trait

## Feature Flags

lodash-rs supports optional features:

- `async` - Async/await support
- `parallel` - Parallel processing with rayon
- `serialize` - Serialization support with serde
- `wasm` - WebAssembly support
- `no-std` - No standard library support

### Using Features

```toml
[dependencies]
lodash-rs = { version = "0.1.0", features = ["async", "parallel"] }
```

## Performance Notes

- All operations are optimized for performance
- Method chaining has zero runtime overhead
- Memory allocations are minimized
- Iterator-based implementations for efficiency
- SIMD-ready for future optimizations
