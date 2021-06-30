#[cfg(test)]
mod tests {
    use crate::connect::connections;
    use crate::connect::settings::Settings;
    use crate::process_block_range;
    use bitcoincore_rpc::bitcoin::BlockHash;
    use bitcoincore_rpc::Client;
    use std::sync::Arc;
    use tokio::sync::mpsc;

    const CONFIG_FILE: &str = "./src/tests/unit_tests.toml";

    #[tokio::test(flavor = "multi_thread")]
    async fn test_process_block_range() {
        // arrange
        let _ = env_logger::try_init();

        let from_height = 400_000;
        let to_height = 400_005;

        let settings = Settings::new(CONFIG_FILE);
        let rpc = connections::init_bitcoin_rpc_client(&settings.bitcoindrpc);
        let (mut tx, mut rx) = mpsc::channel(1);

        // act
        tokio::spawn(async move {
            process_block_range(from_height, to_height, &rpc, &cb1, &mut tx).await;

            tx.send("finished range.".to_string()).await
        });

        // assert
        for block_height in from_height..=to_height {
            let response = rx.recv().await.unwrap();

            assert_eq!(response, format!("{}:{}", block_height, false));
        }

        assert_eq!(rx.recv().await.unwrap(), "finished range.".to_string());
    }

    fn cb1(_: BlockHash, height: u64, streamed: bool, _: Arc<Client>) -> String {
        // test if rpc interface works
        format!("{}:{}", height, streamed)
    }
}
