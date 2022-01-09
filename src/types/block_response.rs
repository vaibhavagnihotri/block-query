use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use web3::types::U64;
use web3::types::{BlockId, BlockNumber, H2048, H256};

#[derive(Debug)]
pub struct BlockResponse(Vec<Block>);

impl BlockResponse {
    pub fn add(&mut self, block: &web3::types::Block<H256>) {
        if let Some(number) = block.number {
            self.0.push(Block {
                hash: block.hash,
                logs_bloom: block.logs_bloom,
                number,
            })
        } else {
            //Skip pending blocks
        }
    }

    pub fn new() -> Self {
        BlockResponse(vec![])
    }
}

impl Serialize for BlockResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(self.0.len()))?;
        let mut block_list = self.0.clone();
        block_list.sort_by(|a, b| b.number.cmp(&a.number));

        for block in block_list {
            s.serialize_entry(&format!("{:x}", block.number), &block);
        }
        s.end()
    }
}

#[derive(Debug, Clone, Serialize)]
struct Block {
    /// Hash of the block
    pub hash: Option<H256>,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<H2048>,
    /// Block number
    /// Skipping serialization of this
    /// because not required in response
    #[serde(skip)]
    pub number: U64,
}
