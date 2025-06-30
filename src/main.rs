use axum::{routing::post, Router, serve};
use std::net::SocketAddr;

mod models;
mod error;
mod utils;
mod handlers;

use handlers::{
    keypair::generate_keypair,
    token::{create_token, mint_token},
    message::{sign_message, verify_message},
    transfer::{send_sol, send_token},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✅ Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
