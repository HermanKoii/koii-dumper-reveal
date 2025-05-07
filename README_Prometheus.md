# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to monitor and track KOII token movements across the Koii network. This open-source tool provides comprehensive transaction intelligence by identifying and flagging significant wallet activities, particularly those involving cryptocurrency exchanges.

### Key Features
- Real-time blockchain transaction monitoring
- Detection of large token transfers and potential market dumping behavior
- Verifiable API for tracking wallet interactions
- Transparent tracking of exchanges like MEXC and Gate.io

### Core Capabilities
- Connects to Koii's mainnet RPC to retrieve transaction data
- Identifies wallets sending tokens to exchange deposit addresses
- Tracks significant wallet balance changes
- Provides a RESTful API for querying transaction and wallet activity

### Benefits
- Enhances blockchain transparency
- Helps identify potentially suspicious trading patterns
- Offers a decentralized approach to transaction analysis
- Enables community-driven monitoring of token movements

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16 or higher)
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

1. Create a `.env` file in the project root directory:
   ```bash
   cp .env.example .env
   ```

2. Configure the following environment variables in the `.env` file:
   - `KOII_RPC_ENDPOINT`: Koii mainnet RPC endpoint (default: `https://mainnet.koii.network`)
   - `LARGE_TRANSFER_THRESHOLD`: Minimum token amount to flag as a significant transfer
   - `EXCHANGE_ADDRESSES`: Comma-separated list of exchange deposit addresses to monitor

### Running the Application

#### Development Mode
```bash
npm run dev
```

#### Production Mode
```bash
npm run build
npm start
```

### Accessing the API

The application provides the following RESTful API endpoints:
- `GET /api/flagged-transactions`: List of flagged transactions
- `GET /api/wallet/{address}`: Wallet transaction history
- `GET /api/alerts`: Real-time transfer alerts

### Troubleshooting

- Ensure all dependencies are correctly installed
- Verify that the `.env` file is properly configured
- Check network connectivity to the Koii RPC endpoint