use crate::internal::block_data::BlockData;
use crate::internal::mempool_space;
use std::thread;
use std::time::Duration;

pub fn compare_block(rpc_data: BlockData, streamed: bool) {
    // wait for mempool.space to catch up
    thread::sleep(Duration::from_secs(10));

    let mut mempool_space_data = mempool_space::get_block(rpc_data.block_hash);
    mempool_space_data.streamed = Some(streamed);

    assert_eq!(rpc_data, mempool_space_data);

    if streamed {
        // check that we're on the same height as mempool.space, allowing for small variance
        let mempool_space_chain_height = mempool_space::get_chain_height();
        assert!(rpc_data.block_height <= mempool_space_chain_height + 1);
        assert!(rpc_data.block_height >= mempool_space_chain_height - 1);
    }
}

pub fn verify_block_is_streamed(rpc_data: BlockData) {
    assert!(rpc_data.streamed.unwrap());
}
