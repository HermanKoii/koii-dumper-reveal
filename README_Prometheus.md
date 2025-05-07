# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis tool designed to monitor and track KOII token transfers across exchanges and identify potential market manipulation. This open-source Koii Task provides a transparent, decentralized solution for tracking significant wallet activities and transaction patterns.

### Key Purpose

The project aims to enhance market transparency by:
- Monitoring blockchain transactions on the Koii network
- Identifying large token transfers to and from cryptocurrency exchanges
- Detecting potential market dumping behaviors
- Providing a verifiable, open-source tracking mechanism

### Core Features

- **Real-time Transaction Monitoring**: Continuously tracks KOII token transactions using Koii's mainnet RPC
- **Exchange Interaction Tracking**: Identifies wallets sending tokens to known exchange deposit addresses
- **Large Transfer Detection**: Flags significant wallet balance changes and potential dumping activities
- **Verifiable API**: Offers transparent, traceable transaction data with block numbers, transaction IDs, and node signatures

### Benefits

- Increases market transparency for KOII token transactions
- Provides an open-source tool for community-driven blockchain analysis
- Enables real-time tracking of large token movements
- Supports decentralized transaction monitoring through Koii's task framework

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (recommended version 16 or later)
- npm (Node Package Manager)
- Git

### Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Configure the environment:
   - Create a `.env` file in the project root
   - Add necessary configuration for the Koii RPC endpoint
   - Set transaction flagging thresholds

### Development Mode

To run the node in development mode:
```bash
npm start
```

### Production Build

To build the project for production:
```bash
npm run build
```

### Configuration Options

The following environment variables can be set in the `.env` file:
- `KOII_RPC_ENDPOINT`: Koii mainnet RPC endpoint (default: `https://mainnet.koii.network`)
- `LARGE_TRANSFER_THRESHOLD`: Minimum transfer amount to flag (e.g., `10000`)
- `POLLING_INTERVAL`: Frequency of blockchain data retrieval (in milliseconds)

### Platform Considerations

This project is primarily designed for Unix-like systems (Linux, macOS). Windows users may need to use Windows Subsystem for Linux (WSL) for optimal compatibility.

### Troubleshooting

- Ensure all dependencies are correctly installed
- Verify that the Koii RPC endpoint is accessible
- Check network connectivity and firewall settings