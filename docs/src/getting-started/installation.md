# Installation

Buffy is distributed as a single binary. Install it with:

```sh
curl -sSL https://pkgs.julian-siebert.de/buffy/install.sh | sh
```

The installer places the `buffy` binary in `~/.local/bin/buffy` and prints
instructions if that directory is not already on your `PATH`.

Verify the installation:

```sh
buffy --version
```

## External tools

Buffy generates code by invoking language-specific tools. You only need the
tools for the languages you actually target — Buffy reports a clear diagnostic
if a tool is missing when you build.

A complete list per language is documented in each profile chapter under the
[Buffy Reference](../reference/profiles.md). The most common tools:

* `protoc` --- The Protocol Buffers compiler. Required by every language.
* `git` --- Required by every `git` profile variant and by Go modules.
* Per language: `go`, `cargo`, `mvn`, `npm`, plus their respective protobuf
  plugins.

To verify the toolchain for a project after configuring it, run:

```sh
buffy check
```

## Updating

Re-running the installer updates Buffy to the latest version:

```sh
curl -sSL https://pkgs.julian-siebert.de/buffy/install.sh | sh
```

## Uninstalling

Remove the binary:

```sh
rm ~/.local/bin/buffy
```
