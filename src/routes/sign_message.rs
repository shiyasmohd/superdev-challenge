use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
};

use crate::routes::error::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

pub async fn sign_message(
    sign_message_request: Result<Json<SignMessageRequest>, axum::extract::rejection::JsonRejection>,
) -> impl IntoResponse {
    let sign_message_request = match sign_message_request {
        Ok(instruction) => instruction,
        Err(err) => {
            return ErrorResponse::new(format!("Failed to deserialize request body: {}", err))
                .into_response();
        }
    };
    let secret_bytes = match bs58::decode(sign_message_request.secret.clone()).into_vec() {
        Ok(secret_bytes) => secret_bytes,
        Err(_) => return ErrorResponse::new("Invalid base58 secret".to_string()).into_response(),
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(keypair) => keypair,
        Err(_) => return ErrorResponse::new("Invalid keypair bytes".to_string()).into_response(),
    };

    let message_bytes = sign_message_request.message.as_bytes();
    let signature: Signature = keypair.sign_message(message_bytes);

    let body = json!({
        "success": true,
        "data": json!({
            "signature": signature.to_string(),
            "public_key": keypair.pubkey().to_string(),
            "message": sign_message_request.message,
        }),
    });
    (axum::http::StatusCode::OK, axum::Json(body)).into_response()
}
