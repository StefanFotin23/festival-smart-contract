# Makefile for festival-smart-contract

# ----------------------------
# Variables
# ----------------------------

# Wallet PEM file
WALLET = "/mnt/c/Users/andre/Desktop/BPDA/wallet.pem"

# Proxy for devnet
PROXY = "https://devnet-gateway.multiversx.com"

# Smart Contract Address
SC_ADDRESS = "erd1qqqqqqqqqqqqqpgq7kpf8d4eyy6umdgeug8la0ss64uxeg4cn2jsuxvsq2"

# Default Gas Limit (Safe for most operations)
GAS_LIMIT = 60000000

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

# ----------------------------
# Owner Interactions
# ----------------------------

# Usage: make add-festival NAME="Summer Fest" START_TIME=...
add-festival:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="addFestival" \
	--arguments "str:$(NAME)" $(START_TIME) $(END_TIME) $(MAX_TICKETS) $(TAX_NORMAL) $(TAX_SOLD_OUT) \
	--send

# Usage: make add-event FESTIVAL_ID=1 ...
add-event:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="addEvent" \
	--arguments $(FESTIVAL_ID) "str:$(NAME)" "str:$(LOCATION)" $(START_TIME) $(END_TIME) \
	--send

# Usage: make add-ticket-price FESTIVAL_ID=1 ...
add-ticket-price:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="addTicketPrice" \
	--arguments $(FESTIVAL_ID) "str:$(NAME)" "str:$(PHASE)" $(PRICE) $(SALE_START_TIME) $(SALE_END_TIME) $(TICKET_TYPE) \
	--send

# Usage: make add-flash-event FESTIVAL_ID=1 ...
add-flash-event:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="addFlashEvent" \
	--arguments $(FESTIVAL_ID) "str:$(NAME)" $(START_TIME) $(END_TIME) $(BONUS_POINTS) \
	--send

# Usage: make set-ticket-token-identifier TOKEN_IDENTIFIER="FESTT-123456"
set-ticket-token-identifier:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="setTicketTokenIdentifier" \
	--arguments "str:$(TOKEN_IDENTIFIER)" \
	--send

# ----------------------------
# User Interactions
# ----------------------------

# Usage: make buy-ticket ...
buy-ticket:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=80000000 --proxy=$(PROXY) --chain=D \
	--function="buyTicket" \
	--arguments $(FESTIVAL_ID) "str:$(TICKET_PRICE_NAME)" \
	--value=$(VALUE) \
	--send

# Usage: make create-participant USERNAME="Alice"
create-participant:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="createParticipant" \
	--arguments "str:$(USERNAME)" \
	--send

# Usage: make check-in ...
check-in:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="checkIn" \
	--esdt-transfer $(TOKEN_ID) $(NONCE) 1 \
	--send

# Usage: make check-out FESTIVAL_ID=1
check-out:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="checkOut" \
	--arguments $(FESTIVAL_ID) \
	--send

# Usage: make claim-flash-event-points ...
claim-flash-event-points:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="claimFlashEventPoints" \
	--arguments $(FESTIVAL_ID) $(FLASH_EVENT_INDEX) \
	--send

# Usage: make put-ticket-for-sale ...
put-ticket-for-sale:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
	--function="putTicketForSale" \
	--arguments $(PRICE) \
	--esdt-transfer $(TOKEN_ID) $(NONCE) 1 \
	--send

# Usage: make buy-resale-ticket ...
buy-resale-ticket:
	mxpy --verbose contract call $(SC_ADDRESS) --pem=$(WALLET) --gas-limit=$(GAS_LIMIT) --proxy=$(PROXY) --chain=D \
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