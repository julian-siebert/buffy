# Java Profiles

The `java` profile generates a Maven project from your `.proto` files using
[`protoc-gen-java`] and publishes it either to a Git repository or to
[Maven Central] via the [Sonatype Central Portal]. Consumers depend on the
artifact through a normal Maven (or Gradle) coordinate.

Available variants:

* [`maven_central`](#the-maven_central-variant) --- Publish to Maven Central.
* [`git`](#the-git-variant) --- Push the generated Maven project to a Git remote.

## Required tools

* `protoc` --- Protocol Buffers compiler.
* `java` --- A JDK (11 or newer recommended), used by Maven.
* `mvn` --- Apache Maven, used to compile and (for `maven_central`) deploy.
* `gpg` --- GnuPG, used to sign artifacts. Required for the `maven_central`
  variant; not used by the `git` variant.
* `git` --- Required only for the `git` variant.

`buffy check` verifies that the relevant tools are installed for the configured
variant.

## The `maven_central` variant

Generates a Maven project under `target/<profile>/`, runs `mvn compile` to
verify, signs the artifacts with GPG, and uploads them to the Sonatype Central
Portal via the [`central-publishing-maven-plugin`].

The Sonatype namespace must be verified for your account before the first
publish (e.g., `io.github.<your-github-user>` is verified by creating a
public repository named after a verification code Sonatype gives you).

### Example

```toml
# .buffy/java-example.toml
[java.maven_central]
group_id = "io.github.example"
artifact_id = "tomato"
url = "https://github.com/example/tomato"
auto_publish = false
wait_until = "uploaded"

[java.maven_central.scm]
connection = "scm:git:git://github.com/example/tomato.git"
url = "https://github.com/example/tomato"
```

### Fields

* [`group_id`](#the-group_id-field) --- Maven group ID.
* [`artifact_id`](#the-artifact_id-field) --- Maven artifact ID.
* [`url`](#the-url-field) --- Project URL embedded in the POM.
* [`scm`](#the-scm-section) --- Source-control coordinates required by Maven Central.
* [`protobuf_version`](#the-protobuf_version-field) --- Pin the `protobuf-java` runtime version.
* [`auto_publish`](#the-auto_publish-field) --- Auto-release after upload validates.
* [`wait_until`](#the-wait_until-field) --- How far to wait in the publishing pipeline.

#### The `group_id` field

The Maven `groupId` of the published artifact. Must match a namespace
verified for your Sonatype account.

```toml
group_id = "io.github.example"
```

#### The `artifact_id` field

The Maven `artifactId` of the published artifact.

```toml
artifact_id = "tomato"
```

#### The `url` field

A project URL written to the `<url>` tag in the generated POM. Maven Central
requires this field to be present.

```toml
url = "https://github.com/example/tomato"
```

#### The `scm` section

Source-control coordinates required by Maven Central. Embedded in the
`<scm>` block of the POM.

```toml
[java.maven_central.scm]
connection = "scm:git:git://github.com/example/tomato.git"
url = "https://github.com/example/tomato"
```

* `connection` --- The SCM connection string, conventionally
  `scm:git:<git-url>`.
* `url` --- A browsable URL for the source repository.

#### The `protobuf_version` field

Optional. Pin a specific version of the `com.google.protobuf:protobuf-java`
runtime dependency. If omitted, Buffy queries Maven Central for the latest
release version.

```toml
protobuf_version = "4.29.3"
```

#### The `auto_publish` field

When `true`, the Sonatype Central Portal automatically releases the artifact
after validation succeeds. When `false`, the artifact lands in the "Validated"
state in the portal and must be released manually.

```toml
auto_publish = false
```

Default: `false`. For first releases, leave this off so you can manually
verify the upload.

#### The `wait_until` field

Controls how long the publish step waits before returning. One of:

| Value       | Waits for                                  | Typical duration |
|-------------|--------------------------------------------|------------------|
| `uploaded`  | Upload to Sonatype completes               | seconds          |
| `validated` | Sonatype validation (schema, GPG, etc.)    | 1–3 minutes      |
| `published` | Indexing into Maven Central completes      | 10–30 minutes    |

```toml
wait_until = "uploaded"
```

Default: `uploaded`.

### Required environment variables

| Variable          | Purpose                                       |
|-------------------|-----------------------------------------------|
| `MAVEN_USERNAME`  | Sonatype Central Portal username token        |
| `MAVEN_PASSWORD`  | Sonatype Central Portal password token        |
| `GPG_KEY_ID`      | GPG key ID used to sign the artifacts         |
| `GPG_PASSPHRASE`  | Passphrase for the GPG key                    |
| `GPG_PRIVATE_KEY` | Optional: armored private key, for CI runners |

See [Environment Variables](../environment-variables.md) for details.

### Example consumer usage

```xml
<dependency>
  <groupId>io.github.example</groupId>
  <artifactId>tomato</artifactId>
  <version>0.1.0</version>
</dependency>
```

## The `git` variant

Generates the same Maven project as `maven_central`, but commits and tags it
to a Git repository instead of uploading. No GPG signing, no Sonatype account.
Useful for internal sharing or quick prototypes.

### Example

```toml
# .buffy/java.toml
[java.git]
group_id = "com.example"
artifact_id = "tomato"
url = "https://github.com/example/tomato-java"
remote = "git@github.com:example/tomato-java.git"
branch = "main"
keep = ["README.md"]

[java.git.scm]
connection = "scm:git:git://github.com/example/tomato-java.git"
url = "https://github.com/example/tomato-java"
```

### Fields

* [`group_id`](#the-group_id-field) --- as in `maven_central`.
* [`artifact_id`](#the-artifact_id-field) --- as in `maven_central`.
* [`url`](#the-url-field) --- as in `maven_central`.
* [`scm`](#the-scm-section) --- as in `maven_central`.
* [`protobuf_version`](#the-protobuf_version-field) --- as in `maven_central`.
* [`remote`](#the-remote-field) --- Git URL the artifact is pushed to.
* [`branch`](#the-branch-field) --- Branch to push to.
* [`keep`](#the-keep-field) --- Files to preserve from the remote.

#### The `remote` field

The Git URL the generated Maven project is pushed to. SSH URLs are
recommended because Buffy disables Git's terminal prompt.

```toml
remote = "git@github.com:example/tomato-java.git"
```

#### The `branch` field

The branch to push to. Buffy force-pushes on every publish.

```toml
branch = "main"
```

#### The `keep` field

A list of file paths (relative to the repository root) to preserve across
publishes by checking them out from the remote before committing.

```toml
keep = ["README.md"]
```

Default: `[]`.

### Example consumer usage

Maven projects can depend on a Git source via [JitPack] or by cloning and
running `mvn install` locally. Example with JitPack:

```xml
<repositories>
  <repository>
    <id>jitpack.io</id>
    <url>https://jitpack.io</url>
  </repository>
</repositories>

<dependency>
  <groupId>com.github.example</groupId>
  <artifactId>tomato-java</artifactId>
  <version>v0.1.0</version>
</dependency>
```

[`protoc-gen-java`]: https://protobuf.dev/reference/java/
[Maven Central]: https://central.sonatype.com/
[Sonatype Central Portal]: https://central.sonatype.com/
[`central-publishing-maven-plugin`]: https://central.sonatype.org/publish/publish-portal-maven/
[JitPack]: https://jitpack.io/
