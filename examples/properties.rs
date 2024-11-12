//! Example showing property matching capabilities.

use json_test::{JsonTest, PropertyAssertions};
use serde_json::json;

fn main() {
    let data = json!({
        "config": {
            "db_settings": {
                "host": "localhost",
                "port": 5432,
                "max_connections": 100
            },
            "api_keys": {
                "key_prod": "pk_live_123",
                "key_test": "pk_test_456",
                "key_dev": "pk_dev_789"
            },
            "feature_flags": {
                "debug_mode": false,
                "maintenance_mode": false,
                "beta_features": true
            },
            "limits": {
                "max_requests": 1000,
                "rate_limit": 60,
                "timeout_ms": 5000
            }
        }
    });

    let mut test = JsonTest::new(&data);

    test.assert_path("$.config.db_settings")
        // Verify all required database properties exist
        .has_properties(vec!["host", "port", "max_connections"])

        // Check database configuration values
        .has_property_value("port", json!(5432))
        .has_property_value("host", json!("localhost"))

        // Test API keys section
        .assert_path("$.config.api_keys")
        // Find all production keys
        .properties_matching(|key| key.starts_with("key_prod"))
        .count(1)
        .and()
        // Verify all keys follow pattern
        .properties_matching(|key| key.starts_with("key_"))
        .count(3)
        .all(|(_, value)| {
            value.as_str()
                .map(|s| s.starts_with("pk_"))
                .unwrap_or(false)
        })
        .and()

        // Check feature flags
        .assert_path("$.config.feature_flags")
        // Count disabled features
        .properties_matching(|_| true)
        .count(3)
        .all(|(_, value)| value.is_boolean())
        .and()
        // Verify specific flags
        .has_property_value("debug_mode", json!(false))
        .has_property_value("beta_features", json!(true))

        // Validate limits
        .assert_path("$.config.limits")
        // All limits should be positive numbers
        .properties_matching(|_| true)
        .all(|(_, value)| {
            value.as_u64()
                .map(|n| n > 0)
                .unwrap_or(false)
        });

    println!("All property assertions passed!");
}