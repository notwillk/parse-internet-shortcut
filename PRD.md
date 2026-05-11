# Implementation Prompt: `parse-internet-shortcut`

Implement a Rust CLI command named `parse-internet-shortcut`.

## Goal

Create a small Rust CLI that parses a standards-based Internet Shortcut file, commonly using the `*.url` extension, and writes the parsed representation as JSON to stdout.

The canonical format is INI-style and typically contains an `[InternetShortcut]` section:

```ini
[InternetShortcut]
URL=https://example.com
IconFile=https://example.com/favicon.ico
IconIndex=0
```

The tool should preserve parsed contents, including unknown fields and extra sections, rather than only extracting `URL`.

## CLI Behavior

Command:

```bash
parse-internet-shortcut <path-to-file>
```

Example:

```bash
parse-internet-shortcut ./example.url
```

Output:

```json
{
  "sections": {
    "InternetShortcut": {
      "URL": "https://example.com",
      "IconFile": "https://example.com/favicon.ico",
      "IconIndex": "0"
    }
  }
}
```

The command should also support reading from stdin:

```bash
cat example.url | parse-internet-shortcut -
```

## Requirements

### Parsing

- Parse `.url` files as INI-style documents.
- Support section headers like `[InternetShortcut]`.
- Support `key=value` pairs.
- Trim surrounding whitespace from section names, keys, and values.
- Ignore blank lines.
- Ignore comment lines beginning with `;` or `#`.
- Preserve unknown sections.
- Preserve unknown keys.
- Preserve values as strings.
- Do not validate that URLs are reachable.
- Do not require the file extension to be `.url`.
- Handle UTF-8 input.
- Strip a UTF-8 BOM if present.
- Return a useful error for malformed lines.

### JSON Shape

Use this output structure:

```json
{
  "sections": {
    "<section-name>": {
      "<key>": "<value>"
    }
  }
}
```

If key/value pairs appear before any section, place them under an empty-string section name:

```json
{
  "sections": {
    "": {
      "URL": "https://example.com"
    }
  }
}
```

### Duplicate Keys

If a key appears more than once in the same section, the last value wins.

Example input:

```ini
[InternetShortcut]
URL=https://old.example
URL=https://new.example
```

Expected output:

```json
{
  "sections": {
    "InternetShortcut": {
      "URL": "https://new.example"
    }
  }
}
```

## Errors

Write errors to stderr and exit non-zero.

Example errors:

```text
error: missing input path
error: failed to read file: ./missing.url
error: malformed line 4: expected key=value or [section]
```

Exit codes:

| Code | Meaning |
|---:|---|
| `0` | Success |
| `1` | Usage error |
| `2` | File/read error |
| `3` | Parse error |
| `4` | Serialization/output error |

## Dependencies

Use stable, common Rust crates:

- `clap` for CLI parsing
- `serde` and `serde_json` for JSON output
- `thiserror` or `anyhow` for error handling

Prefer a small custom parser over a large INI dependency, unless the dependency clearly preserves all behavior required here.

## Project Layout

Create a normal Rust binary crate:

```text
parse-internet-shortcut/
  Cargo.toml
  src/
    main.rs
    parser.rs
  tests/
    fixtures/
      basic.url
      comments.url
      stdin.url
      malformed.url
```

## Tests

Add unit and integration tests covering:

- Basic `[InternetShortcut]` file
- Comments beginning with `;`
- Comments beginning with `#`
- Blank lines
- Whitespace around section names, keys, and values
- Unknown sections
- Unknown fields
- UTF-8 BOM
- Input from stdin
- Malformed line handling
- Duplicate keys where the last value wins

## Formatting

- Pretty-print JSON by default.
- Ensure stdout output ends with a newline.

## Release Requirements

Use GoReleaser.

Support only Linux:

| OS | Architecture |
|---|---|
| linux | amd64 |
| linux | arm64 |

Do not build macOS or Windows artifacts.

Generate:

- compressed archives
- checksums
- `.deb` package
- `.rpm` package

Add `.goreleaser.yml`.

Use this as the starting GoReleaser config:

```yaml
version: 2

project_name: parse-internet-shortcut

before:
  hooks:
    - cargo fetch --locked

builds:
  - id: parse-internet-shortcut
    builder: rust
    binary: parse-internet-shortcut
    targets:
      - x86_64-unknown-linux-gnu
      - aarch64-unknown-linux-gnu

archives:
  - id: default
    formats:
      - tar.gz
    name_template: "{{ .ProjectName }}_{{ .Version }}_{{ .Os }}_{{ if eq .Arch \"amd64\" }}x86_64{{ else if eq .Arch \"arm64\" }}arm64{{ else }}{{ .Arch }}{{ end }}"

checksum:
  name_template: checksums.txt

nfpms:
  - id: packages
    package_name: parse-internet-shortcut
    description: Parse Internet Shortcut .url files and emit JSON.
    license: MIT
    formats:
      - deb
      - rpm
    bindir: /usr/bin

snapshot:
  version_template: "{{ incpatch .Version }}-next"

changelog:
  use: git
```

If the installed GoReleaser version requires different Rust builder syntax, adjust the config while preserving the same release targets and artifact types.

## Documentation

Add a `README.md` with:

- Description
- Install instructions from GitHub Releases
- Usage examples
- Example input/output
- Exit codes
- Supported platforms
- Release process

Add a `LICENSE` file. Use MIT unless the repository already specifies another license.

## Acceptance Criteria

- `cargo test` passes.
- `cargo fmt --check` passes.
- `cargo clippy -- -D warnings` passes.
- Running the binary on a valid `.url` file emits valid pretty JSON.
- Running the binary on malformed input exits with code `3`.
- GoReleaser config only targets Linux `amd64` and `arm64`.
- Release artifacts include checksums.
- Release artifacts include `.deb` and `.rpm` packages.
