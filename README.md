# Rig Portfolio Analyzer Example

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
