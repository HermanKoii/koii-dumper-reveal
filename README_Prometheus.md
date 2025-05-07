# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source project designed to provide comprehensive monitoring and analysis of KOII token transactions. This tool enables decentralized tracking of blockchain activities, with a primary focus on identifying and flagging significant token movements and potential market manipulation.

### Key Features
- Real-time blockchain transaction monitoring on Koii's mainnet
- Automated detection of large token transfers to exchanges
- Identification of wallets with suspicious trading patterns
- Verifiable transaction tracking with cryptographic signatures
- Publicly accessible RESTful API for transaction insights

### Problem Solution
The project addresses critical challenges in blockchain transparency by:
- Providing an open-source mechanism to track substantial token movements
- Offering a decentralized approach to monitoring potential market dumping
- Creating a transparent system for tracking wallet interactions with exchanges

### Benefits
- Enhanced market transparency for KOII token ecosystem
- Community-driven transaction analysis
- Decentralized and verifiable transaction monitoring
- Open API for researchers and traders to gain blockchain insights

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (LTS version recommended)
- npm (Node Package Manager)
- Git
- A stable internet connection

### Installation

```bash
# Clone the repository
git clone https://github.com/YOUR-ORG/koii-analysis-node.git
cd koii-analysis-node

# Install dependencies
npm install
```

### Environment Configuration

Create a `.env` file in the project root with the following configurations:

```
# Koii Blockchain RPC Endpoint
KOII_RPC_ENDPOINT=https://mainnet.koii.network

# Transaction Monitoring Thresholds
LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
DUMP_FREQUENCY_THRESHOLD=3      # Number of significant transfers
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

### Verification

After starting the node, it will:
- Connect to the Koii mainnet RPC
- Begin monitoring blockchain transactions
- Detect exchanges and large transfers
- Expose a local API for transaction queries

### Troubleshooting

- Ensure all dependencies are correctly installed
- Verify your `.env` configurations
- Check network connectivity to the Koii mainnet

### API Endpoints

Once running, the node provides these API endpoints:
- `GET /api/flagged-transactions`: List of flagged transactions
- `GET /api/wallet/{address}`: Wallet historical activity
- `GET /api/alerts`: Real-time transfer alerts