use crate::api::error::ApiError;
use crate::scripts::analyzer::{analyze_script_bytes, ScriptAnalysis};
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DecodeScriptQuery {
    pub hex: String,
}

pub async fn decode_script(
    Query(query): Query<DecodeScriptQuery>,
) -> Result<Json<ScriptAnalysis>, ApiError> {
    let bytes = hex::decode(query.hex.trim())
        .map_err(|e| ApiError::BadRequest(format!("Invalid script hex encoding: {}", e)))?;

    let analysis = analyze_script_bytes(&bytes);
    Ok(Json(analysis))
}
