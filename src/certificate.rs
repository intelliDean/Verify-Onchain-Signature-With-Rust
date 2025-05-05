use crate::models::cert_model::{Certificate, CertificateDTO, ItemCreatedEvent, ItemEvent, Item, ItemInput};
use crate::utility::{to_bytes, AppState};
use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,

};
use ethers::utils::keccak256;
use ethers::{
    abi::RawLog,
    contract::{abigen, EthEvent, EthLogDecode},
    prelude::*,
    signers::Signer,
    types::Signature,
};
use sha2::Digest;

// abi path
abigen!(
    AuthChain,
    "./artifacts/contracts/AuthChain.sol/AuthChain.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[utoipa::path(
    post,
    path = "/create_item",
    request_body = CertificateDTO,
    responses(
        (status = 200, description = "Successful Item Creation", body = String),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_item(
    State(state): State<AppState>,
    Json(cert): Json<CertificateDTO>,
) -> anyhow::Result<Json<String>, axum::http::StatusCode> {
    let certificate: Certificate = cert
        .clone()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // accessing the wallet from SignerMiddleware
    // Sign the certificate
    let signature: Signature = state
        .eth_client
        .signer()
        .sign_typed_data(&certificate)
        .await
        .map_err(|e| {
            eprintln!("Signature error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    eprintln!("Signature: {:?}", signature);

    // Convert to contract certificate
    let contract_cert: auth_chain::Certificate = certificate.into();
    let sig_bytes = to_bytes(signature);

    // Call create_item
    let contract = AuthChain::new(state.auth_chain, state.eth_client.clone());

    let receipt = contract
        .create_item(contract_cert, sig_bytes)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Transaction send error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .await
        .map_err(|e| {
            eprintln!("Transaction confirmation error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    if receipt.status != Some(1.into()) {
        return Err(StatusCode::BAD_REQUEST);
    }


    let event_signature = H256::from(keccak256("ItemCreated(string,bytes32,address)"));


    let mut event_res = ItemCreatedEvent::init();

    for log in receipt.logs.iter() {
        let raw_log = RawLog {
            topics: log.topics.clone(),
            data: log.data.clone().to_vec(),
        };

        if let Ok(event) = <ItemCreatedEvent as EthEvent>::decode_log(&raw_log) {
          

           event_res = ItemCreatedEvent::new(event.name.clone(), event.unique_id, event.owner);

            println!("📦 ItemCreated:");
            println!("    name: {}", event.name);
            println!("    unique_id: {:?}", event.unique_id);
            println!("    owner: {:?}", event.owner);
        }
    }

    Ok(Json(format!("Event: {:?}", event_res)))
}

#[utoipa::path(
    get,
    path = "/get_owner",
    responses(
        (status = 200, description = "Owner Address", body = String)
    )
)]
pub async fn get_owner(
    State(state): State<AppState>,
) -> anyhow::Result<Json<Address>, axum::http::StatusCode> {
    let contract = AuthChain::new(state.auth_chain, state.eth_client.clone());

    let owner = contract.get_owner().call().await.map_err(|e| {
        eprintln!("Transaction send error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;


      Ok(Json(owner))
}
#[utoipa::path(
    get,
    path = "/get_item/{item_id}",
    params(
        ("item_id" = String, Path, description = "Item ID to retrieve")
    ),
    responses(
        (status = 200, description = "Item retrieved successfully", body = Item),
        (status = 400, description = "Invalid item ID"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_item(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<Json<Item>, StatusCode> {
    let contract = AuthChain::new(state.auth_chain, state.eth_client.clone());

    let item = contract
        .get_item(item_id)
        .call()
        .await
        .map_err(|e| {
            eprintln!("Contract call error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let item: Item = item.into(); //convert contract Item to Rust Item

    Ok(Json(item))
}
