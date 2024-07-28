use serde::{Deserialize, Serialize};

use crate::methods::MethodConfig;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Config {
    method: MethodConfig,
}
