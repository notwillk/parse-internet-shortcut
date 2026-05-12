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

This CLI is distributed through [GitHub Releases](https://github.com/notwillk/parse-internet-shortcut/releases).

### From a release archive

Download the archive for your Linux architecture:

- `parse-internet-shortcut_<version>_linux_x86_64.tar.gz`
- `parse-internet-shortcut_<version>_linux_arm64.tar.gz`

Extract the archive and install the binary onto your `PATH`:

```bash
tar -xzf parse-internet-shortcut_<version>_linux_x86_64.tar.gz
sudo install -m 0755 parse-internet-shortcut /usr/local/bin/parse-internet-shortcut
```

### From a package

Each release also includes Linux packages:

- `.deb` for Debian/Ubuntu
- `.rpm` for RHEL/Fedora/openSUSE

Install the package you downloaded with your system package manager, for example:

```bash
sudo dpkg -i ./parse-internet-shortcut_*.deb
```

```bash
sudo rpm -i ./parse-internet-shortcut-*.rpm
```

Verify the installation:

```bash
parse-internet-shortcut --help
```

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

## Local build (for testing)

Use the same Rust toolchain as CI/release (`1.85.x`), then build and test locally:

```bash
cargo build
cargo test
```

Run the local binary directly from the build output:

```bash
./target/debug/parse-internet-shortcut --help
```

## Supported platforms

Release artifacts are produced only for:

- Linux amd64
- Linux arm64

## Release process

Releases are configured with GoReleaser via `.goreleaser.yml` and are published by the manually dispatched `Release` GitHub Actions workflow.

To cut a release:

1. Create and push the Git tag you want to publish, for example `v0.1.0`.
2. Open the repository's **Actions** tab.
3. Run the **Release** workflow and provide that tag as the `tag` input.

The workflow builds and uploads these assets to the matching GitHub Release:

- tar.gz archives
- checksums (`checksums.txt`)
- `.deb` package
- `.rpm` package
