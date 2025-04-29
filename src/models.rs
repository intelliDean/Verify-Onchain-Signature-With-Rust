use std::sync::Arc;
use ethabi::ethereum_types::Address;
use ethers::contract::{Eip712, EthAbiType};
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, LocalWallet, Provider};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::signature_verifier::signature_verifier;

// data model for Ethereum Asset (internal, for EIP-712 signing)
#[derive(Eip712, EthAbiType, Clone, Serialize, Deserialize, Debug)]
#[eip712(
    name = "AssetOwnership",
    version = "1",
    chain_id = 84532,
    verifying_contract = "0x3b5fFD911B70ed3E166e3197880809C1e85b34B3"
)]
pub struct Asset {
    pub name: String,
    pub serial: String,
    pub owner: Address,
}


// data model for API (Swagger-compatible)
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct AssetDto {
    pub name: String,
    pub serial: String,
    #[schema(value_type = String, format = Binary)]
    pub owner: String, // Address as hex string
}

// to convert AssetDto to Asset
impl TryFrom<AssetDto> for Asset {

    type Error = anyhow::Error;

    fn try_from(dto: AssetDto) -> anyhow::Result<Self, Self::Error> {
        Ok(Asset {
            name: dto.name,
            serial: dto.serial,
            owner: dto.owner
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid address format"))?,
        })
    }
}

// to convert the Asset to the smart contract Asset data type
impl From<Asset> for signature_verifier::Asset {
    fn from(a: Asset) -> Self {
        Self {
            name: a.name,
            serial: a.serial,
            owner: a.owner,
        }
    }
}

// App state to hold the project state
#[derive(Clone)]
pub struct AppState {
    pub eth_client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub contract_address: Address,
    pub wallet_address: Address,
}