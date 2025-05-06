use std::env;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Error;
use ethabi::ethereum_types::Address;
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::prelude::{Http, LocalWallet, Provider};
use ethers::signers::Signer;
use crate::utility::AppState;

pub async fn init_app_state() -> anyhow::Result<AppState, Error> {
    
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
    
    Ok(state)
}