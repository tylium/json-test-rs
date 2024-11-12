use super::JsonMatcher;
use serde_json::Value;

#[derive(Debug)]
pub struct TypeMatcher {
    expected_type: &'static str,
}

impl TypeMatcher {
    pub fn new(expected_type: &'static str) -> Self {
        Self { expected_type }
    }

    // Convenience constructors
    pub fn string() -> Self {
        Self::new("string")
    }

    pub fn number() -> Self {
        Self::new("number")
    }

    pub fn boolean() -> Self {
        Self::new("boolean")
    }

    pub fn array() -> Self {
        Self::new("array")
    }

    pub fn object() -> Self {
        Self::new("object")
    }

    pub fn null() -> Self {
        Self::new("null")
    }
}

impl JsonMatcher for TypeMatcher {
    fn matches(&self, value: &Value) -> bool {
        match (self.expected_type, value) {
            ("string", Value::String(_)) => true,
            ("number", Value::Number(_)) => true,
            ("boolean", Value::Bool(_)) => true,
            ("null", Value::Null) => true,
            ("array", Value::Array(_)) => true,
            ("object", Value::Object(_)) => true,
            _ => false,
        }
    }

    fn description(&self) -> String {
        format!("is of type {}", self.expected_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_type_matching() {
        // Test string type
        assert!(TypeMatcher::string().matches(&json!("test")));
        assert!(!TypeMatcher::string().matches(&json!(42)));

        // Test number type
        assert!(TypeMatcher::number().matches(&json!(42)));
        assert!(TypeMatcher::number().matches(&json!(42.5)));
        assert!(!TypeMatcher::number().matches(&json!("42")));

        // Test boolean type
        assert!(TypeMatcher::boolean().matches(&json!(true)));
        assert!(!TypeMatcher::boolean().matches(&json!(1)));

        // Test array type
        assert!(TypeMatcher::array().matches(&json!([1, 2, 3])));
        assert!(!TypeMatcher::array().matches(&json!({"key": "value"})));

        // Test object type
        assert!(TypeMatcher::object().matches(&json!({"key": "value"})));
        assert!(!TypeMatcher::object().matches(&json!([1, 2, 3])));

        // Test null type
        assert!(TypeMatcher::null().matches(&json!(null)));
        assert!(!TypeMatcher::null().matches(&json!(42)));
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(TypeMatcher::string().description(), "is of type string");
        assert_eq!(TypeMatcher::number().description(), "is of type number");
    }
}