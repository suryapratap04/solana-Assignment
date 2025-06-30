use axum::{routing::post, Router, serve};
use axum::{routing::{ get},  response::IntoResponse};


use std::env;
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

async fn root() -> impl IntoResponse {
    "Welcome to the Solana Fellowship Server 🚀"
}


#[tokio::main]
async fn main() {
    // For Render: get port from environment variable "PORT"
    // For local: default to 8080
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("✅ Server running at http://{}", addr);

    let app = Router::new()
        .route("/", get(root))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
