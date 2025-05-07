# Prometheus: Add README for koii-dumper-reveal

## Project Overview

A blockchain transaction analysis node designed to monitor and track KOII token movements across the Koii network. This open-source project provides a sophisticated system for detecting and analyzing significant blockchain transactions, with a focus on exchange interactions and potential market activities.

### Key Features
- Real-time blockchain transaction monitoring on the Koii network
- Detection of wallet interactions with cryptocurrency exchanges
- Identification of large token transfers and potential market manipulation
- Verifiable and transparent transaction tracking through a RESTful API

### Core Benefits
- Enhances network transparency by providing detailed transaction insights
- Enables proactive tracking of significant token movements
- Offers an open-source solution for blockchain transaction analysis
- Supports the Koii ecosystem by providing a decentralized monitoring tool

## Getting Started, Installation, and Setup

### Prerequisites

Before getting started, ensure you have the following:
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
   - Add the Koii RPC endpoint and transaction flagging thresholds
   ```bash
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   LARGE_TRANSACTION_THRESHOLD=10000  # Example threshold in KOII tokens
   ```

### Running the Node

#### Development Mode
To run the node in development mode:
```bash
npm run dev
```

#### Production Mode
To build and run the production release:
```bash
npm run build
npm start
```

### Configuration Options

- Modify the `.env` file to customize:
  - RPC endpoint
  - Transaction flagging thresholds
  - Logging levels
  - Exchange deposit addresses to monitor

### Common Setup Issues

#### Node.js Version
- Ensure you're using Node.js 16 or higher
- Use `node --version` to check your current version
- Consider using `nvm` to manage Node.js versions

#### Dependency Installation
If you encounter issues installing dependencies:
- Verify your npm is up to date: `npm install -g npm@latest`
- Clear npm cache: `npm cache clean --force`
- Retry installation: `npm install`