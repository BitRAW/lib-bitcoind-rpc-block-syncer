# lib-bitcoind-rpc-block-syncer
This library is responsible for the communication with `bitcoind` over the RPC interface.  
It opens connections, syncs through past blocks and listens to new ones (once all past blocks are synced).

### How to use the library
```rust
// create a channel to communicate between threads
// FooBar defines the datatype you want to send back from the spawned thread (rustc may be able to derive this)
let (sender, mut receiver) = tokio::sync::mpsc::channel::<FooBar>(1);

// call the function init passing the following arguments
// config file: path to the configuration file 
// starting block height: from where on the blockchain should be synced
// callback function: a function that is called for every block
// sender: the sender of the communication channel to send information back from the spawned thread
tokio::spawn(async move {
    init(config_file, starting_height, callback, sender).await;
});

// wait for messages from block listener thread (will appear on a per-block basis)
loop {
    let msg = receiver.recv().await.unwrap();
    println!("{}", msg);
}
```

The callback function will have to return a String (which will be sent back through the threads-channel) and accept 4 arguments:
 - the block hash
 - the block height
 - a boolean describing whether the block is a historic one or a freshly mined one (polled)
 - the rpc connection to make further requests

```rust
// Once again, FooBar defines the datatype you want to send back from the spawned thread
fn callback(block_hash: BlockHash, block_height: u64, streamed: bool, rpc: Arc<Client>) -> FooBar {
  // per-block information gathering takes place here ...  
  block_height.to_string()
}
```

#### No historic blocks, only live streaming
If you want to only stream new blocks and don't want to sync any historic blocks, you can call the init function passing `0` for the starting_height.
This will skip all syncing of historic blocks.

### Configure
The library can be configured either over environment variables or over config files (not both at the same time!).

#### Config Files
Copy the file `sample-config.toml` from the root folder, change the settings and move it to wherever you need it:
 - For unit tests: `src/tests/unit_tests.toml`
 - For integration_tests: `integration_tests.toml`

While all the settings from sample config **must** be provided, it's entirely okay for the settings file to contain further settings that are not used by the library.
Consequently, a tool using this library may have its own settings file, in which the library settings are just a subset of all the settings in the config file.

#### Environment Variables
Start any tool that uses this library with the following (cusotmized) environment variables:  
`BTC_RPC_URL=http://our-full-no.de BTC_RPC_PORT=8332 BTC_RPC_USER=bitcoin-rpc BTC_RPC_PASS=passw0rd BTC_RPC_POLLING=10000 BTC_RPC_REVIVAL=10000 RUST_LOG=info ./awesome-bitcoin-tool`

#### Explanation config values
Check the Bitcoin Core installation to find out the values needed.
The polling settings declare the frequency in milliseconds, with which Bitcoin Core should be polled for new blocks.

### Run tests
Quick tests:
```sh
RUST_LOG=info cargo test --verbose -- --nocapture
```
Expensive tests:
```sh
RUST_LOG=info cargo test --verbose -- --ignored --nocapture
```

#### Test configuration
Tests require a proper [configuration](#configure) too.

### Caution
Log in debug mode logs RPC password.
