# Makefile for festival-smart-contract

# ----------------------------
# Variables
# ----------------------------

# Wallet PEM file
WALLET = "/mnt/c/Users/andre/Desktop/BPDA/wallet.pem"

# Proxy for devnet
PROXY = "https://devnet-gateway.multiversx.com"

# Smart Contract Address (replace with your deployed contract address)
SC_ADDRESS = "erd1qqqqqqqqqqqqqpgqwslx6ltkr3z28nn0wjn0evnv7qc67ps5n2js5usuen"

# ----------------------------
# Build
# ----------------------------

.PHONY: build deploy
build:
	cd festival-smart-contract && sc-meta all build

deploy:
	./sc-deploy.sh

# ----------------------------
# Owner Interactions
# ----------------------------



# ----------------------------
# User Interactions
# ----------------------------



# ----------------------------
# Views
# ----------------------------
