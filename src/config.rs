use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};

use alloy::primitives::Address;
use alloy::primitives::Selector;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    /// The port to listen on.
    pub port: u16,
    /// The List of allowed calls
    pub allowed_calls: BTreeMap<Address, BTreeSet<Selector>>,
}

impl AppConfig {
    /// Load the configuration from the given file name.
    /// Note: do not specify the file extension, it will be inferred.
    pub fn load(file_name: &str) -> crate::Result<Self> {
        let config = config::Config::builder()
            .set_default("port", 3564)?
            .set_default("allowed_calls", HashMap::<_, Vec<&'static str>>::new())?
            .add_source(config::File::with_name(file_name))
            .add_source(config::Environment::with_prefix("RELAYER"))
            .build()?;

        config.try_deserialize().map_err(Into::into)
    }
}
