use crate::rpc::client::{RpcRawTransaction, RpcTxIn, RpcTxOutEntry};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTransaction {
    pub txid: String,
    pub version: i32,
    pub locktime: u32,
    pub size: u64,
    pub vsize: u64,
    pub weight: u64,
    pub is_coinbase: bool,
    pub inputs: Vec<ParsedInput>,
    pub outputs: Vec<ParsedOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedInput {
    pub vin: u32,
    pub prev_txid: String,
    pub prev_vout: u32,
    pub sequence: u32,
    pub script_sig_asm: Option<String>,
    pub script_sig_hex: Option<String>,
    pub witness: Option<Vec<String>>,
    pub is_coinbase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedOutput {
    pub vout: u32,
    pub value_sats: i64,
    pub script_pubkey_asm: String,
    pub script_pubkey_hex: String,
    pub script_type: String,
    pub address: Option<String>,
}

pub fn parse_rpc_transaction(raw_tx: &RpcRawTransaction) -> Result<ParsedTransaction> {
    let mut is_coinbase = false;
    let mut inputs = Vec::with_capacity(raw_tx.vin.len());

    for (idx, vin) in raw_tx.vin.iter().enumerate() {
        let input = parse_rpc_input(idx as u32, vin);
        if input.is_coinbase {
            is_coinbase = true;
        }
        inputs.push(input);
    }

    let mut outputs = Vec::with_capacity(raw_tx.vout.len());
    for vout in &raw_tx.vout {
        outputs.push(parse_rpc_output(vout));
    }

    Ok(ParsedTransaction {
        txid: raw_tx.txid.clone(),
        version: raw_tx.version,
        locktime: raw_tx.locktime,
        size: raw_tx.size,
        vsize: raw_tx.vsize,
        weight: raw_tx.weight,
        is_coinbase,
        inputs,
        outputs,
    })
}

fn parse_rpc_input(vin_idx: u32, vin: &RpcTxIn) -> ParsedInput {
    if vin.coinbase.is_some() {
        return ParsedInput {
            vin: vin_idx,
            prev_txid: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            prev_vout: 0xffffffff,
            sequence: vin.sequence,
            script_sig_asm: None,
            script_sig_hex: vin.coinbase.clone(),
            witness: vin.txinwitness.clone(),
            is_coinbase: true,
        };
    }

    let prev_txid = vin.txid.clone().unwrap_or_else(|| {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    });
    let prev_vout = vin.vout.unwrap_or(0);

    let (script_sig_asm, script_sig_hex) = match &vin.script_sig {
        Some(sig) => (Some(sig.asm.clone()), Some(sig.hex.clone())),
        None => (None, None),
    };

    ParsedInput {
        vin: vin_idx,
        prev_txid,
        prev_vout,
        sequence: vin.sequence,
        script_sig_asm,
        script_sig_hex,
        witness: vin.txinwitness.clone(),
        is_coinbase: false,
    }
}

fn parse_rpc_output(vout: &RpcTxOutEntry) -> ParsedOutput {
    // Convert BTC floating point value to exact integer satoshis (1 BTC = 100,000,000 sats)
    let value_sats = (vout.value * 100_000_000.0).round() as i64;

    ParsedOutput {
        vout: vout.n,
        value_sats,
        script_pubkey_asm: vout.script_pub_key.asm.clone(),
        script_pubkey_hex: vout.script_pub_key.hex.clone(),
        script_type: vout.script_pub_key.script_type.clone(),
        address: vout.script_pub_key.address.clone(),
    }
}
