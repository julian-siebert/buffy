use crate::{
    configs::profiles::Profile::{Golang, Java, JavaScript, Kotlin, Python, Rust, TypeScript},
    error::Result,
    targets::{
        context::Context,
        golang::{build_go_profile_target, check_go_profile_target, publish_go_profile_target},
        java::{build_java_profile_target, check_java_profile_target, publish_java_profile_target},
        javascript::{
            build_javascript_profile_target, check_javascript_profile_target,
            publish_javascript_profile_target,
        },
        kotlin::{
            build_kotlin_profile_target, check_kotlin_profile_target, publish_kotlin_profile_target,
        },
        python::{
            build_python_profile_target, check_python_profile_target, publish_python_profile_target,
        },
        rust::{build_rust_profile_target, check_rust_profile_target, publish_rust_profile_target},
        typescript::{
            build_typescript_profile_target, check_typescript_profile_target,
            publish_typescript_profile_target,
        },
    },
};

pub mod context;
mod golang;
mod java;
mod javascript;
mod kotlin;
mod python;
mod rust;
mod typescript;

pub async fn check_profile_target(ctx: Context) -> Result<()> {
    match ctx.profile.kind() {
        Golang(golang) => check_go_profile_target(ctx.clone(), golang).await?,
        Java(java) => check_java_profile_target(ctx.clone(), java).await?,
        Kotlin(kotlin) => check_kotlin_profile_target(ctx.clone(), kotlin).await?,
        JavaScript(js) => check_javascript_profile_target(ctx.clone(), js).await?,
        Rust(rust) => check_rust_profile_target(ctx.clone(), rust).await?,
        TypeScript(ts) => check_typescript_profile_target(ctx.clone(), ts).await?,
        Python(python) => check_python_profile_target(ctx.clone(), python).await?,
    };

    Ok(())
}

pub async fn build_profile_target(ctx: Context) -> Result<()> {
    match ctx.profile.kind() {
        Golang(golang) => build_go_profile_target(ctx.clone(), golang).await?,
        Java(java) => build_java_profile_target(ctx.clone(), java).await?,
        Kotlin(kotlin) => build_kotlin_profile_target(ctx.clone(), kotlin).await?,
        JavaScript(js) => build_javascript_profile_target(ctx.clone(), js).await?,
        Rust(rust) => build_rust_profile_target(ctx.clone(), rust).await?,
        TypeScript(ts) => build_typescript_profile_target(ctx.clone(), ts).await?,
        Python(python) => build_python_profile_target(ctx.clone(), python).await?,
    };

    Ok(())
}

pub async fn publish_profile_target(ctx: Context) -> Result<()> {
    match ctx.profile.kind() {
        Golang(golang) => publish_go_profile_target(ctx.clone(), golang).await?,
        Java(java) => publish_java_profile_target(ctx.clone(), java).await?,
        Kotlin(kotlin) => publish_kotlin_profile_target(ctx.clone(), kotlin).await?,
        JavaScript(js) => publish_javascript_profile_target(ctx.clone(), js).await?,
        Rust(rust) => publish_rust_profile_target(ctx.clone(), rust).await?,
        TypeScript(ts) => publish_typescript_profile_target(ctx.clone(), ts).await?,
        Python(python) => publish_python_profile_target(ctx.clone(), python).await?,
    };

    Ok(())
}
