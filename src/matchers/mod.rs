mod regex;
mod type_matcher;
mod value;

pub use regex::RegexMatcher;
pub use type_matcher::TypeMatcher;
pub use value::ValueMatcher;

use serde_json::Value;
use std::fmt::Debug;

/// Core trait for implementing JSON value matchers.
///
/// This trait allows creation of custom matchers for flexible value validation.
/// Implementors must provide both matching logic and a description of the matcher.
///
/// # Examples
///
/// ```rust
/// use json_test::JsonMatcher;
/// use serde_json::Value;
///
/// #[derive(Debug)]
/// struct DateMatcher;
///
/// impl JsonMatcher for DateMatcher {
///     fn matches(&self, value: &Value) -> bool {
///         value.as_str()
///             .map(|s| s.contains("-"))
///             .unwrap_or(false)
///     }
///
///     fn description(&self) -> String {
///         "is a date string".to_string()
///     }
/// }
/// ```
pub trait JsonMatcher: Debug {
    /// Tests if a JSON value matches this matcher's criteria.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to test
    ///
    /// # Returns
    ///
    /// Returns `true` if the value matches the criteria, `false` otherwise.
    fn matches(&self, value: &Value) -> bool;

    /// Returns a human-readable description of the matcher's criteria.
    ///
    /// This description is used in error messages when assertions fail.
    fn description(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Simple test matcher implementation
    #[derive(Debug)]
    struct TestMatcher(bool);

    impl JsonMatcher for TestMatcher {
        fn matches(&self, _: &Value) -> bool {
            self.0
        }

        fn description(&self) -> String {
            format!("always returns {}", self.0)
        }
    }

    #[test]
    fn test_matcher_trait() {
        let value = json!(42);

        let true_matcher = TestMatcher(true);
        assert!(true_matcher.matches(&value));
        assert_eq!(true_matcher.description(), "always returns true");

        let false_matcher = TestMatcher(false);
        assert!(!false_matcher.matches(&value));
    }
}