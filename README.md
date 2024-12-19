# Trobos Portfolio Analyzer

A Rust-based CLI tool demonstrating the use of Rig with Zapper's GraphQL API and OpenAI to create an interactive portfolio analysis agent.

## Features
- Integration of Rig with Zapper's GraphQL API
- Interactive CLI interface using Rig's agent and tool system
- Multi-network portfolio analysis
- Support for both EVM and Solana addresses
- Pretty-printed table output

## Prerequisites
- Rust and Cargo installed
- OpenAI API key
- Zapper API key

## Setup
1. Clone the repository
2. Create a `.env` file with your API keys:
   ```
   OPENAI_API_KEY=your_openai_key
   ZAPPER_API_KEY=your_zapper_key
   ```
3. Build and run:
   ```bash
   cargo build
   cargo run
   ```

## Usage
After starting the program:
```
> analyze <wallet-address>
```

Example:
```
> analyze 0x123...  # for Ethereum addresses
> analyze your.solana.address  # for Solana addresses
```

## Project Structure
- `src/zapper/` - Zapper API client and types
- `src/tools/` - Rig tool implementations
- `src/types.rs` - Shared type definitions
- `src/main.rs` - CLI interface and agent setup

## Tobos Overview
Agent tools, on the other hand, are functions that allow the agent to perform specific tasks such analyzing your portfolio asset.
In Trobos ai :
The agent uses a tool to query Zapper’s API and GPT to interpret the data / provide insights.
Tools make Trobos agents extensible and focused.
![image](https://github.com/user-attachments/assets/25d7ea78-8326-4de4-b1ee-cd053a59f599)
![image](https://github.com/user-attachments/assets/0c6c461f-6e8f-478a-9634-3b11f818bf1c)

you can easily paste your address to our website for using our tools

![image](https://github.com/user-attachments/assets/0f4f9fe5-1fa5-4131-8cdc-c6fe6f40b17e)

## Contact
Website : 
Twitter : https://x.com/Trobos_ai


