use indicatif::ProgressBar;

use crate::{config::Config, error::Result};

#[async_trait::async_trait]
pub trait Compiler: Send + Sync {
    async fn build(&self, cfg: Config, pb: ProgressBar) -> Result<()>;

    async fn publish(&self, cfg: Config, pb: ProgressBar) -> Result<()>;
}
