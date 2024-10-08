mod config;
mod error;
mod module;

pub mod handlers;
pub mod routes;

use std::sync::{Arc, Mutex};

pub use error::{Error, Result};
use module::ModuleManager;

pub(crate) const PLATFORM_CLIENT: &str = "osc-platform-rs";

pub type Platform = Arc<Mutex<ModuleManager>>;

pub fn init() -> Result<Platform> {
    log::info!("{}", PLATFORM_CLIENT);

    let mut platform = ModuleManager::new("/usr/lib/osconfig")?;
    platform.reload()?;
    Ok(Arc::new(Mutex::new(platform)))
}
