use std::collections::HashMap;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonPathError {
    #[error("{message}\nPath: {path}\nActual Value: {actual}\n{}", context_string(.context, .expected))]
    AssertionFailed {
        message: String,
        path: String,
        actual: Value,
        expected: Option<Value>,
        context: HashMap<String, String>,
    },

    #[error("Invalid JSONPath expression: {0}")]
    InvalidPath(String),
}

/// Helper function for formatting context in error messages
fn context_string(context: &HashMap<String, String>, expected: &Option<Value>) -> String {
    let mut parts = Vec::new();

    // Add expected value if present
    if let Some(exp) = expected {
        parts.push(format!("Expected Value: {}", exp));
    }

    // Add all context key-value pairs
    for (key, value) in context {
        parts.push(format!("{}: {}", key, value));
    }

    parts.join("\n")
}

impl JsonPathError {
    pub fn assertion_failed(
        message: impl Into<String>,
        path: impl Into<String>,
        actual: Value,
        expected: Option<Value>,
        context: HashMap<String, String>,
    ) -> Self {
        JsonPathError::AssertionFailed {
            message: message.into(),
            path: path.into(),
            actual,
            expected,
            context,
        }
    }

    pub fn type_mismatch(path: String, actual: Value, expected_type: &str) -> Self {
        let mut context = HashMap::new();
        context.insert("Expected Type".to_string(), expected_type.to_string());
        context.insert("Actual Type".to_string(), type_name(&actual));

        JsonPathError::AssertionFailed {
            message: format!("Expected value of type {}", expected_type),
            path,
            actual,
            expected: None,
            context,
        }
    }

    pub fn value_mismatch(path: String, actual: Value, expected: Value) -> Self {
        let mut context = HashMap::new();
        context.insert("Operation".to_string(), "Equality".to_string());

        JsonPathError::AssertionFailed {
            message: "Value mismatch".to_string(),
            path,
            actual,
            expected: Some(expected),
            context,
        }
    }

    pub fn comparison_failed(
        path: String,
        actual: Value,
        operation: &str,
        comparison_value: Value,
    ) -> Self {
        let mut context = HashMap::new();
        context.insert("Operation".to_string(), operation.to_string());
        context.insert("Comparison Value".to_string(), comparison_value.to_string());

        JsonPathError::AssertionFailed {
            message: format!("Value comparison failed for operation: {}", operation),
            path,
            actual,
            expected: None,
            context,
        }
    }

    pub fn property_error(
        path: String,
        actual: Value,
        message: String,
        context: HashMap<String, String>,
    ) -> Self {
        JsonPathError::AssertionFailed {
            message,
            path,
            actual,
            expected: None,
            context,
        }
    }
}

/// Helper function to get readable type names
fn type_name(value: &Value) -> String {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }.to_string()
}

/// Extension trait for adding context to errors
pub trait ErrorContext<T> {
    fn with_context<K, V>(self, key: K, value: V) -> Result<T, JsonPathError>
    where
        K: Into<String>,
        V: Into<String>;

    fn with_contexts<I, K, V>(self, contexts: I) -> Result<T, JsonPathError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>;
}

impl<T> ErrorContext<T> for Result<T, JsonPathError> {
    fn with_context<K, V>(self, key: K, value: V) -> Result<T, JsonPathError>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.map_err(|err| {
            if let JsonPathError::AssertionFailed {
                message,
                path,
                actual,
                expected,
                mut context,
            } = err
            {
                context.insert(key.into(), value.into());
                JsonPathError::AssertionFailed {
                    message,
                    path,
                    actual,
                    expected,
                    context,
                }
            } else {
                err
            }
        })
    }

    fn with_contexts<I, K, V>(self, contexts: I) -> Result<T, JsonPathError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.map_err(|err| {
            if let JsonPathError::AssertionFailed {
                message,
                path,
                actual,
                expected,
                mut context,
            } = err
            {
                for (k, v) in contexts {
                    context.insert(k.into(), v.into());
                }
                JsonPathError::AssertionFailed {
                    message,
                    path,
                    actual,
                    expected,
                    context,
                }
            } else {
                err
            }
        })
    }
}