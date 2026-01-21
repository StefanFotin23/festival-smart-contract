#!/bin/bash

# This script handles the upgrade process for the smart contract and the interactor.

set -e

echo "--- Starting Smart Contract Upgrade Process ---"

cd festival-smart-contract

# 1. Build the contract and generate fresh snippets
echo "--- Building contract and generating snippets... ---"
sc-meta all build
sc-meta all snippets --overwrite

# 2. Source the snippets and call the upgrade function
echo "--- Upgrading Smart Contract... ---"
source devnet.snippets.sh
upgradeSC

echo "--- Smart Contract Upgrade complete. ---"

# 3. Update and redeploy the interactor
echo "--- Updating interactor... ---"
cp output/festival_smart_contract_proxy.rs interactor/src/proxy.rs
cd interactor
sc-meta all snippets --overwrite

# Apply gas fix from original deploy script
sed -i 's/30_000_000u64/100_000_000u64/g' src/*.rs

echo "--- Redeploying interactor... ---"
cargo run deploy

echo "--- Upgrade process finished successfully! ---"
