use crate::certificate::auth_chain;
use ethabi::ethereum_types::{Address, H256, U256};
use ethers::contract::EthEvent;
use ethers::types::transaction::eip712::{EIP712Domain, Eip712, Eip712Error};
use ethers::utils::keccak256;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use utoipa::ToSchema;


// Certificate struct for EIP-712
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Certificate {
    pub name: String,
    pub unique_id: String,
    pub serial: String,
    pub date: U256,
    pub owner: Address,
    pub metadata: Vec<String>,
}
impl Eip712 for Certificate {
    type Error = Eip712Error;

    fn domain_separator(&self) -> Result<[u8; 32], Self::Error> {
        let domain = self.domain()?;
        let type_hash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
        );

        let name_hash = keccak256(domain.name.unwrap_or_default().as_bytes());
        let version_hash = keccak256(domain.version.unwrap_or_default().as_bytes());
        let chain_id = U256::from(domain.chain_id.unwrap_or_default());
        let verifying_contract = domain.verifying_contract.unwrap_or_default();

        let encoded = ethers::abi::encode(&[
            ethers::abi::Token::FixedBytes(type_hash.to_vec()),
            ethers::abi::Token::FixedBytes(name_hash.to_vec()),
            ethers::abi::Token::FixedBytes(version_hash.to_vec()),
            ethers::abi::Token::Uint(chain_id),
            ethers::abi::Token::Address(verifying_contract),
        ]);
        Ok(keccak256(&encoded))
    }

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(EIP712Domain {
            name: Some("CertificateAuth".to_string()),
            version: Some("1".to_string()),
            chain_id: Some(U256::from(84532).into()),
            verifying_contract: Some(
                "0xC14CDcDb51EF45111dd2024AB1c003F49144928f"
                    .parse()
                    .unwrap(),
            ),
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(keccak256(
            "Certificate(string name,string uniqueId,string serial,uint256 date,address owner,string[] metadata)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let metadata_bytes = ethers::abi::encode(&[ethers::abi::Token::Array(
            self.metadata
                .iter()
                .map(|s| ethers::abi::Token::String(s.clone()))
                .collect(),
        )]);
        let metadata_hash = keccak256(&metadata_bytes);

        let encoded = ethers::abi::encode(&[
            ethers::abi::Token::FixedBytes(Self::type_hash()?.to_vec()),
            ethers::abi::Token::FixedBytes(keccak256(self.name.as_bytes()).to_vec()),
            ethers::abi::Token::FixedBytes(keccak256(self.unique_id.as_bytes()).to_vec()),
            ethers::abi::Token::FixedBytes(keccak256(self.serial.as_bytes()).to_vec()),
            ethers::abi::Token::Uint(self.date),
            ethers::abi::Token::Address(self.owner),
            ethers::abi::Token::FixedBytes(metadata_hash.to_vec()),
        ]);
        Ok(keccak256(&encoded))
    }

    fn encode_eip712(&self) -> Result<[u8; 32], Self::Error> {
        let domain_separator = self.domain_separator()?;
        let struct_hash = self.struct_hash()?;

        // EIP-712 digest: keccak256("\x19\x01" || domain_separator || struct_hash)
        let mut bytes = Vec::with_capacity(2 + 32 + 32);
        bytes.extend_from_slice(b"\x19\x01");
        bytes.extend_from_slice(&domain_separator);
        bytes.extend_from_slice(&struct_hash);

        Ok(keccak256(&bytes))
    }
}

// Certificate DTO from frontend
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct CertificateDTO {
    pub name: String,
    pub unique_id: String,
    pub serial: String,
    pub date: u64,
    #[schema(value_type = String, format = Binary)]
    pub owner: String,
    pub metadata: Vec<String>,
}

// Convert DTO to Certificate
impl TryFrom<CertificateDTO> for Certificate {
    type Error = anyhow::Error;
    fn try_from(dto: CertificateDTO) -> Result<Self, Self::Error> {
        Ok(Certificate {
            name: dto.name,
            unique_id: dto.unique_id,
            serial: dto.serial,
            date: U256::from(dto.date),
            owner: dto
                .owner
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid address format"))?,
            metadata: dto.metadata,
        })
    }
}

// Convert Certificate to contract Certificate
impl From<Certificate> for auth_chain::Certificate {
    fn from(cert: Certificate) -> Self {
        Self {
            name: cert.name,
            unique_id: cert.unique_id,
            serial: cert.serial,
            date: cert.date,
            owner: cert.owner,
            metadata: cert.metadata,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct ItemInput {
    // #[schema(value_type = String, format = Binary)]
    pub item_id: String
}
//=======================

#[derive(Debug, Clone, EthEvent)]
#[ethevent(name = "ItemCreated", abi = "ItemCreated(string,bytes32,address)")]
#[derive(Default)]
pub struct ItemCreatedEvent {
    pub name: String,
    
    #[ethevent(indexed)]
    pub unique_id: H256,
    
    #[ethevent(indexed)]
    pub owner: Address,
}

impl ItemCreatedEvent {
    pub fn init() -> Self {
        Self {
            name: String::new(),
            unique_id: H256::zero(),
            owner: Address::zero(),
        }
    }
    pub fn new(name: String, unique_id: H256, owner: Address) -> Self {
        Self { name, unique_id, owner }
    }
}

#[derive(Debug, Clone)]
pub struct ItemEvent {
    pub name: String,
    pub unique_id: H256,
    pub owner: Address,
}
