# 🦀 Solana Fellowship HTTP Server (Rust Backend)

This project was built as part of the **Solana Fellowship Assignment**.

A lightweight, fast, and production-ready HTTP server in Rust that exposes Solana on-chain instruction builders over REST APIs.

---

## ✅ Features

- 🔑 Generate Solana keypairs
- 🪙 Create and Mint SPL Tokens
- 💸 Build SOL transfer instructions
- ✉️ Sign and verify messages
- 📦 Built using **Rust**, **Axum**, **Tokio**, and **Solana SDK**
- ✅ Ed25519 for signatures, Base58 for keys, Base64 for binary data
- ✅ Detailed JSON responses with unified success/error formats

---

## 🛠️ Tech Stack

- **Rust**
- **Axum** (HTTP framework)
- **Tokio** (Async runtime)
- **Solana SDK**
- **Serde** (JSON serialization/deserialization)
- **Base58** and **Base64** encoding libraries

---

## 📂 Project Directory Structure

solana_fellowship_server/
├── src/
│   ├── handlers/       # API endpoint logic
│   ├── models/         # Request/response data models
│   ├── utils/          # Utility functions (encoding etc.)
│   ├── error.rs        # Error handling
│   └── main.rs         # Server startup & routing
├── Cargo.toml
└── README.md



---

## Clone, Build, and Run
To get the server up and running, follow these steps:

1. Clone the Repository:
```
git clone https://github.com/suryapratap04/solana-Assignment.git
```

2. Change Directory:
```
cd solana-Assignment
```

3. Set up the .env file by adding PORT:
```
PORT=8080
```

4. Build the Project:
```
cargo build
```
4. Run the Project:
```
cargo run
```
5. Server will start at: 
``` 
http://127.0.0.1:8080
```


---
## ✅ API Endpoints (Input and Output Format)
### ✅ Common Response Formats
Success Response (HTTP 200)
```
{
  "success": true,
  "data": { /* endpoint-specific content */ }
}
```

Error Response (HTTP 400)
```
{
  "success": false,
  "error": "Description of the error"
}

```

## 📡 API Endpoint Details
1. Generate Keypair
POST /keypair

```
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}

```

2. Create Token (Mint Initialization)
POST /token/create

Request Body:

```
{
  "mintAuthority": "base58-encoded-public-key",
  "mint": "base58-encoded-public-key",
  "decimals": 6
}

```

Response
```
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "pubkey",
        "is_signer": false,
        "is_writable": true
      },
      {
        "pubkey": "SysvarRent111111111111111111111111111111111",
        "is_signer": false,
        "is_writable": false
      }
    ],
    "instruction_data": "base64-encoded-data"
  }
}

```


3. Mint Token (Mint-to Instruction)
POST /token/mint

Request Body:
```
{
  "mint": "mint-address",
  "destination": "destination-user-address",
  "authority": "authority-address",
  "amount": 1000000
}
```

Response:

```
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "mint",
        "is_signer": false,
        "is_writable": true
      },
      {
        "pubkey": "destination",
        "is_signer": false,
        "is_writable": true
      },
      {
        "pubkey": "authority",
        "is_signer": true,
        "is_writable": false
      }
    ],
    "instruction_data": "base64-encoded-data"
  }
}

```
4. Sign Message
POST /message/sign

Request Body:
```
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```
Success Response:

```
{
  "success": true,
  "data": {
    "signature": "base64-encoded-signature",
    "public_key": "base58-encoded-public-key",
    "message": "Hello, Solana!"
  }
}

```
Error (Missing Fields):

```
{
  "success": false,
  "error": "Missing required fields"
}
```

5. Verify Message Signature
POST /message/verify

Request Body:
```
{
  "message": "Hello, Solana!",
  "signature": "base64-encoded-signature",
  "pubkey": "base58-encoded-public-key"
}
```
Success Response:

```
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "base58-encoded-public-key"
  }
}
```
Error (Invalid Signature):

```
{
  "success": false,
  "error": "Invalid signature"
}
```
6. Send SOL Transfer (System Program)
POST /send/sol

Request Body:
```
{
  "from": "sender-address",
  "to": "recipient-address",
  "lamports": 100000
}
```
Response:
```
{
  "success": true,
  "data": {
    "program_id": "11111111111111111111111111111111",
    "accounts": [
      "sender-address",
      "recipient-address"
    ],
    "instruction_data": "base64-encoded-data"
  }
}
```
7. Send Token Transfer (SPL Token Transfer Instruction)
POST /send/token

Request Body:
```
{
  "destination": "destination-user-address",
  "mint": "mint-address",
  "owner": "owner-address",
  "amount": 100000
}
```
Response:
```
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "owner",
        "isSigner": false
      },
      {
        "pubkey": "destination",
        "isSigner": false
      },
      {
        "pubkey": "owner",
        "isSigner": true
      }
    ],
    "instruction_data": "base64-encoded-data"
  }
}
```
---
## ✅ Testing Tools Used
📬 Postman

💻 cURL

🧪 Rust Unit Tests (Optional)

## ⚠️ Important Notes
This server only generates instructions for Solana on-chain programs.

It does NOT submit transactions to the blockchain (no RPC broadcast).

No private keys are stored.

Input validation and error handling implemented for all endpoints.







