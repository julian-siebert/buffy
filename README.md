# buffy

**buffy** is a command-line tool that generates and publishes gRPC/Protobuf client stubs for multiple languages from a single `Buffy.toml` configuration file.

Instead of manually running `protoc` with different plugins, managing module configurations, and publishing to various package registries, buffy handles the entire pipeline — from proto files to published packages — in one command.

```
buffy             # generate stubs for all configured languages
buffy --publish   # publish to configured registries
```

---

## Table of contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Quick start](#quick-start)
- [Configuration reference](#configuration-reference)
- [Languages](#languages)
  - [Go](#go)
  - [Java](#java)
  - [Rust](#rust)
- [gRPC support](#grpc-support)
- [Publishing](#publishing)
  - [Go → GitHub](#go--github)
  - [Java → Maven Central](#java--maven-central)
  - [Rust → crates.io or self-hosted](#rust--cratesio-or-self-hosted)
- [Environment variables](#environment-variables)
- [CLI reference](#cli-reference)
- [CI / GitHub Actions](#ci--github-actions)
- [Future languages](#future-languages)

---

## Requirements

buffy requires `protoc` and the relevant language plugins to be installed and available on your `PATH`. Only install what you need for the languages you use.

**Core**

| Tool | Required for |
|------|-------------|
| `protoc` | All languages |

**Go**

| Tool | Install |
|------|---------|
| `protoc-gen-go` | `go install google.golang.org/protobuf/cmd/protoc-gen-go@latest` |
| `protoc-gen-go-grpc` | `go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest` (gRPC only) |
| `go` | [go.dev/doc/install](https://go.dev/doc/install) |
| `git` | For publishing |

**Java**

| Tool | Install |
|------|---------|
| `mvn` | [maven.apache.org/install.html](https://maven.apache.org/install.html) |

**Rust**

| Tool | Install |
|------|---------|
| `protoc-gen-prost` | `cargo install protoc-gen-prost` |
| `protoc-gen-prost-crate` | `cargo install protoc-gen-prost-crate` |
| `protoc-gen-tonic` | `cargo install protoc-gen-tonic` (gRPC only) |

---

## Installation

buffy is currently only available to build from source. A Rust toolchain is required.

```bash
cargo install --git https://github.com/julian-siebert/buffy
```

---

## Quick start

**1. Initialize a project**

```bash
mkdir myservice && cd myservice
buffy init myservice
```

This creates a `Buffy.toml` and an example `src/myservice.proto`. Or create the structure manually:

```
myservice/
├── Buffy.toml
└── src/
    └── myservice/
        └── v1/
            └── service.proto
```

```protobuf
// src/myservice/v1/service.proto
syntax = "proto3";

package myservice.v1;

message PingRequest {
    string message = 1;
}

message PingResponse {
    string message = 1;
}

service PingService {
    rpc Ping(PingRequest) returns (PingResponse);
}
```

> **Tip:** Always declare a `package` in your `.proto` files. buffy uses the package name to build the correct module hierarchy in the generated output.

**2. Configure `Buffy.toml`**

```toml
[package]
name    = "myservice"
version = "0.1.0"
grpc    = true

[source]
path = "./src"

[golang]
module = "github.com/your-org/myservice-go"
remote = "git@github.com:your-org/myservice-go.git"

[rust]
name     = "myservice-proto"
registry = "crates-io"

[java]
group_id    = "com.your-org"
artifact_id = "myservice-proto"
description = "Generated protobuf stubs for myservice"
url         = "https://github.com/your-org/myservice"

[java.license]
name = "Apache-2.0"
url  = "https://www.apache.org/licenses/LICENSE-2.0"

[java.developer]
id    = "your-bot"
name  = "Your Name"
email = "you@your-org.com"

[java.scm]
connection = "scm:git:git://github.com/your-org/myservice.git"
url        = "https://github.com/your-org/myservice"
```

**3. Check your setup**

```bash
buffy check
```

This validates `Buffy.toml` and verifies that all required tools are installed.

**4. Build**

```bash
buffy build
```

**5. Publish**

```bash
buffy publish
```

All configured languages are built and published in parallel.

---

## Configuration reference

### `[package]`

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `name` | string | ✓ | Project name |
| `version` | string | ✓ | Semver version, e.g. `"1.2.3"` |
| `grpc` | bool | `true` | Generate gRPC service stubs in addition to message types |

### `[source]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `path` | path | `"./src"` | Directory containing your `.proto` files. Searched recursively. |

---

## Languages

Configure only the languages you need. Any language section that is absent is simply skipped.

### Go

```toml
[golang]
module = "github.com/your-org/myservice-go"
remote = "git@github.com:your-org/myservice-go.git"
branch = "main"
keep   = ["README.md", "LICENSE", ".gitignore"]
```

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `module` | string | ✓ | Go module path, used in `go mod init` and `--go_opt=module=` |
| `remote` | string | ✓ (publish) | Git remote URL |
| `branch` | string | `"main"` | Branch to push to |
| `keep` | string[] | `[]` | Files fetched from the remote and preserved across publishes (e.g. `README.md`, `LICENSE`) |

**What buffy does during `build`:**

1. Runs `protoc --go_out` (and `--go-grpc_out` if `grpc = true`)
2. Initializes a Go module with `go mod init`
3. Resolves dependencies with `go mod tidy`
4. Verifies the output compiles with `go build ./...`

---

### Java

```toml
[java]
group_id         = "com.your-org"
artifact_id      = "myservice-proto"
description      = "Generated protobuf stubs for myservice"
url              = "https://github.com/your-org/myservice"
protobuf_version = "4.29.3"   # optional – fetched from Maven Central if omitted

[java.license]
name = "Apache-2.0"
url  = "https://www.apache.org/licenses/LICENSE-2.0"

[java.developer]
id    = "your-bot"
name  = "Your Name"
email = "you@your-org.com"

[java.scm]
connection = "scm:git:git://github.com/your-org/myservice.git"
url        = "https://github.com/your-org/myservice"
```

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `group_id` | string | ✓ | Maven group ID |
| `artifact_id` | string | ✓ | Maven artifact ID |
| `description` | string | ✓ | Package description |
| `url` | string | ✓ | Project URL |
| `protobuf_version` | string | — | Pin a specific `protobuf-java` version. If omitted, the latest is fetched from Maven Central automatically. |
| `license.name` | string | ✓ | License name, e.g. `"Apache-2.0"` |
| `license.url` | string | ✓ | License URL |
| `developer.id` | string | ✓ | Developer ID |
| `developer.name` | string | ✓ | Developer display name |
| `developer.email` | string | ✓ | Developer email |
| `scm.connection` | string | ✓ | SCM connection string |
| `scm.url` | string | ✓ | SCM URL |

**What buffy does during `build`:**

1. Fetches the latest `protobuf-java` version from Maven Central (unless pinned)
2. Generates a `pom.xml` with all metadata filled in
3. Runs `protoc --java_out`
4. Verifies with `mvn compile`

The generated `pom.xml` is pre-configured for Maven Central publishing, including GPG signing, sources jar, and javadoc jar — all requirements for Central.

---

### Rust

```toml
[rust]
name          = "myservice-proto"
edition       = "2021"
registry      = "crates-io"
prost_version = "0.13.5"   # optional – fetched from crates.io if omitted
tonic_version = "0.12.3"   # optional – fetched from crates.io if omitted
```

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `name` | string | ✓ | Crate name |
| `edition` | string | `"2021"` | Rust edition |
| `registry` | string | `"crates-io"` | Registry to publish to. Use the name from `~/.cargo/config.toml` for self-hosted registries. |
| `prost_version` | string | — | Pin a specific `prost` version |
| `tonic_version` | string | — | Pin a specific `tonic` version (only used when `grpc = true`) |

**What buffy does during `build`:**

1. Fetches the latest `prost` (and `tonic`) versions from crates.io (unless pinned)
2. Runs `protoc --prost_out` (and `--tonic_out` if `grpc = true`)
3. Assembles all generated files into a single `lib.rs`, correctly resolving module hierarchies derived from proto `package` declarations
4. Verifies with `cargo build`

---

## gRPC support

Controlled by `grpc` under `[package]`. Defaults to `true`.

```toml
[package]
name    = "myservice"
version = "0.1.0"
grpc    = false   # messages only, no service stubs
```

When `grpc = true`, buffy automatically:

- **Go**: adds `--go-grpc_out` to the protoc invocation. Requires `protoc-gen-go-grpc`.
- **Rust**: adds `--tonic_out` and includes `tonic` as a dependency. Requires `protoc-gen-tonic`.
- **Java**: handled natively by the standard `protoc --java_out` plugin.

---

## Publishing

### Go → GitHub

buffy publishes Go stubs by pushing to a Git repository. On each publish:

1. A fresh `git init` is created in the build directory
2. Files listed in `keep` are fetched from the remote and preserved
3. The generated code is committed as `release vX.Y.Z`
4. A version tag is created and force-pushed

The branch is always force-pushed — the generated repository has no meaningful history. Any manual changes to the repository will be overwritten on the next publish. Use the `keep` list for files you want to maintain manually (README, license, etc.).

**Authentication** is handled by your existing Git credentials. buffy never prompts interactively — if credentials are missing, it fails immediately with a clear error. Set up one of the following:

- **SSH** (recommended): add your SSH key to GitHub/GitLab, use `git@github.com:...` URLs
- **HTTPS**: use a personal access token stored in your system keychain via `git credential`

---

### Java → Maven Central

buffy publishes Java stubs to Maven Central via `mvn deploy`.

**Prerequisites:**

1. Register at [central.sonatype.com](https://central.sonatype.com) and verify your namespace (`com.your-org`)
2. Generate a deployment token in the Central Portal
3. Create a GPG key and publish it to a keyserver:

```bash
gpg --gen-key
gpg --keyserver keys.openpgp.org --send-keys YOUR_KEY_ID
```

Set the required environment variables before running `buffy publish`:

```bash
export MAVEN_USERNAME=your-sonatype-username
export MAVEN_PASSWORD=your-sonatype-token
export GPG_KEY_ID=ABCDEF1234567890
export GPG_PASSPHRASE=your-passphrase

# CI only: base64-encoded armored private key
export GPG_PRIVATE_KEY=$(gpg --armor --export-secret-keys YOUR_KEY_ID | base64 -w0)
```

---

### Rust → crates.io or self-hosted

buffy publishes Rust crates via `cargo publish`.

**crates.io:**

```bash
export CARGO_REGISTRY_TOKEN=your-token-from-crates.io
```

**Self-hosted (e.g. [Kellnr](https://kellnr.io/)):**

There is no extra configuration needed in `Buffy.toml` for self-hosted registries beyond the `registry` name. Add the registry to your `~/.cargo/config.toml` once:

```toml
# ~/.cargo/config.toml
[registries]
my-company = { index = "https://kellnr.example.com/index" }
```

Then in `Buffy.toml`:

```toml
[rust]
name     = "myservice-proto"
registry = "my-company"
```

```bash
export CARGO_REGISTRY_TOKEN=your-self-hosted-token
```

---

## Environment variables

buffy automatically loads a `.env` file from the current directory if one exists.

| Variable | Used by | Description |
|----------|---------|-------------|
| `CARGO_REGISTRY_TOKEN` | Rust | API token for crates.io or a self-hosted registry |
| `MAVEN_USERNAME` | Java | Maven Central username |
| `MAVEN_PASSWORD` | Java | Maven Central deployment token |
| `GPG_KEY_ID` | Java | GPG key ID for artifact signing |
| `GPG_PASSPHRASE` | Java | GPG key passphrase |
| `GPG_PRIVATE_KEY` | Java (CI) | Base64-encoded armored GPG private key |

---

## CLI reference

```
buffy [OPTIONS] [COMMAND]
```

**Commands**

| Command | Description |
|---------|-------------|
| `init <NAME> [--path <DIR>]` | Create a new `Buffy.toml` and example proto file |
| `check` | Validate `Buffy.toml` and verify all required tools are installed |
| `build` | Generate stubs for all configured languages |
| `publish` | Publish generated stubs to configured registries |

**Options**

| Option | Description |
|--------|-------------|
| `--version <SEMVER>` | Override the version from `Buffy.toml` at runtime |
| `--publish` | Also publish after building |

**Examples**

```bash
# Initialize a new project
buffy init myservice
buffy init myservice --path ./projects/myservice

# Validate setup
buffy check

# Build only
buffy build

# Build and publish
buffy --publish

# Override version (useful in CI)
buffy --version 1.2.3 --publish
```

---

## CI / GitHub Actions

buffy is designed to work well in CI. The `--version` flag lets you drive the version from your Git tag without modifying `Buffy.toml` (which stays at `0.0.0` in the repository).

```yaml
name: Release
on:
  push:
    tags: ["v[0-9]*.[0-9]*.[0-9]*"]

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Extract version from tag
        id: version
        run: echo "VERSION=${GITHUB_REF_NAME#v}" >> $GITHUB_OUTPUT

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install buffy
        run: cargo install --git https://github.com/julian-siebert/buffy

      - name: Build and publish stubs
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
          MAVEN_USERNAME: ${{ secrets.MAVEN_USERNAME }}
          MAVEN_PASSWORD: ${{ secrets.MAVEN_PASSWORD }}
          GPG_KEY_ID: ${{ secrets.GPG_KEY_ID }}
          GPG_PASSPHRASE: ${{ secrets.GPG_PASSPHRASE }}
          GPG_PRIVATE_KEY: ${{ secrets.GPG_PRIVATE_KEY }}
        run: buffy --version ${{ steps.version.outputs.VERSION }} --publish
```

---

## Future languages

buffy is designed to be extended with additional language targets. Planned support includes:

- **TypeScript / JavaScript** (npm)
- **Python** (PyPI)
- **C#** (.NET / NuGet)
- **Swift** (Swift Package Manager)
- **Kotlin** (Maven Central)
- **PHP** (Packagist)

Contributions are welcome.

---

## License

Apache License 2.0
