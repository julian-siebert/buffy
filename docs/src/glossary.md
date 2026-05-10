# Glossary

### artifact

The build output of a single profile - a Go module, Cargo crate, Maven JAR,
or npm package, produced under `target/<profile-name>/`. With `--publish`,
the artifact is the thing that gets uploaded to a registry or pushed to a
Git remote.

### manifest

The `Buffy.toml` file at the root of a Buffy project. Contains the
language-independent metadata (package name, version, license, authors).
See [The Manifest Format](reference/manifest.md).

### package

The unit of code Buffy operates on: a directory containing a `Buffy.toml`,
a directory of `.proto` files (the *source*), and zero or more profiles
under `.buffy/`. A package produces one artifact per profile.

### profile

A language-and-variant configuration in the `.buffy/` directory. Each profile
is a TOML file selecting one language (e.g., `rust`, `java`) and one publishing
variant (e.g., `crate`, `maven_central`, `git`). The filename without `.toml`
becomes the *profile name*. See [Profiles Format](reference/profiles.md).

### profile name

The filename of a profile in `.buffy/` without the `.toml` extension. Profile
names appear as the subdirectory under `target/` and as the prefix in build
output. They must be unique within a project.

### registry

A central host that distributes packages for a language. Examples:
[crates.io] for Rust, [npm](https://npmjs.org) for JavaScript and TypeScript,
[Maven Central] for Java and Kotlin. Buffy publishes to a registry via the
language's native publishing variant (`crate`, `npm`, `maven_central`).

### source

The directory of `.proto` files configured under the `[source]` section of
`Buffy.toml`. Buffy walks this directory recursively and passes every
`.proto` file it finds to `protoc`. Defaults to `src` if the section is
omitted.

### target

The Buffy term for one published output (a Cargo crate, a Maven artifact,
an npm package). Corresponds to one profile. Not to be confused with the
`target/` directory, which holds the generated content for every target.

### variant

The publishing destination for a profile, selected as the second part of the
nested table name `[<language>.<variant>]`. Each language defines which
variants it supports. For example, `rust` supports `crate` and `git`, while
`golang` supports only `git`. See the per-language profile chapters.

[crates.io]: https://crates.io/
[Maven Central]: https://central.sonatype.com/
