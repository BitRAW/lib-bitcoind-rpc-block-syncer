use super::settings::BitcoindRpc;
use bitcoincore_rpc::{Auth, Client};
use log::{error, info};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn init_bitcoin_rpc_client(settings: &BitcoindRpc) -> Arc<Client> {
    let url = format!("{}:{}", settings.url, settings.port);

    loop {
        match Client::new(
            &url,
            Auth::UserPass(settings.user.to_string(), settings.pass.to_string()),
        ) {
            Ok(client) => {
                info!("RPC connection established: {}", url);
                return Arc::new(client);
            }
            Err(error) => {
                error!(
                    "Cannot connect to bitcoind RPC interface {} with user {}. Error: {}",
                    url, settings.user, error
                );
            }
        }

        info!(
            "Retrying to establish bitcoind RPC connection in {} seconds",
            settings.revival_interval_millis
        );
        thread::sleep(Duration::from_millis(settings.revival_interval_millis));
    }
}
