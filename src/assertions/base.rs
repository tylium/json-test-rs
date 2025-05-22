use crate::JsonTest;
use jsonpath_rust::JsonPath;
use serde_json::{Map, Value};
use std::str::FromStr;

/// Provides assertions for JSON values accessed via JSONPath expressions.
///
/// This struct is created by `JsonTest::assert_path()` and enables a fluent API
/// for testing JSON values. All assertion methods follow a builder pattern,
/// returning `&mut Self` for chaining.
///
/// # Examples
///
/// ```rust
/// use json_test::{JsonTest, PropertyAssertions};
/// use serde_json::json;
///
/// let data = json!({
///     "user": {
///         "name": "John",
///         "age": 30
///     }
/// });
///
/// let mut test = JsonTest::new(&data);
/// test.assert_path("$.user")
///     .exists()
///     .has_property("name")
///     .has_property_value("age", json!(30));
/// ```
#[derive(Debug)]
pub struct JsonPathAssertion<'a> {
    pub(crate) path_str: String,
    pub(crate) current_values: Vec<Value>,
    pub(crate) test: Option<&'a mut JsonTest<'a>>,
}

impl<'a> JsonPathAssertion<'a> {
    pub(crate) fn new_with_test(test: &'a mut JsonTest<'a>, json: &'a Value, path: &str) -> Self {
        let parsed_path = JsonPath::<Value>::from_str(path)
            .unwrap_or_else(|e| panic!("Invalid JSONPath expression: {}", e));

        let result = parsed_path.find(json);
        let current_values = match result {
            Value::Array(values) => {
                if !path.contains('[') && values.len() == 1 {
                    vec![values[0].clone()]
                } else {
                    values
                }
            }
            Value::Null => vec![],
            other => vec![other],
        };

        Self {
            path_str: path.to_string(),
            current_values,
            test: Some(test),
        }
    }

    #[cfg(test)]
    pub fn new_for_test(json: &'a Value, path: &str) -> Self {
        let parsed_path = JsonPath::<Value>::from_str(path)
            .unwrap_or_else(|e| panic!("Invalid JSONPath expression: {}", e));

        let result = parsed_path.find(json);
        let current_values = match result {
            Value::Array(values) => {
                if !path.contains('[') && values.len() == 1 {
                    vec![values[0].clone()]
                } else {
                    values
                }
            }
            Value::Null => vec![],
            other => vec![other],
        };

        Self {
            path_str: path.to_string(),
            current_values,
            test: None,
        }
    }

