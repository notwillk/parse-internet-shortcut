use std::collections::HashMap;

/// Parses the textual contents of a Windows `.url` internet shortcut file
/// and returns a nested map of sections → key/value pairs.
///
/// Keys that appear **before** any section header are stored at the top level
/// under an empty section key `""`.
///
/// # Format
///
/// ```text
/// ; this is a comment
/// # so is this
///
/// [InternetShortcut]
/// URL=https://example.com/
/// IconIndex=0
/// ```
pub fn parse(input: &str) -> HashMap<String, HashMap<String, String>> {
    // Strip optional UTF-8 BOM
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);

    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for raw_line in input.lines() {
        let line = raw_line.trim();

        // Skip blank lines and comments
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        // Section header
        if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            current_section = inner.trim().to_owned();
            result.entry(current_section.clone()).or_default();
            continue;
        }

        // Key=Value pair
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_owned();
            let value = line[eq_pos + 1..].trim().to_owned();
            result
                .entry(current_section.clone())
                .or_default()
                .insert(key, value);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_empty_map() {
        assert!(parse("").is_empty());
    }

    #[test]
    fn bom_is_stripped() {
        let input = "\u{feff}[S]\nK=V\n";
        let map = parse(input);
        assert_eq!(map["S"]["K"], "V");
    }

    #[test]
    fn comments_and_blank_lines_are_ignored() {
        let input = "; comment\n# also comment\n\n[S]\nK=V\n";
        let map = parse(input);
        assert_eq!(map["S"]["K"], "V");
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn keys_before_any_section_go_to_root() {
        let input = "K=V\n";
        let map = parse(input);
        assert_eq!(map[""]["K"], "V");
    }

    #[test]
    fn whitespace_around_key_and_value_is_trimmed() {
        let input = "[S]\n  Key  =  Value  \n";
        let map = parse(input);
        assert_eq!(map["S"]["Key"], "Value");
    }
}
