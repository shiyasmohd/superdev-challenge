use axum::{Json, extract::rejection::JsonRejection, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::routes::error::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignedMessage {
    message: String,
    signature: String,
    pubkey: String,
}

pub async fn verify_signed_message(
    verify_signed_message: Result<Json<VerifySignedMessage>, JsonRejection>,
) -> impl IntoResponse {
    let verify_signed_message = match verify_signed_message {
        Ok(instruction) => instruction,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to deserialize request body: {}", err))
                .into_response();
        }
    };

    let pubkey = match verify_signed_message.pubkey.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => return ErrorResponse::new("Invalid pubkey".to_string()).into_response(),
    };

    let signature_bytes = match base64::decode(verify_signed_message.signature.clone()) {
        Ok(bytes) => bytes,
        Err(_) => return ErrorResponse::new("Invalid signature".to_string()).into_response(),
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return ErrorResponse::new("Invalid signature".to_string()).into_response(),
    };

    let is_valid = signature.verify(pubkey.as_ref(), verify_signed_message.message.as_bytes());

    let body = json!({
        "success": true,
        "data": json!({
            "valid": is_valid,
            "message": verify_signed_message.message,
            "pubkey": verify_signed_message.pubkey,
        }),
    });
    (axum::http::StatusCode::OK, axum::Json(body)).into_response()
}
