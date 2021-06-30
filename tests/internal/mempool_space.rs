use crate::internal::block_data::BlockData;
use bitcoincore_rpc::bitcoin::BlockHash;
use std::process::Command;

pub fn get_chain_height() -> u64 {
    let curl_response = Command::new("curl")
        .arg("https://mempool.space/api/blocks/tip/height")
        .output()
        .unwrap()
        .stdout;

    String::from_utf8(curl_response)
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

pub fn get_block(block_hash: BlockHash) -> BlockData {
    let mut curl_response = String::from_utf8(curl_mempool_space(block_hash, false)).unwrap();

    // block not found, panic
    if curl_response.eq("Block not found") {
        // check whether block is a testnet block to give some more information in error message.
        curl_response = String::from_utf8(curl_mempool_space(block_hash, true)).unwrap();

        if !curl_response.eq("Block not found") {
            panic!("Testnet is not allowed for testing! Test on mainnet.",)
        }

        panic!(
            "Block {} not found on mempool.space, neither on mainnet nor on testnet! Only Bitcoin mainnet allowed for testing.",
            block_hash.to_string()
        )
    }

    BlockData::from_json(&curl_response)
}

fn curl_mempool_space(block_hash: BlockHash, testnet: bool) -> Vec<u8> {
    let mut network = "";
    if testnet {
        network = "testnet/";
    }

    let url = format!("https://mempool.space/{}api/block/", network);
    let url = url + &*block_hash.to_string();

    Command::new("curl").arg(url).output().unwrap().stdout
}
