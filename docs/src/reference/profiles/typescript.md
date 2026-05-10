# TypeScript Profiles

The `typescript` profile generates a TypeScript library from your `.proto`
files using [`ts-proto`] and compiles it with `tsc` before publishing. The
package contains both compiled JavaScript and `.d.ts` declarations.

Available variants:

* [`npm`](#the-npm-variant) --- Publish to npm or another npm-compatible registry.
* [`git`](#the-git-variant) --- Push the generated package to a Git remote.

## Required tools

* `protoc` --- Protocol Buffers compiler.
* `protoc-gen-ts_proto` --- TypeScript generator from [`ts-proto`].
* `node` --- Node.js runtime.
* `npm` --- npm CLI, used for install, build, and publish.
* `tsc` --- TypeScript compiler.
* `git` --- Required only for the `git` variant.

Install the TypeScript tooling globally:

```sh
npm install -g ts-proto typescript
```

## The `npm` variant

Generates a TypeScript package under `target/<profile>/`, runs `npm install`
to fetch dependencies, runs `tsc` via `npm run build` to produce `dist/`, then
publishes via `npm publish`.

### Example

```toml
# .buffy/ts-example.toml
[typescript.npm]
name = "@example/tomato"
registry = "https://registry.npmjs.org/"
access = "public"
repository = "https://github.com/example/tomato"
grpc = true
```

### Fields

* [`name`](#the-name-field) --- npm package name.
* [`registry`](#the-registry-field) --- Registry URL.
* [`access`](#the-access-field) --- `"public"` or `"restricted"` for scoped packages.
* [`repository`](#the-repository-field) --- Repository URL embedded in the package.
* [`homepage`](#the-homepage-field) --- Override the global homepage.
* [`grpc`](#the-grpc-field) --- Generate gRPC service stubs.

The semantics of these fields are the same as in the [JavaScript `npm`
variant](javascript.md#the-npm-variant). The differences in the generated
output are in the code itself, not the configuration:

* TypeScript output uses ES interfaces and types instead of JavaScript classes.
* gRPC stubs use [`nice-grpc`] (Node.js and browser compatible) instead of
  `grpc-web`.
* The package includes a `tsc` build step before publishing, so consumers
  receive precompiled JavaScript and `.d.ts` files.

### Required environment variables

| Variable    | Purpose                                  |
|-------------|------------------------------------------|
| `NPM_TOKEN` | Auth token for the configured registry   |

### Example consumer usage

```sh
npm install @example/tomato
```

```ts
import { HelloRequest } from "@example/tomato";

const req: HelloRequest = { name: "World" };
console.log(req.name);
```

## The `git` variant

Generates the same TypeScript package as `npm`, but commits and tags it to a
Git repository instead of uploading to a registry.

### Example

```toml
# .buffy/typescript.toml
[typescript.git]
name = "@example/tomato"
remote = "git@github.com:example/tomato-ts.git"
branch = "main"
repository = "https://github.com/example/tomato-ts"
grpc = true
keep = ["README.md"]
```

### Fields

The same fields as `npm` (except `registry` and `access`), plus:

* [`remote`](javascript.md#the-remote-field) --- Git URL the package is pushed to.
* [`branch`](javascript.md#the-branch-field) --- Branch to push to.
* [`keep`](javascript.md#the-keep-field) --- Files to preserve from the remote.

These fields behave identically to the same-named fields in
[JavaScript profiles](javascript.md).

### Example consumer usage

```sh
npm install git+ssh://git@github.com/example/tomato-ts.git#v0.1.0
```

[`ts-proto`]: https://github.com/stephenh/ts-proto
[`nice-grpc`]: https://github.com/deeplay-io/nice-grpc
