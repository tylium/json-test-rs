//! Advanced examples showing powerful JSONPath queries and assertions.

use json_test::JsonTest;
use serde_json::json;

fn main() {
    let data = json!({
        "orders": [
            {
                "id": "ord_1",
                "customer": "John Doe",
                "items": [
                    { "product": "Widget", "price": 29.99, "quantity": 2 },
                    { "product": "Gadget", "price": 49.99, "quantity": 1 }
                ],
                "status": "shipped",
                "shipping_address": {
                    "country": "US",
                    "priority": "express"
                }
            },
            {
                "id": "ord_2",
                "customer": "Jane Smith",
                "items": [
                    { "product": "Widget", "price": 29.99, "quantity": 1 }
                ],
                "status": "pending",
                "shipping_address": {
                    "country": "UK",
                    "priority": "standard"
                }
            }
        ],
        "stats": {
            "total_orders": 2,
            "countries": ["US", "UK"],
            "average_order_value": 89.98
        }
    });

    let mut test = JsonTest::new(&data);

    test.assert_path("$.orders[?(@.status == 'shipped')].customer")
        // Find customer with shipped order using JSONPath filter
        .equals(json!("John Doe"))

        // Verify express shipping is for order ord_1
        .assert_path("$.orders[?(@.shipping_address.priority == 'express')].id")
        .equals(json!("ord_1"))

        // Check first order's items array
        .assert_path("$.orders[0].items")
        .is_array()
        .has_length(2)

        // Verify shipping countries by checking stats array directly
        .assert_path("$.stats.countries")
        .is_array()
        .contains(&json!("US"))
        .contains(&json!("UK"))

        // Validate average order value is in expected range
        .assert_path("$.stats.average_order_value")
        .is_number()
        .is_greater_than(80)
        .is_less_than(90)

        // Complex filter: find orders containing Widget product
        .assert_path("$.orders[?(@.items[*].product == 'Widget')].status")
        .is_array()
        .has_length(2);

    println!("All advanced assertions passed!");
}