use super::JsonMatcher;
use regex::Regex;
use serde_json::Value;

#[derive(Debug)]
pub struct RegexMatcher {
    pattern: Regex,
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: Regex::new(pattern)?
        })
    }
}

impl JsonMatcher for RegexMatcher {
    fn matches(&self, value: &Value) -> bool {
        match value {
            Value::String(s) => self.pattern.is_match(s),
            _ => false,
        }
    }

    fn description(&self) -> String {
        format!("matches regex pattern {}", self.pattern.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_regex_matching() {
        let matcher = RegexMatcher::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

        // Test valid date format
        assert!(matcher.matches(&json!("2024-01-01")));

        // Test invalid formats
        assert!(!matcher.matches(&json!("2024/01/01")));
        assert!(!matcher.matches(&json!("not-a-date")));

        // Test non-string values
        assert!(!matcher.matches(&json!(42)));
        assert!(!matcher.matches(&json!(true)));
        assert!(!matcher.matches(&json!(null)));
    }

    #[test]
    fn test_case_sensitive_matching() {
        let matcher = RegexMatcher::new(r"[a-z]+\d+").unwrap();

        assert!(matcher.matches(&json!("test123")));
        assert!(!matcher.matches(&json!("TEST123")));
    }

    #[test]
    fn test_invalid_regex() {
        assert!(RegexMatcher::new(r"[invalid").is_err());
    }

    #[test]
    fn test_description() {
        let matcher = RegexMatcher::new(r"\d+").unwrap();
        assert_eq!(matcher.description(), r#"matches regex pattern \d+"#);
    }
}