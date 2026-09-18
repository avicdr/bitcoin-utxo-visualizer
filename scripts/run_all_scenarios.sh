#!/usr/bin/env bash
# ==============================================================================
# Bitcoin UTXO Visualizer - Deterministic Regtest Scenarios Generator
# ==============================================================================
# Scenarios covered:
# 1. Coinbase Spend (Mining 101 blocks to mature coinbase -> spend)
# 2. Multi-Input Consolidation (5 UTXOs -> 1 UTXO)
# 3. Fan-Out Transaction (1 UTXO -> 8 UTXOs)
# 4. Confirmed Transaction Chain (TX_A -> TX_B -> TX_C in block)
# 5. Unconfirmed Mempool Chain (TX_D -> TX_E -> TX_F in mempool)
# 6. Replace-By-Fee (RBF) replacement
# 7. Child-Pays-For-Parent (CPFP) incentive package
# ==============================================================================

set -euo pipefail

CLI="${BITCOIN_CLI:-docker compose exec -T bitcoind bitcoin-cli -regtest}"

echo "==> Initializing Bitcoin Core Regtest Environment..."
$CLI ping > /dev/null 2>&1 || { echo "Bitcoin Core node is not responding. Ensure bitcoind is running."; exit 1; }

WALLET_NAME="visualizer_regtest"
echo "==> Loading or creating wallet: $WALLET_NAME"
$CLI loadwallet "$WALLET_NAME" > /dev/null 2>&1 || $CLI createwallet "$WALLET_NAME" > /dev/null 2>&1 || true

MINER_ADDR=$($CLI getnewaddress "miner" "bech32")
echo "Miner address: $MINER_ADDR"

echo "==> Maturing Coinbase Outputs (Mining 101 blocks)..."
$CLI generatetoaddress 101 "$MINER_ADDR" > /dev/null

# ------------------------------------------------------------------------------
# Scenario 1: Coinbase Spend
# ------------------------------------------------------------------------------
echo "==> [Scenario 1] Generating Coinbase Spend..."
RECIPIENT_1=$($CLI getnewaddress "coinbase_spend" "bech32")
TXID_1=$($CLI sendtoaddress "$RECIPIENT_1" 25.0)
$CLI generatetoaddress 1 "$MINER_ADDR" > /dev/null
echo "Scenario 1 confirmed in block. TxID: $TXID_1"

# ------------------------------------------------------------------------------
# Scenario 2: Multi-Input Consolidation
# ------------------------------------------------------------------------------
echo "==> [Scenario 2] Generating Multi-Input Consolidation..."
ADDR_CONS_1=$($CLI getnewaddress "cons_prep" "bech32")
ADDR_CONS_2=$($CLI getnewaddress "cons_prep" "bech32")
ADDR_CONS_3=$($CLI getnewaddress "cons_prep" "bech32")

TXID_P1=$($CLI sendtoaddress "$ADDR_CONS_1" 2.0)
TXID_P2=$($CLI sendtoaddress "$ADDR_CONS_2" 3.0)
TXID_P3=$($CLI sendtoaddress "$ADDR_CONS_3" 4.0)
$CLI generatetoaddress 1 "$MINER_ADDR" > /dev/null

CONSOLIDATION_TARGET=$($CLI getnewaddress "consolidated_vault" "bech32m") # Taproot
TXID_CONSOLIDATION=$($CLI sendtoaddress "$CONSOLIDATION_TARGET" 8.999 "")
$CLI generatetoaddress 1 "$MINER_ADDR" > /dev/null
echo "Scenario 2 confirmed in block. Consolidated TxID: $TXID_CONSOLIDATION"

# ------------------------------------------------------------------------------
# Scenario 3: Fan-Out Transaction
# ------------------------------------------------------------------------------
echo "==> [Scenario 3] Generating Fan-Out Transaction (1 -> 5 outputs)..."
FO_1=$($CLI getnewaddress "fanout_1" "bech32")
FO_2=$($CLI getnewaddress "fanout_2" "bech32")
FO_3=$($CLI getnewaddress "fanout_3" "bech32")
FO_4=$($CLI getnewaddress "fanout_4" "bech32")
FO_5=$($CLI getnewaddress "fanout_5" "bech32")

FANOUT_AMOUNTS="{\"$FO_1\": 0.5, \"$FO_2\": 0.75, \"$FO_3\": 1.0, \"$FO_4\": 1.25, \"$FO_5\": 1.5}"
TXID_FANOUT=$($CLI sendmany "" "$FANOUT_AMOUNTS")
$CLI generatetoaddress 1 "$MINER_ADDR" > /dev/null
echo "Scenario 3 confirmed in block. Fanout TxID: $TXID_FANOUT"

