# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an open-source project designed to provide comprehensive monitoring and analysis of blockchain transactions on the Koii network. Its primary purpose is to track and flag significant token movements, particularly focusing on exchanges and potential market manipulation.

### Key Features
- Real-time blockchain transaction monitoring
- Identification of wallets interacting with cryptocurrency exchanges
- Detection of large token transfers and potential dumping behavior
- Verifiable and transparent transaction tracking API

### Core Benefits
- Enhances transparency in blockchain token movements
- Provides insights into significant wallet activities
- Supports community-driven blockchain ecosystem monitoring
- Offers an open-source solution for transaction analysis

The project enables nodes to connect to Koii's mainnet, query transaction data, and provide a RESTful API that allows users to trace and investigate blockchain activities with complete traceability and verification.

## Getting Started, Installation, and Setup

### Prerequisites

Before getting started, ensure you have the following:
- Node.js (version 16.x or higher)
- npm (Node Package Manager)
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/YOUR-ORG/koii-analysis-node.git
cd koii-analysis-node

# Install dependencies
npm install
```

### Configuration

Create a `.env` file in the project root with the following configurations:

```
# Koii Blockchain RPC Endpoint
KOII_RPC_ENDPOINT=https://mainnet.koii.network

# Transaction Monitoring Thresholds
LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
DUMP_DETECTION_FREQUENCY=3      # Number of large transfers to flag
```

### Running the Node

#### Development Mode

```bash
# Start the node in development mode
npm run dev
```

#### Production Mode

```bash
# Build the project
npm run build

# Start the production server
npm start
```

### Verifying Installation

After starting the node, it will begin monitoring blockchain transactions. You can verify the installation by:
- Checking the console for initialization logs
- Accessing the API endpoints at `http://localhost:PORT/api/`
  - `/api/flagged-transactions`
  - `/api/wallet/{address}`
  - `/api/alerts`

### Troubleshooting

- Ensure all dependencies are correctly installed
- Verify your `.env` configuration
- Check network connectivity to the Koii RPC endpoint