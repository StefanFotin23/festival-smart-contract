#!/bin/bash

# This script populates the smart contract with a multi-festival scenario.
# It uses the Makefile targets to interact with the smart contract.

# Exit script if any command fails
set -e

# --- Configuration ---
# Time to wait between transactions to avoid "nonce too low" errors.
# Devnet block time is 6s. 20s is safe to ensure the previous Tx is processed.
SLEEP_TIME=5

# NOTE: If you want to test BUYING tickets later, this must be a valid Token ID 
# that your wallet owns and has the ESDTNFTCreate role for.
TICKET_TOKEN_ID="FESTT-123456" 

# Calculate timestamps
CURRENT_TIMESTAMP=$(date +%s)
ONE_DAY_IN_SECONDS=$((24 * 3600))
ONE_MONTH_IN_SECONDS=$((30 * ONE_DAY_IN_SECONDS))

# --- Script ---



echo "--- Setting Ticket Token Identifier ---"
make set-ticket-token-identifier TOKEN_IDENTIFIER=${TICKET_TOKEN_ID}
echo "Waiting ${SLEEP_TIME}s for transaction..."
sleep ${SLEEP_TIME}
echo "------------------------------------------------------"

# =================================================================
# FESTIVAL 1: Summer Fest 2024
# =================================================================
echo "--- Populating Festival 1: Summer Fest 2024 ---"

FESTIVAL_1_NAME="Summer Fest 2024"
FESTIVAL_1_START_TIME=$(date -d "2026-07-20 16:00:00" +%s)
FESTIVAL_1_END_TIME=$(date -d "2026-07-22 23:59:59" +%s)

echo "--- Creating Festival 1 ---"
make add-festival NAME="${FESTIVAL_1_NAME}" START_TIME=${FESTIVAL_1_START_TIME} END_TIME=${FESTIVAL_1_END_TIME} MAX_TICKETS=5000 TAX_NORMAL=5 TAX_SOLD_OUT=10
sleep ${SLEEP_TIME}

# We assume the first festival created is ID 1.
FESTIVAL_1_ID=1

echo "--- Adding Events to Festival 1 ---"
make add-event FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Opening Ceremony" LOCATION="Main Stage" START_TIME=$(date -d "2026-07-20 17:00:00" +%s) END_TIME=$(date -d "2026-07-20 18:00:00" +%s)
sleep ${SLEEP_TIME}

make add-event FESTIVAL_ID=${FESTIVAL_1_ID} NAME="DJ Set by AI-DJ" LOCATION="Electronic Tent" START_TIME=$(date -d "2026-07-20 22:00:00" +%s) END_TIME=$(date -d "2026-07-21 00:00:00" +%s)
sleep ${SLEEP_TIME}

echo "--- Adding Ticket Prices to Festival 1 ---"
# Full Pass Early Bird (Type 0)
make add-ticket-price FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Full Pass Early Bird" PHASE="Phase 1" PRICE=100000000000000000 SALE_START_TIME=${CURRENT_TIMESTAMP} SALE_END_TIME=$((CURRENT_TIMESTAMP + ONE_MONTH_IN_SECONDS)) TICKET_TYPE=0
sleep ${SLEEP_TIME}

# Day 1 Ticket (Type 1)
make add-ticket-price FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Day 1 Pass" PHASE="Daily" PRICE=50000000000000000 SALE_START_TIME=${CURRENT_TIMESTAMP} SALE_END_TIME=$((CURRENT_TIMESTAMP + ONE_MONTH_IN_SECONDS)) TICKET_TYPE=1
sleep ${SLEEP_TIME}

echo "--- Adding Flash Event to Festival 1 ---"
make add-flash-event FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Surprise Artist" START_TIME=$(date -d "2026-07-21 18:00:00" +%s) END_TIME=$(date -d "2026-07-21 18:30:00" +%s) BONUS_POINTS=200
echo "--- Festival 1 population complete! ---"
echo "------------------------------------------------------"
sleep ${SLEEP_TIME}

# =================================================================
# FESTIVAL 2: Winter Bash 2024
# =================================================================
echo "--- Populating Festival 2: Winter Bash 2024 ---"

FESTIVAL_2_NAME="Winter Bash 2024"
FESTIVAL_2_START_TIME=$(date -d "2026-12-15 18:00:00" +%s)
FESTIVAL_2_END_TIME=$(date -d "2026-12-15 23:59:59" +%s)

