use bitcoin_utxo_backend::scripts::analyzer::{analyze_script_bytes, ScriptType};

#[test]
fn test_detect_p2pkh() {
    // 76 a9 14 <20 bytes> 88 ac
    let hex = "76a914111111111111111111111111111111111111111188ac";
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::P2pkh);
    assert!(!res.is_witness);
    assert!(res.is_spendable);
    assert_eq!(res.required_signatures, Some(1));
    assert!(res.script_asm.starts_with("OP_DUP OP_HASH160"));
}

#[test]
fn test_detect_p2sh() {
    // a9 14 <20 bytes> 87
    let hex = "a914222222222222222222222222222222222222222287";
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::P2sh);
    assert!(!res.is_witness);
    assert!(res.is_spendable);
    assert!(res.script_asm.starts_with("OP_HASH160"));
}

#[test]
fn test_detect_p2wpkh() {
    // 00 14 <20 bytes>
    let hex = "00143333333333333333333333333333333333333333";
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::P2wpkh);
    assert!(res.is_witness);
    assert!(res.is_spendable);
    assert_eq!(res.required_signatures, Some(1));
    assert!(res.script_asm.starts_with("OP_0"));
}

#[test]
fn test_detect_p2wsh() {
    // 00 20 <32 bytes>
    let hex = "00204444444444444444444444444444444444444444444444444444444444444444";
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::P2wsh);
    assert!(res.is_witness);
    assert!(res.is_spendable);
    assert!(res.script_asm.starts_with("OP_0"));
}

#[test]
fn test_detect_p2tr() {
    // 51 20 <32 bytes>
    let hex = "51205555555555555555555555555555555555555555555555555555555555555555";
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::P2tr);
    assert!(res.is_witness);
    assert!(res.is_spendable);
    assert_eq!(res.required_signatures, Some(1));
    assert!(res.script_asm.starts_with("OP_1"));
}

#[test]
fn test_detect_op_return() {
    // 6a 04 62656566
    let hex = "6a0462656566";
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::OpReturn);
    assert!(!res.is_spendable);
    assert!(res.script_asm.starts_with("OP_RETURN"));
}

#[test]
fn test_empty_script() {
    let bytes = vec![];
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::Unknown);
    assert!(!res.is_spendable);
    assert!(!res.is_witness);
    assert_eq!(res.required_signatures, None);
    assert!(res.script_asm.is_empty());
}

#[test]
fn test_detect_multisig() {
    // 1-of-2 multisig: 51 (OP_1) 21 <pubkey1 33 bytes> 21 <pubkey2 33 bytes> 52 (OP_2) ae (OP_CHECKMULTISIG)
    let pubkey1 = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let pubkey2 = "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5";
    let hex = format!("5121{}21{}52ae", pubkey1, pubkey2);
    let bytes = hex::decode(hex).unwrap();
    let res = analyze_script_bytes(&bytes);
    assert_eq!(res.script_type, ScriptType::Multisig);
    assert!(res.is_spendable);
    assert_eq!(res.required_signatures, Some(1));
    assert!(res.script_asm.contains("OP_CHECKMULTISIG"));
}
