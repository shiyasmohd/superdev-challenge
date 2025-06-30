use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::transfer;

use crate::routes::error::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferInstruction {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

pub async fn create_transfer_instruction(
    transfer_instruction: Result<Json<TransferInstruction>, JsonRejection>,
) -> impl IntoResponse {
    let transfer_instruction = match transfer_instruction {
        Ok(instruction) => instruction,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to deserialize request body: {}", err))
                .into_response();
        }
    };

    let destination = match transfer_instruction.destination.parse::<Pubkey>() {
        Ok(destination) => destination,
        Err(_) => {
            return ErrorResponse::new("Invalid destination pubkey".to_string()).into_response();
        }
    };

    let mint = match transfer_instruction.mint.parse::<Pubkey>() {
        Ok(mint) => mint,
        Err(_) => return ErrorResponse::new("Invalid mint pubkey".to_string()).into_response(),
    };

    let owner = match transfer_instruction.owner.parse::<Pubkey>() {
        Ok(owner) => owner,
        Err(_) => return ErrorResponse::new("Invalid owner pubkey".to_string()).into_response(),
    };

    let token_program_id = spl_token::id();

    let result = match transfer(
        &token_program_id,
        &mint,
        &destination,
        &owner,
        &[],
        transfer_instruction.amount,
    ) {
        Ok(result) => result,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to create transfer instruction: {}", err))
                .into_response();
        }
    };

    let instruction_data = base64::encode(result.data);
    let accounts = result
        .accounts
        .iter()
        .map(|account| {
            json!({
                "pubkey": account.pubkey.to_string(),
                "is_signer": account.is_signer
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
