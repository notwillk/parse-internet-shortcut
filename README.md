# parse-internet-shortcut

`parse-internet-shortcut` is a Rust crate that parses Internet Shortcut (`.url`) files. It ships both a library API and a CLI that emits the parsed contents as pretty JSON.

## Library usage

```rust
use parse_internet_shortcut::parse;

let map = parse(
    "[InternetShortcut]\nURL=https://example.com/\nIconIndex=0\n"
);
assert_eq!(map["InternetShortcut"]["URL"], "https://example.com/");
```

## Install (CLI)

Download the latest Linux release artifacts from GitHub Releases and extract the archive for your architecture:

- `linux amd64` (`x86_64`)
- `linux arm64`

Then place `parse-internet-shortcut` on your `PATH`, for example in `/usr/local/bin`.

## Usage (CLI)

```bash
parse-internet-shortcut <path-to-file>
```

Read from stdin with `-`:

```bash
cat example.url | parse-internet-shortcut -
```

## Example

Input:

```ini
[InternetShortcut]
URL=https://example.com
IconFile=https://example.com/favicon.ico
IconIndex=0
```

Output:

```json
{
  "sections": {
    "InternetShortcut": {
      "IconFile": "https://example.com/favicon.ico",
      "IconIndex": "0",
      "URL": "https://example.com"
    }
  }
}
```

## Exit codes

| Code | Meaning |
|---:|---|
| `0` | Success |
| `1` | Usage error |
| `2` | File/read error |
| `3` | Parse error |
| `4` | Serialization/output error |

## Tests

Fixture-based integration coverage lives in `fixtures/`. Each `.url` file has a matching `.json` snapshot. Run `cargo test` to validate all unit tests and fixture pairs automatically.

## Supported platforms

Release artifacts are produced only for:

- Linux amd64
- Linux arm64

## Release process

Releases are configured with GoReleaser via `.goreleaser.yml` and generate:

- tar.gz archives
- checksums (`checksums.txt`)
- `.deb` package
- `.rpm` package

