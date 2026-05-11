# Python Profiles

The `python` profile generates a Python package from your `.proto` files using
the `grpc_tools.protoc` plugin (which is the official Python code generator,
distributed as part of `grpcio-tools`). The package is published either to
PyPI (or any PyPI-compatible registry) or to a Git repository.

Available variants:

* [`pypi`](#the-pypi-variant) --- Publish to PyPI or another PEP 503 registry.
* [`git`](#the-git-variant) --- Push the generated package to a Git remote.

## Required tools

* `protoc` --- Protocol Buffers compiler (used via `grpcio-tools`).
* `python3` --- Python 3.9 or newer.
* `grpcio-tools` --- Python module providing the protoc-based code generator.
* `build` --- PEP 517 build module, used to produce sdist and wheel.
* `twine` --- PyPI upload tool (only for the `pypi` variant).
* `git` --- For the `git` variant.

Install the Python tooling:

```sh
pip install grpcio-tools build twine
```

## The `pypi` variant

Generates a Python package under `target/<profile>/`, runs `python -m build`
to produce sdist and wheel under `dist/`, then uploads them with `twine`.

### Example

```toml
# .buffy/python.toml
[python.pypi]
name = "tomato-proto"
repository_url = "https://upload.pypi.org/legacy/"
repository = "https://github.com/example/tomato"
grpc = true
```

### Fields

* `name` --- PyPI package name. Hyphens are converted to underscores for the
  importable module name.
* `repository_url` --- The PyPI-compatible upload endpoint. Default is the
  public PyPI; use `https://test.pypi.org/legacy/` for testing.
* `repository` --- URL to the source repository.
* `homepage` --- Optional override of the global homepage.
* `grpc` --- When `true`, generates gRPC service stubs via
  `--grpc_python_out=...`.
* `protobuf_version` --- Optional pin for the `protobuf` runtime dependency.
* `grpcio_version` --- Optional pin for the `grpcio` runtime dependency.

### Required environment variables

| Variable     | Purpose                              |
|--------------|--------------------------------------|
| `PYPI_TOKEN` | API token for the configured PyPI    |

Tokens are created at https://pypi.org/manage/account/token/ (or the
equivalent path on a private registry). Include the `pypi-` prefix.

### Example consumer usage

```sh
pip install tomato-proto
```

```python
from tomato_proto import greeter_pb2

req = greeter_pb2.HelloRequest(name="World")
print(req.name)
```

## The `git` variant

Generates the same package as `pypi`, but commits and tags it to a Git
repository.

### Example

```toml
[python.git]
name = "tomato-proto"
remote = "git@github.com:example/tomato-py.git"
branch = "main"
repository = "https://github.com/example/tomato-py"
grpc = true
keep = ["README.md"]
```

### Example consumer usage

```sh
pip install git+https://github.com/example/tomato-py@v0.1.0
```
