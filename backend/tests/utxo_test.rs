use bitcoin_utxo_backend::utxos::models::{Outpoint, OutpointParseError};
use std::str::FromStr;

#[test]
fn test_outpoint_valid_parsing() {
    let raw = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:0";
    let outpoint = Outpoint::from_str(raw).expect("Valid outpoint string must parse");
    assert_eq!(
        outpoint.txid,
        "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
    );
    assert_eq!(outpoint.vout, 0);
    assert_eq!(outpoint.to_string(), raw);
}

#[test]
fn test_outpoint_high_vout() {
    let raw = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:154";
    let outpoint = Outpoint::from_str(raw).expect("Valid outpoint with vout 154");
    assert_eq!(outpoint.vout, 154);
    assert_eq!(outpoint.to_string(), raw);
}

#[test]
fn test_outpoint_invalid_format() {
    let no_colon = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b";
    let res = Outpoint::from_str(no_colon);
    assert!(matches!(res, Err(OutpointParseError::InvalidFormat)));

    let multiple_colons = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:0:1";
    let res = Outpoint::from_str(multiple_colons);
    assert!(matches!(res, Err(OutpointParseError::InvalidFormat)));
}

#[test]
fn test_outpoint_invalid_txid_length() {
    let short_txid = "abc123:0";
    let res = Outpoint::from_str(short_txid);
    assert!(matches!(res, Err(OutpointParseError::InvalidTxid)));
}

#[test]
fn test_outpoint_invalid_vout_number() {
    let non_numeric_vout =
        "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:invalid";
    let res = Outpoint::from_str(non_numeric_vout);
    assert!(matches!(res, Err(OutpointParseError::InvalidVout(_))));
}
