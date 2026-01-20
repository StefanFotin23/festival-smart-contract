# Makefile for festival-smart-contract

# ----------------------------
# Variables
# ----------------------------

# Wallet PEM file
WALLET = "/mnt/c/Users/andre/Desktop/BPDA/wallet.pem"

# Proxy for devnet
PROXY = "https://devnet-gateway.multiversx.com"

# Smart Contract Address (replace with your deployed contract address)
SC_ADDRESS = "erd1qqqqqqqqqqqqqpgq7kpf8d4eyy6umdgeug8la0ss64uxeg4cn2jsuxvsq2"

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
