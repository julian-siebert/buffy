# Contributing to Buffy

Thanks for your interest in contributing! Buffy is in alpha, so feedback,
bug reports, and small-to-medium contributions are all welcome.

## Ways to help

* **Try it on a real project** and report what was unclear, missing, or
  broken. This is the most valuable feedback at this stage.
* **File issues** for bugs, confusing error messages, or missing features.
* **Improve the documentation** for typos, clarifications, additional examples.
* **Add a new language target** or a new variant for an existing language.
* **Fix bugs** or address open issues.

## Before you start

For anything beyond a small fix, please [open an issue][issues] first to
discuss the change. This avoids duplicate work and gives you a sanity check
on the direction.

If you're not sure whether something is wanted, open an issue and ask. There
are no stupid questions while the project is this young.

## Development setup

Buffy is a Rust project. You need:

* Rust 1.80 or newer (install via [rustup]).
* The external tools relevant to whatever language target you're working on - 
  see the [installation docs][install] for the full list.

Clone the repository:

```sh
git clone https://github.com/julian-siebert/buffy
cd buffy
```

Build and run against the example project in `example/`:

```sh
cargo run -- check
cargo run -- --verbose
```

For a smoother iteration loop, build once in release mode and symlink the
binary somewhere on your `PATH`:

```sh
cargo build --release
ln -s "$(pwd)/target/release/buffy" ~/.local/bin/buffy
```

After this, `cargo build --release` makes the new version available
immediately as `buffy` anywhere.

## Testing your changes

Run the test suite:

```sh
cargo test
```

For changes that touch a language target, also run the target end-to-end
against the example project:

```sh
cd example
buffy check
buffy
```

If your change affects publishing, test with a non-public destination first:

* PyPI → Test PyPI (`https://test.pypi.org/legacy/`)
* npm → a local [Verdaccio] instance
* Cargo → a local [Kellnr] instance
* Java/Kotlin → the `git` variant or a local Maven repository

Maven Central, crates.io, and the public npm registry are **unrevocable** -
do not test against them.

## Code style

* Run `cargo fmt` before committing.
* Run `cargo clippy --all-targets -- -D warnings` and address all warnings.
* Errors should be `Diagnostic` with a clear `help` field where the user can
  do something about the failure - see existing `Error` variants for the
  pattern.
* New external commands should go through `Context::run` (not
  `std::process::Command` directly) so output is captured consistently.
* New filesystem operations should go through `crate::io` (not `std::fs`
  directly) so they produce rich diagnostics on errors.

## Adding a new language target

The targets follow a consistent structure. To add one:

1. **Profile types** - `src/configs/profiles/<language>.rs` with one enum
   per language and one struct per variant. Add it to `Profile` in
   `src/configs/profiles/mod.rs`.

2. **Target module** - `src/targets/<language>/` with:
   * `mod.rs` --- dispatch between variants (`check`, `build`, `publish`).
   * `helpers.rs` --- shared code-generation and template-rendering helpers.
   * one file per variant (e.g. `pypi.rs`, `git.rs`).
   * `templates/` --- Tera templates for the manifest of the target ecosystem
     (`pyproject.toml.tera`, `Cargo.toml.tera`, etc.).

3. **Dependencies** - add diagnostics for any new external tools in
   `src/dependencies.rs`.

4. **Dispatch** - wire the new variant into `src/targets/mod.rs` for
   `check_profile_target`, `build_profile_target`, and
   `publish_profile_target`.

5. **Documentation** - add a `docs/src/reference/profiles/<language>.md`
   chapter following the structure of the existing chapters, and update
   `docs/src/SUMMARY.md`.

6. **Example** - add a profile file under `example/.buffy/` and test
   end-to-end against a real registry's test environment.

The Golang target is the smallest and the best starting point for reading.
Rust and Java are good references when you need to deal with a custom
manifest template and a registry-specific publish step.

## Adding a new variant to an existing language

Smaller scope than a new language. The pattern:

1. Add a variant to the language's enum in `src/configs/profiles/<language>.rs`.
2. Add a new file under `src/targets/<language>/` implementing
   `build_<language>_profile_<variant>_target` and
   `publish_<language>_profile_<variant>_target`.
3. Wire the variant into the language's `mod.rs`.
4. Document the variant in `docs/src/reference/profiles/<language>.md`.

## Documentation

The book lives under `docs/`. To work on it:

```sh
cargo install mdbook
mdbook serve docs
```

Then open <http://localhost:3000>.

Documentation changes don't require a separate PR; include them in the same
PR as the code change they describe.

## Commit messages and PRs

* Keep commits focused. One logical change per commit is ideal; squashing
  before merge is fine if your branch grew messy during review.
* PR descriptions should explain *what* and *why*, not just *what*. If the
  PR addresses an issue, link it.
* Mention manual testing you did, especially for changes that touch
  publishing.

## Releasing

Releases are cut by the maintainer. If you want to propose one, open an
issue with the proposed version bump and a summary of what's changed since
the last release.

## Reporting security issues

Please don't open public issues for security problems. Email the maintainer
directly (see the author info in `Cargo.toml`).

## Code of conduct

Be kind and constructive. Disagree with ideas, not people. We follow the
spirit of the [Rust Code of Conduct][coc].

## Questions?

Open an issue with the `question` label, or start a discussion in the
[GitHub Discussions tab][discussions].

[issues]: https://github.com/julian-siebert/buffy/issues
[install]: https://docs.julian-siebert.de/buffy/getting-started/installation
[rustup]: https://rustup.rs/
[Verdaccio]: https://verdaccio.org/
[Kellnr]: https://kellnr.io/
[coc]: https://www.rust-lang.org/policies/code-of-conduct
[discussions]: https://github.com/julian-siebert/buffy/discussions
