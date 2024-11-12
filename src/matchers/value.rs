use super::JsonMatcher;
use serde_json::Value;

#[derive(Debug)]
pub struct ValueMatcher {
    expected: Value,
}

impl ValueMatcher {
    pub fn new(expected: Value) -> Self {
        Self { expected }
    }

    pub fn eq(expected: Value) -> Self {
        Self::new(expected)
    }
}

impl JsonMatcher for ValueMatcher {
    fn matches(&self, value: &Value) -> bool {
        &self.expected == value
    }

    fn description(&self) -> String {
        format!("equals {}", self.expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_matching() {
        let value = json!(42);
        assert!(ValueMatcher::eq(json!(42)).matches(&value));
        assert!(!ValueMatcher::eq(json!(43)).matches(&value));

        let obj = json!({"name": "test", "value": 42});
        assert!(ValueMatcher::eq(json!({"name": "test", "value": 42})).matches(&obj));
        assert!(!ValueMatcher::eq(json!({"name": "other"})).matches(&obj));
    }

    #[test]
    fn test_array_matching() {
        let arr = json!([1, 2, 3]);
        assert!(ValueMatcher::eq(json!([1, 2, 3])).matches(&arr));
        assert!(!ValueMatcher::eq(json!([3, 2, 1])).matches(&arr));
    }

    #[test]
    fn test_null_matching() {
        let null = json!(null);
        assert!(ValueMatcher::eq(json!(null)).matches(&null));
        assert!(!ValueMatcher::eq(json!(42)).matches(&null));
    }
}