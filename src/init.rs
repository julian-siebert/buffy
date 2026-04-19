// src/init.rs
use crate::error::{IoResultExt, Result};
use console::style;
use std::path::PathBuf;

const TEMPLATE: &str = r#"[package]
name    = "{name}"
version = "0.1.0"
grpc = true

[source]
path = "./src"

# Set to false if your .proto files don't define any services

# ── Go ────────────────────────────────────────────────────────────────────────
# [golang]
# module = "github.com/your-org/{name}-go"
# remote = "git@github.com:your-org/{name}-go.git"
# branch = "main"

# ── Rust ──────────────────────────────────────────────────────────────────────
# [rust]
# name     = "{name}-proto"
# registry = "crates-io"

# ── Java ──────────────────────────────────────────────────────────────────────
# [java]
# group_id    = "com.your-org"
# artifact_id = "{name}-proto"
# description = "Generated protobuf stubs for {name}"
# url         = "https://github.com/your-org/{name}"
#
# [java.license]
# name = "Apache-2.0"
# url  = "https://www.apache.org/licenses/LICENSE-2.0"
#
# [java.developer]
# id    = "your-bot"
# name  = "Your Name"
# email = "you@your-org.com"
#
# [java.scm]
# connection = "scm:git:git://github.com/your-org/{name}.git"
# url        = "https://github.com/your-org/{name}"
"#;

pub fn init(name: &str, path: Option<PathBuf>) -> Result<()> {
    let dir = path.unwrap_or_else(|| PathBuf::from("."));

    if !dir.exists() {
        std::fs::create_dir_all(&dir).io_err()?;
    }

    let buffy_toml = dir.join("Buffy.toml");

    if buffy_toml.exists() {
        println!(
            "{} Buffy.toml already exists at {}",
            style("[~]").yellow().bold(),
            style(buffy_toml.display()).dim(),
        );
        return Ok(());
    }

    // src/ Verzeichnis mit einer Beispiel-proto Datei anlegen
    let src_dir = dir.join("src");
    std::fs::create_dir_all(&src_dir).io_err()?;

    let proto_path = src_dir.join(format!("{name}.proto"));
    if !proto_path.exists() {
        std::fs::write(
            &proto_path,
            format!(
                r#"syntax = "proto3";

package {name}.v1;

message ExampleRequest {{
    string message = 1;
}}

message ExampleResponse {{
    string message = 1;
}}

service ExampleService {{
    rpc Example(ExampleRequest) returns (ExampleResponse);
}}
"#
            ),
        )
        .io_err()?;
    }

    let content = TEMPLATE.replace("{name}", name);
    std::fs::write(&buffy_toml, content).io_err()?;

    println!(
        "{} {} {}",
        style("[+]").green().bold(),
        style("Initialized").bold(),
        style(name).magenta().bold().underlined(),
    );
    println!(
        "    {} {}",
        style("→").dim(),
        style(buffy_toml.display()).dim(),
    );
    println!(
        "    {} {}",
        style("→").dim(),
        style(proto_path.display()).dim(),
    );
    println!();
    println!(
        "    Edit {} to configure your target languages, then run {}",
        style("Buffy.toml").bold(),
        style("buffy build").bold().cyan(),
    );

    Ok(())
}
