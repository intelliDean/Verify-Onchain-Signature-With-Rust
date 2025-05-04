use crate::models::sig_model::{Asset, AssetDto};
use crate::utility::{to_bytes, AppState};

use axum::extract::{Json, State};

use anyhow::Result;
use ethers::{
    contract::abigen,
    signers::Signer,
    types::Signature,
};


// abi path
abigen!(
    SignatureVerifier,
    "./artifacts/contracts/SignatureVerifier.sol/SignatureVerifier.json"
);

// Handler for POST /verify
#[utoipa::path(
    post,
    path = "/verify",
    request_body = AssetDto,
    responses(
        (status = 200, description = "Signature verification result", body = bool),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn verify_signature(
    State(state): State<AppState>,
    Json(asset_dto): Json<AssetDto>,
) -> Result<Json<String>, axum::http::StatusCode> {
    // Convert AssetDto to Asset
    let mut asset: Asset = asset_dto.clone()
        .try_into()
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    println!("backend wallet address {:?}", state.wallet_address);

    asset.owner = state.wallet_address; //backend wallet address

    let contract = SignatureVerifier::new(state.signature_verifier, state.eth_client.clone());

    // accessing the wallet from SignerMiddleware
    let signature: Signature = state
        .eth_client
        .signer()
        .sign_typed_data(&asset)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_wallet = asset_dto.owner.parse().unwrap();

    println!("user wallet {:?}", user_wallet);

    let contract_asset: signature_verifier::Asset = asset.into();

    let is_valid = contract
        .verify_asset_signature(contract_asset, to_bytes(signature), user_wallet) //user wallet address
        .call()
        .await
        .map_err(|e| {
            eprintln!("Contract error: {:?}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(format!("Signature valid: {}", is_valid)))
}

//to check the contract availability
#[utoipa::path(
    get,
    path = "/verify/status",
    responses(
        (status = 200, description = "Contract status", body = String)
    )
)]
pub async fn check_status(State(state): State<AppState>) -> Json<String> {
    Json(format!("Contract at {:?}", state.signature_verifier))
}
