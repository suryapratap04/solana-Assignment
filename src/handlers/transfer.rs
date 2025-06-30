use axum::{extract::Json, response::IntoResponse};
use solana_sdk::{pubkey::Pubkey, system_instruction};
use spl_token::instruction as token_instruction;
use crate::{models::{SuccessResponse, SendSolRequest, SendTokenRequest}, utils::{encode_base64}, error::AppError};
use std::str::FromStr;


pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> Result<impl IntoResponse, AppError> {
    let from = Pubkey::from_str(&payload.from).map_err(|_| AppError::InvalidInput("Invalid from address".to_string()))?;
    let to = Pubkey::from_str(&payload.to).map_err(|_| AppError::InvalidInput("Invalid to address".to_string()))?;

    let ix = system_instruction::transfer(&from, &to, payload.lamports);

    let accounts: Vec<_> = ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();

    Ok(Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": encode_base64(&ix.data),
        }),
    }))
}

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> Result<impl IntoResponse, AppError> {
    let dest = Pubkey::from_str(&payload.destination).map_err(|_| AppError::InvalidInput("Invalid destination".to_string()))?;
    let mint = Pubkey::from_str(&payload.mint).map_err(|_| AppError::InvalidInput("Invalid mint".to_string()))?;
    let owner = Pubkey::from_str(&payload.owner).map_err(|_| AppError::InvalidInput("Invalid owner".to_string()))?;

    let ix = token_instruction::transfer(&spl_token::id(), &mint, &dest, &owner, &[], payload.amount)
        .map_err(|_| AppError::InternalError)?;

    let accounts: Vec<_> = ix.accounts.iter().map(|meta| {
        serde_json::json!({
            "pubkey": meta.pubkey.to_string(),
            "isSigner": meta.is_signer,
        })
    }).collect();

    Ok(Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": encode_base64(&ix.data),
        }),
    }))
}
