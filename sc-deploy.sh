#!/bin/bash

set -e

cd festival-smart-contract
sc-meta all build
sc-meta all snippets --overwrite

source devnet.snippets.sh
deploySC

# Extract the contract address from deploy.txt
SC_ADDRESS=$(grep '"contractAddress"' ../deploy.txt | awk -F'"' '{print $$4}')
echo "Deployed Smart Contract Address: $SC_ADDRESS"

# Update the Makefile with the new SC_ADDRESS
# Need to go back to the root directory to modify the Makefile
cd ..
sed -i 's/^SC_ADDRESS = ".*"$$/SC_ADDRESS = "'"$SC_ADDRESS"'"/' Makefile

# Go back to festival-smart-contract for interactor deployment
cd festival-smart-contract

cp output/festival_smart_contract_proxy.rs interactor/src/proxy.rs
cd interactor
sc-meta all snippets --overwrite

# ============================================================
# [FIX] Automatically boost gas limit from 30M to 100M
# This prevents the "insufficient gas" error
# ============================================================
sed -i 's/30_000_000u64/100_000_000u64/g' src/*.rs

cargo run deploy
