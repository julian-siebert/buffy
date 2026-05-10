# JavaScript Profiles

The `javascript` profile generates a CommonJS JavaScript library from your
`.proto` files using [`protoc-gen-js`] and (optionally) [gRPC-Web] for
browser-compatible gRPC. The package is published either to npm or to a Git
repository.

Available variants:

* [`npm`](#the-npm-variant) --- Publish to npm or another npm-compatible registry.
* [`git`](#the-git-variant) --- Push the generated package to a Git remote.

## Required tools

* `protoc` --- Protocol Buffers compiler.
* `protoc-gen-js` --- JavaScript code generator.
* `protoc-gen-grpc-web` --- gRPC-Web plugin (only when `grpc = true`).
* `node` --- Node.js runtime.
* `npm` --- npm CLI, used to install dependencies and publish.
* `git` --- Required only for the `git` variant.

## The `npm` variant

Generates a JavaScript package under `target/<profile>/`, validates the
`package.json` by running `npm install` and `npm publish --dry-run`, then
publishes via `npm publish`.

### Example

```toml
# .buffy/js-example.toml
[javascript.npm]
name = "@example/tomato"
registry = "https://registry.npmjs.org/"
access = "public"
repository = "https://github.com/example/tomato"
homepage = "https://github.com/example/tomato"
grpc = true
```

### Fields

* [`name`](#the-name-field) --- npm package name.
* [`registry`](#the-registry-field) --- Registry URL.
* [`access`](#the-access-field) --- `"public"` or `"restricted"` for scoped packages.
* [`repository`](#the-repository-field) --- Repository URL embedded in the package.
* [`homepage`](#the-homepage-field) --- Override the global homepage.
* [`grpc`](#the-grpc-field) --- Generate gRPC-Web service stubs.

#### The `name` field

The npm package name. Scoped packages (e.g. `@example/tomato`) are supported.

```toml
name = "@example/tomato"
```

#### The `registry` field

Optional. The npm registry URL to publish to. Examples:

* `"https://registry.npmjs.org/"` --- the public npm registry.
* `"https://npm.pkg.github.com/"` --- GitHub Packages.
* `"http://localhost:4873/"` --- a local Verdaccio instance for testing.

```toml
registry = "https://registry.npmjs.org/"
```

Default: `"https://registry.npmjs.org/"`.

#### The `access` field

Optional. Controls the access level for scoped packages:

* `"public"` --- the package is publicly readable.
* `"restricted"` --- the package requires authentication to read (paid npm
  feature).

For unscoped packages, the value has no effect.

```toml
access = "public"
```

Default: `"public"`.

#### The `repository` field

A URL to the source repository, embedded in `package.json` as the
`repository.url` field.

```toml
repository = "https://github.com/example/tomato"
```

#### The `homepage` field

Optional. Overrides the package's homepage URL. If omitted, the value from
the global `[package]` section's `homepage` field is used.

```toml
homepage = "https://example.com/tomato"
```

#### The `grpc` field

When `true`, Buffy invokes `protoc-gen-grpc-web` to generate browser-compatible
gRPC-Web service stubs and includes `grpc-web` in the package's dependencies.

```toml
grpc = true
```

Default: `false`.

### Required environment variables

| Variable    | Purpose                                  |
|-------------|------------------------------------------|
| `NPM_TOKEN` | Auth token for the configured registry   |

The token can come from `npm token create` (npm) or the corresponding
mechanism for your registry (Verdaccio, GitHub Packages, etc.).

### Example consumer usage

```sh
npm install @example/tomato
```

```js
const { HelloRequest } = require("@example/tomato/greeter_pb");

const req = new HelloRequest();
req.setName("World");
console.log(req.getName());
```

## The `git` variant

Generates the same JavaScript package as `npm`, but commits and tags it to a
Git repository instead of uploading to a registry.

### Example

```toml
# .buffy/javascript.toml
[javascript.git]
name = "@example/tomato"
remote = "git@github.com:example/tomato-js.git"
branch = "main"
repository = "https://github.com/example/tomato-js"
grpc = true
keep = ["README.md"]
```

### Fields

The same fields as `npm` (except `registry` and `access`), plus:

* [`remote`](#the-remote-field) --- Git URL the package is pushed to.
* [`branch`](#the-branch-field) --- Branch to push to.
* [`keep`](#the-keep-field) --- Files to preserve from the remote.

#### The `remote` field

```toml
remote = "git@github.com:example/tomato-js.git"
```

#### The `branch` field

```toml
branch = "main"
```

#### The `keep` field

```toml
keep = ["README.md"]
```

Default: `[]`.

### Example consumer usage

npm packages can be installed directly from a Git source:

```sh
npm install git+ssh://git@github.com/example/tomato-js.git#v0.1.0
```

[`protoc-gen-js`]: https://github.com/protocolbuffers/protobuf-javascript
[gRPC-Web]: https://github.com/grpc/grpc-web
