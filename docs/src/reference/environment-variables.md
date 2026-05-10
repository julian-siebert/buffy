# Environment Variables

Buffy reads credentials and a few configuration values from environment
variables rather than from `Buffy.toml` or the profile files. This keeps
secrets out of source control and matches the conventions of the underlying
tooling (Cargo, Maven, npm).

For local development, Buffy auto-loads a `.env` file from the current
directory via [`dotenvy`]. For CI, set the variables through your provider's
secret store (GitHub Actions secrets, GitLab CI variables, etc.).

## Quoting in `.env` files

`dotenvy` follows shell-like rules. Values containing `$`, `#`, whitespace, or
other special characters should be wrapped in **single quotes** to disable
variable expansion and escaping:

```env
GPG_PASSPHRASE='HD$qOdHYiG#jGUCpNJhzSSx5W'
NPM_TOKEN='npm_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
```

Double-quoted values allow `$VAR` expansion, which can silently corrupt
secrets that contain a `$`.

## Variables by language

### Java and Kotlin

Used by the `maven_central` variant; not required for the `git` variant.

| Variable             | Required | Purpose                                                         |
|----------------------|----------|-----------------------------------------------------------------|
| `MAVEN_USERNAME`     | yes      | Sonatype Central Portal username token                          |
| `MAVEN_PASSWORD`     | yes      | Sonatype Central Portal password token                          |
| `GPG_KEY_ID`         | yes      | GPG key ID used for signing artifacts                           |
| `GPG_PASSPHRASE`     | optional | Passphrase for the GPG key (omit if the key has no passphrase)  |
| `GPG_PRIVATE_KEY`    | optional | Armored private key, imported on each run (intended for CI)     |

The `MAVEN_USERNAME` / `MAVEN_PASSWORD` pair is generated as a token from your
account at [central.sonatype.com](https://central.sonatype.com) under
**Profile → User Token**.

`GPG_PRIVATE_KEY` is intended for CI runners that don't have a persistent
keyring. When set, Buffy writes the key to a temporary file, runs
`gpg --import`, and removes the file. Do **not** set this on machines that
already have the key in their local keyring.

### Rust

Used by the `crate` variant; not required for the `git` variant.

| Variable               | Required | Purpose                                          |
|------------------------|----------|--------------------------------------------------|
| `CARGO_REGISTRY_TOKEN` | yes      | API token for the configured Cargo registry      |

For [crates.io], generate the token under **Account → API Tokens**. For
self-hosted registries (e.g. [Kellnr]), use the token-issuing mechanism of the
registry. The registry index URL must additionally be configured in
`~/.cargo/config.toml`:

```toml
[registries]
my-registry = { index = "sparse+https://my-kellnr.example.com/api/v1/crates/" }
```

### JavaScript and TypeScript

Used by the `npm` variant; not required for the `git` variant.

| Variable    | Required | Purpose                                |
|-------------|----------|----------------------------------------|
| `NPM_TOKEN` | yes      | Auth token for the configured registry |

For the public npm registry, generate the token via `npm token create` or in
the npmjs.com web UI. For other registries (Verdaccio, GitHub Packages, etc.),
use the registry's token-issuing mechanism.

Buffy writes a temporary `.npmrc` to the build directory containing the token
and the configured registry URL, then removes it after publishing. Existing
`.npmrc` files in your home directory are not used.

### Golang

The `golang` profile only supports a `git` variant, which uses Git itself for
authentication. Configure your SSH agent or HTTPS credentials via Git as
usual; Buffy does not read any environment variables for this profile.

## Variables by use case

### Local development

The minimal `.env` to publish from a developer machine across all targets:

```env
# Java / Kotlin (Maven Central)
MAVEN_USERNAME='your-sonatype-username'
MAVEN_PASSWORD='your-sonatype-token'
GPG_KEY_ID='YOURGPGKEYID0123456789ABCDEF'
GPG_PASSPHRASE='your-gpg-passphrase'

# Rust (crates.io or other Cargo registry)
CARGO_REGISTRY_TOKEN='your-cargo-token'

# JavaScript / TypeScript (npm)
NPM_TOKEN='your-npm-token'
```

For Git-only profiles, no environment variables are required — Git uses your
SSH agent.

### CI

In CI, additionally set `GPG_PRIVATE_KEY` so that the runner can import the
signing key on each run:

```env
GPG_PRIVATE_KEY='-----BEGIN PGP PRIVATE KEY BLOCK-----
...
-----END PGP PRIVATE KEY BLOCK-----'
```

CI secret stores typically support multiline values directly. If yours does
not, base64-encode the key and decode it in your pipeline before exposing it
as `GPG_PRIVATE_KEY`.

## Behavior

When a required variable is missing or set to an empty string, Buffy emits a
diagnostic identifying the variable, the profile that requires it, and a
short hint on how to obtain it. The build does not proceed past this point.

Variables are only consulted at the moment they are needed. For instance,
`buffy check` does not require any of these to be set, since it does not
publish anything.

[`dotenvy`]: https://crates.io/crates/dotenvy
[crates.io]: https://crates.io/
[Kellnr]: https://kellnr.io/
