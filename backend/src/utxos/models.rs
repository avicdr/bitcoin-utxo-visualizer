use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Outpoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Error, Debug)]
pub enum OutpointParseError {
    #[error("Invalid outpoint format, expected 'txid:vout'")]
    InvalidFormat,
    #[error("Invalid transaction ID hex length")]
    InvalidTxid,
    #[error("Invalid vout index number: {0}")]
    InvalidVout(#[from] std::num::ParseIntError),
}

impl fmt::Display for Outpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.txid, self.vout)
    }
}

impl FromStr for Outpoint {
    type Err = OutpointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(OutpointParseError::InvalidFormat);
        }

        let txid = parts[0].trim();
        if txid.len() != 64 {
            return Err(OutpointParseError::InvalidTxid);
        }

        let vout = parts[1].trim().parse::<u32>()?;

        Ok(Outpoint {
            txid: txid.to_string(),
            vout,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoDetail {
    pub txid: String,
    pub vout: i32,
    pub outpoint: String,
    pub value_sats: i64,
    pub value_btc: f64,
    pub script_pubkey_asm: String,
    pub script_pubkey_hex: String,
    pub script_type: String,
    pub address: Option<String>,
    pub is_spent: bool,
    pub confirmation_state: String,
    pub created_at_height: Option<i64>,
    pub created_at_time: Option<DateTime<Utc>>,
    pub spent_by: Option<SpendingReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingReference {
    pub spending_txid: String,
    pub spending_vin: i32,
    pub spent_at_height: Option<i64>,
    pub spent_at_time: Option<DateTime<Utc>>,
}
