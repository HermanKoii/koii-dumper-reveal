# Koii Blockchain Transaction Analysis API

## 1. Project Overview

This backend service provides a comprehensive REST API for monitoring and analyzing blockchain transactions on the Koii network. The API enables real-time tracking of significant token movements, exchange interactions, and potential market activities.

### Key Features
- 🔍 Real-time blockchain transaction monitoring
- 📊 Wallet activity tracking
- 🚨 Large transfer detection
- 🔐 Verifiable transaction flagging
- 🌐 Open-source and transparent analysis

### Use Cases
- Crypto market intelligence
- Exchange deposit tracking
- Token movement analysis
- Potential market manipulation detection

## 2. Getting Started

### Prerequisites
- Node.js (v16+ recommended)
- npm or yarn
- Access to Koii blockchain RPC endpoint

### Installation
1. Clone the repository
```bash
git clone https://github.com/YOUR-ORG/koii-analysis-node.git
cd koii-analysis-node
```

2. Install dependencies
```bash
npm install
```

3. Configure environment variables
Create a `.env` file with the following:
```env
KOII_RPC_ENDPOINT=https://mainnet.koii.network
LARGE_TRANSFER_THRESHOLD=10000
```

4. Start the development server
```bash
npm run dev
```

## 3. API Documentation

### Endpoints

#### 1. Flagged Transactions
- **GET** `/api/flagged-transactions`
  - Retrieves list of flagged blockchain transactions
  - Params:
    - `limit` (optional): Number of transactions to return
    - `offset` (optional): Pagination offset

  **Example Request:**
  ```bash
  curl http://localhost:3000/api/flagged-transactions?limit=10
  ```

  **Example Response:**
  ```json
  {
    "transactions": [
      {
        "txId": "abc123...",
        "from": "wallet_address_1",
        "to": "exchange_deposit_address",
        "amount": 50000,
        "timestamp": "2023-07-15T10:30:00Z"
      }
    ]
  }
  ```

#### 2. Wallet Activity
- **GET** `/api/wallet/{address}`
  - Retrieve historical activity for a specific wallet
  - Params:
    - `address`: Target wallet address

  **Example Request:**
  ```bash
  curl http://localhost:3000/api/wallet/0x1234...
  ```

#### 3. Real-time Alerts
- **GET** `/api/alerts`
  - WebSocket endpoint for real-time transfer alerts

## 4. Authentication

The API uses API key authentication:

- Include `X-API-KEY` header in requests
- API keys can be generated in user dashboard
- Rate limits apply based on key tier

**Example Header:**
```http
X-API-KEY: your_secret_api_key_here
```

## 5. Project Structure
```
koii-analysis-node/
├── src/
│   ├── routes/         # API route definitions
│   ├── controllers/    # Request handling logic
│   ├── models/         # Data models
│   ├── services/       # Core business logic
│   └── utils/          # Helper functions
├── tests/              # Unit and integration tests
└── config/             # Configuration files
```

## 6. Technologies Used
- Node.js
- Express.js
- TypeScript
- Web3.js
- Koii RPC SDK
- Jest (testing)

## 7. Deployment

### Docker
```bash
docker build -t koii-analysis-node .
docker run -p 3000:3000 koii-analysis-node
```

### Cloud Deployment
- Supports deployment on:
  - AWS EC2
  - Google Cloud Run
  - Heroku

## 8. License

[MIT License](LICENSE) - Open-source, free to use and modify.

## Contribution

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting a pull request.

---

**Note:** This service is part of the decentralized Koii network ecosystem. Stay informed about updates and community developments!