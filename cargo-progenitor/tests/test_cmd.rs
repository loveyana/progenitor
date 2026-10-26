// Copyright 2025 Oxide Computer Company

#[test]
fn test_help() {
    assert_cmd::cargo::cargo_bin_cmd!()
        .arg("progenitor")
        .arg("--help")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic("tests/data/test_help.stdout"));
}

#[test]
fn open_string_enum_flag_reaches_generated_crate() {
    let output =
        std::env::temp_dir().join(format!("progenitor-open-enums-cli-{}", std::process::id(),));
    let result = std::panic::catch_unwind(|| {
        assert_cmd::cargo::cargo_bin_cmd!()
            .args([
                "progenitor",
                "--input",
                "tests/data/open_string_enums.json",
                "--output",
                output.to_str().unwrap(),
                "--name",
                "open-enums-cli-fixture",
                "--version",
                "0.0.0",
                "--open-string-enums",
                "--include-client",
                "true",
            ])
            .assert()
            .success();
        let source = std::fs::read_to_string(output.join("src/lib.rs")).unwrap();
        assert!(source.contains("Unknown(::std::string::String)"));
        assert!(source.contains("provider: &'a types::ProviderKind"));
    });
    let _ = std::fs::remove_dir_all(output);
    result.unwrap();
}
