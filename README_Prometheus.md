# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A specialized blockchain transaction analysis node designed to monitor and track KOII token movements across the Koii network. This open-source project provides a comprehensive solution for detecting and analyzing significant blockchain transactions, with a focus on identifying potential market manipulation and exchange-related activities.

### Key Features
- Real-time blockchain transaction monitoring
- Automated detection of large token transfers
- Identification of wallets interacting with cryptocurrency exchanges
- Verifiable, transparent transaction tracking
- RESTful API for querying blockchain activity

### Core Capabilities
The node connects directly to Koii's mainnet RPC, continuously polling for new blocks and analyzing transaction data. It specializes in:
- Tracking wallet interactions with known exchange deposit addresses
- Detecting sudden and significant wallet balance changes
- Flagging potential token dumping behaviors
- Providing a transparent, verifiable record of blockchain activities

### Benefits
- Enhanced market transparency
- Early detection of potential token manipulation
- Open-source infrastructure for blockchain analysis
- Decentralized approach to transaction monitoring

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 14 or later)
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

1. Create a `.env` file in the project root directory.

2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_FLAG_THRESHOLD=10000  # Example: Flag transactions over 10,000 KOII
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
After starting the node, it will:
- Connect to the Koii mainnet RPC
- Begin monitoring blockchain transactions
- Provide API endpoints for querying transaction data

### API Endpoints
- `/api/flagged-transactions`: List of flagged transactions
- `/api/wallet/{address}`: Wallet historical activity
- `/api/alerts`: Real-time transfer alerts

### Troubleshooting
- Ensure all dependencies are correctly installed
- Verify `.env` file contains the correct RPC endpoint
- Check network connectivity to the Koii mainnet