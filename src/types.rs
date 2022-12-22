use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainInfo {
    pub name: String,
    pub chain: String,
    pub rpc: Vec<String>,
    pub features: Option<Vec<Features>>,
    pub faucets: Vec<String>,
    #[serde(rename = "nativeCurrency")]
    pub native_currency: Currency,
    #[serde(rename = "infoURL")]
    pub info_url: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "networkId")]
    pub network_id: u64,
    pub slip44: Option<u64>,
    pub ens: Option<HashMap<String, String>>,
    pub explorers: Option<Vec<Explorer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Features {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Explorer {
    pub name: String,
    pub url: String,
    pub standard: String,
}
