// Copyright 2026 Oxide Computer Company

#[allow(dead_code)]
mod generated {
    progenitor::generate_api!(
        spec = "../cargo-progenitor/tests/data/open_string_enums.json",
        open_string_enums = true,
    );
}

#[test]
fn macro_passes_option_and_nested_response_round_trips() {
    let raw = serde_json::json!({
        "provider": "future_provider",
        "state": "future_state",
        "providers": ["api_key", "未来供应商"],
        "optional_provider": "optional_future",
        "nullable_state": null
    });
    let decoded: generated::types::Envelope = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(decoded.provider.to_string(), "future_provider");
    let encoded = serde_json::to_value(&decoded).unwrap();
    for key in raw.as_object().unwrap().keys() {
        assert_eq!(encoded[key], raw[key], "{key}");
    }
    assert_eq!(
        generated::types::ProviderKind::default(),
        generated::types::ProviderKind::ApiKey,
    );
    for invalid in ["null", "42", "true", "{}", "[]"] {
        assert!(serde_json::from_str::<generated::types::ProviderKind>(invalid).is_err());
    }
}
