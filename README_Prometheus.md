# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source project designed to provide comprehensive monitoring and analysis of blockchain transactions on the Koii network. Its primary purpose is to track and flag significant token movements, with a specific focus on identifying potential market manipulation activities.

### Key Features

- **Blockchain Transaction Monitoring**: Continuously polls the Koii mainnet to retrieve and analyze real-time transaction data
- **Exchange Interaction Tracking**: Identifies and logs wallet interactions with known cryptocurrency exchanges
- **Large Transfer Detection**: Flags wallets with substantial balance changes or repeated large transfers
- **Verifiable Reporting**: Provides a transparent, traceable API that includes detailed transaction information with node signatures

### Benefits

- **Market Transparency**: Helps community members understand significant token movements
- **Potential Manipulation Detection**: Assists in identifying suspicious trading patterns
- **Open-Source Collaboration**: Enables community-driven blockchain analysis
- **Decentralized Execution**: Designed to run as a Koii Task, ensuring distributed and reliable monitoring

The project serves as a critical tool for maintaining market integrity by providing an transparent, community-driven approach to tracking blockchain transactions.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (v14 or later)
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

- `KOII_RPC_ENDPOINT`: Specifies the Koii blockchain RPC endpoint
- `LARGE_TRANSFER_THRESHOLD`: Define the transaction amount that triggers flagging

### Platform Considerations

#### Linux/macOS
- Ensure bash is available
- Install required system dependencies if prompted

#### Windows
- Use Windows Subsystem for Linux (WSL) recommended
- Alternatively, use Git Bash or PowerShell

### Troubleshooting

- Verify Node.js installation: `node --version`
- Check npm installation: `npm --version`
- Ensure all dependencies are correctly installed