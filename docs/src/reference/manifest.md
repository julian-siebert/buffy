# The Manifest Format

The `Buffy.toml` file for each package is called its *manifest*. It is written
in the [TOML] format. It contains metadata that is needed to compile and publish
the protocol buffers to all supported language's targets.

Every manifest file consists of the following sections:

* [`[package]`](#the-package-section) --- Defines a package.

## The `[package]` section

The first section in a `Buffy.toml` is `[package]`.

```toml
[package]
name = "tomato" # the name of the package
version = "0.1.0" # the current version, obeying semver
```

### The `name` field

The package name is an identifier used to refer to the package.

The name must use only [alphanumeric] characters or `-` or `_`, and cannot be empty.

- Only ASCII characters are allowed.
- Use a maximum of 32 characters of length.
