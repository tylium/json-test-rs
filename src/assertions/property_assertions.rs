use serde_json::Value;
use crate::assertions::property_matcher::PropertyMatcher;

/// Trait providing property testing capabilities for JSON objects.
pub trait PropertyAssertions<'a> {
    /// Asserts that the object has the specified property.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_property("name");
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the value is not an object
    /// - Panics if the property doesn't exist
    fn has_property(&'a mut self, name: &str) -> &'a mut Self;

    /// Asserts that the object has all the specified properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John", "age": 30}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_properties(["name", "age"]);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the value is not an object
    /// - Panics if any of the properties don't exist
    fn has_properties<I, S>(&'a mut self, names: I) -> &'a mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;

    /// Asserts that the object has exactly the expected number of properties.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John", "age": 30}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_property_count(2);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the value is not an object
    /// - Panics if the number of properties doesn't match the expected countfn has_property_count(&'a mut self, expected: usize) -> &'a mut Self;
    fn has_property_count(&'a mut self, expected: usize) -> &'a mut Self;

    /// Asserts that the object has the expected number of properties matching a predicate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"meta_created": "2024-01-01", "meta_updated": "2024-01-02", "name": "John"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_property_count_matching(|key| key.starts_with("meta_"), 2);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the value is not an object
    /// - Panics if the number of matching properties doesn't equal the expected count

    fn has_property_count_matching<F>(&'a mut self, predicate: F, expected: usize) -> &'a mut Self
    where
        F: Fn(&str) -> bool;

    /// Asserts that a property has the expected value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"name": "John", "age": 30}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_property_value("name", json!("John"));
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the value is not an object
    /// - Panics if the property doesn't exist
    /// - Panics if the property value doesn't match the expected value
    fn has_property_value(&'a mut self, name: &str, expected: Value) -> &'a mut Self;

    /// Asserts that a property's value satisfies a predicate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"age": 30}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .has_property_matching("age", |v| v.as_u64().unwrap_or(0) > 18);
    /// ```
    ///
    /// # Panics
    ///
    /// - Panics if the value is not an object
    /// - Panics if the property doesn't exist
    /// - Panics if the property value doesn't satisfy the predicate

    fn has_property_matching<F>(&'a mut self, name: &str, predicate: F) -> &'a mut Self
    where
        F: Fn(&Value) -> bool;

    /// Creates a PropertyMatcher for testing properties that match a predicate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"meta_created": "2024-01-01", "meta_updated": "2024-01-02"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .properties_matching(|key| key.starts_with("meta_"))
    ///     .count(2)
    ///     .and()
    ///     .has_property_count(2);
    /// ```
    fn properties_matching<F>(&'a mut self, predicate: F) -> PropertyMatcher<'a>
    where
        F: Fn(&str) -> bool;
}

impl<'a> PropertyAssertions<'a> for super::base::JsonPathAssertion<'a> {
    fn has_property(&'a mut self, name: &str) -> &'a mut Self {
        let obj = self.assert_object();

        if !obj.contains_key(name) {
            let available = obj.keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            panic!("Property '{}' not found at {}\nAvailable properties: {}",
                   name, self.path_str, available);
        }
        self
    }

    fn has_properties<I, S>(&'_ mut self, names: I) -> &'_ mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let obj = self.assert_object();
        let missing: Vec<String> = names.into_iter()
            .filter(|name| !obj.contains_key(name.as_ref()))
            .map(|name| name.as_ref().to_string())
            .collect();

        if !missing.is_empty() {
            let available = obj.keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            panic!("Missing properties at {}: {}\nAvailable properties: {}",
                   self.path_str, missing.join(", "), available);
        }
        self
    }

    fn has_property_count(&'_ mut self, expected: usize) -> &'_ mut Self {
        let obj = self.assert_object();
        let actual = obj.len();

        if actual != expected {
            let properties = obj.keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            panic!(
                "Incorrect number of properties at {}\nExpected: {}\nActual: {}\nProperties: {}",
                self.path_str, expected, actual, properties
            );
        }
        self
    }

    fn has_property_count_matching<F>(&'_ mut self, predicate: F, expected: usize) -> &'_ mut Self
    where
        F: Fn(&str) -> bool,
    {
        let obj = self.assert_object();
        let matching: Vec<&str> = obj.keys()
            .filter(|k| predicate(k))
            .map(|s| s.as_str())
            .collect();

        if matching.len() != expected {
            panic!(
                "Incorrect number of matching properties at {}\nExpected: {}\nActual: {}\nMatching properties: {}",
                self.path_str, expected, matching.len(), matching.join(", ")
            );
        }
        self
    }

    fn has_property_value(&'_ mut self, name: &str, expected: Value) -> &'_ mut Self {
        let obj = self.assert_object();

        match obj.get(name) {
            Some(actual) if actual == &expected => self,
            Some(actual) => {
                panic!(
                    "Property '{}' value mismatch at {}\nExpected: {}\nActual: {}",
                    name, self.path_str, expected, actual
                );
            },
            None => {
                let available = obj.keys()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");

                panic!(
                    "Property '{}' not found at {}\nAvailable properties: {}",
                    name, self.path_str, available
                );
            }
        }
    }

    fn has_property_matching<F>(&'_ mut self, name: &str, predicate: F) -> &'_ mut Self
    where
        F: Fn(&Value) -> bool,
    {
        let obj = self.assert_object();

        match obj.get(name) {
            Some(value) if predicate(value) => self,
            Some(value) => {
                panic!(
                    "Property '{}' at {} does not match condition\nValue: {}",
                    name, self.path_str, value
                );
            },
            None => {
                let available = obj.keys()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");

                panic!(
                    "Property '{}' not found at {}\nAvailable properties: {}",
                    name, self.path_str, available
                );
            }
        }
    }

    fn properties_matching<F>(&'a mut self, predicate: F) -> PropertyMatcher<'a>
    where
        F: Fn(&str) -> bool,
    {
        let obj = self.assert_object();
        let pairs: Vec<(String, Value)> = obj.iter()
            .filter(|(k, _)| predicate(k))
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();

        PropertyMatcher::new(pairs, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::base::JsonPathAssertion;
    use serde_json::json;

    #[test]
    fn test_property_assertions() {
        let json = json!({
            "user": {
                "name": "John",
                "age": 30,
                "metadata": {
                    "created_at": "2024-01-01",
                    "updated_at": "2024-01-02"
                }
            }
        });

        let mut assertion = JsonPathAssertion::new_for_test(&json, "$.user");

        assertion
            .has_property("name")
            .has_properties(vec!["name", "age"])
            .has_property_count(3)
            .has_property_value("name", json!("John"))
            .has_property_matching("age", |v| v.as_u64().unwrap_or(0) > 20)
            .has_property_count_matching(|k| k.ends_with("at"), 0);
    }

    #[test]
    #[should_panic(expected = "Property 'email' not found")]
    fn test_missing_property() {
        let json = json!({"user": {"name": "John"}});
        let mut assertion = JsonPathAssertion::new_for_test(&json, "$.user");
        assertion.has_property("email");
    }

    #[test]
    #[should_panic(expected = "Incorrect number of properties")]
    fn test_property_count() {
        let json = json!({"user": {"name": "John", "age": 30}});
        let mut assertion = JsonPathAssertion::new_for_test(&json, "$.user");
        assertion.has_property_count(1);
    }

    #[test]
    #[should_panic(expected = "Property 'age' value mismatch")]
    fn test_property_value_mismatch() {
        let json = json!({"user": {"name": "John", "age": 30}});
        let mut assertion = JsonPathAssertion::new_for_test(&json, "$.user");
        assertion.has_property_value("age", json!(25));
    }
}