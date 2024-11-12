# json-test

[![Crates.io](https://img.shields.io/crates/v/json-test.svg)](https://crates.io/crates/json-test)
[![Documentation](https://docs.rs/json-test/badge.svg)](https://docs.rs/json-test)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md#license)

A testing library for JSON Path assertions in Rust, providing a fluent API for validating JSON structures in tests.

## Features

- 🔍 JSONPath-based value extraction and validation
- ⛓️ Chainable, fluent assertion API
- 🎯 Type-safe operations
- 🧩 Property existence and value validation
- 📝 String pattern matching with regex support
- 🔢 Numeric comparisons
- 📦 Array and object validation
- 🎨 Custom matcher support

## Quick Start

Add to your `Cargo.toml`:
```toml
[dev-dependencies]
json-test = "0.1"
```

Basic usage:
```rust
use json_test::JsonTest;
use serde_json::json;

#[test]
fn test_json_structure() {
    let data = json!({
        "user": {
            "name": "John Doe",
            "age": 30,
            "roles": ["user", "admin"]
        }
    });

    let mut test = JsonTest::new(&data);
    
    test.assert_path("$.user.name")
        .exists()
        .is_string()
        .equals(json!("John Doe"));
        
    test.assert_path("$.user.roles")
        .is_array()
        .has_length(2)
        .contains(&json!("admin"));
}
```

## Main Features

### Path Assertions
```rust
test.assert_path("$.user")
    .exists()
    .has_property("name")
    .has_property_value("age", json!(30));
```

### String Operations
```rust
test.assert_path("$.user.email")
    .is_string()
    .contains_string("@")
    .matches_pattern(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$");
```

### Numeric Comparisons
```rust
test.assert_path("$.user.age")
    .is_number()
    .is_greater_than(18)
    .is_less_than(100);
```

### Array Operations
```rust
test.assert_path("$.user.roles")
    .is_array()
    .has_length(2)
    .contains(&json!("admin"));
```

### Property Matching
```rust
test.assert_path("$.user")
    .has_properties(vec!["name", "age", "roles"])
    .properties_matching(|key| key.starts_with("meta_"))
        .count(0)
        .and()
    .has_property_matching("age", |v| v.as_u64().unwrap_or(0) > 18);
```

### Custom Matchers
```rust
test.assert_path("$.timestamp")
    .matches(|value| {
        value.as_str()
            .map(|s| s.parse::<DateTime<Utc>>().is_ok())
            .unwrap_or(false)
    });
```

## Examples

Looking for more examples? Check out the `examples/` directory which showcases:

- Basic JSON validation patterns
- Advanced JSONPath queries
- Property matching capabilities

## Error Messages

The library provides clear, test-friendly error messages:
```text
Property 'email' not found at $.user
Available properties: name, age, roles

Array at $.user.roles has wrong length
Expected: 3
Actual: 2
```

## Status

This library is in active development (0.1.x). While the core API is stabilizing, minor breaking changes might occur before 1.0.

## Roadmap

- Enhanced array operations
- Deep property traversal
- Improved string operations
- Additional numeric assertions

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.