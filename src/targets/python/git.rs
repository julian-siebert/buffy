use console::style;

use crate::{
    configs::profiles::python::Git,
    error::Result,
    git,
    targets::{
        context::Context,
        python::helpers::{
            build_sdist_and_wheel, render_pyproject, render_readme, resolve_pypi_version,
        },
    },
};

pub async fn build_python_profile_git_target(ctx: Context, g: &Git) -> Result<()> {
    ctx.pb.set_message("Resolving dependency versions...");
    let protobuf_version = resolve_pypi_version("protobuf", g.protobuf_version.as_deref()).await?;
    let grpcio_version = if g.grpc {
        resolve_pypi_version("grpcio", g.grpcio_version.as_deref()).await?
    } else {
        String::new()
    };

    ctx.pb.suspend(|| {
        eprintln!(
            "{} {} using protobuf {}{}",
            style("[i]").cyan().bold(),
            style("PYTHON").bold(),
            style(format!("v{protobuf_version}")).yellow(),
            if g.grpc {
                format!(" + grpcio v{grpcio_version}")
            } else {
                String::new()
            },
        );
    });

    ctx.pb.set_message("Generating pyproject.toml...");
    let pyproject = render_pyproject(
        &ctx,
        &g.name,
        &g.repository,
        g.homepage.as_deref(),
        g.grpc,
        &protobuf_version,
        &grpcio_version,
    )?;
    crate::io::write(ctx.target_path.join("pyproject.toml"), pyproject)?;

    ctx.pb.set_message("Generating README.md...");
    let package_name = g.name.replace('-', "_");
    let readme = render_readme(&ctx, &g.name, &package_name)?;
    crate::io::write(ctx.target_path.join("README.md"), readme)?;

    crate::gitignore::ensure_entries_in_gitignore(
        &ctx.target_path,
        &["dist", "*.egg-info", "__pycache__"],
    )?;

    // verify the build works
    build_sdist_and_wheel(&ctx).await?;

    Ok(())
}

pub async fn publish_python_profile_git_target(ctx: Context, g: &Git) -> Result<()> {
    let version = ctx.package.version.to_string();
    let tag = format!("v{version}");
    let remote = &g.remote;
    let branch = &g.branch;

    ctx.pb.set_message("Initializing git repository...");
    git!(ctx, "init", "-b", branch)?;

    ctx.pb.set_message("Configuring remote...");
    if git!(ctx, "remote", "add", "origin", remote).is_err() {
        git!(ctx, "remote", "set-url", "origin", remote)?;
    }

    ctx.pb.set_message("Fetching existing files from remote...");
    let fetch_result = git!(
        ctx,
        env: [("GIT_TERMINAL_PROMPT", "0")],
        "fetch", "origin", branch
    );

    if fetch_result.is_ok() {
        for file in &g.keep {
            let result = git!(
                ctx,
                "checkout",
                &format!("origin/{branch}"),
                "--",
                file.as_str()
            );
            if result.is_err() {
                ctx.pb.suspend(|| {
                    eprintln!(
                        "{} {} not found on remote, skipping",
                        style("[~]").yellow().bold(),
                        style(file).dim(),
                    );
                });
            }
        }
    }

    git!(ctx, "add", ".")?;
    git!(ctx, "commit", "-m", &format!("release {tag}"))?;

    ctx.pb.set_message(format!("Tagging {tag}..."));
    git!(ctx, "tag", "-f", &tag)?;

    ctx.pb.set_message(format!("Pushing {tag} to {branch}..."));
    git!(
        ctx,
        env: [("GIT_TERMINAL_PROMPT", "0")],
        "push", "--force", "origin", &format!("HEAD:{branch}")
    )?;
    git!(
        ctx,
        env: [("GIT_TERMINAL_PROMPT", "0")],
        "push", "--force", "origin", "--tags"
    )?;

    ctx.finish_publish(&tag, remote);

    Ok(())
}
