use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use parse_internet_shortcut::parse;

/// Walk the `fixtures/` directory and, for every `.url` file found, parse it
/// and compare the result against the matching `.json` snapshot.
#[test]
fn all_fixtures_match_json_snapshots() {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");

    let mut url_files: Vec<PathBuf> = fs::read_dir(&fixtures_dir)
        .expect("fixtures/ directory should exist")
        .map(|e| e.expect("should read every entry in fixtures/"))
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("url"))
        .collect();

    assert!(
        !url_files.is_empty(),
        "Expected at least one .url fixture in fixtures/"
    );

    url_files.sort();

    for url_path in &url_files {
        let json_path = url_path.with_extension("json");

        assert!(
            json_path.exists(),
            "Missing JSON snapshot for fixture {}",
            url_path.display()
        );

        let shortcut_src =
            fs::read_to_string(url_path).expect("should read .url fixture");
        let expected_src =
            fs::read_to_string(&json_path).expect("should read .json snapshot");

        let parsed = parse(&shortcut_src);

        // The JSON snapshots use one of two shapes:
        //   1. { "SectionName": { "Key": "Value" } }  — sectioned file
        //   2. { "Key": "Value" }                      — no section header (keys at root)
        //
        // Our parser always returns HashMap<section, HashMap<key, value>>.
        // Root-level keys are stored under the empty string key "".
        // We convert the snapshot into the same shape for comparison.
        let expected_value: serde_json::Value =
            serde_json::from_str(&expected_src).expect("snapshot should be valid JSON");

        let expected_parsed = json_to_parsed_map(&expected_value, url_path.display().to_string());

        assert_eq!(
            parsed, expected_parsed,
            "Fixture {} did not match snapshot {}",
            url_path.display(),
            json_path.display()
        );
    }
}

/// Convert a JSON snapshot value into the same nested-map shape that `parse()`
/// returns so the two can be compared directly.
///
/// Snapshots that have all-string leaf values directly represent a section map
/// (no-section case, stored under `""`). Snapshots whose leaf values are
/// objects represent the sectioned case.
fn json_to_parsed_map(
    value: &serde_json::Value,
    fixture_name: String,
) -> HashMap<String, HashMap<String, String>> {
    let obj = value
        .as_object()
        .unwrap_or_else(|| panic!("snapshot {fixture_name} should be a JSON object"));

    // Detect whether this is a flat (no-section) snapshot: every value is a
    // plain string.
    let is_flat = obj.values().all(|v| v.is_string());

    if is_flat {
        // All keys live under the root (empty-string) section.
        let section: HashMap<String, String> = obj
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_owned()))
            .collect();
        let mut result = HashMap::new();
        result.insert(String::new(), section);
        result
    } else {
        // Each top-level key is a section name whose value is an object of
        // key/value string pairs.
        obj.iter()
            .map(|(section_name, section_value)| {
                let section_obj = section_value.as_object().unwrap_or_else(|| {
                    panic!("snapshot {fixture_name}: section {section_name} should be a JSON object")
                });
                let kv: HashMap<String, String> = section_obj
                    .iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_owned()))
                    .collect();
                (section_name.clone(), kv)
            })
            .collect()
    }
}
