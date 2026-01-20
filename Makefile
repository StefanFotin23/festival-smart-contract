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

# Example usage:
# make add-festival NAME="My Festival" START_TIME=1234567890 END_TIME=1234567899 MAX_TICKETS=1000 TAX_NORMAL=5 TAX_SOLD_OUT=10
add-festival:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="addFestival" --arguments str:$(NAME) $(START_TIME) $(END_TIME) $(MAX_TICKETS) $(TAX_NORMAL) $(TAX_SOLD_OUT)

# Example usage:
# make add-event FESTIVAL_ID=1 NAME="Artist Name" LOCATION="Main Stage" START_TIME=1234567890 END_TIME=1234567899
add-event:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="addEvent" --arguments $(FESTIVAL_ID) str:$(NAME) str:$(LOCATION) $(START_TIME) $(END_TIME)

# Example usage:
# make add-ticket-price FESTIVAL_ID=1 NAME="Early Bird" PHASE="Phase 1" PRICE=1000000000000000000
add-ticket-price:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="addTicketPrice" --arguments $(FESTIVAL_ID) str:$(NAME) str:$(PHASE) $(PRICE)

# Example usage:
# make add-flash-event FESTIVAL_ID=1 NAME="Flash Sale" START_TIME=1234567890 END_TIME=1234567899 BONUS_POINTS=100
add-flash-event:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="addFlashEvent" --arguments $(FESTIVAL_ID) str:$(NAME) $(START_TIME) $(END_TIME) $(BONUS_POINTS)

# Example usage:
# make set-ticket-token-identifier TOKEN_IDENTIFIER="FESTT-123456"
set-ticket-token-identifier:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="setTicketTokenIdentifier" --arguments str:$(TOKEN_IDENTIFIER)

# ----------------------------
# User Interactions
# ----------------------------

# Example usage:
# make buy-ticket FESTIVAL_ID=1 TICKET_PRICE_NAME="Early Bird" VALUE=1000000000000000000
buy-ticket:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="buyTicket" --arguments $(FESTIVAL_ID) str:$(TICKET_PRICE_NAME) --value=$(VALUE)

# Example usage:
# make create-participant USERNAME="MyUsername"
create-participant:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="createParticipant" --arguments str:$(USERNAME)

# Example usage:
# make check-in FESTIVAL_ID=1
check-in:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="checkIn" --arguments $(FESTIVAL_ID)

# Example usage:
# make check-out FESTIVAL_ID=1
check-out:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="checkOut" --arguments $(FESTIVAL_ID)

# Example usage:
# make claim-flash-event-points FESTIVAL_ID=1 FLASH_EVENT_INDEX=0
claim-flash-event-points:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="claimFlashEventPoints" --arguments $(FESTIVAL_ID) $(FLASH_EVENT_INDEX)

# Example usage:
# make put-ticket-for-sale FESTIVAL_ID=1 PRICE=1200000000000000000 TOKEN_ID="FESTT-123456" NONCE=1
put-ticket-for-sale:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="putTicketForSale" --arguments $(FESTIVAL_ID) $(PRICE) --esdt-transfer $(TOKEN_ID) $(NONCE) 1

# Example usage:
# make buy-resale-ticket TICKET_NONCE=1 VALUE=1200000000000000000
buy-resale-ticket:
	mxpy --verbose contract call $(SC_ADDRESS) --recall-nonce --pem=$(WALLET) --gas-limit=10000000 --proxy=$(PROXY) --chain=D --function="buyResaleTicket" --arguments $(TICKET_NONCE) --value=$(VALUE)

# ----------------------------
# Views
# ----------------------------

# Example usage:
# make view-festival-data ID=1
view-festival-data:
	mxpy --verbose contract query $(SC_ADDRESS) --proxy=$(PROXY) --function="getFestivalData" --arguments $(ID)

# Example usage:
# make view-ticket-prices FESTIVAL_ID=1
view-ticket-prices:
	mxpy --verbose contract query $(SC_ADDRESS) --proxy=$(PROXY) --function="getTicketPricesView" --arguments $(FESTIVAL_ID)

# Example usage:
# make view-events FESTIVAL_ID=1
view-events:
	mxpy --verbose contract query $(SC_ADDRESS) --proxy=$(PROXY) --function="getEventsView" --arguments $(FESTIVAL_ID)
