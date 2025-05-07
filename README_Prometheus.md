# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A decentralized blockchain transaction analysis node designed to monitor and track significant KOII token movements across the Koii network. This open-source task provides real-time insights into cryptocurrency transactions, focusing on exchange interactions and large transfer detection.

### Key Features
- Real-time blockchain transaction monitoring
- Identification of wallets interacting with cryptocurrency exchanges
- Detection of large token transfers and potential market dumping behavior
- Verifiable, transparent transaction tracking through a RESTful API

### Core Benefits
- Enhanced market transparency for KOII token movements
- Automated tracking of significant wallet activities
- Open-source solution for blockchain transaction analysis
- Decentralized execution leveraging the Koii network infrastructure

### Primary Use Cases
- Tracking large token transfers
- Identifying potential market manipulation
- Providing transparent, verifiable transaction data
- Supporting community-driven blockchain insights

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16.x or later)
- npm (Node Package Manager)
- Git

### Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/koii-network/transaction-analysis-node.git
   cd transaction-analysis-node
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Configure the environment:
   Create a `.env` file in the project root with the following configurations:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_THRESHOLD=10000  # Example large transaction threshold
   ```

### Development Mode

To run the node in development mode:
```bash
npm run dev
```

### Production Build

1. Build the project:
   ```bash
   npm run build
   ```

2. Start the production server:
   ```bash
   npm start
   ```

### Verifying Installation

After installation, the node will:
- Connect to Koii's mainnet RPC
- Begin monitoring blockchain transactions
- Start the verifiable API server

### Docker Support

A `Dockerfile` is included for containerized deployment. To build and run:
```bash
docker build -t koii-analysis-node .
docker run -p 3000:3000 koii-analysis-node
```

### Environment Variables

- `KOII_RPC_ENDPOINT`: Koii blockchain RPC endpoint
- `TRANSACTION_THRESHOLD`: Minimum KOII amount to flag as a large transfer
- `API_PORT`: Port for the verification API (default: 3000)

### Troubleshooting

- Ensure all dependencies are installed correctly
- Check that the Koii RPC endpoint is accessible
- Verify network connectivity and firewall settings