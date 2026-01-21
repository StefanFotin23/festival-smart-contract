# Makefile for festival-smart-contract

# ----------------------------
# Variables
# ----------------------------

# Wallet PEM file
WALLET = "/mnt/c/Users/andre/Desktop/BPDA/wallet.pem"

# Proxy for devnet
PROXY = "https://devnet-gateway.multiversx.com"

CHAIN_ID = D

# Smart Contract Address
SC_ADDRESS = "erd1qqqqqqqqqqqqqpgq7kpf8d4eyy6umdgeug8la0ss64uxeg4cn2jsuxvsq2"

ESDT_SC_ADDRESS = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"

# Default Gas Limit (Safe for most operations)
GAS_LIMIT = 60000000

# Token Variables (No quotes here to avoid double quoting in arguments)
TOKEN_ID = FEST-c51ed4
TOKEN_NAME = SummerFest
TOKEN_TICKER = FEST

# ----------------------------
# Build
# ----------------------------

.PHONY: build deploy upgrade
build:
	cd festival-smart-contract && sc-meta all build

deploy:
	./sc-deploy.sh

upgrade:
	export SC_ADDRESS=$(SC_ADDRESS) && ./sc-upgrade.sh

setup_smart_contract:
	make set-nft-role
	make set-ticket-token

# ----------------------------
# Smart Contract Configurations
# ----------------------------

# 1. Issue the Token (Cost: 0.05 EGLD)
# Usage: make issue-token
issue-token:
	mxpy --verbose contract call $(ESDT_SC_ADDRESS) \
	--pem=$(WALLET) \
	--gas-limit=$(GAS_LIMIT) \
	--proxy=$(PROXY) \
	--chain=$(CHAIN_ID) \
	--value=50000000000000000 \
	--function="issueNonFungible" \
	--arguments "str:$(TOKEN_NAME)" "str:$(TOKEN_TICKER)" \
	--send

# 2. Assign "NFTCreate" Role to Smart Contract
# Usage: make set-nft-role
set-nft-role:
	mxpy --verbose contract call $(ESDT_SC_ADDRESS) \
	--pem=$(WALLET) \
	--gas-limit=$(GAS_LIMIT) \
	--proxy=$(PROXY) \
	--chain=$(CHAIN_ID) \
	--function="setSpecialRole" \
	--arguments "str:$(TOKEN_ID)" "$(SC_ADDRESS)" "str:ESDTRoleNFTCreate" \
	--send

# 3. Tell Smart Contract which Token ID to use
# Usage: make set-ticket-token
set-ticket-token:
	mxpy --verbose contract call $(SC_ADDRESS) \
	--pem=$(WALLET) \
	--gas-limit=$(GAS_LIMIT) \
	--proxy=$(PROXY) \
	--chain=$(CHAIN_ID) \
	--function="setTicketTokenIdentifier" \
	--arguments "str:$(TOKEN_ID)" \
	--send

# 4. (Optional) Force-enable token properties if minting fails
# Usage: make enable-token-changes
enable-token-changes:
	mxpy --verbose contract call $(ESDT_SC_ADDRESS) \
	--pem=$(WALLET) \
	--gas-limit=$(GAS_LIMIT) \
	--proxy=$(PROXY) \
	--chain=$(CHAIN_ID) \
	--function="controlChanges" \
	--arguments "str:$(TOKEN_ID)" "str:canAddSpecialRoles" "true" \
	--send

# 5. (Debug) Check exactly what Token ID the contract is trying to use
# Usage: make verify-contract-token
verify-contract-token:
	@echo "🔍 checking contract storage for 'ticketTokenIdentifier'..."
	@curl -s "$(PROXY)/address/$(SC_ADDRESS)/key/7469636b6574546f6b656e4964656e746966696572" \
	| grep -o '"value":"[^"]*"' | cut -d'"' -f4 | xxd -r -p
	@echo ""

# ----------------------------
# Owner Interactions
# ----------------------------

# Usage: make add-festival NAME="Summer Fest" START_TIME=...
add-festival:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="addFestival" \
	--arguments "str:$(NAME)" $(START_TIME) $(END_TIME) $(MAX_TICKETS) $(TAX_NORMAL) $(TAX_SOLD_OUT) \
	--send

# Usage: make add-event FESTIVAL_ID=1 ...
add-event:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="addEvent" \
	--arguments $(FESTIVAL_ID) "str:$(NAME)" "str:$(LOCATION)" $(START_TIME) $(END_TIME) \
	--send

# Usage: make add-ticket-price FESTIVAL_ID=1 ...
add-ticket-price:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="addTicketPrice" \
	--arguments $(FESTIVAL_ID) "str:$(NAME)" "str:$(PHASE)" $(PRICE) $(SALE_START_TIME) $(SALE_END_TIME) $(TICKET_TYPE) \
	--send

# Usage: make add-flash-event FESTIVAL_ID=1 ...
add-flash-event:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="addFlashEvent" \
	--arguments $(FESTIVAL_ID) "str:$(NAME)" $(START_TIME) $(END_TIME) $(BONUS_POINTS) \
	--send

# Usage: make set-ticket-token-identifier
set-ticket-token-identifier:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="setTicketTokenIdentifier" \
	--arguments "str:$(TOKEN_ID)" \
	--send

# ----------------------------
# User Interactions
# ----------------------------

# Usage: make buy-ticket ...
buy-ticket:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="buyTicket" \
	--arguments $(FESTIVAL_ID) "str:$(TICKET_PRICE_NAME)" \
	--value=$(VALUE) \
	--send

# Usage: make create-participant USERNAME="Alice"
create-participant:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="createParticipant" \
	--arguments "str:$(USERNAME)" \
	--send

# Usage: make check-in NONCE=1
check-in:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="checkIn" \
	--token-transfers "$(TOKEN_ID):$(NONCE):1" \
	--send

# Usage: make check-out FESTIVAL_ID=1
check-out:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="checkOut" \
	--arguments $(FESTIVAL_ID) \
	--send

# Usage: make claim-flash-event-points ...
claim-flash-event-points:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="claimFlashEventPoints" \
	--arguments $(FESTIVAL_ID) $(FLASH_EVENT_INDEX) \
	--send

# Usage: make put-ticket-for-sale ...
put-ticket-for-sale:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="putTicketForSale" \
	--arguments $(PRICE) \
	--token-transfers "$(TOKEN_ID):$(NONCE):1" \
	--send

# Usage: make buy-resale-ticket ...
buy-resale-ticket:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=$(CHAIN_ID) \
	--function="buyResaleTicket" \
	--arguments $(TICKET_NONCE) \
	--value=$(VALUE) \
	--send

# ----------------------------
# Views (Queries do NOT need --send)
# ----------------------------

view-festival-data:
	mxpy --verbose contract query $(SC_ADDRESS) --proxy=$(PROXY) --function="getFestivalData" --arguments $(ID)

view-ticket-prices:
	mxpy --verbose contract query $(SC_ADDRESS) --proxy=$(PROXY) --function="getTicketPrices" --arguments $(FESTIVAL_ID)

view-events:
	mxpy --verbose contract query $(SC_ADDRESS) --proxy=$(PROXY) --function="getEvents" --arguments $(FESTIVAL_ID)