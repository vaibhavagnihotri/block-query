use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlockInput {
    pub limit: Option<u32>,
}
