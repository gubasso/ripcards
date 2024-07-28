use std::fs::write;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use toml::to_string;

use crate::methods::MethodConfig;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    method: MethodConfig,
}

pub fn create_config_file() -> Result<()> {
    let config = Config::default();
    let config_string = to_string(&config)?;
    write("ripc/config.toml", config_string)?;
    Ok(())
}
