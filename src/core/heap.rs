use regex::Regex;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yml::Value;

/// A shared data store for workflow execution
///
/// The `Heap` provides a thread-safe way to share data between workflow nodes.
/// It supports variable substitution using a template-like syntax.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heap {
    data: HashMap<String, Option<Value>>,
}

impl Heap {
    /// Creates a new empty heap
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    ///
    /// let heap = Heap::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Gets a value from the heap by key
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns a reference to the value if it exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// heap.insert("name", Some(Value::String("John".to_string())));
    ///
    /// assert_eq!(heap.get("name"), Some(&Value::String("John".to_string())));
    /// assert_eq!(heap.get("nonexistent"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key).and_then(|opt_value| opt_value.as_ref())
    }

    /// Inserts a value into the heap
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the value
    /// * `value` - The value to insert
    ///
    /// # Returns
    ///
    /// Returns the previous value if the key already existed, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// let old_value = heap.insert("name", Some(Value::String("John".to_string())));
    /// assert_eq!(old_value, None);
    ///
    /// let old_value = heap.insert("name", Some(Value::String("Jane".to_string())));
    /// assert_eq!(old_value, Some(Value::String("John".to_string())));
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: Option<Value>) -> Option<Value> {
        self.data.insert(key.into(), value).flatten()
    }

    /// Checks if a key exists in the heap
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the key exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// heap.insert("name", Some(Value::String("John".to_string())));
    ///
    /// assert!(heap.contains_key("name"));
    /// assert!(!heap.contains_key("nonexistent"));
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Removes a key-value pair from the heap
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Returns
    ///
    /// Returns the removed value if it existed, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// heap.insert("name", Some(Value::String("John".to_string())));
    ///
    /// let removed = heap.remove("name");
    /// assert_eq!(removed, Some(Value::String("John".to_string())));
    /// assert!(!heap.contains_key("name"));
    /// ```
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.data.remove(key).flatten()
    }

    /// Returns the number of key-value pairs in the heap
    ///
    /// # Returns
    ///
    /// Returns the number of entries in the heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// assert_eq!(heap.len(), 0);
    ///
    /// heap.insert("name", Some(Value::String("John".to_string())));
    /// heap.insert("age", Some(Value::Number(30.into())));
    /// assert_eq!(heap.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the heap is empty
    ///
    /// # Returns
    ///
    /// Returns `true` if the heap contains no key-value pairs, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    ///
    /// let heap = Heap::new();
    /// assert!(heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears all key-value pairs from the heap
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// heap.insert("name", Some(Value::String("John".to_string())));
    /// heap.insert("age", Some(Value::Number(30.into())));
    ///
    /// heap.clear();
    /// assert!(heap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Parses a string value and replaces variables with values from the heap
    ///
    /// # Arguments
    ///
    /// * `value` - The value to parse, which may contain variable references
    ///
    /// # Returns
    ///
    /// Returns the parsed value with variables substituted, or the original value if no substitution is needed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// heap.insert("name", Some(Value::String("John".to_string())));
    /// heap.insert("age", Some(Value::Number(30.into())));
    ///
    /// let input = Value::String("Hello ${{name}}, you are ${{age}} years old".to_string());
    /// let result = heap.parse(Some(input));
    /// assert_eq!(result, Some(Value::String("Hello John, you are 30 years old".to_string())));
    /// ```
    pub fn parse(&self, value: Option<Value>) -> Option<Value> {
        match value {
            Some(Value::String(s)) => {
                let re = Regex::new(r"\$\{\{([^}]+)\}\}").unwrap();
                let mut result = s.clone();

                for cap in re.captures_iter(&s) {
                    if let Some(key) = cap.get(1) {
                        let key = key.as_str().trim();
                        if let Some(val) = self.get(key) {
                            let replacement = self.value_to_string(val);
                            result = result.replace(&cap[0], &replacement);
                        }
                    }
                }

                Some(Value::String(result))
            }
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Converts a value to a string representation
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert
    ///
    /// # Returns
    ///
    /// Returns a string representation of the value.
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => serde_yml::to_string(value).unwrap_or_else(|_| "".to_string()),
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<String, Option<Value>>> for Heap {
    fn from(data: HashMap<String, Option<Value>>) -> Self {
        Self { data }
    }
}

impl Into<HashMap<String, Option<Value>>> for Heap {
    fn into(self) -> HashMap<String, Option<Value>> {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_new() {
        let heap = Heap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_heap_default() {
        let heap = Heap::default();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_heap_insert_and_get() {
        let mut heap = Heap::new();

        // Insert a string value
        let old_value = heap.insert("name", Some(Value::String("John".to_string())));
        assert_eq!(old_value, None);
        assert_eq!(heap.get("name"), Some(&Value::String("John".to_string())));

        // Insert a number value
        heap.insert("age", Some(Value::Number(30.into())));
        assert_eq!(heap.get("age"), Some(&Value::Number(30.into())));

        // Insert a boolean value
        heap.insert("active", Some(Value::Bool(true)));
        assert_eq!(heap.get("active"), Some(&Value::Bool(true)));

        // Test non-existent key
        assert_eq!(heap.get("nonexistent"), None);
    }

    #[test]
    fn test_heap_insert_overwrite() {
        let mut heap = Heap::new();

        heap.insert("name", Some(Value::String("John".to_string())));
        let old_value = heap.insert("name", Some(Value::String("Jane".to_string())));

        assert_eq!(old_value, Some(Value::String("John".to_string())));
        assert_eq!(heap.get("name"), Some(&Value::String("Jane".to_string())));
    }

    #[test]
    fn test_heap_contains_key() {
        let mut heap = Heap::new();

        assert!(!heap.contains_key("name"));

        heap.insert("name", Some(Value::String("John".to_string())));
        assert!(heap.contains_key("name"));
        assert!(!heap.contains_key("age"));
    }

    #[test]
    fn test_heap_remove() {
        let mut heap = Heap::new();

        heap.insert("name", Some(Value::String("John".to_string())));
        heap.insert("age", Some(Value::Number(30.into())));

        assert_eq!(heap.len(), 2);

        let removed = heap.remove("name");
        assert_eq!(removed, Some(Value::String("John".to_string())));
        assert_eq!(heap.len(), 1);
        assert!(!heap.contains_key("name"));
        assert!(heap.contains_key("age"));

        // Remove non-existent key
        let removed = heap.remove("nonexistent");
        assert_eq!(removed, None);
    }

    #[test]
    fn test_heap_clear() {
        let mut heap = Heap::new();

        heap.insert("name", Some(Value::String("John".to_string())));
        heap.insert("age", Some(Value::Number(30.into())));

        assert_eq!(heap.len(), 2);
        assert!(!heap.is_empty());

        heap.clear();

        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_heap_from_hashmap() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(Value::String("John".to_string())));
        data.insert("age".to_string(), Some(Value::Number(30.into())));

        let heap = Heap::from(data);

        assert_eq!(heap.len(), 2);
        assert_eq!(heap.get("name"), Some(&Value::String("John".to_string())));
        assert_eq!(heap.get("age"), Some(&Value::Number(30.into())));
    }

    #[test]
    fn test_heap_into_hashmap() {
        let mut heap = Heap::new();
        heap.insert("name", Some(Value::String("John".to_string())));
        heap.insert("age", Some(Value::Number(30.into())));

        let data: HashMap<String, Option<Value>> = heap.into();

        assert_eq!(data.len(), 2);
        assert_eq!(
            data.get("name"),
            Some(&Some(Value::String("John".to_string())))
        );
        assert_eq!(data.get("age"), Some(&Some(Value::Number(30.into()))));
    }

    #[test]
    fn test_heap_parse_simple_variables() {
        let mut heap = Heap::new();
        heap.insert("name", Some(Value::String("John".to_string())));
        heap.insert("age", Some(Value::Number(30.into())));

        let input = Value::String("Hello ${{name}}, you are ${{age}} years old".to_string());
        let result = heap.parse(Some(input));

        assert_eq!(
            result,
            Some(Value::String(
                "Hello John, you are 30 years old".to_string()
            ))
        );
    }

    #[test]
    fn test_heap_parse_no_variables() {
        let heap = Heap::new();
        let input = Value::String("Hello, World!".to_string());
        let result = heap.parse(Some(input.clone()));

        assert_eq!(result, Some(input));
    }

    #[test]
    fn test_heap_parse_unknown_variables() {
        let heap = Heap::new();
        let input = Value::String("Hello ${{unknown}}".to_string());
        let result = heap.parse(Some(input.clone()));

        // Unknown variables should be left as-is
        assert_eq!(result, Some(input));
    }

    #[test]
    fn test_heap_parse_non_string_values() {
        let heap = Heap::new();

        // Number value
        let number_input = Value::Number(42.into());
        let result = heap.parse(Some(number_input.clone()));
        assert_eq!(result, Some(number_input));

        // Boolean value
        let bool_input = Value::Bool(true);
        let result = heap.parse(Some(bool_input.clone()));
        assert_eq!(result, Some(bool_input));

        // Null value
        let null_input = Value::Null;
        let result = heap.parse(Some(null_input.clone()));
        assert_eq!(result, Some(null_input));
    }

    #[test]
    fn test_heap_parse_none_value() {
        let heap = Heap::new();
        let result = heap.parse(None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_heap_value_to_string() {
        let heap = Heap::new();

        // String
        assert_eq!(
            heap.value_to_string(&Value::String("test".to_string())),
            "test"
        );

        // Number
        assert_eq!(heap.value_to_string(&Value::Number(42.into())), "42");

        // Boolean
        assert_eq!(heap.value_to_string(&Value::Bool(true)), "true");
        assert_eq!(heap.value_to_string(&Value::Bool(false)), "false");

        // Null
        assert_eq!(heap.value_to_string(&Value::Null), "null");
    }

    #[test]
    fn test_heap_insert_none_value() {
        let mut heap = Heap::new();

        // Insert None value
        heap.insert("null_key", None);
        assert_eq!(heap.get("null_key"), None);
        assert!(heap.contains_key("null_key"));

        // Insert Some(None) value
        heap.insert("null_key2", Some(Value::Null));
        assert_eq!(heap.get("null_key2"), Some(&Value::Null));
    }
}
