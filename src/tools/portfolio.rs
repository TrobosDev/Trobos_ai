// src/tools/portfolio.rs
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use serde_json::json;
use crate::{
    types::{PortfolioArgs, PortfolioError, PortfolioOverview},
    zapper::client::ZapperClient,
};

pub struct PortfolioOverviewTool {
    zapper: ZapperClient,
}

impl PortfolioOverviewTool {
    pub fn new(api_key: &str) -> Self {
        Self {
            zapper: ZapperClient::new(api_key),
        }
    }
}

impl Tool for PortfolioOverviewTool {
    const NAME: &'static str = "get_portfolio_overview";
    
    type Error = PortfolioError;
    type Args = PortfolioArgs;
    type Output = PortfolioOverview;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_portfolio_overview".to_string(),
            description: "Get a complete overview of a wallet's portfolio including tokens, apps, and NFTs across all supported networks".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "address": {
                        "type": "string",
                        "description": "The wallet address to analyze (supports both EVM and Solana addresses)"
                    },
                    "networks": {
                        "type": "array",
                        "items": {
                            "type": "string"
                        },
                        "description": "Optional: Specific networks to analyze. If not provided, all networks will be queried."
                    }
                },
                "required": ["address"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Query the portfolio data
        let portfolio = self.zapper
            .query_portfolio(vec![args.address], args.networks)
            .await?;
        
        // Transform into our output type
        Ok(PortfolioOverview {
            total_value: portfolio.totals.total,
            total_with_nft: portfolio.totals.total_with_nft,
            apps_total: portfolio.totals.apps_total,
            holdings: portfolio.totals.holdings,
            network_breakdown: portfolio.totals.total_by_network,
        })
    }
}