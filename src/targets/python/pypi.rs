use console::style;
use tokio::process::Command;

use crate::{
    configs::profiles::python::Pypi,
    dependencies::twine,
    error::{Error, Result},
    targets::{
        context::Context,
        python::helpers::{
            build_sdist_and_wheel, env_nonempty, render_pyproject, render_readme,
            resolve_pypi_version,
        },
    },
};

pub async fn build_python_profile_pypi_target(ctx: Context, p: &Pypi) -> Result<()> {
    ctx.pb.set_message("Resolving dependency versions...");
    let protobuf_version = resolve_pypi_version("protobuf", p.protobuf_version.as_deref()).await?;
    let grpcio_version = if p.grpc {
        resolve_pypi_version("grpcio", p.grpcio_version.as_deref()).await?
    } else {
        String::new()
    };

    ctx.pb.suspend(|| {
        eprintln!(
            "{} {} using protobuf {}{}",
            style("[i]").cyan().bold(),
            style("PYTHON").bold(),
            style(format!("v{protobuf_version}")).yellow(),
            if p.grpc {
                format!(" + grpcio v{grpcio_version}")
            } else {
                String::new()
            },
        );
    });

    ctx.pb.set_message("Generating pyproject.toml...");
    let pyproject = render_pyproject(
        &ctx,
        &p.name,
        &p.repository,
        p.homepage.as_deref(),
        p.grpc,
        &protobuf_version,
        &grpcio_version,
    )?;
    crate::io::write(ctx.target_path.join("pyproject.toml"), pyproject)?;

    ctx.pb.set_message("Generating README.md...");
    let package_name = p.name.replace('-', "_");
    let readme = render_readme(&ctx, &p.name, &package_name)?;
    crate::io::write(ctx.target_path.join("README.md"), readme)?;

    build_sdist_and_wheel(&ctx).await?;

    Ok(())
}

pub async fn publish_python_profile_pypi_target(ctx: Context, p: &Pypi) -> Result<()> {
    let token = env_nonempty("PYPI_TOKEN");
    if token.is_none() {
        return Err(Error::MissingEnv {
            name: "PYPI_TOKEN".into(),
            hint: indoc::indoc! {"
                Set this environment variable before publishing:

                PYPI_TOKEN – API token from https://pypi.org/manage/account/token/
                             (or https://test.pypi.org/manage/account/token/ for Test PyPI)

                Use the full token including the `pypi-` prefix.
            "}
            .into(),
        });
    }

    let version = ctx.package.version.to_string();
    ctx.pb.set_message(format!(
        "Publishing {} v{version} to {}...",
        p.name, p.repository_url
    ));

    // twine reads credentials from env when -u and -p are not given.
    // Username `__token__` is the convention for PyPI API tokens.
    let mut cmd = Command::new(twine()?);
    cmd.args([
        "upload",
        "--non-interactive",
        "--repository-url",
        &p.repository_url,
        "dist/*",
    ])
    .current_dir(&ctx.target_path)
    .env("TWINE_USERNAME", "__token__")
    .env("TWINE_PASSWORD", token.unwrap());

    ctx.run(&mut cmd).await?;

    ctx.finish_publish(&version, &p.repository_url);

    Ok(())
}
