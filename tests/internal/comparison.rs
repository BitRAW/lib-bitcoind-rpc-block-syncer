use crate::internal::bitcoin_cli;
use crate::internal::block_data::BlockData;

pub fn compare_block(rpc_data: BlockData, streamed: bool) {
    assert_eq!(rpc_data, bitcoin_cli::get_block(&rpc_data.block_hash));

    if streamed {
        // check that the block has the same height as the bitcoin-cli defines as the current tip of the entire blockchain
        assert!(rpc_data.block_height == bitcoin_cli::get_chain_height());
    }
}

pub fn verify_block_is_streamed(rpc_data: &BlockData) {
    assert!(rpc_data.streamed.unwrap());
}

pub fn verify_block_is_not_streamed(rpc_data: &BlockData) {
    assert_eq!(rpc_data.streamed.unwrap(), false);
}
