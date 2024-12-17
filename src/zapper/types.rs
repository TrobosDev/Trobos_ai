use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Portfolio {
    pub app_balances: Vec<AppBalance>,
    pub token_balances: Vec<TokenBalance>,
    pub nft_balances: Vec<NftBalance>,
    pub totals: PortfolioTotals,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AppBalance {
    pub app_name: String,
    #[serde(rename = "balanceUSD")]
    pub balance_usd: f64,
    pub app_id: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TokenBalance {
    pub address: String,
    pub network: String,
    pub token: Token,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Token {
    #[serde(rename = "balanceUSD")]
    pub balance_usd: f64,
    pub balance: f64,
    pub base_token: BaseToken,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BaseToken {
    pub symbol: String,
    pub name: String,
    pub network: String,
    pub img_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NftBalance {
    pub network: String,
    #[serde(rename = "balanceUSD")]
    pub balance_usd: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioTotals {
    pub total: f64,
    #[serde(rename = "totalWithNFT")]
    pub total_with_nft: f64,
    #[serde(rename = "appsTotal")]
    pub apps_total: f64,
    #[serde(rename = "totalByNetwork")]
    pub total_by_network: Vec<NetworkTotal>,
    pub holdings: Vec<Holding>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkTotal {
    pub network: String,
    pub total: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Holding {
    pub label: String,
    #[serde(rename = "balanceUSD")]
    pub balance_usd: f64,
    pub pct: f64,
}