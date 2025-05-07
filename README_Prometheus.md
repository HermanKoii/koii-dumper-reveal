# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an open-source project designed to provide comprehensive monitoring and analysis of KOII token transactions on the Koii blockchain. Its primary purpose is to track and flag significant blockchain activities, particularly focusing on exchange interactions and large token transfers.

### Key Features
- Real-time blockchain transaction monitoring
- Identification of wallets interacting with cryptocurrency exchanges
- Detection of large token transfers and potential market dumping behavior
- Verifiable and transparent transaction tracking
- Comprehensive RESTful API for transaction and wallet activity queries

### Core Benefits
- Enhances blockchain transparency by providing detailed transaction insights
- Enables proactive monitoring of token movement patterns
- Supports community-driven blockchain analysis
- Provides an open-source solution for tracking potentially suspicious blockchain activities

The project leverages Koii's mainnet RPC infrastructure to continuously poll and analyze blockchain transactions, offering a robust and decentralized approach to transaction monitoring.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (version 16 or higher)
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
   - Add the following configuration:
     ```
     KOII_RPC_ENDPOINT=https://mainnet.koii.network
     TRANSACTION_THRESHOLD=10000  # Example large transaction threshold in KOII tokens
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

- Modify transaction thresholds in the `.env` file
- Customize exchange deposit addresses in the configuration
- Adjust polling intervals for blockchain querying

### Environment Variables

| Variable            | Description                          | Default Value                      |
|--------------------|--------------------------------------|-------------------------------------|
| `KOII_RPC_ENDPOINT`| Koii blockchain RPC endpoint         | `https://mainnet.koii.network`     |
| `TRANSACTION_THRESHOLD` | Minimum transfer amount to flag | `10000` |

### Platform Considerations

- **Linux/macOS:** Fully supported, no additional setup required
- **Windows:** Requires Node.js with WSL or Git Bash recommended