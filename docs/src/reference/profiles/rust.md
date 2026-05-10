# Rust Profiles

The `rust` profile generates a Cargo crate from your `.proto` files using
[`prost`] and (optionally) [`tonic`] for gRPC. The crate is published either
to [crates.io] (or any Cargo-compatible registry) or to a Git repository.

Available variants:

* [`crate`](#the-crate-variant) --- Publish to a Cargo registry.
* [`git`](#the-git-variant) --- Push the generated crate to a Git remote.

## Required tools

* `protoc` --- Protocol Buffers compiler.
* `protoc-gen-prost` --- Rust message generator.
* `protoc-gen-prost-crate` --- Crate-layout generator.
* `protoc-gen-tonic` --- gRPC plugin (only when `grpc = true`).
* `cargo` --- Rust toolchain, used for `cargo build` and `cargo publish`.
* `git` --- Required only for the `git` variant.

The `protoc-gen-*` plugins are installed via Cargo:

```sh
cargo install protoc-gen-prost protoc-gen-prost-crate protoc-gen-tonic
```

## The `crate` variant

Generates a Cargo crate under `target/<profile>/`, runs `cargo build` to
verify, then runs `cargo publish` against the configured registry.

### Example

```toml
# .buffy/rust-example.toml
[rust.crate]
name = "tomato"
edition = "2021"
repository = "https://github.com/example/tomato"
documentation = "https://docs.rs/tomato"
registry = "crates-io"
grpc = true
```

### Fields

* [`name`](#the-name-field) --- Crate name as it appears in `Cargo.toml`.
* [`edition`](#the-edition-field) --- Rust edition.
* [`repository`](#the-repository-field) --- Repository URL embedded in the crate.
* [`documentation`](#the-documentation-field) --- Documentation URL.
* [`registry`](#the-registry-field) --- Cargo registry name.
* [`prost_version`](#the-prost_version-field) --- Pin `prost` runtime.
* [`tonic_version`](#the-tonic_version-field) --- Pin `tonic` runtime.
* [`grpc`](#the-grpc-field) --- Generate gRPC service stubs.

#### The `name` field

The crate name as it will appear in `Cargo.toml` and on the registry. Must
follow Cargo naming rules (lowercase, hyphens permitted).

```toml
name = "tomato"
```

The library name (i.e. the Rust module name) is derived by replacing
hyphens with underscores: `tomato-rs` becomes `tomato_rs`.

#### The `edition` field

The Rust edition for the generated crate.

```toml
edition = "2021"
```

Common values: `"2021"`, `"2024"`. The edition is written verbatim to the
generated `Cargo.toml`.

#### The `repository` field

A URL to the source repository. Embedded in the crate's `Cargo.toml` as the
`repository` field.

```toml
repository = "https://github.com/example/tomato"
```

#### The `documentation` field

A URL to the crate's documentation page.

```toml
documentation = "https://docs.rs/tomato"
```

#### The `registry` field

The name of the Cargo registry to publish to. The special value
`"crates-io"` targets [crates.io] directly. Any other value must correspond
to a registry configured in the user's `~/.cargo/config.toml`:

```toml
# ~/.cargo/config.toml
[registries]
my-registry = { index = "sparse+https://my-kellnr.example.com/api/v1/crates/" }
```

```toml
# .buffy/rust.toml
registry = "my-registry"
```

Default: `"crates-io"`.

#### The `prost_version` field

Optional. Pin the `prost` runtime version. If omitted, Buffy queries
crates.io for the latest release.

```toml
prost_version = "0.13"
```

#### The `tonic_version` field

Optional. Pin the `tonic` runtime version (and `tonic-prost` for tonic 0.13+).
Only used when `grpc = true`. If omitted, Buffy queries crates.io for the
latest release.

```toml
tonic_version = "0.13"
```

#### The `grpc` field

When `true`, Buffy invokes `protoc-gen-tonic` and includes `tonic` and
`tonic-prost` in the crate's dependencies.

```toml
grpc = true
```

Default: `false`.

### Required environment variables

| Variable               | Purpose                                          |
|------------------------|--------------------------------------------------|
| `CARGO_REGISTRY_TOKEN` | Token for the configured registry (publish auth) |

For non-`crates-io` registries, the index URL must be configured in
`~/.cargo/config.toml` as shown above.

### Example consumer usage

```sh
cargo add tomato
```

```rust
use tomato::greeter::HelloRequest;

let req = HelloRequest { name: "World".into() };
println!("{}", req.name);
```

## The `git` variant

Generates the same crate as `crate`, but commits and tags it to a Git
repository instead of uploading to a registry. Useful when you want to
distribute internally without setting up a registry, or to give consumers
a reproducible Git pin.

### Example

```toml
# .buffy/rust.toml
[rust.git]
name = "tomato"
edition = "2021"
remote = "git@github.com:example/tomato-rs.git"
branch = "main"
repository = "https://github.com/example/tomato-rs"
documentation = "https://github.com/example/tomato-rs"
grpc = true
keep = ["README.md"]
```

### Fields

The same fields as `crate` (except `registry`, which doesn't apply), plus:

* [`remote`](#the-remote-field) --- Git URL the crate is pushed to.
* [`branch`](#the-branch-field) --- Branch to push to.
* [`keep`](#the-keep-field) --- Files to preserve from the remote.

#### The `remote` field

```toml
remote = "git@github.com:example/tomato-rs.git"
```

#### The `branch` field

```toml
branch = "main"
```

#### The `keep` field

```toml
keep = ["README.md"]
```

Default: `[]`.

### Example consumer usage

```toml
# in the consumer's Cargo.toml
[dependencies]
tomato = { git = "https://github.com/example/tomato-rs", tag = "v0.1.0" }
```

[`prost`]: https://github.com/tokio-rs/prost
[`tonic`]: https://github.com/hyperium/tonic
[crates.io]: https://crates.io/
