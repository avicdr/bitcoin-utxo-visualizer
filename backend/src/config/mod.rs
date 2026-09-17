use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub bitcoin_rpc_url: String,
    pub bitcoin_rpc_user: String,
    pub bitcoin_rpc_password: String,
    pub bitcoin_network: String,
    pub bitcoin_zmq_rawtx: String,
    pub bitcoin_zmq_hashblock: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/bitcoin_utxo".to_string()
            }),
            bitcoin_rpc_url: env::var("BITCOIN_RPC_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:18443".to_string()),
            bitcoin_rpc_user: env::var("BITCOIN_RPC_USER")
                .unwrap_or_else(|_| "regtest".to_string()),
            bitcoin_rpc_password: env::var("BITCOIN_RPC_PASSWORD")
                .unwrap_or_else(|_| "regtest".to_string()),
            bitcoin_network: env::var("BITCOIN_NETWORK").unwrap_or_else(|_| "regtest".to_string()),
            bitcoin_zmq_rawtx: env::var("BITCOIN_ZMQ_RAWTX")
                .unwrap_or_else(|_| "tcp://127.0.0.1:28332".to_string()),
            bitcoin_zmq_hashblock: env::var("BITCOIN_ZMQ_HASHBLOCK")
                .unwrap_or_else(|_| "tcp://127.0.0.1:28333".to_string()),
        }
    }
}
