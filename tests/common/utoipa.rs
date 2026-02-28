#![allow(dead_code)]

use utoipa::openapi::RefOr;
use utoipa::openapi::schema::Schema;
use utoipa::{PartialSchema, ToSchema};

/// Verify that the schema is `{"type": "string"}` — identical to String's OpenAPI representation.
pub fn schema_is_string<F>()
where
    F: PartialSchema,
{
    let schema = F::schema();
    let RefOr::T(Schema::Object(obj)) = &schema else {
        panic!("expected Object schema, got a Ref");
    };
    let json = serde_json::to_value(obj).unwrap();
    assert_eq!(json, serde_json::json!({"type": "string"}));
}

/// Verify that name() returns "String" so it's indistinguishable from String in OpenAPI output.
pub fn name_is_string<F>()
where
    F: ToSchema,
{
    assert_eq!(F::name(), "String");
}