# ------------------------------------------------------------------------------
# Scenario 4: Confirmed Transaction Chain (TX_A -> TX_B -> TX_C)
# ------------------------------------------------------------------------------
echo "==> [Scenario 4] Generating Confirmed Transaction Chain (TX_A -> TX_B -> TX_C)..."
CHAIN_ADDR_A=$($CLI getnewaddress "chain_a" "bech32")
CHAIN_ADDR_B=$($CLI getnewaddress "chain_b" "bech32")
CHAIN_ADDR_C=$($CLI getnewaddress "chain_c" "bech32")

TXID_A=$($CLI sendtoaddress "$CHAIN_ADDR_A" 5.0)
TX_A_HEX=$($CLI getrawtransaction "$TXID_A" 1)
VOUT_A=0
RAW_TX_B=$($CLI createrawtransaction "[{\"txid\":\"$TXID_A\",\"vout\":$VOUT_A}]" "{\"$CHAIN_ADDR_B\":4.999}")
SIGNED_B=$($CLI signrawtransactionwithwallet "$RAW_TX_B")
TXID_B=$($CLI sendrawtransaction "$(echo "$SIGNED_B" | grep -oP '"hex":\s*"\K[^"]+')")

RAW_TX_C=$($CLI createrawtransaction "[{\"txid\":\"$TXID_B\",\"vout\":0}]" "{\"$CHAIN_ADDR_C\":4.998}")
SIGNED_C=$($CLI signrawtransactionwithwallet "$RAW_TX_C")
TXID_C=$($CLI sendrawtransaction "$(echo "$SIGNED_C" | grep -oP '"hex":\s*"\K[^"]+')")

$CLI generatetoaddress 1 "$MINER_ADDR" > /dev/null
echo "Scenario 4 confirmed in block. Chain: $TXID_A -> $TXID_B -> $TXID_C"

# ------------------------------------------------------------------------------
# Scenario 5: Unconfirmed Transaction Chain (In Mempool)
# ------------------------------------------------------------------------------
echo "==> [Scenario 5] Generating Unconfirmed Transaction Chain in Mempool..."
MEM_ADDR_1=$($CLI getnewaddress "mem_chain_1" "bech32")
MEM_ADDR_2=$($CLI getnewaddress "mem_chain_2" "bech32")

TXID_MEM_1=$($CLI sendtoaddress "$MEM_ADDR_1" 3.0)
RAW_TX_MEM_2=$($CLI createrawtransaction "[{\"txid\":\"$TXID_MEM_1\",\"vout\":0}]" "{\"$MEM_ADDR_2\":2.999}")
SIGNED_MEM_2=$($CLI signrawtransactionwithwallet "$RAW_TX_MEM_2")
TXID_MEM_2=$($CLI sendrawtransaction "$(echo "$SIGNED_MEM_2" | grep -oP '"hex":\s*"\K[^"]+')")
echo "Scenario 5 in mempool (0-conf): $TXID_MEM_1 -> $TXID_MEM_2"

# ------------------------------------------------------------------------------
# Scenario 6: Replace-By-Fee (RBF)
# ------------------------------------------------------------------------------
echo "==> [Scenario 6] Generating Replace-By-Fee (RBF)..."
RBF_RECIPIENT=$($CLI getnewaddress "rbf_dest" "bech32")
RBF_ORIGINAL_TXID=$($CLI sendtoaddress "$RBF_RECIPIENT" 1.0 "" "" false true 1)
echo "Original RBF tx: $RBF_ORIGINAL_TXID (in mempool)"
RBF_BUMP_RES=$($CLI bumpfee "$RBF_ORIGINAL_TXID")
RBF_NEW_TXID=$(echo "$RBF_BUMP_RES" | grep -oP '"txid":\s*"\K[^"]+')
echo "Scenario 6 replaced: $RBF_ORIGINAL_TXID replaced by $RBF_NEW_TXID"

# ------------------------------------------------------------------------------
# Scenario 7: Child-Pays-For-Parent (CPFP)
# ------------------------------------------------------------------------------
echo "==> [Scenario 7] Generating Child-Pays-For-Parent (CPFP)..."
CPFP_PARENT_ADDR=$($CLI getnewaddress "cpfp_parent" "bech32")
CPFP_PARENT_TXID=$($CLI sendtoaddress "$CPFP_PARENT_ADDR" 2.0 "" "" false false 1000)
CPFP_CHILD_ADDR=$($CLI getnewaddress "cpfp_child" "bech32")
RAW_CPFP_CHILD=$($CLI createrawtransaction "[{\"txid\":\"$CPFP_PARENT_TXID\",\"vout\":0}]" "{\"$CPFP_CHILD_ADDR\":1.99}")
SIGNED_CPFP_CHILD=$($CLI signrawtransactionwithwallet "$RAW_CPFP_CHILD")
CPFP_CHILD_TXID=$($CLI sendrawtransaction "$(echo "$SIGNED_CPFP_CHILD" | grep -oP '"hex":\s*"\K[^"]+')")
echo "Scenario 7 CPFP in mempool: Parent=$CPFP_PARENT_TXID, Child=$CPFP_CHILD_TXID"

echo "==> All 7 Regtest UTXO scenarios generated successfully!"
