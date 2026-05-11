use tera::{Context as TeraContext, Tera};
use tokio::process::Command;

use crate::{
    configs::profiles::python::Python,
    dependencies::python,
    error::{Error, Result},
    targets::context::Context,
};

const PYPROJECT_TEMPLATE: &str = include_str!("templates/pyproject.toml.tera");
const README_TEMPLATE: &str = include_str!("templates/README.md.tera");

pub async fn generate_python_code(ctx: &Context, py: &Python) -> Result<()> {
    let name = match py {
        Python::Pypi(p) => &p.name,
        Python::Git(g) => &g.name,
    };
    let grpc = match py {
        Python::Pypi(p) => p.grpc,
        Python::Git(g) => g.grpc,
    };

    // python package names use underscores, not hyphens
    let package_name = name.replace('-', "_");
    let pkg_dir = ctx.target_path.join("src").join(&package_name);
    crate::io::create_dir_all(&pkg_dir)?;

    // empty __init__.py so the package is importable
    crate::io::write(pkg_dir.join("__init__.py"), "")?;

    ctx.pb.set_message("Generating Python code...");

    let py_bin = python()?;
    let mut cmd = Command::new(py_bin);
    cmd.args(["-m", "grpc_tools.protoc"])
        .arg(format!("--python_out={}", pkg_dir.display()))
        .arg(format!("--proto_path={}", ctx.source.path.display()));

    if grpc {
        cmd.arg(format!("--grpc_python_out={}", pkg_dir.display()));
    }

    cmd.args(ctx.proto_files());
    ctx.run(&mut cmd).await?;

    Ok(())
}

#[derive(serde::Serialize)]
struct AuthorView<'a> {
    name: &'a str,
    email: Option<&'a str>,
}

pub fn render_pyproject(
    ctx: &Context,
    name: &str,
    repository: &str,
    homepage: Option<&str>,
    grpc: bool,
    protobuf_version: &str,
    grpcio_version: &str,
) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("pyproject.toml", PYPROJECT_TEMPLATE)
        .map_err(|e| Error::Internal(format!("Tera template error: {e}")))?;
    tera.autoescape_on(vec![]);

    let authors_view: Vec<AuthorView> = ctx
        .package
        .authors
        .iter()
        .map(|a| AuthorView {
            name: &a.name,
            email: a.email.as_deref(),
        })
        .collect();

    let mut tctx = TeraContext::new();
    tctx.insert("name", name);
    tctx.insert("version", &ctx.package.version.to_string());
    tctx.insert("description", &ctx.package.description);
    tctx.insert("license", &ctx.package.license);
    tctx.insert("authors", &authors_view);
    tctx.insert("repository", repository);
    tctx.insert(
        "homepage",
        homepage.unwrap_or_else(|| ctx.package.homepage.as_str()),
    );
    tctx.insert("grpc", &grpc);
    tctx.insert("protobuf_version", protobuf_version);
    tctx.insert("grpcio_version", grpcio_version);

    tera.render("pyproject.toml", &tctx)
        .map_err(|e| Error::Internal(format!("Tera render error: {e}")))
}

pub fn render_readme(ctx: &Context, name: &str, package_name: &str) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("README.md", README_TEMPLATE)
        .map_err(|e| Error::Internal(format!("Tera template error: {e}")))?;
    tera.autoescape_on(vec![]);

    let mut tctx = TeraContext::new();
    tctx.insert("name", name);
    tctx.insert("package_name", package_name);
    tctx.insert("description", &ctx.package.description);

    tera.render("README.md", &tctx)
        .map_err(|e| Error::Internal(format!("Tera render error: {e}")))
}

pub async fn resolve_pypi_version(package: &str, configured: Option<&str>) -> Result<String> {
    if let Some(v) = configured {
        return Ok(v.to_string());
    }

    let client = reqwest::Client::builder()
        .user_agent("buffy-build-tool/1.0")
        .build()
        .map_err(|e| Error::Internal(format!("HTTP client error: {e}")))?;

    let url = format!("https://pypi.org/pypi/{package}/json");
    let body = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::Internal(format!("PyPI API unreachable: {e}")))?
        .text()
        .await
        .map_err(|e| Error::Internal(format!("PyPI API read error: {e}")))?;

    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| Error::Internal(format!("PyPI API parse error: {e}")))?;

    json["info"]["version"]
        .as_str()
        .map(String::from)
        .ok_or_else(|| {
            Error::Internal(format!(
                "Could not resolve {package} version from PyPI.\n\
                 Pin it manually in your profile."
            ))
        })
}

pub async fn build_sdist_and_wheel(ctx: &Context) -> Result<()> {
    ctx.pb
        .set_message("Building source distribution and wheel...");
    let py_bin = python()?;
    let mut cmd = Command::new(py_bin);
    cmd.args(["-m", "build", "--sdist", "--wheel"])
        .current_dir(&ctx.target_path);
    ctx.run(&mut cmd).await?;
    Ok(())
}

pub fn env_nonempty(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|s| !s.is_empty())
}
