use axum::response::IntoResponse;
use serde_json::json;
use solana_sdk::{signature::Keypair, signer::Signer};

pub async fn generate_keypair() -> impl IntoResponse {
    let keypair = Keypair::new();
    let secret_bytes: Vec<u8> = keypair.to_bytes().to_vec();
    let body = json!({
        "success": true,
        "data": json!({
            "pubkey": keypair.pubkey().to_string(),
            "secret": bs58::encode(secret_bytes).into_string(),
        }),
    });
    (axum::http::StatusCode::OK, axum::Json(body))
}
