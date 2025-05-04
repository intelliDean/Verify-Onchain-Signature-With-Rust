use crate::models::cert_model::{ Certificate, CertificateDTO};
use crate::utility::{to_bytes, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use ethers::{
    abi::RawLog,
    contract::{abigen, EthEvent},
    prelude::*,
    signers::Signer,
    types::Signature,
};
use ethers::utils::keccak256;
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
    Json(cert): Json<CertificateDTO>
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
        . await
        .map_err(|e| {
            eprintln!("Signature error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    eprintln!("Signature: {:?}", signature);

    // Convert to contract certificate
    let contract_cert: auth_chain::Certificate = certificate.into();
    let sig_bytes = to_bytes(signature);

    // Call create_item
    let contract =
        AuthChain::new(state.auth_chain, state.eth_client.clone());

   contract
        .create_item(contract_cert.clone(), sig_bytes)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Transaction send error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;









   //  let contract_cert: auth_chain::Certificate = certificate.into();
   //
   //  let contract = AuthChain::new(state.auth_chain, state.eth_client.clone());
   //
   //  let sig_bytes = to_bytes(signature);
   //
   // contract
   //      .create_item(contract_cert, sig_bytes)
   //      .send()
   //      .await
   //      .map_err(|e| {
   //          eprintln!("Transaction send error: {:?}", e);
   //          StatusCode::INTERNAL_SERVER_ERROR
   //      })?;

    // let receipt = contract
    //     .create_item(contract_cert, sig_bytes)
    //     .send()
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Transaction send error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Transaction confirmation error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?
    //     .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    // // let events = contract
    //     .item_created_filter()
    //     .from_block(receipt.block_number.unwrap())
    //     .query()
    //     .await.unwrap();
    //
    // for event in events {
    //     println!("✅ ItemCreated: {:?}", event.struct_hash);
    // }

//ok
    // let receipt = contract
    //     .create_item(contract_cert, sig_bytes)
    //     .send()
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Transaction send error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Transaction confirmation error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?
    //     .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    //
    // if receipt.status != Some(1.into()) {
    //     return Err(StatusCode::BAD_REQUEST);
    // }
    //
    // let events = contract
    //     .item_created_filter()
    //     .from_block(receipt.block_number.unwrap())
    //     .to_block(receipt.block_number.unwrap())
    //     .query()
    //     .await.unwrap();
    //
    // for event in events {
    //     println!("✅ ItemCreated: {:?}", event.struct_hash);
    // }

    // let receipt = contract
    //     .create_item(contract_cert, sig_bytes)
    //     .send()
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Transaction send error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?
    //     .await
    //     .map_err(|e| {
    //         eprintln!("Transaction confirmation error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?
    //     .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    //
    // if receipt.status != Some(1.into()) {
    //     return Err(StatusCode::BAD_REQUEST);
    // }
    // let mut cert_hash = H256::zero();
    // // Listen for CertificateRegistered event
    // for log in receipt.logs {
    //     // Check if the log is for CertificateRegistered (topic[0] is event signature)
    //     let event_signature = H256::from(keccak256("ItemCreated(bytes32)"));
    //     if log.topics[0] == event_signature {
    //         // Decode the event (first topic is event signature, data is certHash)
    //          cert_hash = H256::from_slice(&log.data);
    //         println!("ItemCreated event detected! certHash: {:?}", cert_hash.clone());
    //     }
    // }


    Ok(Json(format!("Item created successfully: {:?}", cert.unique_id)))
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

     let owner = contract
        .get_owner()
        .call()
        .await
        .map_err(|e| {
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
    Json(itemId): Json<String>, //this is the struckHash
) {


}
