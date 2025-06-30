use axum::{extract::Json, response::IntoResponse};
use solana_sdk::{pubkey::Pubkey, instruction::Instruction};
use spl_token::instruction as token_instruction;
use crate::{models::{SuccessResponse, TokenCreateRequest, MintTokenRequest}, utils::{ encode_base64}};
use crate::error::AppError;
use std::str::FromStr;


pub async fn create_token(Json(payload): Json<TokenCreateRequest>) -> Result<impl IntoResponse, AppError> {
    let mint_pubkey = Pubkey::from_str(&payload.mint).map_err(|_| AppError::InvalidInput("Invalid mint pubkey".to_string()))?;
    let authority_pubkey = Pubkey::from_str(&payload.mint_authority).map_err(|_| AppError::InvalidInput("Invalid authority pubkey".to_string()))?;

    let ix: Instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &authority_pubkey,
        None,
        payload.decimals,
    ).map_err(|_| AppError::InternalError)?;

    let accounts: Vec<_> = ix.accounts.iter().map(|meta| {
        serde_json::json!({
            "pubkey": meta.pubkey.to_string(),
            "is_signer": meta.is_signer,
            "is_writable": meta.is_writable
        })
    }).collect();

    Ok(Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": encode_base64(&ix.data)
        }),
    }))
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> Result<impl IntoResponse, AppError> {
    let mint = Pubkey::from_str(&payload.mint).map_err(|_| AppError::InvalidInput("Invalid mint".to_string()))?;
    let dest = Pubkey::from_str(&payload.destination).map_err(|_| AppError::InvalidInput("Invalid destination".to_string()))?;
    let auth = Pubkey::from_str(&payload.authority).map_err(|_| AppError::InvalidInput("Invalid authority".to_string()))?;

    let ix = token_instruction::mint_to(&spl_token::id(), &mint, &dest, &auth, &[], payload.amount)
        .map_err(|_| AppError::InternalError)?;

    let accounts: Vec<_> = ix.accounts.iter().map(|meta| {
        serde_json::json!({
            "pubkey": meta.pubkey.to_string(),
            "is_signer": meta.is_signer,
            "is_writable": meta.is_writable
        })
    }).collect();

    Ok(Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "program_id": ix.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": encode_base64(&ix.data)
        }),
    }))
}
