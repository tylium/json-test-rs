//! A testing library for JSON Path assertions in Rust.
//!
//! `json-test` provides a fluent API for testing JSON structures using JSONPath expressions.
//! It's designed to make writing tests for JSON data structures clear, concise, and maintainable.
//!
//! # Core Concepts
//!
//! - **JsonTest**: The main entry point, providing methods to start assertions
//! - **JsonPathAssertion**: Chainable assertions on JSON values
//! - **PropertyAssertions**: Object property validation
//! - **Matchers**: Flexible value matching and validation
//!
//! # Features
//!
//! - JSONPath-based value extraction and validation
//! - Chainable, fluent assertion API
//! - Type-safe operations
//! - Property existence and value validation
//! - String pattern matching with regex support
//! - Numeric comparisons
//! - Array and object validation
//! - Custom matcher support
//!
//! # Examples
//!
//! ## Value Assertions
//!
//! ```rust
//! use json_test::JsonTest;
//! use serde_json::json;
//!
//! let data = json!({
//!     "user": {
//!         "name": "John Doe",
//!         "age": 30
//!     }
//! });
//!
//! let mut test = JsonTest::new(&data);
//!
//! // Chain multiple assertions on a single value
//! test.assert_path("$.user.name")
//!     .exists()
//!     .is_string()
//!     .equals(json!("John Doe"));
//! ```
//!
//! ## Numeric Validation
//!
//! ```rust
//! # use json_test::JsonTest;
//! # use serde_json::json;
//! # let data = json!({"score": 85, "time": 1.4});
//! # let mut test = JsonTest::new(&data);
//! test.assert_path("$.score")
//!     .is_number()
//!     .is_greater_than(80)
//!     .is_greater_than_f64(80.2)
//!     .is_less_than(90)
//!     .is_less_than_f64(90.1)
//!     .is_between(0, 100)
//!     .assert_path("$.time")
//!     .is_between_f64(0.0, 100.0);
//! ```
//!
//! ## Array Testing
//!
//! ```rust
//! # use json_test::JsonTest;
//! # use serde_json::json;
//! # let data = json!({"roles": ["user", "admin"]});
//! # let mut test = JsonTest::new(&data);
//! test.assert_path("$.roles")
//!     .is_array()
//!     .has_length(2)
//!     .contains(&json!("admin"));
//! ```
//!
//! ## Property Chaining
//!
//! ```rust
//! # use json_test::{JsonTest, PropertyAssertions};
//! # use serde_json::json;
//! let data = json!({
//!     "user": {
//!         "name": "John",
//!         "settings": {
//!             "theme": "dark",
//!             "notifications": true
//!         }
//!     }
//! });
//!
//! let mut test = JsonTest::new(&data);
//!
//! // Chain property assertions
//! test.assert_path("$.user")
//!     .has_property("name")
//!     .has_property("settings")
//!     .properties_matching(|key| !key.starts_with("_"))
//!         .count(2)
//!         .and()
//!     .has_property_value("name", json!("John"));
//! ```
//!
//! ## Advanced Matching
//!
//! ```rust
//! # use json_test::JsonTest;
//! # use serde_json::json;
//! # let data = json!({"user": {"email": "test@example.com"}});
//! # let mut test = JsonTest::new(&data);
//! test.assert_path("$.user.email")
//!     .is_string()
//!     .contains_string("@")
//!     .matches_pattern(r"^[^@]+@[^@]+\.[^@]+$")
//!     .matches(|value| {
//!         value.as_str()
//!             .map(|s| !s.starts_with("admin@"))
//!             .unwrap_or(false)
//!     });
//! ```
//!
//! # Error Messages
//!
//! The library provides clear, test-friendly error messages:
//!
//! ```text
//! Property 'email' not found at $.user
//! Available properties: name, age, roles
//! ```
//!
//! ```text
//! Value mismatch at $.user.age
//! Expected: 25
//! Actual: 30
//! ```
//!
//! # Current Status
//!
//! This library is in active development (0.1.x). While the core API is stabilizing,
//! minor breaking changes might occur before 1.0.

mod assertions;
mod error;
mod matchers;

pub use assertions::base::JsonPathAssertion;
pub use assertions::property_assertions::PropertyAssertions;
pub use error::{ErrorContext, JsonPathError};
pub use matchers::{JsonMatcher, RegexMatcher, TypeMatcher, ValueMatcher};
use serde_json::Value;

/// Main entry point for JSON testing.
///
/// `JsonTest` provides methods to create assertions on JSON values using JSONPath expressions.
/// It maintains a reference to the JSON being tested and enables creation of chainable assertions.
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
///         "settings": {
///             "theme": "dark"
///         }
///     }
/// });
///
/// let mut test = JsonTest::new(&data);
///
/// // Test a single path with chained assertions
/// test.assert_path("$.user")
///     .has_property("name")
///     .has_property("settings")
///     .has_property_value("name", json!("John"));
/// ```
#[derive(Debug)]
pub struct JsonTest<'a> {
    json: &'a Value,
}

impl<'a> JsonTest<'a> {
    /// Creates a new JSON test instance.
    ///
    /// Takes a reference to a JSON value that will be tested. The JSON value
    /// must live at least as long as the test instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::JsonTest;
    /// # use serde_json::json;
    /// let data = json!({"key": "value"});
    /// let test = JsonTest::new(&data);
    /// ```
    pub fn new(json: &'a Value) -> Self {
        Self { json }
    }

    /// Creates a new assertion for the given JSONPath expression.
    ///
    /// The path must be a valid JSONPath expression. Invalid paths will cause
    /// a panic with a descriptive error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// let data = json!({
    ///     "users": [
    ///         {"name": "John", "role": "admin"},
    ///         {"name": "Jane", "role": "user"}
    ///     ]
    /// });
    ///
    /// let mut test = JsonTest::new(&data);
    ///
    /// // Test array element with chained assertions
    /// test.assert_path("$.users[0]")
    ///     .has_property("name")
    ///     .has_property_value("role", json!("admin"));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the JSONPath expression is invalid. This is appropriate for
    /// testing scenarios where invalid paths indicate test specification errors.
    pub fn assert_path(&'a mut self, path: &str) -> JsonPathAssertion<'a> {
        JsonPathAssertion::new_with_test(self, self.json, path)
    }
}