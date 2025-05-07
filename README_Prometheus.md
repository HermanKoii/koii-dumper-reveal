# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an open-source project designed to provide comprehensive monitoring and analysis of KOII token transactions on the Koii blockchain. Its primary purpose is to enhance transparency and detect potentially suspicious trading activities by tracking and flagging significant wallet movements and exchange interactions.

### Key Features
- **Blockchain Transaction Monitoring**: Continuously polls the Koii mainnet to retrieve and analyze transaction data in real-time.
- **Exchange Interaction Tracking**: Identifies and tracks wallets sending KOII tokens to known exchange deposit addresses.
- **Large Transfer Detection**: Flags wallets with significant balance changes that might indicate potential market manipulation.
- **Verifiable API**: Provides a transparent, traceable API that allows external users to query transaction data with cryptographic verification.

### Benefits
- Increases transparency in KOII token trading
- Helps identify potential market dumping activities
- Offers a decentralized approach to transaction monitoring
- Provides an open-source solution for blockchain transaction analysis

The project is designed to run as a Koii Task, enabling decentralized and collaborative transaction tracking across multiple nodes.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16 or later)
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

1. Create a `.env` file in the project root directory with the following configurations:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_THRESHOLD=10000  # Example threshold for large transactions
   ```

   Customize the following environment variables:
   - `KOII_RPC_ENDPOINT`: Koii network RPC endpoint
   - `TRANSACTION_THRESHOLD`: Minimum KOII amount to flag a large transaction

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

### Usage

After starting the node, it will automatically:
- Connect to the Koii mainnet
- Monitor blockchain transactions
- Analyze wallet activities
- Provide API endpoints for querying transaction data

Access the API endpoints:
- Flagged Transactions: `http://localhost:PORT/api/flagged-transactions`
- Wallet Activity: `http://localhost:PORT/api/wallet/{address}`
- Real-time Alerts: `http://localhost:PORT/api/alerts`

### Troubleshooting

- Ensure you have the latest version of Node.js installed
- Check your internet connection to the Koii RPC endpoint
- Verify that all environment variables are correctly set