echo "--- Creating Festival 2 ---"
make add-festival NAME="${FESTIVAL_2_NAME}" START_TIME=${FESTIVAL_2_START_TIME} END_TIME=${FESTIVAL_2_END_TIME} MAX_TICKETS=1500 TAX_NORMAL=8 TAX_SOLD_OUT=12
sleep ${SLEEP_TIME}

# We assume the second festival created is ID 2.
FESTIVAL_2_ID=2

echo "--- Adding Events to Festival 2 ---"
make add-event FESTIVAL_ID=${FESTIVAL_2_ID} NAME="Ice Sculpture Contest" LOCATION="Chill Zone" START_TIME=$(date -d "2026-12-15 19:00:00" +%s) END_TIME=$(date -d "2026-12-15 20:00:00" +%s)
sleep ${SLEEP_TIME}

echo "--- Adding Ticket Prices to Festival 2 ---"
# General Access Full Pass (Type 0)
make add-ticket-price FESTIVAL_ID=${FESTIVAL_2_ID} NAME="General Access" PHASE="Standard" PRICE=250000000000000000 SALE_START_TIME=$((CURRENT_TIMESTAMP + ONE_MONTH_IN_SECONDS)) SALE_END_TIME=$((CURRENT_TIMESTAMP + 2 * ONE_MONTH_IN_SECONDS)) TICKET_TYPE=0
sleep ${SLEEP_TIME}

echo "--- Festival 2 population complete! ---"
echo "------------------------------------------------------"
sleep ${SLEEP_TIME}

# =================================================================
# FESTIVAL 3: Spring Wave 2025
# =================================================================
echo "--- Populating Festival 3: Spring Wave 2025 ---"

FESTIVAL_3_NAME="Spring Wave 2025"
FESTIVAL_3_START_TIME=$(date -d "2027-04-10 12:00:00" +%s)
FESTIVAL_3_END_TIME=$(date -d "2027-04-12 23:00:00" +%s)

echo "--- Creating Festival 3 ---"
make add-festival NAME="${FESTIVAL_3_NAME}" START_TIME=${FESTIVAL_3_START_TIME} END_TIME=${FESTIVAL_3_END_TIME} MAX_TICKETS=10000 TAX_NORMAL=4 TAX_SOLD_OUT=8
sleep ${SLEEP_TIME}

# We assume the third festival created is ID 3.
FESTIVAL_3_ID=3

echo "--- Adding Events to Festival 3 ---"
make add-event FESTIVAL_ID=${FESTIVAL_3_ID} NAME="Beach Cleanup" LOCATION="Beach Stage" START_TIME=$(date -d "2027-04-10 13:00:00" +%s) END_TIME=$(date -d "2027-04-10 15:00:00" +%s)
sleep ${SLEEP_TIME}

make add-event FESTIVAL_ID=${FESTIVAL_3_ID} NAME="Sunset Bonfire" LOCATION="Beach Stage" START_TIME=$(date -d "2027-04-11 19:00:00" +%s) END_TIME=$(date -d "2027-04-11 21:00:00" +%s)
sleep ${SLEEP_TIME}

echo "--- Adding Ticket Prices to Festival 3 ---"
# Super Early Bird Full Pass (Type 0)
make add-ticket-price FESTIVAL_ID=${FESTIVAL_3_ID} NAME="Super Early Bird" PHASE="Launch" PRICE=50000000000000000 SALE_START_TIME=$((CURRENT_TIMESTAMP + 2 * ONE_MONTH_IN_SECONDS)) SALE_END_TIME=$((CURRENT_TIMESTAMP + 3 * ONE_MONTH_IN_SECONDS)) TICKET_TYPE=0
sleep ${SLEEP_TIME}

# VIP Pass Full Pass (Type 0)
make add-ticket-price FESTIVAL_ID=${FESTIVAL_3_ID} NAME="VIP Pass" PHASE="Premium" PRICE=500000000000000000 SALE_START_TIME=$((CURRENT_TIMESTAMP + 3 * ONE_MONTH_IN_SECONDS)) SALE_END_TIME=$((CURRENT_TIMESTAMP + 4 * ONE_MONTH_IN_SECONDS)) TICKET_TYPE=0
sleep ${SLEEP_TIME}

echo "--- Festival 3 population complete! ---"
echo "------------------------------------------------------"

echo "--- All scenarios populated successfully! ---"