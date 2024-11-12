//! Basic examples demonstrating the fluent API and JSONPath capabilities.

use json_test::JsonTest;
use serde_json::json;

fn main() {
    let data = json!({
        "users": [
            {
                "id": 1,
                "name": "John Doe",
                "email": "john@example.com",
                "roles": ["admin", "user"],
                "active": true
            },
            {
                "id": 2,
                "name": "Jane Smith",
                "email": "jane@example.com",
                "roles": ["user"],
                "active": true
            }
        ],
        "metadata": {
            "total_users": 2,
            "last_updated": "2024-01-01T12:00:00Z"
        }
    });

    let mut test = JsonTest::new(&data);

    test.assert_path("$.users[0].name")
        // Verify first user's name exists and is a string
        .exists()
        .is_string()
        .equals(json!("John Doe"))

        // Check first user's roles - should include admin
        .assert_path("$.users[0].roles")
        .is_array()
        .contains(&json!("admin"))

        // Verify second user has exactly one role
        .assert_path("$.users[1].roles")
        .is_array()
        .has_length(1)

        // Validate email format for second user
        .assert_path("$.users[1].email")
        .contains_string("@")
        .matches_pattern(r"^[^@]+@example\.com$")

        // Check user count in metadata
        .assert_path("$.metadata.total_users")
        .is_number()
        .equals(json!(2))

        // Validate timestamp format in metadata
        .assert_path("$.metadata.last_updated")
        .is_string()
        .starts_with("2024")
        .contains_string("T")
        .ends_with("Z");

    println!("All basic assertions passed!");
}