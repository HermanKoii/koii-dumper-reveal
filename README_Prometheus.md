# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A specialized blockchain analysis node designed to monitor and track KOII token transactions across the Koii network. This open-source project provides a robust solution for identifying and analyzing significant wallet activities, with a focus on detecting potential market manipulation.

### Key Features
- Real-time blockchain transaction monitoring
- Exchange deposit address tracking
- Large transfer detection
- Verifiable transaction flagging
- Comprehensive RESTful API for transaction queries

### Core Capabilities
The node connects directly to Koii's mainnet RPC, enabling powerful transaction analysis capabilities. It systematically identifies and flags wallets involved in substantial KOII transfers, particularly those interacting with cryptocurrency exchanges. By providing transparent, traceable transaction data, the project helps maintain market integrity and offers insights into token movement patterns.

### Benefits
- Enhanced transparency in blockchain token movements
- Proactive detection of potential market dumping
- Open-source, community-driven transaction analysis
- Flexible API for developers and researchers
- Decentralized approach to transaction monitoring

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (recommended version 16.x or later)
- npm (Node Package Manager)
- Git

### Quick Start

1. Clone the repository:
   ```sh
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install dependencies:
   ```sh
   npm install
   ```

3. Configure the environment:
   Create a `.env` file in the project root and add the following configuration:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_THRESHOLD=10000  # Example large transaction threshold in KOII tokens
   ```

### Development Mode

To run the node in development mode:
```sh
npm run dev
```

### Production Build

1. Build the project:
   ```sh
   npm run build
   ```

2. Start the production server:
   ```sh
   npm start
   ```

### Configuration Options

- Modify the `.env` file to adjust:
  - Koii RPC endpoint
  - Transaction flagging thresholds
  - Other environment-specific settings

### Recommended Workflow

1. Install dependencies
2. Configure environment variables
3. Run in development mode for testing
4. Build and deploy to production

### Troubleshooting

- Ensure all dependencies are correctly installed
- Verify network connectivity to Koii's mainnet
- Check `.env` file for correct configuration