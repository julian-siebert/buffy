use std::sync::Arc;

use console::style;
use indicatif::{MultiProgress, ProgressBar};

use crate::{
    compiler::Compiler,
    config::Config,
    error::{Error, Result},
};

pub async fn publish(cfg: Config, compilers: Vec<Arc<dyn Compiler>>) -> Result<()> {
    let mp = MultiProgress::new();

    println!(
        "{} {} {} {}",
        style("[-]").green().bold(),
        style("Publishing").bold(),
        style(format!("{}", cfg.package.name))
            .magenta()
            .bold()
            .underlined(),
        style(format!("v{}", cfg.package.version)).yellow(),
    );

    let mut handles = Vec::new();

    for compiler in compilers {
        let mp_clone = mp.clone();
        let cfg_clone = cfg.clone();

        let handle = tokio::task::spawn(async move {
            let pb = mp_clone.add(ProgressBar::new_spinner());
            compiler.publish(cfg_clone, pb).await
        });

        handles.push(handle);
    }

    let mut errors: Vec<Error> = Vec::new();

    for handle in handles {
        match handle.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => errors.push(e),
            Err(e) => errors.push(Error::Internal(format!("compiler task panicked: {e}"))),
        }
    }

    mp.clear()?;

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("{:?}", miette::Report::new_boxed(Box::new(e.clone())));
        }

        return Err(Error::BuildFailed {
            count: errors.len(),
        });
    }

    println!(
        "{} {} {} {}",
        style("[+]").green().bold(),
        style("Published").bold(),
        style(format!("{}", cfg.package.name))
            .magenta()
            .bold()
            .underlined(),
        style(format!("v{}", cfg.package.version)).yellow(),
    );

    Ok(())
}
