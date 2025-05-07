# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source task designed to provide comprehensive monitoring and analysis of KOII token transactions. Its primary purpose is to track and flag significant blockchain activities, focusing on identifying potential market manipulation and providing transparent insights into token movements.

### Key Features
- **Blockchain Transaction Monitoring**: Continuously tracks transactions on the Koii network, connecting to the mainnet RPC endpoint to retrieve and analyze real-time transaction data.
- **Exchange Interaction Detection**: Identifies and flags wallet interactions with known cryptocurrency exchanges, helping to highlight potential large-scale token transfers.
- **Large Transfer Tracking**: Monitors substantial wallet balance changes and flags wallets that demonstrate repeated significant token movements.
- **Verifiable API**: Provides a transparent, traceable API that allows users to query flagged transactions, wallet activities, and balance trends with full transaction verification.

### Benefits
- Enhances transparency in blockchain token movements
- Helps detect potential market manipulation
- Offers a decentralized, open-source approach to transaction analysis
- Provides real-time insights into KOII token ecosystem dynamics

## Getting Started, Installation, and Setup

### Prerequisites
- Node.js (version 16 or higher)
- npm (Node Package Manager)
- Git

### Installation Steps
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
1. Create a `.env` file in the project root directory
2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_LARGE_TRANSFER_THRESHOLD=1000 # Example threshold in KOII tokens
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

### Quick Start
1. Install dependencies
2. Configure your `.env` file
3. Run the node
4. Access the API at `http://localhost:3000`

### API Endpoints
- `/api/flagged-transactions`: List of flagged transactions
- `/api/wallet/{address}`: Wallet historical activity
- `/api/alerts`: Real-time transfer alerts

### Verification
Each flagged transaction includes:
- Block number
- Transaction ID
- Node signature