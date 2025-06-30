use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::mint_to;

use crate::routes::error::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct MintToInstruction {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

pub async fn create_mint_to_instruction(
    mint_to_instruction: Result<Json<MintToInstruction>, JsonRejection>,
) -> impl IntoResponse {
    let mint_to_instruction = match mint_to_instruction {
        Ok(instruction) => instruction,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to deserialize request body: {}", err))
                .into_response();
        }
    };
    let mint_pubkey = match mint_to_instruction.mint.parse::<Pubkey>() {
        Ok(mint_pubkey) => mint_pubkey,
        Err(_) => return ErrorResponse::new("Invalid mint address".to_string()).into_response(),
    };
    let destination_pubkey = match mint_to_instruction.destination.parse::<Pubkey>() {
        Ok(destination_pubkey) => destination_pubkey,
        Err(_) => {
            return ErrorResponse::new("Invalid destination address".to_string()).into_response();
        }
    };
    let authority_pubkey = match mint_to_instruction.authority.parse::<Pubkey>() {
        Ok(authority_pubkey) => authority_pubkey,
        Err(_) => {
            return ErrorResponse::new("Invalid authority address".to_string()).into_response();
        }
    };

    let token_program_id = spl_token::id();

    let result = mint_to(
        &token_program_id,
        &mint_pubkey,
        &destination_pubkey,
        &authority_pubkey,
        &[], // No multisig signer seeds
        mint_to_instruction.amount,
    )
    .expect("Failed to create mint_to instruction");

    let instruction_data = base64::encode(result.data);
    let accounts = result
        .accounts
        .iter()
        .map(|account| {
            json!({
                "pubkey": account.pubkey.to_string(),
                "is_signer": false,
                "is_writable": true
            })
        })
        .collect::<Vec<_>>();

    let body = json!({
        "program_id": result.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": instruction_data
    });

    (axum::http::StatusCode::OK, axum::Json(body)).into_response()
}
