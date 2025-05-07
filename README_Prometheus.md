# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to monitor and track KOII token movements across the Koii network. This open-source tool provides comprehensive transaction monitoring and analysis capabilities, focusing on identifying significant wallet activities and potential market manipulation.

### Key Features
- Real-time blockchain transaction monitoring on Koii's mainnet
- Detection of large token transfers and wallet interactions with exchanges
- Verifiable API for transparent transaction tracking
- Identification of potential token "dumping" behavior

### Core Capabilities
- Connect to Koii's RPC endpoint to retrieve blockchain transaction data
- Track wallets sending tokens to known exchange deposit addresses
- Flag and analyze significant wallet balance changes
- Provide a RESTful API for querying transaction and wallet activity

### Benefits
- Enhances transparency in token movement tracking
- Enables community-driven blockchain activity monitoring
- Supports open-source collaboration in blockchain analytics
- Provides a decentralized approach to transaction verification

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16+ recommended)
- npm package manager
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

3. Configure environment variables:
   Create a `.env` file in the project root with the following configuration:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
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

### Configuration Options

- Modify `config.json` to adjust transaction monitoring parameters
- Update exchange deposit addresses in the designated configuration file

### Troubleshooting

- Ensure all dependencies are correctly installed
- Check that the Koii RPC endpoint is accessible
- Verify network connectivity before running the node