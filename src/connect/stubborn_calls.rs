use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::{Client, Error, RpcApi};
use log::{error, info};
use std::thread;
use std::time::Duration;

pub(crate) fn get_block_count(rpc: &Client) -> u64 {
    loop {
        match rpc.get_block_count() {
            Ok(count) => return count,
            Err(err) => fail_call("chain_height", err),
        }
    }
}

pub(crate) fn get_block_hash(rpc: &Client, height: u64) -> BlockHash {
    loop {
        match rpc.get_block_hash(height) {
            Ok(hash) => return hash,
            Err(err) => fail_call("block_hash", err),
        }
    }
}

fn fail_call(endpoint: &str, err: Error) {
    error!(
        "Unable to read {} from bitcoind RPC interface. Received error: {}",
        endpoint, err
    );

    info!("Retrying in 2 seconds");
    thread::sleep(Duration::from_secs(2));
}
