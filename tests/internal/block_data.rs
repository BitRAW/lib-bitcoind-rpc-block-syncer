use bitcoincore_rpc::bitcoin::BlockHash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct BlockData {
    // on mempool.space the hash is called id, not block_hash
    // --> https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce
    #[serde(rename(deserialize = "id"), rename(deserialize = "block_hash"))]
    pub block_hash: BlockHash,
    // on mempool.space the hash is called height, not block_height
    // --> https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce
    #[serde(rename(deserialize = "height"), rename(deserialize = "block_height"))]
    pub block_height: u64,
    pub difficulty: u64,
    pub streamed: Option<bool>,
}

impl BlockData {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str::<Self>(json).unwrap()
    }
}
