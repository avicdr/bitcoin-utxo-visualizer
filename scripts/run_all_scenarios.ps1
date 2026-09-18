# ==============================================================================
# Bitcoin UTXO Visualizer - Deterministic Regtest Scenarios Generator (PowerShell)
# ==============================================================================
# Scenarios covered:
# 1. Coinbase Spend (Mining 101 blocks to mature coinbase -> spend)
# 2. Multi-Input Consolidation (5 UTXOs -> 1 UTXO)
# 3. Fan-Out Transaction (1 UTXO -> 5 UTXOs)
# 4. Confirmed Transaction Chain (TX_A -> TX_B -> TX_C in block)
# 5. Unconfirmed Mempool Chain (TX_D -> TX_E -> TX_F in mempool)
# 6. Replace-By-Fee (RBF) replacement
# 7. Child-Pays-For-Parent (CPFP) incentive package
# ==============================================================================

$ErrorActionPreference = "Stop"

function Invoke-BtcCli {
    param([string]$Args)
    $output = docker compose exec -T bitcoind bitcoin-cli -regtest $Args
    return $output
}

Write-Host "==> Initializing Bitcoin Core Regtest Environment..." -ForegroundColor Cyan

$walletName = "visualizer_regtest"
Write-Host "==> Loading or creating wallet: $walletName" -ForegroundColor Cyan
try {
    docker compose exec -T bitcoind bitcoin-cli -regtest loadwallet $walletName | Out-Null
} catch {
    try {
        docker compose exec -T bitcoind bitcoin-cli -regtest createwallet $walletName | Out-Null
    } catch {
        # Already loaded or created
    }
}

$minerAddr = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "miner" "bech32").Trim()
Write-Host "Miner address: $minerAddr"

Write-Host "==> Maturing Coinbase Outputs (Mining 101 blocks)..." -ForegroundColor Cyan
docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 101 $minerAddr | Out-Null

# ------------------------------------------------------------------------------
# Scenario 1: Coinbase Spend
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 1] Generating Coinbase Spend..." -ForegroundColor Green
$recip1 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "coinbase_spend" "bech32").Trim()
$txid1 = (docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $recip1 25.0).Trim()
docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 1 $minerAddr | Out-Null
Write-Host "Scenario 1 confirmed in block. TxID: $txid1" -ForegroundColor Yellow

# ------------------------------------------------------------------------------
# Scenario 2: Multi-Input Consolidation
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 2] Generating Multi-Input Consolidation..." -ForegroundColor Green
$addrCons1 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "cons_prep" "bech32").Trim()
$addrCons2 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "cons_prep" "bech32").Trim()
$addrCons3 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "cons_prep" "bech32").Trim()

docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $addrCons1 2.0 | Out-Null
docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $addrCons2 3.0 | Out-Null
docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $addrCons3 4.0 | Out-Null
docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 1 $minerAddr | Out-Null

$consTarget = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "consolidated_vault" "bech32m").Trim()
$txidCons = (docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $consTarget 8.999 "").Trim()
docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 1 $minerAddr | Out-Null
Write-Host "Scenario 2 confirmed in block. Consolidated TxID: $txidCons" -ForegroundColor Yellow

# ------------------------------------------------------------------------------
# Scenario 3: Fan-Out Transaction
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 3] Generating Fan-Out Transaction..." -ForegroundColor Green
$fo1 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "fo1" "bech32").Trim()
$fo2 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "fo2" "bech32").Trim()
$fo3 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "fo3" "bech32").Trim()

$jsonAmounts = "{`"$fo1`": 1.0, `"$fo2`": 1.5, `"$fo3`": 2.0}"
$txidFanout = (docker compose exec -T bitcoind bitcoin-cli -regtest sendmany "" $jsonAmounts).Trim()
docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 1 $minerAddr | Out-Null
Write-Host "Scenario 3 confirmed in block. Fanout TxID: $txidFanout" -ForegroundColor Yellow

# ------------------------------------------------------------------------------
# Scenario 4: Confirmed Transaction Chain (TX_A -> TX_B -> TX_C)
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 4] Generating Confirmed Transaction Chain..." -ForegroundColor Green
$chainA = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "chain_a" "bech32").Trim()
$chainB = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "chain_b" "bech32").Trim()
$chainC = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "chain_c" "bech32").Trim()

