use serde::{Deserialize, Serialize};

use crate::methods::MethodConfig;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    method: MethodConfig,
}
