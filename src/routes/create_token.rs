use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;

use crate::routes::error::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeMintInstruction {
    pub mint: String,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub decimals: u8,
}

pub async fn create_initialize_mint_instruction(
    mint_instruction: Result<Json<InitializeMintInstruction>, JsonRejection>,
) -> impl IntoResponse {
    let mint_instruction = match mint_instruction {
        Ok(instruction) => instruction,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to deserialize request body: {}", err))
                .into_response();
        }
    };
    let mint = match mint_instruction.mint.parse::<Pubkey>() {
        Ok(mint) => mint,
        Err(_) => return ErrorResponse::new("Invalid mint pubkey".to_string()).into_response(),
    };

    let mint_authority = match mint_instruction.mint_authority.parse::<Pubkey>() {
        Ok(mint_authority) => mint_authority,
        Err(_) => {
            return ErrorResponse::new("Invalid mint authority pubkey".to_string()).into_response();
        }
    };
    let freeze_authority = Some(mint_authority);
    let token_program_id = spl_token::id();
    let result = match initialize_mint(
        &token_program_id,
        &mint,
        &mint_authority,
        freeze_authority.as_ref(),
        mint_instruction.decimals,
    ) {
        Ok(result) => result,
        Err(_) => {
            return ErrorResponse::new("Failed to create initialize_mint instruction".to_string())
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
                "is_signer": account.is_signer,
                "is_writable": account.is_writable
            })
        })
        .collect::<Vec<_>>();

    let body = json!({
        "success": true,
        "data": json!({
            "program_id": token_program_id.to_string(),
            "accounts": accounts,
            "instruction_data": instruction_data
        }),
    });
    (axum::http::StatusCode::OK, axum::Json(body)).into_response()
}
