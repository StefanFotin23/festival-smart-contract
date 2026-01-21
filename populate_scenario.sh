#!/bin/bash

# This script populates the smart contract with a multi-festival scenario.
# It uses the Makefile targets to interact with the smart contract.

# Exit script if any command fails
set -e

# --- Global Variables ---
TICKET_TOKEN_ID="FESTT-abcdef" # Replace with your actual ticket token identifier

# --- Script ---

echo "--- Setting Ticket Token Identifier (one-time setup) ---"
make set-ticket-token-identifier TOKEN_IDENTIFIER=${TICKET_TOKEN_ID}
echo "------------------------------------------------------"
sleep 10

# =================================================================
# FESTIVAL 1: Summer Fest 2024
# =================================================================
echo "--- Populating Festival 1: Summer Fest 2024 ---"

FESTIVAL_1_NAME="Summer Fest 2024"
FESTIVAL_1_START_TIME=$(date -d "2024-07-20 16:00:00" +%s)
FESTIVAL_1_END_TIME=$(date -d "2024-07-22 23:59:59" +%s)

echo "--- Creating Festival 1 ---"
make add-festival NAME="${FESTIVAL_1_NAME}" START_TIME=${FESTIVAL_1_START_TIME} END_TIME=${FESTIVAL_1_END_TIME} MAX_TICKETS=5000 TAX_NORMAL=5 TAX_SOLD_OUT=10
sleep 10

# We assume the first festival created is ID 1.
FESTIVAL_1_ID=1

echo "--- Adding Events to Festival 1 ---"
make add-event FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Opening Ceremony" LOCATION="Main Stage" START_TIME=$(date -d "2024-07-20 17:00:00" +%s) END_TIME=$(date -d "2024-07-20 18:00:00" +%s)
sleep 10
make add-event FESTIVAL_ID=${FESTIVAL_1_ID} NAME="DJ Set by AI-DJ" LOCATION="Electronic Tent" START_TIME=$(date -d "2024-07-20 22:00:00" +%s) END_TIME=$(date -d "2024-07-21 00:00:00" +%s)
sleep 10

echo "--- Adding Ticket Price to Festival 1 ---"
make add-ticket-price FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Early Bird" PHASE="Phase 1" PRICE=100000000000000000 # 0.1 EGLD
sleep 10

echo "--- Adding Flash Event to Festival 1 ---"
make add-flash-event FESTIVAL_ID=${FESTIVAL_1_ID} NAME="Surprise Artist" START_TIME=$(date -d "2024-07-21 18:00:00" +%s) END_TIME=$(date -d "2024-07-21 18:30:00" +%s) BONUS_POINTS=200
echo "--- Festival 1 population complete! ---"
echo "------------------------------------------------------"
sleep 10

# =================================================================
# FESTIVAL 2: Winter Bash 2024
# =================================================================
echo "--- Populating Festival 2: Winter Bash 2024 ---"

FESTIVAL_2_NAME="Winter Bash 2024"
FESTIVAL_2_START_TIME=$(date -d "2024-12-15 18:00:00" +%s)
FESTIVAL_2_END_TIME=$(date -d "2024-12-15 23:59:59" +%s)

echo "--- Creating Festival 2 ---"
make add-festival NAME="${FESTIVAL_2_NAME}" START_TIME=${FESTIVAL_2_START_TIME} END_TIME=${FESTIVAL_2_END_TIME} MAX_TICKETS=1500 TAX_NORMAL=8 TAX_SOLD_OUT=12
sleep 10

# We assume the second festival created is ID 2.
FESTIVAL_2_ID=2

echo "--- Adding Events to Festival 2 ---"
make add-event FESTIVAL_ID=${FESTIVAL_2_ID} NAME="Ice Sculpture Contest" LOCATION="Chill Zone" START_TIME=$(date -d "2024-12-15 19:00:00" +%s) END_TIME=$(date -d "2024-12-15 20:00:00" +%s)
sleep 10

echo "--- Adding Ticket Price to Festival 2 ---"
make add-ticket-price FESTIVAL_ID=${FESTIVAL_2_ID} NAME="General Access" PHASE="Standard" PRICE=250000000000000000 # 0.25 EGLD
echo "--- Festival 2 population complete! ---"
echo "------------------------------------------------------"
sleep 10

# =================================================================
# FESTIVAL 3: Spring Wave 2025
# =================================================================
echo "--- Populating Festival 3: Spring Wave 2025 ---"

FESTIVAL_3_NAME="Spring Wave 2025"
FESTIVAL_3_START_TIME=$(date -d "2025-04-10 12:00:00" +%s)
FESTIVAL_3_END_TIME=$(date -d "2025-04-12 23:00:00" +%s)

echo "--- Creating Festival 3 ---"
make add-festival NAME="${FESTIVAL_3_NAME}" START_TIME=${FESTIVAL_3_START_TIME} END_TIME=${FESTIVAL_3_END_TIME} MAX_TICKETS=10000 TAX_NORMAL=4 TAX_SOLD_OUT=8
sleep 10

# We assume the third festival created is ID 3.
FESTIVAL_3_ID=3

echo "--- Adding Events to Festival 3 ---"
make add-event FESTIVAL_ID=${FESTIVAL_3_ID} NAME="Beach Cleanup" LOCATION="Beach Stage" START_TIME=$(date -d "2025-04-10 13:00:00" +%s) END_TIME=$(date -d "2025-04-10 15:00:00" +%s)
sleep 10
make add-event FESTIVAL_ID=${FESTIVAL_3_ID} NAME="Sunset Bonfire" LOCATION="Beach Stage" START_TIME=$(date -d "2025-04-11 19:00:00" +%s) END_TIME=$(date -d "2025-04-11 21:00:00" +%s)
sleep 10

echo "--- Adding Ticket Prices to Festival 3 ---"
make add-ticket-price FESTIVAL_ID=${FESTIVAL_3_ID} NAME="Super Early Bird" PHASE="Launch" PRICE=50000000000000000 # 0.05 EGLD
sleep 10
make add-ticket-price FESTIVAL_ID=${FESTIVAL_3_ID} NAME="VIP Pass" PHASE="Premium" PRICE=500000000000000000 # 0.5 EGLD
echo "--- Festival 3 population complete! ---"
echo "------------------------------------------------------"

echo "--- All scenarios populated successfully! ---"

