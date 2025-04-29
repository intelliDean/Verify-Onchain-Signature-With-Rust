use crate::signature::__path_signature;
use crate::signature_verifier::__path_check_status;
use crate::signature_verifier::{__path_verify_signature, check_status, verify_signature};
use crate::signature::signature;
use crate::models::{AppState, AssetDto};

use axum::{routing::get, routing::post, Router};

use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use anyhow::Result;
use dotenv::dotenv;
// use ethers::prelude::*;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
    prelude::*,
};
use std::{env, sync::Arc, time::Duration};


// Swagger/OpenAPI configuration
#[derive(OpenApi)]
#[openapi(
    paths(verify_signature, check_status, signature), 
    components(schemas(AssetDto))
)]
struct ApiDoc;

pub async fn server() -> Result<()> {
    eprintln!("PROJECT STARTING...");
    // Load environment variables
    dotenv().ok();

    // Initialize Ethereum client
    let rpc_url = env::var("BASE_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let contract_address: Address = env::var("CONTRACT_ADDRESS")?
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid contract address"))?;

    let provider = Provider::<Http>::try_from(&rpc_url)?.interval(Duration::from_millis(1000));
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let eth_client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    // Initialize app state
    let state = AppState {
        eth_client,
        contract_address,
        wallet_address: wallet.address(), //will remove after test
    };

    // Define routes
    let app = Router::new()
        .route("/verify", post(verify_signature))
        .route("/verify/status", get(check_status))
        .route("/signature", post(signature))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
        .layer(CorsLayer::permissive()); // Optional: Enable CORS

    eprintln!("Project started and listening on 127.0.0.1:8080");

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(()) // another way to say return nothing
}
