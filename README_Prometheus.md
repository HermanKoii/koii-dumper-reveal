# Prometheus: Add README for koii-dumper-reveal

## Project Overview

This open-source Koii Task is designed to provide comprehensive blockchain transaction analysis for the Koii network. The primary purpose is to monitor and track significant KOII token transactions, with a focus on identifying potential market manipulation and exchange interactions.

### Key Features
- Real-time blockchain transaction monitoring on Koii's mainnet
- Detailed tracking of wallet interactions with cryptocurrency exchanges
- Identification of large token transfers and potential dumping behavior
- Verifiable, transparent API for transaction data and wallet activity

### Core Capabilities
- Automated detection of KOII token transfers to known exchange deposit addresses
- Flagging of wallets with substantial balance changes
- Comprehensive transaction tracing with block number and transaction ID verification
- Scalable and decentralized analysis through Koii's task framework

### Benefits
- Enhances market transparency for KOII token ecosystem
- Provides tools for tracking and understanding token movement
- Enables community-driven monitoring of blockchain transactions
- Offers an open-source solution for transaction analysis

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16.x or later)
- npm (Node Package Manager)
- Access to Koii blockchain RPC endpoint

### Quick Start

To quickly get started with the Koii Transaction Analysis Node:

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Configure environment:
   Create a `.env` file in the project root with the following configurations:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
   EXCHANGE_ADDRESSES=mexc_address,gateio_address
   ```

4. Run the node in development mode:
   ```bash
   npm start
   ```

### Development Setup

#### Local Development
- Use `npm run dev` for development with hot-reloading
- Run tests with `npm test`

#### Building for Production
To create a production build:
```bash
npm run build
npm run start:prod
```

### Configuration Options

Modify the `.env` file to customize node behavior:
- `KOII_RPC_ENDPOINT`: Blockchain RPC endpoint
- `LARGE_TRANSFER_THRESHOLD`: Minimum token amount to flag
- `EXCHANGE_ADDRESSES`: Comma-separated list of exchange deposit addresses

### Supported Platforms
- Linux
- macOS
- Windows (with Node.js installed)

### Troubleshooting
- Ensure all dependencies are installed correctly
- Check network connectivity to Koii RPC endpoint
- Verify `.env` file configurations