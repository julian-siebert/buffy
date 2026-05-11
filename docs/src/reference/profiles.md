# Profiles Format

Profiles configure language-specific publishing targets. They live in the
`.buffy/` directory next to `Buffy.toml`, with one TOML file per profile.

```
my-project/
├── Buffy.toml
├── proto/
│   └── greeter.proto
└── .buffy/
    ├── golang-github.toml
    ├── java.toml
    ├── kotlin.toml
    ├── rust-crates-io.toml
    ├── js.toml
    └── typescript.toml
```

The filename (without `.toml`) becomes the *profile name*. The profile name
is used as the directory under `target/` where build artifacts are written
(e.g., `target/golang-github/`, `target/rust-crates-io/`) and as the prefix in progress
output.

## Profile naming

Profile names must be unique within a project. Buffy reports an error if two
files in `.buffy/` resolve to the same name (e.g., on case-insensitive
filesystems where `Rust.toml` and `rust.toml` collide).

The filename has no relationship to the *language* the profile configures —
the language is selected by the table syntax inside the file. A profile
called `internal-go.toml` can configure a `golang` target, and a project may
contain multiple profiles for the same language under different names (for
example, one for an internal Git remote and another for a public registry).

## File structure

Each profile file selects exactly one language and exactly one publishing
variant, using nested-table syntax:

```toml
[<language>.<variant>]
# fields specific to the language and variant
```

For example, a Rust profile that publishes to crates.io:

```toml
[rust.crate]
name = "tomato"
edition = "2021"
repository = "https://github.com/example/tomato"
documentation = "https://docs.rs/tomato"
registry = "crates-io"
grpc = true
```

A Go profile that publishes via Git:

```toml
[golang.git]
module = "github.com/example/tomato-go"
remote = "git@github.com:example/tomato-go.git"
branch = "main"
grpc = true
keep = ["README.md"]
```

A profile file must contain exactly one `[<language>.<variant>]` table.
Multiple variants in the same file are not supported; use multiple profile
files instead.

## Available languages and variants

| Language                              | Variants                  | Default destination                        |
|---------------------------------------|---------------------------|--------------------------------------------|
| [`golang`](profiles/golang.md)        | `git`                     | A Git remote (Go modules use Git tags)     |
| [`java`](profiles/java.md)            | `maven_central`, `git`    | Sonatype Central Portal                    |
| [`kotlin`](profiles/kotlin.md)        | `maven_central`, `git`    | Sonatype Central Portal                    |
| [`rust`](profiles/rust.md)            | `crate`, `git`            | crates.io or another Cargo registry        |
| [`javascript`](profiles/javascript.md)| `npm`, `git`              | npmjs.org or any npm-compatible registry   |
| [`typescript`](profiles/typescript.md)| `npm`, `git`              | npmjs.org or any npm-compatible registry   |
| [`python`](profiles/python.md)        | `pypi`, `git`             | pypi.org                                   |

Each language is documented in its own chapter, with the full list of fields
per variant.

## The `git` variant

Every language supports a `git` variant. Instead of publishing to a registry,
Buffy commits the generated artifact to a Git repository and tags the commit
with `v<version>`. Consumers depend on the package by its Git URL.

Common fields across all `git` variants:

* `remote` --- The Git URL the artifact is pushed to. SSH URLs are
  recommended; Buffy disables Git's terminal prompt during operations, so
  HTTPS URLs work only if credentials are pre-cached.
* `branch` --- The branch to push to. The previous content is replaced
  (force-push), with the exception of files listed in `keep`.
* `keep` --- A list of paths to fetch from the remote before committing,
  preserving them across publishes. Useful for human-maintained files such
  as `README.md` that should not be overwritten by code generation.

The exact list of fields varies by language because the build artifact
itself is language-specific (a `Cargo.toml`, a Maven POM, a `package.json`,
etc.) and those fields are part of the profile.

## Build output

Each profile is built into its own directory under `target/`:

```
target/
├── golang/        # output of the `golang.toml` profile
├── java/          # output of the `java.toml` profile
└── ...
```

The directory is cleared at the start of each build, so all generated content
reflects the current run. Buffy automatically adds `target/` to a `.gitignore`
at the repository root.

## Parallelism

All profiles are built in parallel. With `--publish`, the publish step is
also parallel. If any profile fails, Buffy continues with the others and
reports a combined diagnostic at the end of the run, listing every profile
that succeeded and every profile that failed with its underlying error.

## Environment variables

Most variants require credentials to publish, supplied via environment
variables. See [Environment Variables](environment-variables.md) for the
complete list.
