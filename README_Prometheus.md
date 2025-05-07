# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to provide transparent monitoring of KOII token transactions across exchanges. The project focuses on tracking and analyzing blockchain activities, with a primary goal of identifying potential token dumping behaviors and providing verifiable insights into significant wallet movements.

### Key Features
- Real-time blockchain transaction monitoring on Koii's mainnet
- Identification of large token transfers and exchange interactions
- Comprehensive API for querying transaction data and wallet activities
- Verifiable transaction tracking with node signatures
- Automated detection of potential token dumping patterns

### Core Capabilities
- Connect to Koii's blockchain network and retrieve transaction data
- Track wallet interactions with known exchange deposit addresses
- Flag and log significant token movements
- Provide a transparent, open-source mechanism for blockchain analysis

### Benefits
- Enhances blockchain transparency for KOII token ecosystem
- Offers real-time insights into token movement and trading behaviors
- Supports community-driven monitoring of token transactions
- Enables decentralized, verifiable transaction tracking

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

### Environment Configuration

1. Create a `.env` file in the project root directory
2. Configure the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_LARGE_TRANSFER_THRESHOLD=10000  # Example threshold
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

After starting the node, it will begin monitoring blockchain transactions. You can verify the node is running by:
- Checking console logs for initialization messages
- Accessing the API endpoints:
  - `/api/flagged-transactions`
  - `/api/wallet/{address}`
  - `/api/alerts`

### Troubleshooting

- Ensure you have the latest version of Node.js installed
- Check that all dependencies are correctly installed
- Verify your `.env` file contains the correct configuration
- Review the logs for any connection or processing errors