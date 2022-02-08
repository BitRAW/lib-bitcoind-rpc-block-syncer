#!/bin/sh

if [ -z "$DATA_DIR" ]
then
  DATA_DIR=/tmp/lib-bitcoind-rpc-block-syncer_bitcoind-data-dir
fi

if [ -z "$RPC_SERVER" ]
then
  RPC_SERVER=127.0.0.1
fi

if [ -z "$RPC_PORT" ]
then
  RPC_PORT=12349
fi

if [ -z "$RPC_USER" ]
then
  RPC_USER=user
fi

if [ -z "$RPC_PASS" ]
then
  RPC_PASS=passw0rd
fi

rm -rf ${DATA_DIR}
mkdir -p ${DATA_DIR}

# To kill any remaining open bitcoind.
killall -q -9 bitcoind

bitcoind -regtest \
    -datadir=${DATA_DIR} \
    -printtoconsole=0 \
    -disablewallet \
    -rpcport=${RPC_PORT} \
    -rpcuser=${RPC_USER} \
    -rpcpassword=${RPC_PASS} &

PID=$!

# mine 500 blocks
bitcoin-cli -rpcwait \
  -rpcconnect=${RPC_SERVER} \
  -rpcport=${RPC_PORT} \
  -rpcuser=${RPC_USER} \
  -rpcpassword=${RPC_PASS} \
  generatetoaddress 500 bcrt1qs758ursh4q9z627kt3pp5yysm78ddny6txaqgw;

# mine 100 blocks within 33.3 minutes
for i in `seq 1 100`;
do
   bitcoin-cli -rpcconnect=${RPC_SERVER} \
      -rpcport=${RPC_PORT} \
      -rpcuser=${RPC_USER} \
      -rpcpassword=${RPC_PASS} \
      generatetoaddress 1 bcrt1qs758ursh4q9z627kt3pp5yysm78ddny6txaqgw;
   sleep 20;
done

kill -9 $PID