$txidA = (docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $chainA 5.0).Trim()
$inputsB = "[{`"txid`":`"$txidA`",`"vout`":0}]"
$outputsB = "{`"$chainB`":4.999}"
$rawB = (docker compose exec -T bitcoind bitcoin-cli -regtest createrawtransaction $inputsB $outputsB).Trim()
$signedBJson = docker compose exec -T bitcoind bitcoin-cli -regtest signrawtransactionwithwallet $rawB | ConvertFrom-Json
$txidB = (docker compose exec -T bitcoind bitcoin-cli -regtest sendrawtransaction $signedBJson.hex).Trim()

$inputsC = "[{`"txid`":`"$txidB`",`"vout`":0}]"
$outputsC = "{`"$chainC`":4.998}"
$rawC = (docker compose exec -T bitcoind bitcoin-cli -regtest createrawtransaction $inputsC $outputsC).Trim()
$signedCJson = docker compose exec -T bitcoind bitcoin-cli -regtest signrawtransactionwithwallet $rawC | ConvertFrom-Json
$txidC = (docker compose exec -T bitcoind bitcoin-cli -regtest sendrawtransaction $signedCJson.hex).Trim()

docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 1 $minerAddr | Out-Null
Write-Host "Scenario 4 confirmed in block. Chain: $txidA -> $txidB -> $txidC" -ForegroundColor Yellow

# ------------------------------------------------------------------------------
# Scenario 5: Unconfirmed Transaction Chain (In Mempool)
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 5] Generating Unconfirmed Transaction Chain in Mempool..." -ForegroundColor Green
$mem1 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "mem1" "bech32").Trim()
$mem2 = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "mem2" "bech32").Trim()

$txidMem1 = (docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $mem1 3.0).Trim()
$inputsMem2 = "[{`"txid`":`"$txidMem1`",`"vout`":0}]"
$outputsMem2 = "{`"$mem2`":2.999}"
$rawMem2 = (docker compose exec -T bitcoind bitcoin-cli -regtest createrawtransaction $inputsMem2 $outputsMem2).Trim()
$signedMem2Json = docker compose exec -T bitcoind bitcoin-cli -regtest signrawtransactionwithwallet $rawMem2 | ConvertFrom-Json
$txidMem2 = (docker compose exec -T bitcoind bitcoin-cli -regtest sendrawtransaction $signedMem2Json.hex).Trim()
Write-Host "Scenario 5 in mempool (0-conf): $txidMem1 -> $txidMem2" -ForegroundColor Yellow

# ------------------------------------------------------------------------------
# Scenario 6: Replace-By-Fee (RBF)
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 6] Generating Replace-By-Fee (RBF)..." -ForegroundColor Green
$rbfDest = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "rbf_dest" "bech32").Trim()
$rbfOrigTxid = (docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $rbfDest 1.0 "" "" $false $true 1).Trim()
Write-Host "Original RBF tx: $rbfOrigTxid"
$rbfBumpRes = docker compose exec -T bitcoind bitcoin-cli -regtest bumpfee $rbfOrigTxid | ConvertFrom-Json
$rbfNewTxid = $rbfBumpRes.txid
Write-Host "Scenario 6 replaced: $rbfOrigTxid replaced by $rbfNewTxid" -ForegroundColor Yellow

# ------------------------------------------------------------------------------
# Scenario 7: Child-Pays-For-Parent (CPFP)
# ------------------------------------------------------------------------------
Write-Host "==> [Scenario 7] Generating Child-Pays-For-Parent (CPFP)..." -ForegroundColor Green
$cpfpParentAddr = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "cpfp_parent" "bech32").Trim()
$cpfpParentTxid = (docker compose exec -T bitcoind bitcoin-cli -regtest sendtoaddress $cpfpParentAddr 2.0 "" "" $false $false 1000).Trim()
$cpfpChildAddr = (docker compose exec -T bitcoind bitcoin-cli -regtest getnewaddress "cpfp_child" "bech32").Trim()
$rawCpfpChild = (docker compose exec -T bitcoind bitcoin-cli -regtest createrawtransaction "[{`"txid`":`"$cpfpParentTxid`",`"vout`":0}]" "{`"$cpfpChildAddr`":1.99}").Trim()
$signedCpfpChild = docker compose exec -T bitcoind bitcoin-cli -regtest signrawtransactionwithwallet $rawCpfpChild | ConvertFrom-Json
$cpfpChildTxid = (docker compose exec -T bitcoind bitcoin-cli -regtest sendrawtransaction $signedCpfpChild.hex).Trim()
Write-Host "Scenario 7 CPFP in mempool: Parent=$cpfpParentTxid, Child=$cpfpChildTxid" -ForegroundColor Yellow

Write-Host "==> All 7 Regtest UTXO scenarios generated successfully!" -ForegroundColor Cyan
