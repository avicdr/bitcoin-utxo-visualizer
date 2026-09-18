use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptType {
    P2pkh,
    P2sh,
    P2wpkh,
    P2wsh,
    P2tr,
    OpReturn,
    Multisig,
    Unknown,
}

impl ScriptType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScriptType::P2pkh => "p2pkh",
            ScriptType::P2sh => "p2sh",
            ScriptType::P2wpkh => "p2wpkh",
            ScriptType::P2wsh => "p2wsh",
            ScriptType::P2tr => "p2tr",
            ScriptType::OpReturn => "op_return",
            ScriptType::Multisig => "multisig",
            ScriptType::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptAnalysis {
    pub script_hex: String,
    pub script_asm: String,
    pub script_type: ScriptType,
    pub is_witness: bool,
    pub is_spendable: bool,
    pub required_signatures: Option<usize>,
    pub explanation: String,
}

pub fn analyze_script_bytes(bytes: &[u8]) -> ScriptAnalysis {
    let script_hex = hex::encode(bytes);

    // 0. Empty script
    if bytes.is_empty() {
        return ScriptAnalysis {
            script_hex,
            script_asm: String::new(),
            script_type: ScriptType::Unknown,
            is_witness: false,
            is_spendable: false,
            required_signatures: None,
            explanation: "Empty script.".to_string(),
        };
    }

    // 1. OP_RETURN Check (0x6a)
    if bytes.first() == Some(&0x6a) {
        let asm = disassemble(bytes);
        return ScriptAnalysis {
            script_hex,
            script_asm: asm,
            script_type: ScriptType::OpReturn,
            is_witness: false,
            is_spendable: false,
            required_signatures: None,
            explanation: "Provably unspendable data-carrying output (OP_RETURN). Destroys satoshis to embed arbitrary data into the blockchain.".to_string(),
        };
    }

    // 2. P2WPKH: 0x00 0x14 {20 bytes} (Total 22 bytes)
    if bytes.len() == 22 && bytes[0] == 0x00 && bytes[1] == 0x14 {
        return ScriptAnalysis {
            script_hex,
            script_asm: format!("OP_0 {}", hex::encode(&bytes[2..])),
            script_type: ScriptType::P2wpkh,
            is_witness: true,
            is_spendable: true,
            required_signatures: Some(1),
            explanation: "Native SegWit v0 Public Key Hash (P2WPKH). Unlocked by providing witness stack containing ECDSA signature and uncompressed/compressed public key matching the 20-byte hash.".to_string(),
        };
    }

    // 3. P2WSH: 0x00 0x20 {32 bytes} (Total 34 bytes)
    if bytes.len() == 34 && bytes[0] == 0x00 && bytes[1] == 0x20 {
        return ScriptAnalysis {
            script_hex,
            script_asm: format!("OP_0 {}", hex::encode(&bytes[2..])),
            script_type: ScriptType::P2wsh,
            is_witness: true,
            is_spendable: true,
            required_signatures: None,
            explanation: "Native SegWit v0 Script Hash (P2WSH). Unlocked by providing witness stack satisfying the witnessScript whose SHA256 hash matches the 32-byte commitment.".to_string(),
        };
    }

    // 4. P2TR: 0x51 0x20 {32 bytes} (Total 34 bytes)
    if bytes.len() == 34 && bytes[0] == 0x51 && bytes[1] == 0x20 {
        return ScriptAnalysis {
            script_hex,
            script_asm: format!("OP_1 {}", hex::encode(&bytes[2..])),
            script_type: ScriptType::P2tr,
            is_witness: true,
            is_spendable: true,
            required_signatures: Some(1),
            explanation: "Taproot SegWit v1 (P2TR - BIP 341/342). Unlocked via key-path spending with a single 64-byte Schnorr signature, or via Merkle script-path tree revelation.".to_string(),
        };
    }

    // 5. P2PKH: 0x76 0xa9 0x14 {20 bytes} 0x88 0xac (Total 25 bytes)
    if bytes.len() == 25
        && bytes[0] == 0x76 // OP_DUP
        && bytes[1] == 0xa9 // OP_HASH160
        && bytes[2] == 0x14 // Push 20 bytes
        && bytes[23] == 0x88 // OP_EQUALVERIFY
        && bytes[24] == 0xac
    // OP_CHECKSIG
    {
        return ScriptAnalysis {
            script_hex,
            script_asm: format!(
                "OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG",
                hex::encode(&bytes[3..23])
            ),
            script_type: ScriptType::P2pkh,
            is_witness: false,
            is_spendable: true,
            required_signatures: Some(1),
            explanation: "Legacy Pay to Public Key Hash (P2PKH). Unlocked by scriptSig containing signature and public key.".to_string(),
        };
    }

    // 6. P2SH: 0xa9 0x14 {20 bytes} 0x87 (Total 23 bytes)
    if bytes.len() == 23
        && bytes[0] == 0xa9 // OP_HASH160
        && bytes[1] == 0x14 // Push 20 bytes
        && bytes[22] == 0x87
    // OP_EQUAL
    {
        return ScriptAnalysis {
            script_hex,
            script_asm: format!("OP_HASH160 {} OP_EQUAL", hex::encode(&bytes[2..22])),
            script_type: ScriptType::P2sh,
            is_witness: false,
            is_spendable: true,
            required_signatures: None,
            explanation: "Pay to Script Hash (P2SH - BIP 16). Unlocked by providing serialized redeemScript matching the 20-byte HASH160.".to_string(),
        };
    }

    // 7. Multisig: OP_M ... OP_N OP_CHECKMULTISIG
    if bytes.len() >= 3 && bytes.last() == Some(&0xae) {
        let first_byte = bytes[0];
        let n_byte = bytes[bytes.len() - 2];
        if (0x51..=0x60).contains(&first_byte) && (0x51..=0x60).contains(&n_byte) {
            let m = (first_byte - 0x50) as usize;
            let n = (n_byte - 0x50) as usize;
            if m <= n {
                let asm = disassemble(bytes);
                return ScriptAnalysis {
                    script_hex,
                    script_asm: asm,
                    script_type: ScriptType::Multisig,
                    is_witness: false,
                    is_spendable: true,
                    required_signatures: Some(m),
                    explanation: format!(
                        "Bare Multisig ({}-of-{}). Unlocked by providing {} valid signatures matching the set of {} public keys.",
                        m, n, m, n
                    ),
                };
            }
        }
    }

    // 8. General Opcode Disassembly
    let asm = disassemble(bytes);
    ScriptAnalysis {
        script_hex,
        script_asm: asm,
        script_type: ScriptType::Unknown,
        is_witness: false,
        is_spendable: true,
        required_signatures: None,
        explanation: "Non-standard or custom Bitcoin script.".to_string(),
    }
}

fn disassemble(bytes: &[u8]) -> String {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let op = bytes[i];
        i += 1;

        match op {
            0x00 => tokens.push("OP_0".to_string()),
            0x51..=0x60 => tokens.push(format!("OP_{}", op - 0x50)),
            0x6a => tokens.push("OP_RETURN".to_string()),
            0x76 => tokens.push("OP_DUP".to_string()),
            0xa9 => tokens.push("OP_HASH160".to_string()),
            0x88 => tokens.push("OP_EQUALVERIFY".to_string()),
            0x87 => tokens.push("OP_EQUAL".to_string()),
            0xac => tokens.push("OP_CHECKSIG".to_string()),
            0xae => tokens.push("OP_CHECKMULTISIG".to_string()),
            1..=75 => {
                let len = op as usize;
                if i + len <= bytes.len() {
                    tokens.push(hex::encode(&bytes[i..i + len]));
                    i += len;
                } else {
                    tokens.push(format!("OP_UNKNOWN_0x{:02x}", op));
                }
            }
            _ => tokens.push(format!("OP_0x{:02x}", op)),
        }
    }

    tokens.join(" ")
}
