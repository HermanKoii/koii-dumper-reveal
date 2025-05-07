# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis tool designed to enhance transparency in the Koii network by monitoring and tracking KOII token transfers. This open-source project provides a decentralized solution for identifying and analyzing significant wallet activities and transaction patterns.

### Purpose

The project aims to bring unprecedented visibility to KOII token transactions by:
- Monitoring blockchain activities on the Koii network
- Identifying large token transfers to and from cryptocurrency exchanges
- Detecting potential market manipulation and dumping behaviors
- Providing a transparent, verifiable tracking mechanism

### Key Features

- **Real-time Transaction Monitoring**: Continuous tracking of KOII token transactions using Koii's mainnet RPC
- **Exchange Interaction Tracking**: Identification of wallets interacting with known exchange deposit addresses
- **Large Transfer Detection**: Flagging of significant wallet balance changes and potential dumping activities
- **Verifiable API**: Transparent transaction data reporting with block numbers, transaction IDs, and node signatures

### Benefits

- Increases market transparency for KOII token transactions
- Enables community-driven blockchain analysis
- Provides real-time tracking of large token movements
- Supports decentralized transaction monitoring through Koii's task framework

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16 or later)
- npm (Node Package Manager)
- Git

### Quick Start

```bash
# Clone the repository
git clone https://github.com/YOUR-ORG/koii-analysis-node.git
cd koii-analysis-node

# Install dependencies
npm install

# Create environment configuration
touch .env
```

### Environment Configuration

Create a `.env` file in the project root with the following optional configurations:

```
KOII_RPC_ENDPOINT=https://mainnet.koii.network
LARGE_TRANSFER_THRESHOLD=10000
POLLING_INTERVAL=60000
```

### Running the Application

#### Development Mode

To run the node in development mode:

```bash
npm start
```

#### Production Build

To build the project for production:

```bash
npm run build
```

### Platform Compatibility

This project is primarily designed for Unix-like systems (Linux, macOS). Windows users may need to use Windows Subsystem for Linux (WSL) for optimal compatibility.

### Recommended Workflow

1. Install dependencies
2. Configure your `.env` file
3. Start the node in development mode
4. Monitor the console for transaction tracking and analysis

### Troubleshooting

- Verify all dependencies are correctly installed
- Ensure the Koii RPC endpoint is accessible
- Check network connectivity and firewall settings