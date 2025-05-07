# Prometheus: Add README for koii-dumper-reveal

## Project Overview

The Koii Blockchain Transaction Analysis Node is an innovative open-source project designed to provide comprehensive monitoring and analysis of blockchain transactions on the Koii network. Its primary purpose is to track and identify significant financial activities, particularly focusing on exchange interactions and potential token dumping behaviors.

### Key Features
- **Blockchain Transaction Monitoring**: Real-time tracking of KOII token transfers across the network
- **Exchange Interaction Detection**: Identifies wallet transactions with known exchange deposit addresses
- **Large Transfer Tracking**: Flags and monitors significant wallet balance changes
- **Verifiable API**: Provides a transparent, traceable API for transaction insights

### Benefits
- Enhances network transparency by providing detailed transaction analysis
- Helps identify potentially disruptive trading behaviors
- Offers an open-source solution for blockchain transaction monitoring
- Supports the Koii network's ecosystem by providing valuable transactional intelligence

The project enables users to gain deep insights into KOII token movements, supporting more informed decision-making and network health monitoring.

## Getting Started, Installation, and Setup

### Prerequisites

- Node.js (LTS version recommended)
- npm (Node Package Manager)
- Git

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR-ORG/koii-analysis-node.git
   cd koii-analysis-node
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

### Configuration

1. Create a `.env` file in the project root directory.
2. Add the following environment variables:
   ```
   KOII_RPC_ENDPOINT=https://mainnet.koii.network
   TRANSACTION_THRESHOLD=10000  # Example large transaction threshold in KOII tokens
   ```

### Running the Node

#### Development Mode
To run the node in development mode:
```bash
npm run dev
```

#### Production Mode
To build and start the production version:
```bash
npm run build
npm start
```

### Verification

After starting the node, verify it's running correctly by checking the console output for connection and synchronization messages.

### Docker Support (if applicable)

To run using Docker:
```bash
docker build -t koii-analysis-node .
docker run -p 3000:3000 koii-analysis-node
```

### Notes
- Ensure you have a stable internet connection
- The node requires continuous running to monitor blockchain transactions
- Performance may vary based on your hardware and network conditions