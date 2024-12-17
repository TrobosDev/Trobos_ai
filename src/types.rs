use serde::{Deserialize, Serialize};
use crate::zapper::types::{NetworkTotal, Holding};

#[derive(Debug, thiserror::Error)]
#[error("Portfolio error: {0}")]
pub struct PortfolioError(pub String);

#[derive(Deserialize)]
pub struct PortfolioArgs {
    pub address: String,
    pub networks: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct PortfolioOverview {
    pub total_value: f64,
    pub total_with_nft: f64,
    pub apps_total: f64,
    pub holdings: Vec<Holding>,
    pub network_breakdown: Vec<NetworkTotal>,
}