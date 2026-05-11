use std::collections::HashMap;

use thiserror::Error;

/// Error returned by [`parse`] when the input contains a malformed line.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("malformed line {line}: expected key=value or [section]")]
pub struct ParseError {
    /// 1-based line number of the offending line.
    pub line: usize,
}

/// Parses the textual contents of a Windows `.url` internet shortcut file
/// and returns a nested map of sections → key/value pairs.
///
/// Keys that appear **before** any section header are stored at the top level
/// under an empty section key `""`.
///
/// # Errors
///
/// Returns [`ParseError`] if any non-blank, non-comment line is neither a
/// valid `[section]` header nor a `key=value` pair with a non-empty key.
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
pub fn parse(input: &str) -> Result<HashMap<String, HashMap<String, String>>, ParseError> {
    // Strip optional UTF-8 BOM
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);

    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line_number = idx + 1;
        let line = raw_line.trim();

        // Skip blank lines and comments
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        // Section header
        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(ParseError { line: line_number });
            }
            current_section = line[1..line.len() - 1].trim().to_owned();
            result.entry(current_section.clone()).or_default();
            continue;
        }

        // Key=Value pair
        let Some((raw_key, raw_value)) = line.split_once('=') else {
            return Err(ParseError { line: line_number });
        };

        let key = raw_key.trim();
        if key.is_empty() {
            return Err(ParseError { line: line_number });
        }

        let value = raw_value.trim();
        result
            .entry(current_section.clone())
            .or_default()
            .insert(key.to_owned(), value.to_owned());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_empty_map() {
        assert!(parse("").unwrap().is_empty());
    }

    #[test]
    fn bom_is_stripped() {
        let input = "\u{feff}[S]\nK=V\n";
        let map = parse(input).unwrap();
        assert_eq!(map["S"]["K"], "V");
    }

    #[test]
    fn comments_and_blank_lines_are_ignored() {
        let input = "; comment\n# also comment\n\n[S]\nK=V\n";
        let map = parse(input).unwrap();
        assert_eq!(map["S"]["K"], "V");
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn keys_before_any_section_go_to_root() {
        let input = "K=V\n";
        let map = parse(input).unwrap();
        assert_eq!(map[""]["K"], "V");
    }

    #[test]
    fn whitespace_around_key_and_value_is_trimmed() {
        let input = "[S]\n  Key  =  Value  \n";
        let map = parse(input).unwrap();
        assert_eq!(map["S"]["Key"], "Value");
    }

    #[test]
    fn malformed_line_returns_error() {
        let err = parse("[S]\nnot-a-pair\n").unwrap_err();
        assert_eq!(err.line, 2);
    }

    #[test]
    fn empty_key_returns_error() {
        let err = parse("[S]\n=value\n").unwrap_err();
        assert_eq!(err.line, 2);
    }

    #[test]
    fn unclosed_section_header_returns_error() {
        let err = parse("[S\n").unwrap_err();
        assert_eq!(err.line, 1);
    }
}
