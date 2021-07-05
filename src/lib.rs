use crate::connect::stubborn_calls as sc;
use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::Client;
use connect::connections;
use connect::settings::Settings;
use log::{debug, info};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

mod connect;
mod tests;

// set starting_height to 0 if you don't want to sync historic blocks
pub async fn init<F, T>(
    config_file_path: &str,
    starting_height: u64,
    callback: F,
    responder: Sender<T>,
) where
    F: Fn(BlockHash, u64, bool, Arc<Client>) -> T + Send + Sync + 'static + Copy,
    T: Send + 'static + Debug,
{
    let mut chain_tip = starting_height;
    let settings = Settings::new(config_file_path);
    let rpc = connections::init_bitcoin_rpc_client(&settings.bitcoindrpc);

    if chain_tip == 0 {
        chain_tip = sc::get_block_count(&rpc);
    } else {
        let mut responder = responder.clone();
        chain_tip = process_past_blocks(&rpc, chain_tip, callback, &mut responder).await;

        info!("Synced up to blockchain tip! [Block #{}]", chain_tip);
    }

    // Polling new blocks over RPC
    info!(
        "Start polling blocks with a frequency of {} milliseconds",
        settings.bitcoindrpc.polling_frequency_millis
    );
    loop {
        sleep(Duration::from_millis(
            settings.bitcoindrpc.polling_frequency_millis,
        ))
        .await;

        if sc::get_block_count(&rpc) > chain_tip {
            chain_tip += 1;
            let mut responder = responder.clone();
            process_block(chain_tip, &rpc, true, &callback, &mut responder).await;
        }
    }
}

async fn process_past_blocks<F, T>(
    rpc: &Arc<Client>,
    mut starting_height: u64,
    cb: F,
    responder: &mut Sender<T>,
) -> u64
where
    F: Fn(BlockHash, u64, bool, Arc<Client>) -> T + Send + Sync + 'static + Copy,
    T: Debug,
{
    let mut chain_height = 0;

    while chain_height != sc::get_block_count(&rpc) {
        chain_height = sc::get_block_count(&rpc);

        if starting_height <= chain_height {
            info!(
                "Requesting blocks {} to {} from bitcoind RPC interface",
                starting_height, chain_height
            );

            process_block_range(starting_height, chain_height, rpc, cb, responder).await;
            starting_height = chain_height + 1;
        }
    }

    chain_height
}

async fn process_block_range<F, T>(
    from_height: u64,
    to_height: u64,
    rpc: &Arc<Client>,
    cb: F,
    responder: &mut Sender<T>,
) where
    F: Fn(BlockHash, u64, bool, Arc<Client>) -> T + Send + Sync + 'static,
    T: Debug,
{
    for chain_pointer in from_height..=to_height {
        process_block(chain_pointer, rpc, false, &cb, responder).await;

        if chain_pointer % 10_000 == 0 {
            info!("Synced up to block {}", chain_pointer);
        }
    }
}

async fn process_block<F, T>(
    chain_pointer: u64,
    rpc: &Arc<Client>,
    streamed: bool,
    cb: &F,
    responder: &mut Sender<T>,
) where
    F: Fn(BlockHash, u64, bool, Arc<Client>) -> T + Send + Sync + 'static,
    T: Debug,
{
    debug!("Processing block {}", chain_pointer);

    let block_hash = sc::get_block_hash(&rpc, chain_pointer);

    responder
        .send(cb(block_hash, chain_pointer, streamed, rpc.clone()))
        .await
        .unwrap();
}
