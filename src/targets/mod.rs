use crate::{
    config::Package,
    configs::{
        Error,
        profiles::{
            NamedProfile,
            Profile::{Golang, Java, JavaScript, Kotlin, Rust, TypeScript},
        },
    },
    targets::context::Context,
};

pub mod context;
mod golang;
mod java;
mod javascript;
mod kotlin;
mod rust;
mod typescript;

pub async fn check_profile_target(package: Package, profile: NamedProfile) -> Result<(), Error> {
    let ctx = Context::new(package, profile)?;

    match ctx.profile.kind() {
        Golang(golang) => {}
        Java(java) => {}
        Kotlin(kotlin) => {}
        JavaScript(java_script) => {}
        Rust(rust) => {}
        TypeScript(type_script) => {}
    };

    Ok(())
}

pub async fn build_profile_target(package: Package, profile: NamedProfile) -> Result<(), Error> {
    let ctx = Context::new(package, profile)?;

    match ctx.profile.kind() {
        Golang(golang) => {}
        Java(java) => {}
        Kotlin(kotlin) => {}
        JavaScript(java_script) => {}
        Rust(rust) => {}
        TypeScript(type_script) => {}
    };

    Ok(())
}

pub async fn publish_profile_target(package: Package, profile: NamedProfile) -> Result<(), Error> {
    let ctx = Context::new(package, profile)?;

    match ctx.profile.kind() {
        Golang(golang) => {}
        Java(java) => {}
        Kotlin(kotlin) => {}
        JavaScript(java_script) => {}
        Rust(rust) => {}
        TypeScript(type_script) => {}
    };

    Ok(())
}
