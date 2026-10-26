// Copyright 2026 Oxide Computer Company

use progenitor_impl::{GenerationSettings, Generator};
use serde_json::json;

fn spec() -> openapiv3::OpenAPI {
    serde_json::from_value(json!({
        "openapi": "3.0.3",
        "info": {"title": "Open enums", "version": "1"},
        "paths": {},
        "components": {"schemas": {
            "ProviderKind": {"type": "string", "enum": ["api_key", "oauth_client"]},
            "Envelope": {
                "type": "object",
                "required": ["provider", "state"],
                "properties": {
                    "provider": {"$ref": "#/components/schemas/ProviderKind"},
                    "state": {"type": "string", "enum": ["pending", "done"]}
                }
            }
        }}
    }))
    .unwrap()
}

#[test]
fn open_string_enums_are_opt_in() {
    let default = Generator::default()
        .generate_tokens(&spec())
        .unwrap()
        .to_string();
    let explicit_closed =
        Generator::new(GenerationSettings::default().with_open_string_enums(false))
            .generate_tokens(&spec())
            .unwrap()
            .to_string();
    assert_eq!(default, explicit_closed);
    assert!(!default.contains("Unknown (:: std :: string :: String)"));

    let open = Generator::new(GenerationSettings::default().with_open_string_enums(true))
        .generate_tokens(&spec())
        .unwrap()
        .to_string();
    assert!(
        open.contains("Unknown (:: std :: string :: String)"),
        "opt-in must reach referenced and inline enums"
    );
    assert_eq!(
        open.matches("Unknown (:: std :: string :: String)").count(),
        2
    );
}

#[test]
fn explicit_discriminator_preserves_its_string_enum() {
    let api = serde_json::from_value(json!({
        "openapi": "3.0.3",
        "info": {"title": "Discriminator fixture", "version": "1"},
        "paths": {},
        "components": {"schemas": {
            "Kind": {"type": "string", "enum": ["tag_a", "tag_b"]},
            "Envelope": {
                "type": "object",
                "required": ["kind"],
                "discriminator": {"propertyName": "kind"},
                "properties": {"kind": {"$ref": "#/components/schemas/Kind"}}
            },
            "OtherKind": {"type": "string", "enum": ["plain_a", "plain_b"]}
        }}
    }))
    .unwrap();
    let output = Generator::new(GenerationSettings::default().with_open_string_enums(true))
        .generate_tokens(&api)
        .unwrap()
        .to_string();
    assert_eq!(
        output
            .matches("Unknown (:: std :: string :: String)")
            .count(),
        1
    );
}

#[test]
fn disabled_policy_keeps_upstream_discriminator_output() {
    let mut value = json!({
        "openapi": "3.0.3",
        "info": {"title": "Default discriminator fixture", "version": "1"},
        "paths": {},
        "components": {"schemas": {
            "Envelope": {
                "type": "object",
                "required": ["kind"],
                "discriminator": {"propertyName": "kind"},
                "properties": {"kind": {"type": "string", "enum": ["tag_a", "tag_b"]}}
            }
        }}
    });
    let with_discriminator = Generator::default()
        .generate_tokens(&serde_json::from_value(value.clone()).unwrap())
        .unwrap()
        .to_string();
    // Upstream ignores discriminator metadata during this conversion.
    value["components"]["schemas"]["Envelope"]
        .as_object_mut()
        .unwrap()
        .remove("discriminator");
    let without_discriminator = Generator::default()
        .generate_tokens(&serde_json::from_value(value).unwrap())
        .unwrap()
        .to_string();
    assert_eq!(with_discriminator, without_discriminator);
}
