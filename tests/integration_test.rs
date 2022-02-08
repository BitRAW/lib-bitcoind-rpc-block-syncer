mod internal;

use crate::internal::bitcoind_interface::BitcoindInterface;
use crate::internal::comparison;

use crate::internal::block_data::BlockData;
use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::Client;
use std::sync::Arc;
use tokio::sync::mpsc;

// must match tests/internal/bitcoind_interface.rs --> AMT_OF_HISTORIC_BLOCKS_TO_PROCESS
const AMT_OF_HISTORIC_BLOCKS_TO_PROCESS: u8 = 3;
const AMT_OF_STREAMED_BLOCKS_TO_PROCESS: u8 = 3;

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn compare_results() {
    // arrange
    let _ = env_logger::try_init();
    let amt_of_blocks = AMT_OF_HISTORIC_BLOCKS_TO_PROCESS + AMT_OF_STREAMED_BLOCKS_TO_PROCESS;
    let (tx, mut rx) = mpsc::channel::<BlockData>(1);

    // act
    BitcoindInterface::process_old_blocks_and_listen_to_new_ones(process_incoming_block, tx);

    // assert
    for block_count in 0..amt_of_blocks {
        let block_data = rx.recv().await.unwrap();
        let block_should_be_streamed = block_count >= AMT_OF_HISTORIC_BLOCKS_TO_PROCESS;

        // compares hash and height
        comparison::compare_block(block_data, block_should_be_streamed);
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn compare_results_to_bitcoin_cli_historic_blocks_only() {
    // arrange
    let _ = env_logger::try_init();
    let (tx, mut rx) = mpsc::channel::<BlockData>(1);

    // act
    BitcoindInterface::process_old_blocks_and_listen_to_new_ones(process_incoming_block, tx);

    // assert
    for _ in 0..AMT_OF_HISTORIC_BLOCKS_TO_PROCESS {
        let block_data = rx.recv().await.unwrap();

        // compares hash, height and whether block was streamed or not
        comparison::verify_block_is_not_streamed(&block_data);
        comparison::compare_block(block_data, false);
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn starting_point_0_should_not_load_any_historic_blocks() {
    // arrange
    let _ = env_logger::try_init();
    let (tx, mut rx) = mpsc::channel::<BlockData>(1);

    // act
    BitcoindInterface::only_listen_to_new_blocks(process_incoming_block, tx);
    let block_data = rx.recv().await.unwrap();

    // assert
    comparison::verify_block_is_streamed(&block_data);
}

fn process_incoming_block(
    block_hash: BlockHash,
    block_height: u64,
    streamed: bool,
    _: Arc<Client>,
) -> BlockData {
    let streamed = Some(streamed);
    BlockData {
        block_hash,
        block_height,
        streamed,
        time: None,
    }
}
