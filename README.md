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

Download the matching archive from the [latest release](https://github.com/notwillk/parse-internet-shortcut/releases/latest), then run:

x86_64:

```bash
tar -xzf parse-internet-shortcut_*_linux_x86_64.tar.gz && sudo install -m 0755 parse-internet-shortcut /usr/local/bin/parse-internet-shortcut
```

arm64:

```bash
tar -xzf parse-internet-shortcut_*_linux_arm64.tar.gz && sudo install -m 0755 parse-internet-shortcut /usr/local/bin/parse-internet-shortcut
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

### From package repositories (APT and DNF/YUM)

When repository publishing is enabled in the release workflow, package repositories are published at:

- APT: `https://notwillk.github.io/parse-internet-shortcut/apt`
- YUM/DNF: `https://notwillk.github.io/parse-internet-shortcut/rpm`

Debian/Ubuntu:

```bash
sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL https://notwillk.github.io/parse-internet-shortcut/repo-signing-key.gpg \
  | sudo tee /usr/share/keyrings/parse-internet-shortcut-archive-keyring.gpg >/dev/null
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/parse-internet-shortcut-archive-keyring.gpg] https://notwillk.github.io/parse-internet-shortcut/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/parse-internet-shortcut.list >/dev/null
sudo apt-get update
sudo apt-get install -y parse-internet-shortcut
```

Fedora/RHEL (DNF/YUM):

```bash
sudo tee /etc/yum.repos.d/parse-internet-shortcut.repo >/dev/null <<'EOF'
[parse-internet-shortcut]
name=parse-internet-shortcut
baseurl=https://notwillk.github.io/parse-internet-shortcut/rpm
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey=https://notwillk.github.io/parse-internet-shortcut/repo-signing-key.asc
EOF
sudo dnf makecache
sudo dnf install -y parse-internet-shortcut
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
4. If you want repository publishing in the same run, set `publish_repositories=true`.

The workflow builds and uploads these assets to the matching GitHub Release:

- tar.gz archives
- checksums (`checksums.txt`)
- `.deb` package
- `.rpm` package

When `publish_repositories=true`, the workflow also:

- creates signed APT metadata (`InRelease`, `Release.gpg`)
- creates signed YUM/DNF metadata (`repodata/repomd.xml.asc`)
- publishes detached package signatures for `.deb` and `.rpm` artifacts (`*.asc`)
- publishes repositories and the public signing key on GitHub Pages

Required GitHub secrets for repository publishing:

- `REPO_GPG_PRIVATE_KEY` (ASCII-armored private key)
- `REPO_GPG_KEY_ID` (GPG key ID to use for signatures)
- `REPO_GPG_PASSPHRASE` (passphrase for the signing key)

## Release QA for package repositories

Use the manually dispatched **Package Repository QA** workflow after repository publishing:

1. Set `repository_base_url` (default: `https://notwillk.github.io/parse-internet-shortcut`)
2. Set `expected_version` to the version you just released.
3. Optionally set `previous_version` to validate upgrade-path behavior from an older release.

The QA workflow validates:

- signed metadata consumption (`apt`/`dnf` repository checks)
- fresh installation on Ubuntu and Fedora
- optional upgrade visibility/path checks when `previous_version` is provided
