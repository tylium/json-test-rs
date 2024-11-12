use serde_json::Value;

/// Matches and collects properties based on custom predicates.
///
/// This struct provides advanced property matching capabilities, allowing
/// filtering and validation of object properties that match specific criteria.
///
/// # Examples
///
/// ```rust
/// # use json_test::{JsonTest, PropertyAssertions};
/// # use serde_json::json;
/// # let data = json!({
/// #     "user": {
/// #         "name": "John",
/// #         "meta_created": "2024-01-01",
/// #         "meta_updated": "2024-01-02"
/// #     }
/// # });
/// # let mut test = JsonTest::new(&data);
/// test.assert_path("$.user")
///     .properties_matching(|key| key.starts_with("meta_"))
///     .count(2)
///     .and()
///     .has_property("name");
/// ```
pub struct PropertyMatcher<'a> {
    pairs: Vec<(String, Value)>,
    assertion: &'a mut super::base::JsonPathAssertion<'a>,
}

impl<'a> PropertyMatcher<'a> {
    pub(crate) fn new(pairs: Vec<(String, Value)>, assertion: &'a mut super::base::JsonPathAssertion<'a>) -> Self {
        Self { pairs, assertion }
    }

    /// Asserts that the number of matching properties equals the expected count.
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
    ///     .count(2);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the number of matching properties doesn't equal the expected count.
    pub fn count(self, expected: usize) -> Self {
        assert_eq!(
            self.pairs.len(),
            expected,
            "Expected {} matching properties but found {} at {}",
            expected,
            self.pairs.len(),
            self.assertion.path_str
        );
        self
    }

    /// Asserts that all matching properties satisfy a predicate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"config": {"debug_mode": true, "debug_level": 3}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.config")
    ///     .properties_matching(|key| key.starts_with("debug_"))
    ///     .all(|(key, _)| key.len() > 5);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if any matching property fails to satisfy the predicate.
    pub fn all<F>(self, predicate: F) -> Self
    where
        F: Fn((&str, &Value)) -> bool
    {
        for (k, v) in &self.pairs {
            assert!(
                predicate((k, v)),
                "Property {:?} did not match predicate at {}",
                (k, v),
                self.assertion.path_str
            );
        }
        self
    }

    /// Collects matching property values into a vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"meta_created": "2024-01-01", "meta_updated": "2024-01-02"}});
    /// # let mut test = JsonTest::new(&data);
    /// let meta_values = test.assert_path("$.user")
    ///     .properties_matching(|key| key.starts_with("meta_"))
    ///     .collect_values();
    /// ```
    pub fn collect_values(self) -> Vec<Value> {
        self.pairs.into_iter().map(|(_, v)| v).collect()
    }

    /// Collects matching property keys into a vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"meta_created": "2024-01-01", "meta_updated": "2024-01-02"}});
    /// # let mut test = JsonTest::new(&data);
    /// let meta_keys = test.assert_path("$.user")
    ///     .properties_matching(|key| key.starts_with("meta_"))
    ///     .collect_keys();
    /// assert_eq!(meta_keys.len(), 2);
    /// ```
    pub fn collect_keys(self) -> Vec<String> {
        self.pairs.into_iter().map(|(k, _)| k).collect()
    }

    /// Collects matching property key-value pairs into a vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"meta_created": "2024-01-01", "meta_updated": "2024-01-02"}});
    /// # let mut test = JsonTest::new(&data);
    /// let meta_pairs = test.assert_path("$.user")
    ///     .properties_matching(|key| key.starts_with("meta_"))
    ///     .collect_pairs();
    /// assert_eq!(meta_pairs.len(), 2);
    /// ```
    pub fn collect_pairs(self) -> Vec<(String, Value)> {
        self.pairs
    }

    /// Returns to the parent assertion for further chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use json_test::{JsonTest, PropertyAssertions};
    /// # use serde_json::json;
    /// # let data = json!({"user": {"meta_created": "2024-01-01", "name": "John"}});
    /// # let mut test = JsonTest::new(&data);
    /// test.assert_path("$.user")
    ///     .properties_matching(|key| key.starts_with("meta_"))
    ///     .count(1)
    ///     .and()
    ///     .has_property("name");
    /// ```
    pub fn and(self) -> &'a mut super::base::JsonPathAssertion<'a> {
        self.assertion
    }
}