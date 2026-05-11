<p align="center">
  <img src="docs/theme/favicon.svg" alt="buffy" width="180">
</p>

<h1 align="center">buffy</h1>

<p align="center">
  <em>A cute Protobuf manager (in alpha)</em>
</p>

<p align="center">
  <a href="https://github.com/julian-siebert/buffy/actions/workflows/ci.yml">
    <img src="https://github.com/julian-siebert/buffy/actions/workflows/ci.yml/badge.svg" alt="CI">
  </a>
  <a href="https://github.com/julian-siebert/buffy/releases">
    <img src="https://img.shields.io/github/v/release/julian-siebert/buffy?color=blueviolet" alt="Release">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License">
  </a>
</p>

---

Buffy builds and publishes Protocol Buffer schemas across multiple language ecosystems from a single configuration file. Define your schema once, ship it to crates.io, Maven Central, npm, and Go modules - without juggling five build systems.

## Supported targets

| Language   | Variants                | Destination                              |
|------------|-------------------------|------------------------------------------|
| Go         | `git`                   | Git remote (Go modules use Git tags)     |
| Java       | `maven_central`, `git`  | Sonatype Central Portal                  |
| Kotlin     | `maven_central`, `git`  | Sonatype Central Portal                  |
| Rust       | `crate`, `git`          | crates.io or another Cargo registry      |
| JavaScript | `npm`, `git`            | npmjs.org or any npm-compatible registry |
| TypeScript | `npm`, `git`            | npmjs.org or any npm-compatible registry |
| Python     | `pypi`, `git`           | pypi.org                                 |

## Installation

```sh
curl -sSL https://pkgs.julian-siebert.de/buffy/install.sh | sh
```

For specific versions or other installation methods, see the [documentation](https://docs.julian-siebert.de/buffy/getting-started/installation).

## Quick example

Define what your package is in `Buffy.toml`:

```toml
[package]
name = "tomato"
description = "Tomato protocol buffers"
version = "0.1.0"
license = "MIT"
homepage = "https://github.com/example/tomato"
authors = ["Jane Doe <jane@example.com>"]

[source]
path = "proto"
```

Configure one profile per language target in `.buffy/`:

```toml
# .buffy/rust.toml
[rust.crate]
name = "tomato"
edition = "2021"
repository = "https://github.com/example/tomato"
documentation = "https://docs.rs/tomato"
registry = "crates-io"
grpc = true
```

```toml
# .buffy/golang.toml
[golang.git]
module = "github.com/example/tomato-go"
remote = "git@github.com:example/tomato-go.git"
branch = "main"
grpc = true
```

Verify the toolchain, build, and publish:

```sh
buffy check
buffy
buffy --publish
```

Each profile is built and published in parallel.

## Use in GitHub Actions

```yaml
- uses: julian-siebert/buffy@v0
- run: buffy --publish --publish-version "${GITHUB_REF_NAME#v}"
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    NPM_TOKEN:            ${{ secrets.NPM_TOKEN }}
    MAVEN_USERNAME:       ${{ secrets.MAVEN_USERNAME }}
    MAVEN_PASSWORD:       ${{ secrets.MAVEN_PASSWORD }}
    GPG_KEY_ID:           ${{ secrets.GPG_KEY_ID }}
    GPG_PASSPHRASE:       ${{ secrets.GPG_PASSPHRASE }}
    GPG_PRIVATE_KEY:      ${{ secrets.GPG_PRIVATE_KEY }}
```

Set only the secrets for the targets you actually publish to.

## Documentation

Full documentation is available at [docs.julian-siebert.de/buffy](https://docs.julian-siebert.de/buffy/), including:

- [The manifest format](https://docs.julian-siebert.de/buffy/reference/manifest)
- [Profile configuration per language](https://docs.julian-siebert.de/buffy/reference/profiles)
- [Environment variables](https://docs.julian-siebert.de/buffy/reference/environment-variables)
- [Command-line interface](https://docs.julian-siebert.de/buffy/reference/cli)

## Status

Buffy is in alpha. The CLI and configuration format may change before 1.0. It is, however, used in projects by the author for personal protocol-buffer libraries.

Bug reports, feature requests, and feedback are very welcome! Please open an issue.

## License

Buffy is licensed under the [Apache License, Version 2.0](LICENSE).
