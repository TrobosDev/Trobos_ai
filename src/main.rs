use dotenv::dotenv;
use prettytable::{Table, row, format};
use rig::{
    providers,
    completion::Prompt,
};
use std::{
    env,
    io::Write,
};
use tools::portfolio::PortfolioOverviewTool;
use serde_json::Value;

mod types;
mod tools;
mod zapper;

fn format_usd(amount: f64) -> String {
    if amount == 0.0 {
        "$ 0.00".to_string()
    } else if amount >= 1_000_000.0 {
        format!("$ {:.2}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("$ {:.2}K", amount / 1_000.0)
    } else {
        format!("$ {:.2}", amount)
    }
}

fn display_portfolio_table(response: &str) {
    let portfolio: Value = serde_json::from_str(response).unwrap();

    // Create Holdings Table
    let mut holdings_table = Table::new();
    holdings_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    holdings_table.set_titles(row!["Holdings", "Balance", "Percentage"]);

    if let Some(holdings) = portfolio["holdings"].as_array() {
        for holding in holdings {
            holdings_table.add_row(row![
                holding["label"].as_str().unwrap_or(""),
                format_usd(holding["balanceUSD"].as_f64().unwrap_or(0.0)),
                // Multiply by 100 since pct is in decimal form
                format!("{:.2}%", holding["pct"].as_f64().unwrap_or(0.0) * 100.0)
            ]);
        }
    }

    // Create Network Breakdown Table
    let mut network_table = Table::new();
    network_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    network_table.set_titles(row!["Network", "Total Value"]);

    if let Some(networks) = portfolio["network_breakdown"].as_array() {
        for network in networks.iter().filter(|n| n["total"].as_f64().unwrap_or(0.0) > 0.0) {
            network_table.add_row(row![
                network["network"].as_str().unwrap_or(""),
                format_usd(network["total"].as_f64().unwrap_or(0.0))
            ]);
        }
    }

    // Print Summary
    println!("\n📊 Portfolio Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Total Portfolio Value: {}", format_usd(portfolio["total_value"].as_f64().unwrap_or(0.0)));
    println!("Total with NFTs: {}", format_usd(portfolio["total_with_nft"].as_f64().unwrap_or(0.0)));
    println!("DeFi Apps Value: {}", format_usd(portfolio["apps_total"].as_f64().unwrap_or(0.0)));
    
    // Print Tables
    println!("\n📈 Holdings Breakdown");
    holdings_table.printstd();
    
    println!("\n🌐 Network Distribution");
    network_table.printstd();
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get API keys from environment
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let zapper_api_key = env::var("ZAPPER_API_KEY").expect("ZAPPER_API_KEY not set");

    // Initialize OpenAI client
    let openai_client = providers::openai::Client::new(&openai_api_key);

    // Create portfolio analysis tool with Zapper client
    let portfolio_tool = PortfolioOverviewTool::new(&zapper_api_key);

    // Create agent with portfolio analysis tool
    let portfolio_agent = openai_client
        .agent("gpt-4")
        .preamble("I am a portfolio analysis assistant that can help analyze crypto wallets using Zapper data. I can provide detailed breakdowns of portfolios including tokens, NFTs, and DeFi positions across multiple networks. I can help you understand:
            - Total portfolio value (with and without NFTs)
            - DeFi positions and their values
            - Token balances and values
            - NFT holdings and estimated values
            - Portfolio distribution across networks
            How can I help you analyze a portfolio today?")
        .tool(portfolio_tool)
        .build();

    println!("Portfolio Analysis Agent Ready! (Type 'exit' to quit)");
    println!("Example: Analyze the portfolio for address 0x123...");

    // Start interactive chat loop
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let response = portfolio_agent.prompt(input).await?;
        display_portfolio_table(&response);
    }

    Ok(())
}