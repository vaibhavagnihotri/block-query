use serde::Serialize;
use std::collections::HashMap;
use web3::types::U64;
use web3::types::{BlockId, BlockNumber, H2048, H256};

#[derive(Debug, Serialize)]
pub struct BlockResponse(HashMap<U64, Block>);

impl BlockResponse {
    pub fn add(&mut self, block: &web3::types::Block<H256>) {
        match block.number {
            Some(ref number) => {
                self.0.insert(
                    number.clone(),
                    Block {
                        hash: block.hash,
                        logs_bloom: block.logs_bloom,
                    },
                );
            }
            None => {
                //Pending Block
            }
        }
    }

    pub fn new() -> Self {
        BlockResponse(HashMap::new())
    }
}

#[derive(Debug, Serialize)]
struct Block {
    /// Hash of the block
    pub hash: Option<H256>,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<H2048>,
}
