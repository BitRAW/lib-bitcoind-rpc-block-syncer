use crate::internal::mempool_space;
use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::Client;
use lib_bitcoind_rpc_block_syncer::init;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

const CONFIG_FILE: &str = "./tests/integration_tests.toml";
const AMT_OF_HISTORIC_BLOCKS_TO_PROCESS: u64 = 3;

pub struct BitcoindInterface;

impl BitcoindInterface {
    pub fn process_old_blocks_and_listen_to_new_ones<F, T>(callback: F, responder: Sender<T>)
    where
        F: Fn(BlockHash, u64, bool, Arc<Client>) -> T + Send + Sync + 'static + Copy,
        T: Send + 'static + Debug,
    {
        tokio::spawn(async move {
            init(CONFIG_FILE, get_chain_pointer(), callback, responder).await;
        });
    }

    pub fn only_listen_to_new_blocks<F, T>(callback: F, responder: Sender<T>)
    where
        F: Fn(BlockHash, u64, bool, Arc<Client>) -> T + Send + Sync + 'static + Copy,
        T: Send + 'static + Debug,
    {
        tokio::spawn(async move {
            init(CONFIG_FILE, 0, callback, responder).await;
        });
    }
}

fn get_chain_pointer() -> u64 {
    let chain_height = mempool_space::get_chain_height();
    chain_height - (AMT_OF_HISTORIC_BLOCKS_TO_PROCESS - 1)
}
