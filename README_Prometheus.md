# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to monitor and track KOII token transactions with a focus on identifying potential market manipulation. This open-source Koii Task provides a comprehensive solution for tracking and analyzing blockchain activities, specifically targeting exchange interactions and large wallet transfers.

### Key Features
- Real-time monitoring of KOII token transactions
- Identification of wallets interacting with cryptocurrency exchanges
- Detection of large transfers and potential market dumping behavior
- Verifiable and transparent transaction tracking
- Comprehensive RESTful API for querying transaction data

### Core Benefits
- Enhances market transparency for KOII token ecosystem
- Provides early warning system for significant token movements
- Enables community-driven blockchain activity analysis
- Supports decentralized monitoring through Koii's task framework

The solution empowers users and stakeholders with detailed insights into KOII token transfers, helping maintain market integrity and providing valuable transaction intelligence.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (LTS version recommended)
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

### Configuration

Create a `.env` file in the project root with the following configuration:

```bash
# Koii RPC Endpoint
KOII_RPC_ENDPOINT=https://mainnet.koii.network

# Transaction Flagging Thresholds
LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
DUMP_TRANSACTION_FREQUENCY=3    # Number of large transfers to flag
```

### Running the Node

#### Development Mode
To run the node in development mode:
```bash
npm run dev
```

#### Production Mode
To build and start the production version:
```bash
npm run build
npm start
```

### Verifying Installation

After starting the node, verify it's running correctly by checking the console logs. The node should connect to the Koii RPC endpoint and begin monitoring transactions.

### Troubleshooting

- Ensure all dependencies are correctly installed
- Check that your `.env` file is properly configured
- Verify network connectivity to the Koii RPC endpoint

### API Endpoints

Once running, the node provides the following API endpoints:

- `GET /api/flagged-transactions`: List of flagged transactions
- `GET /api/wallet/{address}`: Historical activity for a specific wallet
- `GET /api/alerts`: Real-time transfer alerts