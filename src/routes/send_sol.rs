use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use solana_sdk::{pubkey::Pubkey, system_instruction};

use crate::routes::error::ErrorResponse;

#[derive(Debug, Deserialize)]
pub struct SolTransferInstruction {
    from: String,
    to: String,
    lamports: u64,
}

pub async fn create_sol_transfer_instruction(
    sol_transfer_instruction: Result<Json<SolTransferInstruction>, JsonRejection>,
) -> impl IntoResponse {
    let sol_transfer_instruction = match sol_transfer_instruction {
        Ok(instruction) => instruction,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to deserialize request body: {}", err))
                .into_response();
        }
    };

    if sol_transfer_instruction.lamports == 0 {
        return ErrorResponse::new("Lamports must be greater than 0".to_string()).into_response();
    }

    let from_pubkey = match sol_transfer_instruction.from.parse::<Pubkey>() {
        Ok(from_pubkey) => from_pubkey,
        Err(_) => return ErrorResponse::new("Invalid sender address".to_string()).into_response(),
    };

    let to_pubkey = match sol_transfer_instruction.to.parse::<Pubkey>() {
        Ok(to_pubkey) => to_pubkey,
        Err(_) => {
            return ErrorResponse::new("Invalid recipient address".to_string()).into_response();
        }
    };

    let instruction =
        system_instruction::transfer(&from_pubkey, &to_pubkey, sol_transfer_instruction.lamports);

    let instruction_data = base64::encode(&instruction.data);
    let accounts = instruction
        .accounts
        .iter()
        .map(|account| account.pubkey.to_string())
        .collect::<Vec<_>>();

    let body = json!({
        "success": true,
        "data": json!({
            "program_id": instruction.program_id.to_string(),
            "accounts": accounts,
            "instruction_data": instruction_data
        }),
    });
    (axum::http::StatusCode::OK, axum::Json(body)).into_response()
}
