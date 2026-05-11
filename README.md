# parse-internet-shortcut

Parse Windows `.url` internet shortcut files into a nested map of sections and key/value pairs.

## Usage

```rust
use parse_internet_shortcut::parse;

let map = parse(
    "[InternetShortcut]\nURL=https://example.com/\nIconIndex=0\n"
);
assert_eq!(map["InternetShortcut"]["URL"], "https://example.com/");
```

## Tests

Fixture-based integration coverage lives in `fixtures/`. Each `.url` file has a matching `.json` snapshot. Run `cargo test` to validate every pair automatically.
