use axum::{Router, routing::post};

use crate::routes::{
    create_token::create_initialize_mint_instruction, generate_keypair::generate_keypair,
    mint_token::create_mint_to_instruction, send_sol::create_sol_transfer_instruction,
    send_token::create_transfer_instruction, sign_message::sign_message,
    verify_message::verify_signed_message,
};

pub mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_initialize_mint_instruction))
        .route("/token/mint", post(create_mint_to_instruction))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_signed_message))
        .route("/send/sol", post(create_sol_transfer_instruction))
        .route("/send/token", post(create_transfer_instruction));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
