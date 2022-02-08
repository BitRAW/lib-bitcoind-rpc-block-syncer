use bitcoincore_rpc::bitcoin::BlockHash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockData {
    #[serde(rename(deserialize = "hash"), rename(deserialize = "block_hash"))]
    pub block_hash: BlockHash,
    #[serde(rename(deserialize = "height"), rename(deserialize = "block_height"))]
    pub block_height: u64,
    pub streamed: Option<bool>,
    pub time: Option<u64>,
}

impl BlockData {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str::<Self>(json).unwrap()
    }
}

impl PartialEq for BlockData {
    fn eq(&self, other: &Self) -> bool {
        self.block_hash.eq(&other.block_hash)
            && self.block_height == other.block_height
            && self.streamed.eq(&other.streamed)
    }
}
impl Eq for BlockData {}
