use web3::types::U64;
use web3::types::{BlockId, BlockNumber};

pub fn get_block_id(block_number: Option<U64>) -> BlockId {
    match block_number {
        Some(x) => BlockId::Number(BlockNumber::Number(x)),
        None => BlockId::Number(BlockNumber::Latest),
    }
}
