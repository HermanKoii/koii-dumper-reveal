# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an open-source tool designed to monitor and analyze blockchain transactions on the Koii network. This project provides a sophisticated solution for tracking and identifying significant cryptocurrency movements, with a focus on exchange interactions and potential market manipulation.

### Key Features

- **Comprehensive Transaction Monitoring**: Connects to Koii's mainnet RPC to retrieve and analyze real-time blockchain transaction data
- **Exchange Interaction Tracking**: Identifies and flags wallets sending or receiving tokens from known exchange deposit addresses
- **Large Transfer Detection**: Monitors and highlights substantial wallet balance changes that may indicate potential market dumping
- **Verifiable Blockchain Analysis**: Provides a transparent, traceable API that includes block numbers, transaction IDs, and node signatures for each flagged transaction

### Core Benefits

- Enhances blockchain transparency by providing detailed insights into token movements
- Helps identify potential market manipulation strategies
- Offers an open-source, community-driven approach to blockchain transaction analysis
- Enables real-time tracking of significant cryptocurrency transactions

The project serves as a crucial tool for researchers, traders, and blockchain enthusiasts seeking to understand complex token transfer patterns on the Koii network.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16.x or later)
- npm (Node Package Manager)
- Access to Koii Network RPC endpoint

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install project dependencies:
   ```bash
   npm install
   ```

### Configuration

1. Create a `.env` file in the project root directory.
2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   LARGE_TRANSFER_THRESHOLD=10000 # KOII tokens
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

### Verification

After starting the node, verify the following:
- The node successfully connects to the Koii RPC endpoint
- Transaction monitoring is active
- API endpoints are accessible at `http://localhost:PORT`

### Troubleshooting

- Ensure all dependencies are installed correctly
- Check network connectivity to the Koii RPC endpoint
- Verify environment variables are set properly