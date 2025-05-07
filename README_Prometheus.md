# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source project designed to provide transparent monitoring and analysis of KOII token transactions on the Koii blockchain. This solution addresses the critical need for tracking and identifying significant wallet movements and potential market manipulation.

#### Key Features
- Real-time monitoring of blockchain transactions
- Detection of large transfers and potential token dumping activities
- Comprehensive tracking of wallet interactions with cryptocurrency exchanges
- Verifiable and transparent transaction flagging mechanism

#### Core Capabilities
- Connects directly to Koii's mainnet RPC to retrieve and analyze transaction data
- Identifies wallets sending tokens to exchange deposit addresses
- Tracks substantial wallet balance changes
- Provides a public, traceable API for transaction insights

#### Benefits
- Enhances blockchain transparency
- Helps identify potential market manipulation
- Offers open-source, community-driven transaction monitoring
- Supports the overall health and integrity of the Koii token ecosystem

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (LTS version recommended)
- npm (Node Package Manager)
- Git

### Quick Start

To get the Koii Blockchain Transaction Analysis Node up and running quickly:

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
   TRANSACTION_FLAG_THRESHOLD=10000  # Example large transaction threshold
   ```

### Development Mode

To run the node in development mode:
```bash
npm run dev
```

### Production Build

To build the project for production:
```bash
npm run build
npm start
```

### Configuration Options

- Modify `config.json` to customize:
  - Exchange deposit addresses
  - Transaction monitoring parameters
  - API endpoint settings

### Platform-Specific Notes

#### Linux/macOS
- Ensure you have the latest Node.js LTS version installed
- Use `nvm` for easy Node.js version management

#### Windows
- Use Windows Subsystem for Linux (WSL) for best compatibility
- Ensure you have Node.js LTS installed via official installer

### Troubleshooting

- Verify Node.js installation: `node --version`
- Check npm installation: `npm --version`
- For dependency issues, run: `npm ci`

### API Access

After starting the node, access the APIs:
- Transaction flags: `http://localhost:PORT/api/flagged-transactions`
- Wallet activity: `http://localhost:PORT/api/wallet/{address}`
- Real-time alerts: `http://localhost:PORT/api/alerts`