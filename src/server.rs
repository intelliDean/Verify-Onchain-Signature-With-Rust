use crate::models::sig_model::{AssetDto};
use crate::utility::AppState;
use crate::signature::{signature, __path_signature};
use crate::signature_verifier::{
    __path_check_status,
    __path_verify_signature,
    check_status,
    verify_signature,
};

use axum::{routing::get, routing::post, Router};

use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use anyhow::Result;
use dotenv::dotenv;
// use ethers::prelude::*;
use crate::certificate::{__path_create_item, __path_get_item, __path_get_owner, create_item, get_item, get_owner};
use ethers::{
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::{env, sync::Arc, time::Duration};


// Swagger/OpenAPI configuration
#[derive(OpenApi)]
#[openapi(
    paths(verify_signature, check_status, signature, create_item, get_item, get_owner),
    components(schemas(AssetDto))
)]
struct ApiDoc;

pub async fn server() -> Result<()> {
    eprintln!("PROJECT STARTING...");
    // Load environment variables
    dotenv().ok();

    //when sharing the .env file
    // dotenv::from_path("../.env").ok();

    // Initialize Ethereum client
    let rpc_url = env::var("BASE_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let signature_verifier: Address = env::var("SIGNATURE_VERIFIER_CONTRACT")?
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid contract address"))?;
    let auth_chain: Address = env::var("AUTH_CHAIN_CONTRACT")?
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid contract address"))?;

    let provider = Provider::<Http>::try_from(&rpc_url)?.interval(Duration::from_millis(1000));
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let eth_client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    // Initialize app state
    let state = AppState {
        eth_client,
        signature_verifier,
        auth_chain,
        wallet_address: wallet.address(), //will remove after test
    };

    // Define routes
    let app = Router::new()
        .route("/verify", post(verify_signature))
        .route("/verify/status", get(check_status))
        .route("/signature", post(signature))
        .route("/create_item", post(create_item))
        .route("/get_item", get(get_item))
        .route("/get_owner", get(get_owner))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
        .layer(CorsLayer::permissive()); // Optional: Enable CORS

    eprintln!("Project started and listening on 127.0.0.1:8080");

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(()) // another way to say return nothing
}
