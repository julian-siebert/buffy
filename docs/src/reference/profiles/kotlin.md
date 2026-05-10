# Kotlin Profiles

The `kotlin` profile generates a Maven project from your `.proto` files. Buffy
uses `protoc-gen-java` to generate the message classes and packages them
together with the Kotlin standard library so the artifact can be consumed
naturally from Kotlin code. Kotlin can call Java protobuf classes directly, so
no separate Kotlin code generation is required.

Available variants:

* [`maven_central`](#the-maven_central-variant) --- Publish to Maven Central.
* [`git`](#the-git-variant) --- Push the generated Maven project to a Git remote.

## Required tools

* `protoc` --- Protocol Buffers compiler.
* `java` --- A JDK (11 or newer recommended).
* `mvn` --- Apache Maven; the Kotlin compiler is downloaded automatically as a
  Maven plugin, no separate `kotlin` binary is required.
* `gpg` --- GnuPG, for the `maven_central` variant.
* `git` --- For the `git` variant.

## The `maven_central` variant

Generates a Maven project that compiles both Java (from `protoc`) and Kotlin
sources, then publishes the resulting JAR (with sources and Javadoc) to Maven
Central via the Sonatype Central Portal.

### Example

```toml
# .buffy/kotlin-example.toml
[kotlin.maven_central]
group_id = "io.github.example"
artifact_id = "tomato-kotlin"
url = "https://github.com/example/tomato"
auto_publish = false
wait_until = "uploaded"

[kotlin.maven_central.scm]
connection = "scm:git:git://github.com/example/tomato.git"
url = "https://github.com/example/tomato"
```

### Fields

* [`group_id`](#the-group_id-field) --- Maven group ID.
* [`artifact_id`](#the-artifact_id-field) --- Maven artifact ID.
* [`url`](#the-url-field) --- Project URL embedded in the POM.
* [`scm`](#the-scm-section) --- Source-control coordinates.
* [`protobuf_version`](#the-protobuf_version-field) --- Pin `protobuf-java` runtime.
* [`kotlin_version`](#the-kotlin_version-field) --- Pin Kotlin compiler version.
* [`auto_publish`](#the-auto_publish-field) --- Auto-release after upload validates.
* [`wait_until`](#the-wait_until-field) --- How far to wait in the pipeline.

#### The `group_id` field

The Maven `groupId`. Must match a namespace verified for your Sonatype
account.

```toml
group_id = "io.github.example"
```

#### The `artifact_id` field

The Maven `artifactId`.

```toml
artifact_id = "tomato-kotlin"
```

#### The `url` field

The project URL embedded in the POM `<url>` tag.

```toml
url = "https://github.com/example/tomato"
```

#### The `scm` section

```toml
[kotlin.maven_central.scm]
connection = "scm:git:git://github.com/example/tomato.git"
url = "https://github.com/example/tomato"
```

#### The `protobuf_version` field

Optional. Pin a specific version of the `com.google.protobuf:protobuf-java`
runtime dependency. If omitted, Buffy queries Maven Central for the latest
release.

```toml
protobuf_version = "4.29.3"
```

#### The `kotlin_version` field

Optional. Pin a specific version of the Kotlin compiler and standard library.
If omitted, Buffy queries Maven Central for the latest release of
`org.jetbrains.kotlin:kotlin-stdlib`.

```toml
kotlin_version = "2.0.21"
```

#### The `auto_publish` field

See [`auto_publish` in Java profiles](java.md#the-auto_publish-field).

#### The `wait_until` field

See [`wait_until` in Java profiles](java.md#the-wait_until-field).

### Required environment variables

Same as [Java](java.md#required-environment-variables).

### Example consumer usage

```xml
<dependency>
  <groupId>io.github.example</groupId>
  <artifactId>tomato-kotlin</artifactId>
  <version>0.1.0</version>
</dependency>
```

```kotlin
import io.github.example.tomato.GreeterOuterClass.HelloRequest

val req = HelloRequest.newBuilder().setName("World").build()
println(req.name)
```

## The `git` variant

Like the Java `git` variant, but with Kotlin tooling configured in the POM.

### Example

```toml
# .buffy/kotlin.toml
[kotlin.git]
group_id = "com.example"
artifact_id = "tomato-kotlin"
url = "https://github.com/example/tomato-kotlin"
remote = "git@github.com:example/tomato-kotlin.git"
branch = "main"
keep = ["README.md"]

[kotlin.git.scm]
connection = "scm:git:git://github.com/example/tomato-kotlin.git"
url = "https://github.com/example/tomato-kotlin"
```

### Fields

The same fields as `maven_central`, plus:

* [`remote`](java.md#the-remote-field) --- Git URL the artifact is pushed to.
* [`branch`](java.md#the-branch-field) --- Branch to push to.
* [`keep`](java.md#the-keep-field) --- Files to preserve from the remote.

These behave identically to the same-named fields in [Java profiles](java.md).
