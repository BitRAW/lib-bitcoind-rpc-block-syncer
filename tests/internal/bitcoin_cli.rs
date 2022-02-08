use crate::BlockData;
use bitcoincore_rpc::bitcoin::BlockHash;
use std::process::{Command, ExitStatus};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_chain_height() -> u64 {
    bitcoin_cli_available_check();

    let return_value = Command::new("bitcoin-cli")
        .arg("getblockcount")
        .output()
        .expect("Cannot call bitcoin-cli on this host system");
    if return_value.status.success() {
        let output = String::from_utf8(return_value.stdout)
            .unwrap()
            .trim()
            .to_string();
        println!("output: {}", output);
        output.parse::<u64>().unwrap()
    } else {
        panic!(
            "Calling 'bitcoin-cli getblockcount' failed: {} ({})",
            return_value.status.to_string(),
            String::from_utf8(return_value.stderr).unwrap()
        );
    }
}

pub fn get_block(block_hash: &BlockHash) -> BlockData {
    bitcoin_cli_available_check();

    match query_block(block_hash) {
        Ok(block_data) => {
            let mut cli_block_data = BlockData::from_json(&block_data);

            let two_secs_ago = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 2u64;
            cli_block_data.streamed = Some(cli_block_data.time.unwrap() > two_secs_ago);

            cli_block_data
        }
        Err(_) => {
            panic!(
                "Block {} not found with the bitcoin-cli tool.",
                block_hash.to_string()
            )
        }
    }
}

fn bitcoin_cli_available_check() {
    Command::new("bitcoin-cli")
        .arg("getblockchaininfo")
        .output()
        .expect("Failed to call bitcoin-cli getblockchaininfo. Is bitcoin-cli installed?");
}

fn query_block(block_hash: &BlockHash) -> Result<String, ExitStatus> {
    let block_data = Command::new("bitcoin-cli")
        .arg("getblock")
        .arg(block_hash.to_string())
        .output()
        .expect("Cannot call bitcoin-cli on this host system");

    if block_data.status.success() {
        Ok(String::from_utf8(block_data.stdout)
            .unwrap()
            .trim()
            .to_string())
    } else {
        println!("{}", String::from_utf8(block_data.stderr).unwrap());
        Err(block_data.status)
    }
}
