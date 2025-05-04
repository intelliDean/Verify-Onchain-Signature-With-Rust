use crate::models::cert_model::{Certificate, CertificateDTO, ItemCreatedEvent, ItemEvent};
use crate::utility::{to_bytes, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
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
    // let mut cert_hash = H256::zero();
    // // Listen for CertificateRegistered event
    // for log in receipt.logs {
    //     // Check if the log is for CertificateRegistered (topic[0] is event signature)
    //     let event_signature = H256::from(keccak256("ItemCreated(string,bytes32,address)"));
    //     if log.topics[0] == event_signature {
    //         // Decode the event (first topic is event signature, data is certHash)
    //         cert_hash = H256::from_slice(&log.data);
    //         println!(
    //             "ItemCreated event detected! certHash: {:?}",
    //             cert_hash.clone()
    //         );
    //     }
    // }

    let event_signature = H256::from(keccak256("ItemCreated(string,bytes32,address)"));



    // for log in receipt.logs.iter() {
    //     if log.topics.len() == 4 && log.topics[0] == event_signature {
    //          name_hash = log.topics[1]; // You won't recover the original string, only the hash
    //          unique_id = H256::from(log.topics[2]); // bytes32
    //          owner = Address::from_slice(&log.topics[3].as_bytes()[12..]); // address is last 20 bytes
    //
    //         println!("📦 ItemCreated event:");
    //         println!("    name hash: {:?}", name_hash);
    //         println!("    unique_id: {:?}", unique_id);
    //         println!("    owner: {:?}", owner);
    //     }
    // }


    let mut event_res = ItemCreatedEvent::init();

    for log in receipt.logs.iter() {
        let raw_log = RawLog {
            topics: log.topics.clone(),
            data: log.data.clone().to_vec(),
        };

        if let Ok(event) = <ItemCreatedEvent as EthEvent>::decode_log(&raw_log) {
            // name = event.name.clone();
            // unique_id = event.unique_id;
            // owner = event.owner;

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
        (status = 200, description = "Contract status", body = String)
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

    // Json(format!("Contract at {:?}", state.signature_verifier))
}

#[utoipa::path(
    get,
    path = "/get_item",
    responses(
        (status = 200, description = "Contract status", body = String)
    )
)]
pub async fn get_item(
    State(state): State<AppState>,
    Json(item_input): Json<String>, //this is the struckHash
) -> anyhow::Result<Json<auth_chain::Item>, axum::http::StatusCode> {

    let contract = AuthChain::new(state.auth_chain, state.eth_client.clone());

    // let item_id = item_input;

    let item = contract
        .get_item(item_input)
        .call()
        .await
        .map_err(|e| {
            eprintln!("Transaction send error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(item))
}
