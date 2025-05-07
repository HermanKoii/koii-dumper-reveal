# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source project designed to provide comprehensive monitoring and analysis of KOII token transactions. Its primary purpose is to track and flag significant blockchain activities, particularly focusing on detecting potential token dumping behaviors and monitoring wallet interactions with cryptocurrency exchanges.

### Key Features
- **Real-time Transaction Monitoring**: Continuously tracks transactions on the Koii blockchain
- **Exchange Interaction Detection**: Identifies wallet transactions with major exchanges like MEXC and Gate.io
- **Large Transfer Tracking**: Flags wallets with substantial balance changes or repeated large transfers
- **Verifiable API**: Provides a transparent, traceable API for blockchain transaction insights

### Benefits
- Enhanced transparency in blockchain token movements
- Early detection of potential market manipulation
- Open-source solution for community-driven blockchain analysis
- Decentralized approach to transaction monitoring

The project leverages Koii's JSON-RPC API to retrieve and analyze blockchain data, offering a robust system for tracking and understanding token transfer patterns. By providing a verifiable and open-source platform, it enables community members to gain deeper insights into blockchain transaction dynamics.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (recommended version 16.x or later)
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

### Environment Configuration

1. Create a `.env` file in the project root directory:
   ```bash
   touch .env
   ```

2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
   ```

### Running the Node

#### Development Mode
To run the node in development mode with hot reloading:
```bash
npm run dev
```

#### Production Mode
To build and run the production version:
```bash
npm run build
npm start
```

### Verification
After starting the node, verify it's running correctly by checking the console output or logs.

### Usage Tips
- Ensure your system meets the [Technical Requirements](#technical-requirements)
- Monitor the console for blockchain transaction tracking logs
- Access the API endpoints to retrieve transaction data