    /// Asserts that the path exists and has at least one value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user.name")
    ///     .exists();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the path does not exist in the JSON structure.
    pub fn exists(&'a mut self) -> &'a mut Self {
        if self.current_values.is_empty() {
            panic!("Path {} does not exist", self.path_str);
        }
        self
    }

    /// Asserts that the path does not exist or has no values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user.email")
    ///     .does_not_exist();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the path exists in the JSON structure.
    pub fn does_not_exist(&'a mut self) -> &'a mut Self {
        if !self.current_values.is_empty() {
            panic!("Path {} exists but should not. Found values: {:?}",
                   self.path_str, self.current_values);
        }
        self
    }

    /// Asserts that the value at the current path equals the expected value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user.name")
    ///     .equals(json!("John"));
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value doesn't match the expected value
    pub fn equals(&'a mut self, expected: Value) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(actual) if actual == &expected => self,
            Some(actual) => panic!(
                "Value mismatch at {}\nExpected: {}\nActual: {}",
                self.path_str, expected, actual
            ),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the value at the current path is a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"message": "Hello"});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.message")
    ///     .is_string();
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a string
    pub fn is_string(&'a mut self) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::String(_)) => self,
            Some(v) => panic!("Expected string at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the string value contains the given substring.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"email": "test@example.com"});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.email")
    ///     .contains_string("@example");
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a string
    /// - Panics if the string does not contain the substring
    pub fn contains_string(&'a mut self, substring: &str) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::String(s)) if s.contains(substring) => self,
            Some(Value::String(s)) => panic!(
                "String at {} does not contain '{}'\nActual: {}",
                self.path_str, substring, s
            ),
            Some(v) => panic!("Expected string at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the string value starts with the given prefix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"id": "user_123"});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.id")
    ///     .starts_with("user_");
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a string
    /// - Panics if the string does not start with the prefix
    pub fn starts_with(&'a mut self, prefix: &str) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::String(s)) if s.starts_with(prefix) => self,
            Some(Value::String(s)) => panic!(
                "String at {} does not start with '{}'\nActual: {}",
                self.path_str, prefix, s
            ),
            Some(v) => panic!("Expected string at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the string value ends with the given suffix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"file": "document.pdf"});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.file")
    ///     .ends_with(".pdf");
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a string
    /// - Panics if the string does not end with the suffix
    pub fn ends_with(&'a mut self, suffix: &str) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::String(s)) if s.ends_with(suffix) => self,
            Some(Value::String(s)) => panic!(
                "String at {} does not end with '{}'\nActual: {}",
                self.path_str, suffix, s
            ),
            Some(v) => panic!("Expected string at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the string value matches the given regular expression pattern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"email": "test@example.com"});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.email")
    ///     .matches_pattern(r"^[^@]+@[^@]+\.[^@]+$");
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a string
    /// - Panics if the pattern is invalid
    /// - Panics if the string does not match the pattern

    pub fn matches_pattern(&'a mut self, pattern: &str) -> &'a mut Self {
        let regex = regex::Regex::new(pattern)
            .unwrap_or_else(|e| panic!("Invalid regex pattern: {}", e));

        match self.current_values.get(0) {
            Some(Value::String(s)) if regex.is_match(s) => self,
            Some(Value::String(s)) => panic!(
                "String at {} does not match pattern '{}'\nActual: {}",
                self.path_str, pattern, s
            ),
            Some(v) => panic!("Expected string at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the value at the current path is a number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"count": 42});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.count")
    ///     .is_number();
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    pub fn is_number(&'a mut self) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(_)) => self,
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the numeric value is greater than the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"age": 21});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.age")
    ///     .is_greater_than(18);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    /// - Panics if the value is not greater than the given value
    pub fn is_greater_than(&'a mut self, value: i64) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(n)) if n.as_i64().map_or(false, |x| x > value) => self,
            Some(Value::Number(n)) => panic!(
                "Number at {} is not greater than {}\nActual: {}",
                self.path_str, value, n
            ),
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the numeric value is greater than the given float value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"age": 21.4});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.age")
    ///     .is_greater_than_f64(18.5);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    /// - Panics if the value is not greater than the given value
    pub fn is_greater_than_f64(&'a mut self, value: f64) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(n)) if n.as_f64().map_or(false, |x| x > value) => self,
            Some(Value::Number(n)) => panic!(
                "Number at {} is not greater than {}\nActual: {}",
                self.path_str, value, n
            ),
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the numeric value is less than the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"temperature": 36});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.temperature")
    ///     .is_less_than(40);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    /// - Panics if the value is not less than the given value
    pub fn is_less_than(&'a mut self, value: i64) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(n)) if n.as_i64().map_or(false, |x| x < value) => self,
            Some(Value::Number(n)) => panic!(
                "Number at {} is not less than {}\nActual: {}",
                self.path_str, value, n
            ),
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the numeric value is less than the given float value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"temperature": 36.8});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.temperature")
    ///     .is_less_than_f64(40.0);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    /// - Panics if the value is not less than the given value
    pub fn is_less_than_f64(&'a mut self, value: f64) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(n)) if n.as_f64().map_or(false, |x| x < value) => self,
            Some(Value::Number(n)) => panic!(
                "Number at {} is not less than {}\nActual: {}",
                self.path_str, value, n
            ),
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the numeric value is between the given minimum and maximum values (inclusive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"score": 85});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.score")
    ///     .is_between(0, 100);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    /// - Panics if the value is not between min and max (inclusive)
    pub fn is_between(&'a mut self, min: i64, max: i64) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(n)) if n.as_i64().map_or(false, |x| x >= min && x <= max) => self,
            Some(Value::Number(n)) => panic!(
                "Number at {} is not between {} and {}\nActual: {}",
                self.path_str, min, max, n
            ),
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

     /// Asserts that the numeric value is between the given minimum and maximum float values (inclusive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"score": 85.6});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.score")
    ///     .is_between_f64(0.0, 100.0);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not a number
    /// - Panics if the value is not between min and max (inclusive)
    pub fn is_between_f64(&'a mut self, min: f64, max: f64) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Number(n)) if n.as_f64().map_or(false, |x| x >= min && x <= max) => self,
            Some(Value::Number(n)) => panic!(
                "Number at {} is not between {} and {}\nActual: {}",
                self.path_str, min, max, n
            ),
            Some(v) => panic!("Expected number at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the value at the current path is an array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"tags": ["rust", "testing"]});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.tags")
    ///     .is_array();
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not an array
    pub fn is_array(&'a mut self) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Array(_)) => self,
            Some(v) => panic!("Expected array at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the array has the expected length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"tags": ["rust", "testing"]});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.tags")
    ///     .is_array()
    ///     .has_length(2);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not an array
    /// - Panics if the array length doesn't match the expected length
    pub fn has_length(&'a mut self, expected: usize) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Array(arr)) if arr.len() == expected => self,
            Some(Value::Array(arr)) => panic!(
                "Array at {} has wrong length\nExpected: {}\nActual: {}",
                self.path_str, expected, arr.len()
            ),
            Some(v) => panic!("Expected array at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the array contains the expected value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"roles": ["user", "admin"]});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.roles")
    ///     .is_array()
    ///     .contains(&json!("admin"));
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not an array
    /// - Panics if the array does not contain the expected value
    pub fn contains(&'a mut self, expected: &Value) -> &'a mut Self {
        match self.current_values.get(0) {
            Some(Value::Array(arr)) if arr.contains(expected) => self,
            Some(Value::Array(arr)) => panic!(
                "Array at {} does not contain expected value\nExpected: {}\nArray: {:?}",
                self.path_str, expected, arr
            ),
            Some(v) => panic!("Expected array at {}, got {:?}", self.path_str, v),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the value matches a custom predicate.
    ///
    /// This method allows for complex value validation using custom logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"timestamp": "2024-01-01T12:00:00Z"});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.timestamp")
    ///     .matches(|value| {
    ///         value.as_str()
    ///             .map(|s| s.contains("T") && s.ends_with("Z"))
    ///             .unwrap_or(false)
    ///     });
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value doesn't satisfy the predicate
    pub fn matches<F>(&'a mut self, predicate: F) -> &'a mut Self
    where
        F: FnOnce(&Value) -> bool,
    {
        match self.current_values.get(0) {
            Some(value) if predicate(value) => self,
            Some(value) => panic!(
                "Value at {} does not match predicate\nActual value: {}",
                self.path_str, value
            ),
            None => panic!("No value found at {}", self.path_str),
        }
    }

    /// Asserts that the value is an object and returns it for further testing.
    ///
    /// This method is primarily used internally by property assertions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John", "age": 30}});
    /// # let mut test = JsonTest::new(&data);
    /// let obj = test.assert_path("$.user")
    ///     .assert_object();
    /// assert!(obj.contains_key("name"));
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if no value exists at the path
    /// - Panics if the value is not an object
    pub fn assert_object(&self) -> Map<String, Value> {
        match &self.current_values[..] {
            [Value::Object(obj)] => obj.clone(),
            _ => panic!(
                "Expected object at {}, got: {:?}",
                self.path_str, self.current_values
            ),
        }
    }

    /// Creates a new assertion for a different path while maintaining the test context.
    ///
    /// This method enables chaining assertions across different paths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({
    /// #     "user": {"name": "John"},
    /// #     "settings": {"theme": "dark"}
    /// # });
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_property("name")
    ///     .assert_path("$.settings")
    ///     .has_property("theme");
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if called on an assertion without test context
    pub fn assert_path(&'a mut self, path: &str) -> JsonPathAssertion<'a> {
        match &mut self.test {
            Some(test) => test.assert_path(path),
            None => panic!("Cannot chain assertions without JsonTest context"),
        }
    }
}