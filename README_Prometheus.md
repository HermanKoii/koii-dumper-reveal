# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source project designed to provide comprehensive monitoring and analysis of blockchain transactions on the Koii network. Its primary purpose is to enhance transparency and detect potential market manipulation by tracking significant token movements.

### Key Features

- **Blockchain Transaction Monitoring**: Continuously tracks transactions on the Koii mainnet, focusing on token transfers and wallet interactions with exchanges.
- **Exchange Interaction Tracking**: Identifies and flags wallet transactions involving known exchange deposit addresses.
- **Large Transfer Detection**: Monitors and alerts on substantial wallet balance changes that might indicate potential token dumping.
- **Verifiable API**: Provides a transparent, traceable API that allows querying of transaction data with cryptographic proof.

### Core Benefits

- **Transparency**: Offers an open-source solution for tracking blockchain activity, making market movements more visible.
- **Security Insights**: Helps community members and investors understand significant token movements.
- **Decentralized Monitoring**: Leverages Koii's decentralized task framework to distribute transaction analysis across multiple nodes.

The project aims to create a robust, community-driven system for monitoring blockchain transactions, ultimately promoting fair and transparent token economics.

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

# Copy the example environment file
cp .env.example .env

# Configure your environment variables
# Edit the .env file with your specific settings
```

### Configuration

Create a `.env` file in the project root with the following configurations:

```plaintext
# Koii RPC Endpoint
KOII_RPC_ENDPOINT=https://mainnet.koii.network

# Transaction monitoring thresholds
LARGE_TRANSFER_THRESHOLD=10000  # KOII tokens
FLAGGING_PERCENTAGE=10  # Percentage of wallet balance change

# Optional: API and logging settings
API_PORT=3000
LOG_LEVEL=info
```

### Running the Application

#### Development Mode

```bash
# Start the application in development mode
npm run dev
```

#### Production Build

```bash
# Build the application
npm run build

# Start the production server
npm start
```

### Verifying Installation

After starting the application, the following endpoints will be available:

- `http://localhost:3000/api/flagged-transactions`
- `http://localhost:3000/api/wallet/{address}`
- `http://localhost:3000/api/alerts`

### Troubleshooting

- Ensure you have the correct Node.js version installed
- Check that all environment variables are properly set
- Verify network connectivity to the Koii RPC endpoint