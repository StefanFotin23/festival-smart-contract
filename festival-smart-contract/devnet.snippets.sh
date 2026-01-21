WASM_PATH="output/festival-smart-contract.wasm"
  WALLET_PEM="/mnt/c/Users/andre/Desktop/BPDA/wallet.pem"
  PROXY="https://devnet-gateway.multiversx.com"

  deploySC() {
      mxpy --verbose contract deploy \
          --bytecode=${WASM_PATH} \
          --pem=${WALLET_PEM} \
          --proxy=${PROXY} \
          --send > ../deploy.txt || return
  }

  upgradeSC() {
      mxpy --verbose contract upgrade \
          --project=${WASM_PATH} \
          --pem=${WALLET_PEM} \
          --proxy=${PROXY} \
          --recall-nonce \
          --gas-limit=60000000 \
          --metadata-not-upgradable \
          --send > ../upgrade.txt || return
  }