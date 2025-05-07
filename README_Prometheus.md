# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to monitor and track significant KOII token movements across the Koii network. The project provides a comprehensive solution for identifying and flagging potential token dumping behaviors by analyzing blockchain transactions in real-time.

### Key Features
- Real-time blockchain transaction monitoring
- Identification of large wallet transfers and exchange interactions
- Verifiable transaction tracking with comprehensive API endpoints
- Transparent and open-source blockchain analysis

### Core Capabilities
- Connect to Koii's mainnet RPC to retrieve transaction data
- Detect and flag wallets sending large KOII amounts to exchanges
- Generate traceable reports of significant token movements
- Provide a RESTful API for querying transaction and wallet activity

### Benefits
- Enhanced transparency in token ecosystem
- Early detection of potential market manipulation
- Decentralized approach to blockchain transaction analysis
- Community-driven monitoring of token flows

## Getting Started, Installation, and Setup

The Koii Blockchain Transaction Analysis Node is a powerful tool for monitoring and analyzing blockchain transactions. Follow these steps to get started quickly.

### Prerequisites

- Node.js (LTS version recommended)
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
Create a `.env` file in the project root and set the following variables:
```bash
KOII_RPC_ENDPOINT=https://mainnet.koii.network
TRANSACTION_THRESHOLD=10000  # Example large transaction threshold in KOII tokens
```

### Running the Node

#### Development Mode
```bash
npm start
```

#### Production Build
```bash
npm run build
npm run prod
```

### Configuration Options

- Modify the `.env` file to customize:
  - RPC endpoint
  - Transaction size thresholds
  - Logging levels

### Recommended System Requirements

- CPU: 2+ cores
- RAM: 4GB+
- Storage: 20GB free disk space
- Network: Stable internet connection with low latency

### Supported Platforms

- Linux
- macOS
- Windows (with WSL2 recommended)