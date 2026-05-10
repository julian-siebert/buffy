# Frequently Asked Questions

## How is Buffy different from `buf`?

[`buf`] is a feature-rich linter, formatter, and code generator that
orchestrates `protoc` plugins. Buffy is a build and publishing tool: it cares
less about lint rules and breaking-change detection, and more about turning a
set of `.proto` files into a publishable Go module, Cargo crate, npm package,
or Maven artifact in one command. The two tools are complementary! You can
run `buf lint` and `buf breaking` in CI alongside `buffy --publish`.

## Why generate per-language packages instead of using a Buf Schema Registry?

The Buf Schema Registry is an excellent option if you control both the
producer and the consumer side and want strong centralization. Buffy is for
teams that need protocol buffer libraries to land in the *native* package
ecosystem of each language; so consumers depend on a normal Cargo crate or
Maven artifact and don't have to learn anything new.

## Can I add a new language target?

Not as configuration. Language targets are part of Buffy's source code. The
existing targets share a common skeleton (a profile, a code generator, a
`build` step, a `publish` step), so adding one is straightforward. See the
existing modules under `targets/` in the Buffy repository for templates.

## Why does the `--publish` step push with `--force`?

Buffy treats the published artifact's repository (or the registry version) as
a derived output: every release is rebuilt from scratch. Hand-edits inside a
generated repository would be overwritten on the next publish, so Buffy
force-pushes to make the destination an exact mirror of the generated content.
Files listed in the `keep` field of `git` profiles are preserved across
publishes.

## A profile failed but the others succeeded - what happens?

Buffy continues building the remaining profiles and reports a combined
diagnostic at the end, listing every profile that succeeded and every profile
that failed with its underlying error. The successful profiles are *not*
rolled back; if the failure occurred during `--publish`, the artifacts that
made it out are already live.

## Can I use Buffy with private registries?

Yes:

* **Cargo** --- Configure the registry index in `~/.cargo/config.toml` and
  set `registry = "<name>"` in the profile.
* **npm** --- Set the `registry` field in the profile to your registry's URL
  (e.g., a Verdaccio instance, GitHub Packages, or AWS CodeArtifact).
* **Maven** --- The `maven_central` variant is hard-wired to the Sonatype
  Central Portal. For private Maven repositories (Nexus, Artifactory), use
  the `git` variant for now and consume from the resulting repository, or
  open an issue to discuss adding a generic `maven` variant.

## Why doesn't `buffy check` need network access?

`buffy check` only verifies that the local toolchain is installed. It does
not invoke `protoc`, query crates.io for versions, or talk to any registry.
This makes it suitable as a fast pre-commit hook or CI gate.

## How do I pin dependency versions in generated artifacts?

Each language's profile has optional fields for pinning runtime versions:
`protobuf_version` for Java/Kotlin, `prost_version` and `tonic_version` for
Rust, etc. When omitted, Buffy queries the language's registry for the latest
release version.

## What happens if `Buffy.toml` and a profile both define the same field?

Profiles override the manifest where they overlap. For example, the
package-level `homepage` is the default, but the JavaScript profile's
`homepage` field can override it for the JavaScript package only. Most
profile fields don't overlap with the manifest; they configure
language-specific concerns.

[`buf`]: https://buf.build/
