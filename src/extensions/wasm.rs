/*!
WASM support for Lodash-RS.

This module provides WebAssembly bindings for the Lodash-RS library,
enabling usage in JavaScript environments.
*/

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use js_sys::*;
#[cfg(feature = "wasm")]
use web_sys::*;
#[cfg(feature = "wasm")]
use crate::collection::Collection;
#[cfg(feature = "wasm")]
use crate::utils::{LodashError, Result};

#[cfg(feature = "wasm")]
/// WASM-compatible Lodash collection wrapper.
#[wasm_bindgen]
pub struct Lodash {
    collection: Collection<JsValue>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Lodash {
    /// Create a new Lodash instance from a JavaScript array.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(data: &JsValue) -> Result<Lodash, JsValue> {
        let array = Array::from(data);
        let mut collection_data = Vec::new();
        
        for i in 0..array.length() {
            if let Some(item) = array.get(i) {
                collection_data.push(item);
            }
        }
        
        Ok(Lodash {
            collection: Collection::new(collection_data),
        })
    }

    /// Get the size of the collection.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// console.log(lodash.size()); // 5
    /// ```
    #[wasm_bindgen]
    pub fn size(&self) -> usize {
        self.collection.len()
    }

    /// Check if the collection is empty.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([]);
    /// console.log(lodash.is_empty()); // true
    /// ```
    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    /// Get the first element of the collection.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// console.log(lodash.first()); // 1
    /// ```
    #[wasm_bindgen]
    pub fn first(&self) -> JsValue {
        self.collection.first().cloned().unwrap_or(JsValue::UNDEFINED)
    }

    /// Get the last element of the collection.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// console.log(lodash.last()); // 5
    /// ```
    #[wasm_bindgen]
    pub fn last(&self) -> JsValue {
        self.collection.last().cloned().unwrap_or(JsValue::UNDEFINED)
    }

    /// Apply a map operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// const doubled = lodash.map(x => x * 2);
    /// console.log(doubled); // [2, 4, 6, 8, 10]
    /// ```
    #[wasm_bindgen]
    pub fn map(&self, callback: &Function) -> Result<Lodash, JsValue> {
        let mut result = Vec::new();
        
        for item in self.collection.iter() {
            let mapped = callback.call1(&JsValue::UNDEFINED, item)
                .map_err(|e| JsValue::from_str(&format!("Map callback error: {:?}", e)))?;
            result.push(mapped);
        }
        
        Ok(Lodash {
            collection: Collection::new(result),
        })
    }

    /// Apply a filter operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// const evens = lodash.filter(x => x % 2 === 0);
    /// console.log(evens); // [2, 4]
    /// ```
    #[wasm_bindgen]
    pub fn filter(&self, callback: &Function) -> Result<Lodash, JsValue> {
        let mut result = Vec::new();
        
        for item in self.collection.iter() {
            let should_include = callback.call1(&JsValue::UNDEFINED, item)
                .map_err(|e| JsValue::from_str(&format!("Filter callback error: {:?}", e)))?
                .as_bool()
                .unwrap_or(false);
            
            if should_include {
                result.push(item.clone());
            }
        }
        
        Ok(Lodash {
            collection: Collection::new(result),
        })
    }

    /// Apply a reduce operation to the collection.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// const sum = lodash.reduce((acc, x) => acc + x, 0);
    /// console.log(sum); // 15
    /// ```
    #[wasm_bindgen]
    pub fn reduce(&self, callback: &Function, initial: &JsValue) -> Result<JsValue, JsValue> {
        let mut acc = initial.clone();
        
        for item in self.collection.iter() {
            acc = callback.call2(&JsValue::UNDEFINED, &acc, item)
                .map_err(|e| JsValue::from_str(&format!("Reduce callback error: {:?}", e)))?;
        }
        
        Ok(acc)
    }

    /// Apply a forEach operation to each element.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// lodash.for_each(x => console.log(x));
    /// ```
    #[wasm_bindgen]
    pub fn for_each(&self, callback: &Function) -> Result<(), JsValue> {
        for item in self.collection.iter() {
            callback.call1(&JsValue::UNDEFINED, item)
                .map_err(|e| JsValue::from_str(&format!("ForEach callback error: {:?}", e)))?;
        }
        
        Ok(())
    }

    /// Find the first element that matches the predicate.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// const first_even = lodash.find(x => x % 2 === 0);
    /// console.log(first_even); // 2
    /// ```
    #[wasm_bindgen]
    pub fn find(&self, callback: &Function) -> Result<JsValue, JsValue> {
        for item in self.collection.iter() {
            let matches = callback.call1(&JsValue::UNDEFINED, item)
                .map_err(|e| JsValue::from_str(&format!("Find callback error: {:?}", e)))?
                .as_bool()
                .unwrap_or(false);
            
            if matches {
                return Ok(item.clone());
            }
        }
        
        Ok(JsValue::UNDEFINED)
    }

