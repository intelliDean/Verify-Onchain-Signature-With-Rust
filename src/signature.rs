use dotenv::dotenv;
use ethers::signers::{LocalWallet, Signer};
use serde_json::json;
use std::env;
use axum::extract::State;
use axum::Json;
use ethers::types::Signature;
use tiny_keccak::{Hasher, Keccak};
use crate::models::sig_model::AssetDto;

// Handler for POST /verify
#[utoipa::path(
    post,
    path = "/signature",
    request_body = AssetDto,
    responses(
        (status = 200, description = "To sign an object", body = String),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn signature(Json(asset_dto): Json<AssetDto>)  -> Result<Json<Signature>, axum::http::StatusCode> {
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").unwrap();
    let wallet: LocalWallet = private_key.parse().unwrap();

    // to create asset metadata
    let asset = json!({
        "name": asset_dto.name,
        "serial": asset_dto.serial,
        "owner": wallet.address()
    });

    let asset_serialized = serde_json::to_string(&asset).unwrap();

    println!("Asset JSON: {}", asset_serialized);

    // hash it
    let mut keccak = Keccak::v256();
    keccak.update(asset_serialized.as_bytes());
    let mut asset_hash = [0u8; 32];
    keccak.finalize(&mut asset_hash);

    println!("Asset hash: 0x{}", hex::encode(asset_hash));

    // to sign the hash
    let signature = wallet.sign_message(asset_hash).await;
    println!("Signature: {:?}", signature);

    Ok(Json(signature.unwrap()))
}