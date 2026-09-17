-- Initial PostgreSQL Schema for Bitcoin UTXO Visualizer

-- 1. Blocks Table
CREATE TABLE IF NOT EXISTS blocks (
    hash VARCHAR(64) PRIMARY KEY,
    height BIGINT NOT NULL UNIQUE,
    prev_hash VARCHAR(64),
    timestamp TIMESTAMPTZ NOT NULL,
    tx_count INTEGER NOT NULL DEFAULT 0,
    size INTEGER NOT NULL DEFAULT 0,
    weight INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_blocks_height ON blocks(height);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp);

-- 2. Transactions Table
CREATE TABLE IF NOT EXISTS transactions (
    txid VARCHAR(64) PRIMARY KEY,
    block_hash VARCHAR(64) REFERENCES blocks(hash) ON DELETE SET NULL,
    block_height BIGINT,
    version INTEGER NOT NULL DEFAULT 2,
    locktime BIGINT NOT NULL DEFAULT 0,
    size INTEGER NOT NULL DEFAULT 0,
    vsize INTEGER NOT NULL DEFAULT 0,
    weight INTEGER NOT NULL DEFAULT 0,
    is_coinbase BOOLEAN NOT NULL DEFAULT FALSE,
    fee BIGINT, -- in satoshis (NULL for coinbase transactions)
    fee_rate DOUBLE PRECISION, -- satoshis per vByte
    status VARCHAR(20) NOT NULL DEFAULT 'confirmed', -- 'confirmed', 'mempool', 'replaced'
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_transactions_block_height ON transactions(block_height);
CREATE INDEX IF NOT EXISTS idx_transactions_status ON transactions(status);
CREATE INDEX IF NOT EXISTS idx_transactions_first_seen ON transactions(first_seen);

-- 3. Transaction Outputs (The UTXO state and historical outputs table)
-- Every output created by a transaction begins its life here.
-- While is_spent = false, it represents an active UTXO.
-- When consumed by an input, is_spent is set to true and spent_by_txid/spent_by_vin are recorded.
CREATE TABLE IF NOT EXISTS transaction_outputs (
    txid VARCHAR(64) NOT NULL REFERENCES transactions(txid) ON DELETE CASCADE,
    vout INTEGER NOT NULL,
    value BIGINT NOT NULL, -- satoshis (8 decimal places in BTC)
    script_pubkey BYTEA NOT NULL,
    script_pubkey_asm TEXT NOT NULL,
    script_type VARCHAR(32) NOT NULL DEFAULT 'unknown', -- p2pkh, p2sh, p2wpkh, p2wsh, p2tr, op_return, unknown
    address VARCHAR(120),
    is_spent BOOLEAN NOT NULL DEFAULT FALSE,
    spent_by_txid VARCHAR(64),
    spent_by_vin INTEGER,
    spent_at_height BIGINT,
    spent_at_time TIMESTAMPTZ,
    PRIMARY KEY (txid, vout)
);
CREATE INDEX IF NOT EXISTS idx_outputs_is_spent ON transaction_outputs(is_spent);
CREATE INDEX IF NOT EXISTS idx_outputs_address ON transaction_outputs(address);
CREATE INDEX IF NOT EXISTS idx_outputs_script_type ON transaction_outputs(script_type);
CREATE INDEX IF NOT EXISTS idx_outputs_spent_by_txid ON transaction_outputs(spent_by_txid);
CREATE INDEX IF NOT EXISTS idx_outputs_value ON transaction_outputs(value);

-- 4. Transaction Inputs (Spending references pointing to previous outpoints)
CREATE TABLE IF NOT EXISTS transaction_inputs (
    txid VARCHAR(64) NOT NULL REFERENCES transactions(txid) ON DELETE CASCADE,
    vin INTEGER NOT NULL,
    prev_txid VARCHAR(64) NOT NULL,
    prev_vout INTEGER NOT NULL,
    sequence BIGINT NOT NULL,
    script_sig BYTEA,
    script_sig_asm TEXT,
    witness_items JSONB,
    value BIGINT, -- resolved satoshi value of the spent output
    PRIMARY KEY (txid, vin)
);
CREATE INDEX IF NOT EXISTS idx_inputs_prevout ON transaction_inputs(prev_txid, prev_vout);

-- 5. Mempool Events Log
CREATE TABLE IF NOT EXISTS mempool_events (
    id BIGSERIAL PRIMARY KEY,
    txid VARCHAR(64) NOT NULL,
    event_type VARCHAR(32) NOT NULL, -- 'tx_added', 'tx_confirmed', 'tx_rbf_replaced', 'tx_dropped'
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    details JSONB
);
CREATE INDEX IF NOT EXISTS idx_mempool_events_txid ON mempool_events(txid);
CREATE INDEX IF NOT EXISTS idx_mempool_events_time ON mempool_events(timestamp);

-- 6. Sync State & Metadata
CREATE TABLE IF NOT EXISTS sync_state (
    key VARCHAR(64) PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
