# LumenPerks Monorepo

Customer loyalty rewards app built on Stellar Soroban, orchestrated with Turborepo.

## Structure

```
lumenperks-monorepo/
├── apps/
│   ├── frontend/        # Next.js 14 (App Router)
│   └── backend/         # Node.js / Express API
├── contracts/
│   └── lumenperks/      # Rust Soroban smart contract
├── packages/
│   └── tsconfig/        # Shared TypeScript config
├── Cargo.toml           # Rust workspace
├── package.json         # npm workspace root
└── turbo.json           # Turborepo pipeline
```

## Getting Started

### Prerequisites
- Node.js 20+
- Rust + `soroban-cli` (`cargo install --locked soroban-cli`)

### Install dependencies
```bash
npm install
```

### Run all apps in dev mode
```bash
npm run dev
```

| App      | URL                    |
|----------|------------------------|
| Frontend | http://localhost:3000  |
| Backend  | http://localhost:4000  |

### Build everything
```bash
npm run build
```

## Smart Contract

The Soroban contract lives in `contracts/lumenperks/` and exposes four functions:

| Function     | Description                        |
|--------------|------------------------------------|
| `register`   | Register a new customer            |
| `add_points` | Credit loyalty points              |
| `get_points` | Query a customer's point balance   |
| `redeem`     | Deduct points for a reward         |

### Run contract tests
```bash
cd contracts/lumenperks
cargo test
```

### Deploy to Stellar testnet
```bash
soroban contract build
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/lumenperks_contract.wasm --network testnet
```

## Backend API

| Method | Path              | Description           |
|--------|-------------------|-----------------------|
| GET    | /health           | Health check          |
| GET    | /api/customers    | List customers        |
| POST   | /api/customers    | Register a customer   |

Copy `apps/backend/.env.example` to `apps/backend/.env` and fill in `CONTRACT_ID` after deploying.
