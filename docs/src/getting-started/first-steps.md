# First Steps with Buffy

This walkthrough creates a small Buffy project, defines a `.proto` schema, and
generates libraries for two languages - Go and Rust - using the `git` variant
so no registry accounts are needed.

## Create a project

Create a directory for your project and add the standard layout:

```sh
mkdir tomato && cd tomato
mkdir proto .buffy
```

## Define the manifest

Create a `Buffy.toml` at the project root:

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

See [The Manifest Format](../reference/manifest.md) for all available fields.

## Add a `.proto` file

Create `proto/greeter.proto`:

```proto
syntax = "proto3";

package greeter;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

## Configure profiles

Create one profile file per language target.

`.buffy/golang.toml`:

```toml
[golang.git]
module = "github.com/example/tomato-go"
remote = "git@github.com:example/tomato-go.git"
branch = "main"
grpc = true
```

`.buffy/rust.toml`:

```toml
[rust.git]
name = "tomato"
edition = "2021"
remote = "git@github.com:example/tomato-rs.git"
branch = "main"
repository = "https://github.com/example/tomato-rs"
documentation = "https://github.com/example/tomato-rs"
grpc = true
```

The filename (e.g., `golang.toml`) becomes the *profile name* and shows up in
build output. See [Profiles Format](../reference/profiles.md) for the full
list of options per language.

## Verify the toolchain

Before building, check that all required tools are installed:

```sh
buffy check
```

If anything is missing, Buffy prints an installation hint specific to your
platform.

## Build

Run Buffy without arguments to build every profile in parallel:

```sh
buffy
```

Each profile produces a complete, self-contained package under `target/`:
