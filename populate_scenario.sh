#!/bin/bash

# =================================================================
#  SCENARIO: LEGENDS OF SOUND 2026 (Real Festival Simulation)
# =================================================================

# Exit script if any command fails
set -e

# --- Configuration ---
# Time to wait between transactions (Devnet block time is ~6s)
SLEEP_TIME=6

# Wallet & Contract Config (Must match your Makefile/Project)
# We redefine these here for the raw mxpy commands (addProduct, etc.)
# which don't have Makefile targets yet.
WALLET="/mnt/c/Users/andre/Desktop/BPDA/wallet.pem"
PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"
SC_ADDRESS="erd1qqqqqqqqqqqqqpgq7kpf8d4eyy6umdgeug8la0ss64uxeg4cn2jsuxvsq2"
GAS_LIMIT=60000000
TICKET_TOKEN_ID="FEST-c51ed4" # Ensure this matches your actual token

# Calculate Timestamps for 2026
CURRENT_TIMESTAMP=$(date +%s)
YEAR_2026="2026"
AUG_06=$(date -d "${YEAR_2026}-08-06 14:00:00" +%s) # Festival Start
AUG_09=$(date -d "${YEAR_2026}-08-09 23:59:59" +%s) # Festival End

# Eminem Concert (Friday Night)
EMINEM_START=$(date -d "${YEAR_2026}-08-07 21:00:00" +%s)
EMINEM_END=$(date -d "${YEAR_2026}-08-07 23:00:00" +%s)

# Setup Ticket Phases (Relative to Now for testing, or fixed dates)
PHASE_1_END=$((CURRENT_TIMESTAMP + 30 * 24 * 3600))     # +30 Days
PHASE_2_END=$((CURRENT_TIMESTAMP + 60 * 24 * 3600))     # +60 Days
PHASE_3_END=$((AUG_06 - 3600))                          # Until Start

# -----------------------------------------------------------------
# 1. SETUP & CONFIGURATION
# -----------------------------------------------------------------
echo "--- [1/6] Configuring Contract (Token, Rates, Bonus) ---"

# Set Ticket Token (Using Makefile)
make set-ticket-token-identifier TOKEN_ID=${TICKET_TOKEN_ID}
sleep ${SLEEP_TIME}

# Set EGLD Rate (1 EGLD = $50 USD) - Raw mxpy call
echo "   -> Setting EGLD:USD Rate to 50..."
mxpy contract call ${SC_ADDRESS} --pem=${WALLET} --gas-limit=${GAS_LIMIT} --proxy=${PROXY} --chain=${CHAIN_ID} \
--function="setEgldToUsdRate" --arguments 50 \
--send || true
sleep ${SLEEP_TIME}

# Set Bonus (10%) - Raw mxpy call
echo "   -> Setting Top-up Bonus to 10%..."
mxpy contract call ${SC_ADDRESS} --pem=${WALLET} --gas-limit=${GAS_LIMIT} --proxy=${PROXY} --chain=${CHAIN_ID} \
--function="setBonusPercentage" --arguments 10 \
--send || true
sleep ${SLEEP_TIME}


# -----------------------------------------------------------------
# 2. CREATE FESTIVAL
# -----------------------------------------------------------------
echo "--- [2/6] Creating Festival: Legends of Sound 2026 ---"

FESTIVAL_NAME="Legends of Sound 2026"
make add-festival NAME="${FESTIVAL_NAME}" START_TIME=${AUG_06} END_TIME=${AUG_09} MAX_TICKETS=75000 TAX_NORMAL=5 TAX_SOLD_OUT=10
sleep ${SLEEP_TIME}

# We assume this is the next ID. If contract was empty, it's 1. 
# Adjust manually if you have existing festivals.
FEST_ID=8  


# -----------------------------------------------------------------
# 3. ADD EVENTS (Lineup)
# -----------------------------------------------------------------
echo "--- [3/6] Adding Lineup (Eminem, etc.) ---"

# Eminem (Main Event)
make add-event FESTIVAL_ID=${FEST_ID} NAME="Eminem - HEADLINER" LOCATION="Main Stage" START_TIME=${EMINEM_START} END_TIME=${EMINEM_END}
sleep ${SLEEP_TIME}

# Opening Act
OPENING_START=$(date -d "${YEAR_2026}-08-06 18:00:00" +%s)
OPENING_END=$(date -d "${YEAR_2026}-08-06 20:00:00" +%s)
make add-event FESTIVAL_ID=${FEST_ID} NAME="Grand Opening Fireworks" LOCATION="Festival Grounds" START_TIME=${OPENING_START} END_TIME=${OPENING_END}
sleep ${SLEEP_TIME}

# Techno Late Night
TECHNO_START=$(date -d "${YEAR_2026}-08-08 00:00:00" +%s)
TECHNO_END=$(date -d "${YEAR_2026}-08-08 04:00:00" +%s)
make add-event FESTIVAL_ID=${FEST_ID} NAME="Afterhours Techno" LOCATION="The Bunker" START_TIME=${TECHNO_START} END_TIME=${TECHNO_END}
sleep ${SLEEP_TIME}


