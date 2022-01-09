use crate::types::Result;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub infuria_key: String,
}

impl Settings {
    fn new() -> Result<Self> {
        let mut s = Config::new();
        s = s
            .with_merged(File::with_name("conf/default.yaml"))?
            .with_merged(File::with_name("conf/settings.yaml").required(false))?;

        s.try_into().map_err(|e| e.into())
    }
}

lazy_static::lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}
