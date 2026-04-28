<p align="center">
  <img src="docs/theme/favicon.svg" alt="buffy" width="180">
</p>

<h1 align="center">buffy</h1>

<p align="center">
  <em>A cute Protobuf manager (in alpha)</em>
</p>

<div align="center">

<p align="center">
  <a href="https://github.com/julian-siebert/buffy/actions/workflows/ci.yml">
    <img src="https://github.com/julian-siebert/buffy/actions/workflows/ci.yml/badge.svg" alt="CI">
  </a>
  <a href="https://github.com/julian-siebert/buffy/releases">
    <img src="https://img.shields.io/github/v/release/julian-siebert/buffy?color=blueviolet" alt="Release">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License">
  </a>
</p>

---

Buffy builds and publishes Protocol Buffer schemas across multiple language ecosystems from a single configuration file. Define your schema once, ship it to crates.io, Maven Central, and Go modules - without juggling five build systems.

## Installation

```sh
curl -sSL https://pkgs.julian-siebert.de/buffy/install.sh | sh
```

For specific versions or other installation methods, see the [documentation](https://doc.julian-siebert.de/buffy/installation.html).

## Use in GitHub Actions

```yaml
- uses: julian-siebert/buffy@v0
- run: buffy build
```

## Documentation

Full documentation is available at [docs.julian-siebert.de/buffy](https://docs.julian-siebert.de/buffy/).

## Support

If buffy is useful to you, you can support its development:

- [GitHub Sponsors](https://github.com/sponsors/julian-siebert) for individuals
- [Open Collective](https://opencollective.com/buffy) for organizations (invoices available)

## License

Buffy is licensed under the [Apache License, Version 2.0](LICENSE).
