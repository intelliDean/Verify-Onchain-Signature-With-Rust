use utoipa::{OpenApi, ToSchema};

use anyhow::Result;
use dotenv::dotenv;
use ethers::core::types::transaction::eip712::Eip712;
use ethers::prelude::*;
use ethers::{
    abi::AbiDecode
    ,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use serde::{Deserialize, Serialize};
use sha3::Digest;
use std::{env, sync::Arc, time::Duration};
use axum::Json;
use tiny_keccak::Hasher;
use crate::models::AssetDto;

// Generate contract bindings from ABI
abigen!(
    Ownership,
    "./artifacts/contracts/Ownership.sol/Ownership.json"
);


pub async fn ownership() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Get RPC URL and private key from environment variables
    let rpc_url = env::var("BASE_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;

    // Initialize HTTP provider
    let provider = Provider::<Http>::try_from(&rpc_url)?
        .interval(Duration::from_millis(1000)); // Less aggressive polling

    // Fetch chain ID from provider
    let chain_id = provider.get_chainid().await?.as_u64();

    println!("Using chain ID: {}", chain_id);

    // Initialize wallet with chain ID
    let wallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);

    // Create client with signer
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Initialize contract with deployed address
    let contract_address: Address = env::var("CONTRACT_ADDRESS")?
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid contract address"))?;

    let contract = Ownership::new(contract_address, client);

    // Example: Call the verifyOwnership function
    let item_id = "asset001";

    // contract.register_asset(item_id.to_string()).send().await
    //     .expect("Error!");

    println!("Register Asset called");

    match contract.get_owner(item_id.to_string()).call().await { // send().wait for update
        Ok(owner) => println!("Owner: {:?}", owner),
        Err(e) => eprintln!("Contract call failed: ",),
    }

    match contract.verify_ownership(item_id.to_string()).call().await { // send().wait for update
        Ok(result) => println!("Verification result: {:?}", result),
        Err(e) => eprintln!("Contract call failed: ",),
    }

    Ok(())
}