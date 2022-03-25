#!/bin/bash

# rm -r ./target
cargo build --target wasm32-unknown-unknown --release
near dev-deploy ./target/wasm32-unknown-unknown/release/status_message.wasm
export $(cat ./neardev/dev-account.env  | xargs)
NEARID=nftholder.testnet
DAO_ID=autodao.sputnikv2.testnet
#near call ${CONTRACT_NAME}  approve_members '{"dao_id":"'$DAO_ID'",  "nft_id":"example-nft.testnet"}' --gas 300000000000000 --accountId ${NEARID}
near call example-nft.testnet nft_mint '{"token_id": "Moops test NFT'$RANDOM'", "receiver_id": "'${NEARID}'", "token_metadata": { "title": "GO TEAM", "description": "The Team Goes", "media": "https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/", "copies": 1}}' --accountId ${NEARID} --deposit 0.1
near call ${CONTRACT_NAME}  approve_members '{"dao_id":"autodao.sputnikv2.testnet", "nft_id":"example-nft.testnet"}' --gas 300000000000000 --accountId ${NEARID}

#near call ${CONTRACT_NAME}  something '{"arg1":" blah"}' --accountId moopaloo.testnet
