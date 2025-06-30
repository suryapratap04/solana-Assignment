use axum::Json;
use solana_sdk::signature::{Keypair, Signer};
use crate::utils::encode_base58;
use serde_json::json;

pub async fn generate_keypair() -> Json<serde_json::Value> {
    let keypair = Keypair::new();
    let pubkey = encode_base58(&keypair.pubkey().to_bytes());
    let secret = encode_base58(&keypair.to_bytes());

    Json(json!({
        "public_key": pubkey,
        "secret_key": secret,
    }))
}
