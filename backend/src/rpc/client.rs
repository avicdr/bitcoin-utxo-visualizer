use anyhow::{anyhow, Result};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct BitcoinRpcClient {
    url: String,
    user: String,
    pass: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct JsonRpcRequest<'a, T> {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'a str,
    params: T,
}

#[derive(Deserialize, Debug)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: u64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub pruned: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkInfo {
    pub version: u64,
    pub subversion: String,
    pub protocolversion: u64,
    pub connections: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MempoolInfo {
    pub loaded: bool,
    pub size: u64,
    pub bytes: u64,
    pub usage: u64,
    pub maxmempool: u64,
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcBlockHeader {
    pub hash: String,
    pub confirmations: i64,
    pub height: u64,
    pub version: i32,
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u64,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcBlockVerbose {
    pub hash: String,
    pub confirmations: i64,
    pub size: u64,
    pub strippedsize: u64,
    pub weight: u64,
    pub height: u64,
    pub version: i32,
    pub merkleroot: String,
    pub tx: Vec<String>, // list of txids when verbosity = 1
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u64,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcTxOut {
    pub bestblock: String,
    pub confirmations: i64,
    pub value: f64,
    pub script_pub_key: RpcScriptPubKey,
    pub coinbase: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcScriptPubKey {
    pub asm: String,
    pub hex: String,
    #[serde(rename = "type")]
    pub script_type: String,
    pub address: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcRawTransaction {
    pub txid: String,
    pub hash: String,
    pub version: i32,
    pub size: u64,
    pub vsize: u64,
    pub weight: u64,
    pub locktime: u32,
    pub vin: Vec<RpcTxIn>,
    pub vout: Vec<RpcTxOutEntry>,
    pub hex: String,
    pub blockhash: Option<String>,
    pub confirmations: Option<i64>,
    pub time: Option<u64>,
    pub blocktime: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcTxIn {
    pub coinbase: Option<String>,
    pub txid: Option<String>,
    pub vout: Option<u32>,
    pub script_sig: Option<RpcScriptSig>,
    pub sequence: u32,
    pub txinwitness: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcTxOutEntry {
    pub value: f64,
    pub n: u32,
    pub script_pub_key: RpcScriptPubKey,
}

impl BitcoinRpcClient {
    pub fn new(url: String, user: String, pass: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        Self {
            url,
            user,
            pass,
            client,
        }
    }

    async fn call<P: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: P,
    ) -> Result<R> {
        let req_body = JsonRpcRequest {
            jsonrpc: "1.0",
            id: "bitcoin-utxo-visualizer",
            method,
            params,
        };

        let response = self
            .client
            .post(&self.url)
            .basic_auth(&self.user, Some(&self.pass))
            .header(CONTENT_TYPE, "application/json")
            .json(&req_body)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP request to Bitcoin Core RPC failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("RPC HTTP error {}: {}", status, text));
        }

        let rpc_res: JsonRpcResponse<R> = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to deserialize Bitcoin RPC JSON: {}", e))?;

        if let Some(err) = rpc_res.error {
            return Err(anyhow!("Bitcoin RPC Error [{}]: {}", err.code, err.message));
        }

        rpc_res
            .result
            .ok_or_else(|| anyhow!("RPC response contained null result"))
    }

    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.call("getblockchaininfo", ()).await
    }

    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        self.call("getnetworkinfo", ()).await
    }

    pub async fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.call("getmempoolinfo", ()).await
    }

    pub async fn get_raw_mempool(&self) -> Result<Vec<String>> {
        self.call("getrawmempool", ()).await
    }

    pub async fn get_block_hash(&self, height: u64) -> Result<String> {
        self.call("getblockhash", [height]).await
    }

    pub async fn get_block_verbose(&self, hash: &str) -> Result<RpcBlockVerbose> {
        self.call("getblock", (hash, 1)).await
    }

    pub async fn get_raw_transaction(
        &self,
        txid: &str,
        verbose: bool,
    ) -> Result<RpcRawTransaction> {
        self.call("getrawtransaction", (txid, verbose)).await
    }

    pub async fn get_tx_out(
        &self,
        txid: &str,
        vout: u32,
        include_mempool: bool,
    ) -> Result<Option<RpcTxOut>> {
        self.call("gettxout", (txid, vout, include_mempool)).await
    }
}
