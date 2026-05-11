use crate::{
    configs::profiles::python::Python,
    dependencies::{grpcio_tools, python, python_build, twine},
    error::Result,
    targets::{
        context::Context,
        python::{
            git::{build_python_profile_git_target, publish_python_profile_git_target},
            helpers::generate_python_code,
            pypi::{build_python_profile_pypi_target, publish_python_profile_pypi_target},
        },
    },
};

pub mod git;
mod helpers;
pub mod pypi;

pub async fn check_python_profile_target(ctx: Context, py: &Python) -> Result<()> {
    ctx.pb.set_message("Checking python3...");
    python()?;

    ctx.pb.set_message("Checking grpcio-tools...");
    grpcio_tools()?;

    ctx.pb.set_message("Checking build module...");
    python_build()?;

    match py {
        Python::Pypi(_) => {
            ctx.pb.set_message("Checking twine...");
            twine()?;
        }
        Python::Git(_) => {
            ctx.pb.set_message("Checking git...");
            crate::dependencies::git()?;
        }
    }

    ctx.finish_check();

    Ok(())
}

pub async fn build_python_profile_target(ctx: Context, py: &Python) -> Result<()> {
    generate_python_code(&ctx, py).await?;

    match py {
        Python::Pypi(p) => build_python_profile_pypi_target(ctx.clone(), p).await?,
        Python::Git(g) => build_python_profile_git_target(ctx.clone(), g).await?,
    }

    ctx.finish_build();

    Ok(())
}

pub async fn publish_python_profile_target(ctx: Context, py: &Python) -> Result<()> {
    match py {
        Python::Pypi(p) => publish_python_profile_pypi_target(ctx.clone(), p).await?,
        Python::Git(g) => publish_python_profile_git_target(ctx.clone(), g).await?,
    }
    Ok(())
}