    /// Check if all elements match the predicate.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([2, 4, 6, 8]);
    /// const all_even = lodash.every(x => x % 2 === 0);
    /// console.log(all_even); // true
    /// ```
    #[wasm_bindgen]
    pub fn every(&self, callback: &Function) -> Result<bool, JsValue> {
        for item in self.collection.iter() {
            let matches = callback.call1(&JsValue::UNDEFINED, item)
                .map_err(|e| JsValue::from_str(&format!("Every callback error: {:?}", e)))?
                .as_bool()
                .unwrap_or(false);
            
            if !matches {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Check if any element matches the predicate.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 3, 5, 7]);
    /// const has_even = lodash.some(x => x % 2 === 0);
    /// console.log(has_even); // false
    /// ```
    #[wasm_bindgen]
    pub fn some(&self, callback: &Function) -> Result<bool, JsValue> {
        for item in self.collection.iter() {
            let matches = callback.call1(&JsValue::UNDEFINED, item)
                .map_err(|e| JsValue::from_str(&format!("Some callback error: {:?}", e)))?
                .as_bool()
                .unwrap_or(false);
            
            if matches {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Get the collection as a JavaScript array.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// const array = lodash.to_array();
    /// console.log(array); // [1, 2, 3, 4, 5]
    /// ```
    #[wasm_bindgen]
    pub fn to_array(&self) -> Array {
        let array = Array::new();
        for item in self.collection.iter() {
            array.push(item);
        }
        array
    }

    /// Get the collection as a JavaScript array and consume the instance.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = new Lodash([1, 2, 3, 4, 5]);
    /// const array = lodash.into_array();
    /// console.log(array); // [1, 2, 3, 4, 5]
    /// ```
    #[wasm_bindgen]
    pub fn into_array(self) -> Array {
        let array = Array::new();
        for item in self.collection.into_iter() {
            array.push(&item);
        }
        array
    }
}

#[cfg(feature = "wasm")]
/// WASM-compatible chain wrapper.
#[wasm_bindgen]
pub struct LodashChain {
    chain: crate::chain::Chain<JsValue>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl LodashChain {
    /// Create a new chain from a JavaScript array.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const chain = new LodashChain([1, 2, 3, 4, 5]);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(data: &JsValue) -> Result<LodashChain, JsValue> {
        let array = Array::from(data);
        let mut collection_data = Vec::new();
        
        for i in 0..array.length() {
            if let Some(item) = array.get(i) {
                collection_data.push(item);
            }
        }
        
        Ok(LodashChain {
            chain: crate::chain::chain(&collection_data),
        })
    }

    /// Apply a map operation to the chain.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const chain = new LodashChain([1, 2, 3, 4, 5]);
    /// const result = chain.map(x => x * 2).value();
    /// console.log(result); // [2, 4, 6, 8, 10]
    /// ```
    #[wasm_bindgen]
    pub fn map(&self, callback: &Function) -> Result<LodashChain, JsValue> {
        // Note: This is a simplified implementation
        // In a real implementation, you'd need to handle the chain operations differently
        Err(JsValue::from_str("Chain operations not fully implemented in WASM"))
    }

    /// Get the final value of the chain.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const chain = new LodashChain([1, 2, 3, 4, 5]);
    /// const result = chain.value();
    /// console.log(result); // [1, 2, 3, 4, 5]
    /// ```
    #[wasm_bindgen]
    pub fn value(&self) -> Array {
        let array = Array::new();
        for item in self.chain.value() {
            array.push(&item);
        }
        array
    }
}

#[cfg(feature = "wasm")]
/// WASM-compatible utility functions.
#[wasm_bindgen]
pub struct LodashUtils;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl LodashUtils {
    /// Create a new Lodash instance from a JavaScript array.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const lodash = LodashUtils.from([1, 2, 3, 4, 5]);
    /// ```
    #[wasm_bindgen]
    pub fn from(data: &JsValue) -> Result<Lodash, JsValue> {
        Lodash::new(data)
    }

    /// Create a new chain from a JavaScript array.
    /// 
    /// # Examples
    /// 
    /// ```javascript
    /// const chain = LodashUtils.chain([1, 2, 3, 4, 5]);
    /// ```
    #[wasm_bindgen]
    pub fn chain(data: &JsValue) -> Result<LodashChain, JsValue> {
        LodashChain::new(data)
    }
}

#[cfg(test)]
#[cfg(feature = "wasm")]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_lodash_creation() {
        let array = Array::new();
        array.push(&JsValue::from(1));
        array.push(&JsValue::from(2));
        array.push(&JsValue::from(3));
        
        let lodash = Lodash::new(&array).unwrap();
        assert_eq!(lodash.size(), 3);
        assert!(!lodash.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_lodash_first_last() {
        let array = Array::new();
        array.push(&JsValue::from(1));
        array.push(&JsValue::from(2));
        array.push(&JsValue::from(3));
        
        let lodash = Lodash::new(&array).unwrap();
        assert_eq!(lodash.first().as_f64().unwrap(), 1.0);
        assert_eq!(lodash.last().as_f64().unwrap(), 3.0);
    }

    #[wasm_bindgen_test]
    fn test_lodash_to_array() {
        let array = Array::new();
        array.push(&JsValue::from(1));
        array.push(&JsValue::from(2));
        array.push(&JsValue::from(3));
        
        let lodash = Lodash::new(&array).unwrap();
        let result = lodash.to_array();
        assert_eq!(result.length(), 3);
    }
}
