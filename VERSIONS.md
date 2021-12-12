Version History
===
### 0.2.4

* Define used rust version in Cargo.toml
* GitHub Actions
  * Split up into mulitple jobs
  * Always make sure the newest Rust version is being installed
* Update dependencies:
  * serde_json from 1.0.68 to 1.0.72
  * tokio 1.13.0 to 1.14.0
  * Rebased bitcoincore-rpc to latest commits

### 0.2.3

* Upgrade to Rust edition 2021
* Fix clippy warning (reference immediately dereferenced)
* Run GitHub workflow 'Build and Test' on all branches
* Update dependencies:
  * env_logger 0.8.4 to 0.9.0
  * serde 1.0.126 to 1.0.130
  * serde_json from 1.0.64 to 1.0.68
  * tokio 1.8.1 to 1.13.0
  * Rebased bitcoincore-rpc to latest commits

### 0.2.2

* Fix syncing (double entries of blocks)
* Complete the renaming of the library
* Fix README
  * Add shields to README
* Update tokio 1.7.1 to 1.8.0

### 0.2.1
* Fix errors that arised from purging ZMQ
* Fix feature "only sync non-historic blocks"
* Minor improvements

### 0.2.0
* Purge all usage of ZMQ interface
* Purge all GitHub history by moving code to a new repository
* Update dependencies: tokio 1.6.1 -> 1.7.1, stream-cancel 0.8.0 -> 0.8.1, env_logger 0.8.3 -> 0.8.4

### 0.1.16
* Change flag no-zmq to --no-zmq
* Update tokio from 1.6.0 to 1.6.1

### 0.1.15
* Fix double syncing of blocks when circumventing ZMQ

### 0.1.14
* Minor fix: Also do historic sync if only one historic block is left to sync

### 0.1.13
* Fix use of arguments introduced with 0.1.12

### 0.1.12
* Add temporary possibility to poll new blocks instead of using ZMQ (use arg "no-zmq")
