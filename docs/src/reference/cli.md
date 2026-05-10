# Command-Line Interface

The `buffy` CLI reads `Buffy.toml` and the profiles in `.buffy/`, builds all
configured language targets in parallel, and (with `--publish`) pushes them to
their respective registries or remotes.

## Synopsis

```
buffy [OPTIONS] [COMMAND]
```

When invoked without a subcommand, Buffy builds every profile in `.buffy/`. If
the `.buffy/` directory does not exist or contains no profiles, Buffy exits
with a notice and no error.

## Commands

* [`buffy`](#buffy) --- Build all configured profiles.
* [`buffy check`](#buffy-check) --- Verify that required tools are installed.

### `buffy`

Build (and optionally publish) every profile.

```sh
buffy
```

Each profile is built independently and in parallel. If any profile fails,
Buffy continues building the others and reports a combined diagnostic at the
end, listing every profile that succeeded and every profile that failed with
its underlying error.

With `--publish`, every profile that built successfully is then published to
its configured destination.

### `buffy check`

Verify that all required external tools are installed and reachable on the
`PATH` for the configured profiles. No code is generated and nothing is built.

```sh
buffy check
```

The check is profile-aware: it only verifies the tools needed by the
configured profiles. For instance, a project with only a `rust` profile does
not require `mvn` to be installed.

If a tool is missing, Buffy emits a diagnostic with the tool name and an
installation hint specific to common platforms. See the per-language profile
chapter for the full list of tools each variant requires.

## Options

### `-p, --publish`

Publish every profile after a successful build.

```sh
buffy --publish
```

Each profile is published to the destination configured in its
`.buffy/<name>.toml` file:

* `crate` for Rust → the configured Cargo registry (defaulting to crates.io).
* `npm` for JavaScript / TypeScript → the configured npm registry.
* `maven_central` for Java / Kotlin → the Sonatype Central Portal.
* `git` for any language → the configured Git remote, with a `v<version>` tag.

If a required environment variable is missing, Buffy reports a diagnostic
with the variable name and a description of how to obtain it. See
[Environment Variables](environment-variables.md).

### `--publish-version <VERSION>`

Override the `version` from `Buffy.toml` for this run.

```sh
buffy --publish --publish-version 1.2.3
```

The value must be a valid [SemVer] version. This is intended for CI pipelines
that derive the release version from a Git tag rather than committing it into
`Buffy.toml`.

The override applies only to the current invocation; the file on disk is not
modified.

### `--verbose`

Print the full output (both `stdout` and `stderr`) of every external tool
Buffy invokes.

```sh
buffy --verbose
```

By default, Buffy streams tool output to your terminal as it arrives, prefixed
with the profile name and the program being run. With `--verbose`, the
unfiltered streams are shown, which is useful when debugging:

* Maven build failures, where the relevant `[ERROR]` lines are mixed in with a
  long `[INFO]` log on `stdout`.
* npm install issues that print resolution diagnostics on `stdout`.
* Cargo warnings about deprecated APIs.

Without `--verbose`, Buffy still surfaces the relevant error context when a
command fails.

### `-h, --help`

Print a short help message.

### `-V, --version`

Print the Buffy version.

## Exit codes

* `0` --- All profiles built (and, with `--publish`, published) successfully.
* `1` --- One or more profiles failed. The diagnostic at the end of the run
  lists every failure.

## Examples

Build all profiles:

```sh
buffy
```

Verify the toolchain without building:

```sh
buffy check
```

Build and publish using the version from `Buffy.toml`:

```sh
buffy --publish
```

Override the version (typical CI usage):

```sh
buffy --publish --publish-version "${GITHUB_REF_NAME#v}"
```

Get verbose output for debugging:

```sh
buffy --publish --verbose
```

[SemVer]: https://semver.org/