# -----------------------------------------------------------------
# 4. ADD TICKET PRICES (6 Categories)
# -----------------------------------------------------------------
echo "--- [4/6] Setting Ticket Prices (Early, Standard, Last Minute) ---"

# --- PHASE 1: EARLY BIRD ---
# Full Pass (Type 0) - 0.5 EGLD
make add-ticket-price FESTIVAL_ID=${FEST_ID} NAME="Early Bird Full Pass" PHASE="Phase 1" PRICE=500000000000000000 \
SALE_START_TIME=${CURRENT_TIMESTAMP} SALE_END_TIME=${PHASE_1_END} TICKET_TYPE=0
sleep ${SLEEP_TIME}

# Day Pass (Type 1) - 0.2 EGLD
make add-ticket-price FESTIVAL_ID=${FEST_ID} NAME="Early Bird Day 1" PHASE="Phase 1" PRICE=200000000000000000 \
SALE_START_TIME=${CURRENT_TIMESTAMP} SALE_END_TIME=${PHASE_1_END} TICKET_TYPE=1
sleep ${SLEEP_TIME}

# --- PHASE 2: STANDARD ---
# Full Pass (Type 0) - 1.0 EGLD
make add-ticket-price FESTIVAL_ID=${FEST_ID} NAME="Standard Full Pass" PHASE="Phase 2" PRICE=1000000000000000000 \
SALE_START_TIME=${PHASE_1_END} SALE_END_TIME=${PHASE_2_END} TICKET_TYPE=0
sleep ${SLEEP_TIME}

# Day Pass (Type 1) - 0.4 EGLD
make add-ticket-price FESTIVAL_ID=${FEST_ID} NAME="Standard Day 1" PHASE="Phase 2" PRICE=400000000000000000 \
SALE_START_TIME=${PHASE_1_END} SALE_END_TIME=${PHASE_2_END} TICKET_TYPE=1
sleep ${SLEEP_TIME}

# --- PHASE 3: LAST MINUTE ---
# Full Pass (Type 0) - 1.5 EGLD
make add-ticket-price FESTIVAL_ID=${FEST_ID} NAME="Last Minute Full Pass" PHASE="Phase 3" PRICE=1500000000000000000 \
SALE_START_TIME=${PHASE_2_END} SALE_END_TIME=${PHASE_3_END} TICKET_TYPE=0
sleep ${SLEEP_TIME}

# Day Pass (Type 1) - 0.6 EGLD
make add-ticket-price FESTIVAL_ID=${FEST_ID} NAME="Last Minute Day 1" PHASE="Phase 3" PRICE=600000000000000000 \
SALE_START_TIME=${PHASE_2_END} SALE_END_TIME=${PHASE_3_END} TICKET_TYPE=1
sleep ${SLEEP_TIME}


# -----------------------------------------------------------------
# 5. ADD PRODUCTS (Merch & Drinks)
# -----------------------------------------------------------------
echo "--- [5/6] Stocking Products (T-Shirts, Beer, Water) ---"

# Product 1: Festival T-Shirt (0.3 EGLD)
echo "   -> Adding T-Shirt..."
mxpy contract call ${SC_ADDRESS} --pem=${WALLET} --gas-limit=${GAS_LIMIT} --proxy=${PROXY} --chain=${CHAIN_ID} \
--function="addProduct" \
--arguments ${FEST_ID} 101 "str:Legends Tee 2026" 300000000000000000 "str:100% Cotton Black Tee" "str:http://img/tee.png" \
--send || true
sleep ${SLEEP_TIME}

# Product 2: Premium Beer (0.05 EGLD)
echo "   -> Adding Beer..."
mxpy contract call ${SC_ADDRESS} --pem=${WALLET} --gas-limit=${GAS_LIMIT} --proxy=${PROXY} --chain=${CHAIN_ID} \
--function="addProduct" \
--arguments ${FEST_ID} 102 "str:Cold Draught Beer" 50000000000000000 "str:500ml Lager" "str:http://img/beer.png" \
--send || true
sleep ${SLEEP_TIME}

# Product 3: Water (0.02 EGLD)
echo "   -> Adding Water..."
mxpy contract call ${SC_ADDRESS} --pem=${WALLET} --gas-limit=${GAS_LIMIT} --proxy=${PROXY} --chain=${CHAIN_ID} \
--function="addProduct" \
--arguments ${FEST_ID} 103 "str:Mineral Water" 20000000000000000 "str:500ml Still" "str:http://img/water.png" \
--send || true
sleep ${SLEEP_TIME}


# -----------------------------------------------------------------
# 6. ADD FLASH EVENT
# -----------------------------------------------------------------
echo "--- [6/6] Creating Flash Event (Meet & Greet) ---"

# Meet & Greet (1 Hour before Eminem)
MEET_START=$((EMINEM_START - 3600))
MEET_END=${EMINEM_START}

make add-flash-event FESTIVAL_ID=${FEST_ID} NAME="Secret Meet & Greet" START_TIME=${MEET_START} END_TIME=${MEET_END} BONUS_POINTS=500
sleep ${SLEEP_TIME}

echo "------------------------------------------------------"
echo "✅ Scenario Populated Successfully!"
echo "   Festival ID: ${FEST_ID}"
echo "   Headliner: Eminem"
echo "------------------------------------------------------"