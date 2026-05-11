use std::collections::BTreeMap;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ParsedShortcut {
    pub sections: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("malformed line {line}: expected key=value or [section]")]
pub struct ParseError {
    pub line: usize,
}

pub fn parse_shortcut(input: &str) -> Result<ParsedShortcut, ParseError> {
    let mut sections: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut current_section = String::new();
    let content = input.strip_prefix('\u{feff}').unwrap_or(input);

    for (line_number, raw_line) in content.lines().enumerate() {
        let line_number = line_number + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(ParseError { line: line_number });
            }

            let section_name = line[1..line.len() - 1].trim().to_owned();
            current_section = section_name;
            sections.entry(current_section.clone()).or_default();
            continue;
        }

        let Some((raw_key, raw_value)) = line.split_once('=') else {
            return Err(ParseError { line: line_number });
        };

        let key = raw_key.trim();
        if key.is_empty() {
            return Err(ParseError { line: line_number });
        }

        let value = raw_value.trim();
        sections
            .entry(current_section.clone())
            .or_default()
            .insert(key.to_owned(), value.to_owned());
    }

    Ok(ParsedShortcut { sections })
}

#[cfg(test)]
mod tests {
    use super::parse_shortcut;

    #[test]
    fn parses_duplicate_keys_last_wins() {
        let parsed = parse_shortcut(
            "[InternetShortcut]\nURL=https://old.example\nURL=https://new.example\n",
        )
        .expect("should parse");

        assert_eq!(
            parsed.sections["InternetShortcut"]["URL"],
            "https://new.example"
        );
    }

    #[test]
    fn strips_utf8_bom() {
        let parsed = parse_shortcut("\u{feff}[InternetShortcut]\nURL=https://example.com\n")
            .expect("should parse");

        assert_eq!(
            parsed.sections["InternetShortcut"]["URL"],
            "https://example.com"
        );
    }

    #[test]
    fn puts_preamble_keys_in_empty_section() {
        let parsed = parse_shortcut("URL=https://example.com\n").expect("should parse");

        assert_eq!(parsed.sections[""]["URL"], "https://example.com");
    }

    #[test]
    fn errors_on_malformed_line() {
        let error = parse_shortcut("[InternetShortcut]\nnot-a-pair\n").expect_err("should fail");
        assert_eq!(error.line, 2);
    }
}
