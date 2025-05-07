# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an open-source tool designed to provide comprehensive monitoring and analysis of KOII token transactions on the Koii blockchain. Its primary purpose is to track and flag significant wallet activities, particularly those involving exchanges and large token transfers.

### Key Features
- Real-time blockchain transaction monitoring
- Identification of wallet interactions with cryptocurrency exchanges
- Detection of large token transfers and potential market dumping behavior
- Verifiable and transparent transaction tracking

### Main Capabilities
- Connect to Koii's mainnet RPC to retrieve and analyze transaction data
- Maintain a database of exchange deposit addresses
- Track and flag wallets with substantial balance changes
- Provide a RESTful API for querying transaction and wallet activity

### Benefits
- Enhances transparency in KOII token transactions
- Helps identify potential market manipulation
- Offers an open-source solution for blockchain transaction analysis
- Enables real-time monitoring of significant token movements

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16.x or later)
- npm (Node Package Manager)
- Git

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

### Environment Configuration

1. Create a `.env` file in the project root directory
2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_THRESHOLD=10000  # Example large transaction threshold in KOII tokens
   ```

### Running the Node

#### Development Mode
```bash
npm run dev
```

#### Production Mode
```bash
npm run build
npm start
```

### Quick Start

1. Ensure all dependencies are installed
2. Configure the `.env` file with your specific settings
3. Start the node to begin monitoring blockchain transactions

### Verification

After starting the node, you can:
- Check logs for transaction monitoring status
- Access the API endpoints to verify transaction tracking
  - `/api/flagged-transactions`
  - `/api/wallet/{address}`
  - `/api/alerts`