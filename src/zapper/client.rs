// src/zapper/client.rs
use crate::types::PortfolioError;
use super::types::Portfolio;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest;
use serde_json::json;

pub struct ZapperClient {
    client: reqwest::Client,
    api_key: String,
}

impl ZapperClient {
    pub fn new(api_key: &str) -> Self {
        let encoded_key = STANDARD.encode(api_key);
        let client = reqwest::Client::new();
        
        Self {
            client,
            api_key: encoded_key,
        }
    }

    pub async fn query_portfolio(&self, addresses: Vec<String>, networks: Option<Vec<String>>) -> Result<Portfolio, PortfolioError> {
        // Note: GraphQL query uses camelCase as that's what the API expects
        let query = r#"
            query GetCompletePortfolio($addresses: [Address!]!, $networks: [Network!]) {
                portfolio(addresses: $addresses, networks: $networks) {
                    appBalances {
                        appName
                        balanceUSD
                        appId
                        network
                        products {
                            label
                            assets {
                                address
                            }
                        }
                    }
                    tokenBalances {
                        address
                        network
                        token {
                            balanceUSD
                            balance
                            baseToken {
                                symbol
                                name
                                network
                                imgUrl
                            }
                        }
                    }
                    nftBalances {
                        network
                        balanceUSD
                    }
                    totals {
                        total
                        totalWithNFT
                        appsTotal
                        totalByNetwork {
                            network
                            total
                        }
                        holdings {
                            label
                            balanceUSD
                            pct
                        }
                    }
                }
            }
        "#;

        let response = self.client
            .post("https://public.zapper.xyz/graphql")
            .header("Authorization", format!("Basic {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "query": query,
                "variables": json!({
                    "addresses": addresses,
                    "networks": networks
                })
            }))
            .send()
            .await
            .map_err(|e| PortfolioError(format!("Request failed: {}", e)))?;

        let data = response.json::<serde_json::Value>()
            .await
            .map_err(|e| PortfolioError(format!("JSON parsing failed: {}", e)))?;

        // Check for GraphQL errors
        if let Some(errors) = data.get("errors") {
            return Err(PortfolioError(format!("GraphQL errors: {}", errors)));
        }

        // Parse the response into our Portfolio struct
        // The serde attributes in the types will handle camelCase -> snake_case conversion
        match data.get("data").and_then(|d| d.get("portfolio")) {
            Some(portfolio_data) => {
                serde_json::from_value(portfolio_data.clone())
                    .map_err(|e| PortfolioError(format!("Portfolio parsing failed: {}", e)))
            },
            None => Err(PortfolioError("Missing portfolio data in response".to_string()))
        }
    }
}