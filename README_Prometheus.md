# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to monitor and track cryptocurrency transactions on the Koii network. The project provides a transparent, open-source solution for identifying and flagging significant wallet activities and potential token dumping behaviors.

### Key Features
- Real-time blockchain transaction monitoring
- Exchange deposit address tracking
- Large transfer detection
- Verifiable transaction API with comprehensive wallet activity insights

### Purpose
The primary objective is to enhance transparency in cryptocurrency transactions by:
- Detecting wallets sending tokens to exchanges
- Identifying sudden large balance changes
- Providing a traceable, community-driven transaction analysis tool

### Benefits
- Open-source blockchain transaction intelligence
- Decentralized transaction monitoring
- Comprehensive wallet activity tracking
- Community-driven approach to crypto market transparency

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 14.x or later)
- npm (Node Package Manager)
- Git

### Installation Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install project dependencies:
   ```sh
   npm install
   ```

### Environment Configuration

1. Create a `.env` file in the project root directory
2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_FLAG_THRESHOLD=10000  # Example large transaction threshold
   ```

### Running the Node

#### Development Mode
To run the node in development mode:
```sh
npm run dev
```

#### Production Mode
To build and start the production version:
```sh
npm run build
npm start
```

### Verification
After starting the node, it will:
- Connect to the Koii mainnet RPC endpoint
- Begin monitoring blockchain transactions
- Provide a local API for transaction queries

### Troubleshooting
- Ensure all dependencies are correctly installed
- Verify that the `.env` file contains the correct configuration
- Check network connectivity to the Koii mainnet

### API Endpoints
Once running, the node provides the following endpoints:
- `/api/flagged-transactions`: List of flagged transactions
- `/api/wallet/{address}`: Wallet historical activity
- `/api/alerts`: Real-time transfer alerts