use axum::{extract::Json, response::IntoResponse};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier, SECRET_KEY_LENGTH};
use crate::{models::{SuccessResponse, SignMessageRequest, VerifyMessageRequest}, utils::{decode_base58, encode_base64}, error::AppError};

pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> Result<impl IntoResponse, AppError> {
    let secret_bytes = decode_base58(&payload.secret).map_err(|_| AppError::InvalidInput("Invalid secret key".to_string()))?;
    if secret_bytes.len() != 64 {
        return Err(AppError::InvalidInput("Secret key length invalid".to_string()));
    }
    let keypair = Keypair::from_bytes(&secret_bytes).map_err(|_| AppError::InternalError)?;

    let signature: Signature = keypair.sign(payload.message.as_bytes());

    Ok(Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "signature": encode_base64(&signature.to_bytes()),
            "public_key": bs58::encode(keypair.public.to_bytes()).into_string(),
            "message": payload.message,
        }),
    }))
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> Result<impl IntoResponse, AppError> {
    let pubkey_bytes = decode_base58(&payload.pubkey).map_err(|_| AppError::InvalidInput("Invalid public key".to_string()))?;
    let pubkey = PublicKey::from_bytes(&pubkey_bytes).map_err(|_| AppError::InvalidInput("Invalid pubkey bytes".to_string()))?;

    let sig_bytes = base64::decode(&payload.signature).map_err(|_| AppError::InvalidInput("Invalid signature base64".to_string()))?;
    let signature = Signature::from_bytes(&sig_bytes).map_err(|_| AppError::InvalidInput("Invalid signature bytes".to_string()))?;

    let valid = pubkey.verify(payload.message.as_bytes(), &signature).is_ok();

    Ok(Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "valid": valid,
            "message": payload.message,
            "pubkey": payload.pubkey,
        }),
    }))
}
