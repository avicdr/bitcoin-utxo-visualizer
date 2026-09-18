# Bitcoin UTXO Visualizer

A technically serious, high-performance developer tool and protocol explorer for analyzing Bitcoin's **Unspent Transaction Output (UTXO) model**, transaction dependencies, value flows, and script types using **Rust**, **PostgreSQL**, **Next.js 14**, and **Bitcoin Core**.

---

## Architecture & Core Philosophy

1. **Strict Bipartite DAG Topology**:
   - Bitcoin transactions do not consume other transactions; they consume specific previous outputs (`Outpoints` = `TXID:VOUT`).
   - The graph structure is strictly alternating: $\text{Transaction} \to \text{UTXO} \to \text{Transaction}$.
2. **Value Conservation & Fee Destruction**:
   - Transactions do not generate value; they redistribute satoshis.
   - Fees are implicit and non-conserved: $\text{Fee} = \sum V_{\text{in}} - \sum V_{\text{out}}$.
3. **Consensus Facts vs. Heuristics**:
   - Explicit separation between on-chain cryptographic facts (scripts, locktimes, outpoints, confirmations) and speculative clustering heuristics (change detection, address reuse).
4. **Zero Fabricated Data**:
   - All transactions, blocks, and states are either indexed directly from a running Bitcoin Core node or handled with clear 0-conf mempool indicators.

---

## Technology Stack

- **Backend**: Rust 1.80+ (Axum 0.7, Tokio 1.38, SQLx 0.8 with async PostgreSQL, rust-bitcoin 0.32, Tower HTTP, Criterion 0.5)
- **Database**: PostgreSQL 16+ with structured schema, JSONB script metadata, and relational outpoint indexing
- **Frontend**: Next.js 14 (App Router), TypeScript, Tailwind CSS, `@xyflow/react` (React Flow DAG rendering), Recharts (Distributions and analytics), Lucide Icons
- **Bitcoin Node**: Bitcoin Core 27.0+ (Regtest / Testnet / Mainnet) with RPC and ZMQ support

---

## Project Structure

```
├── backend/                  # Rust Axum Backend
│   ├── benches/              # Criterion performance benchmarks
│   ├── src/
│   │   ├── analytics/        # UTXO age bands, value buckets, script stats
│   │   ├── api/              # Axum REST & SSE event endpoints
│   │   ├── blocks/           # Block indexing & persistence
│   │   ├── events/           # Real-time SSE broadcaster
│   │   ├── graph/            # UTXO DAG builder & cluster collapsing
│   │   ├── mempool/          # 0-conf UTXO mempool tracker
│   │   ├── rpc/              # Typed Bitcoin Core RPC client
│   │   ├── scripts/          # Script analyzer & opcode disassembler
│   │   ├── transactions/     # Transaction parsing & fee calculation
│   │   └── utxos/            # Outpoint tracking & lifecycle queries
│   └── tests/                # 29 integration test suites
├── frontend/                 # Next.js 14 React Application
│   ├── app/                  # Next.js App Router (Dashboard)
│   ├── features/
│   │   ├── analytics/        # Recharts distributions & KPI cards
│   │   ├── explorer/         # Transaction detail inspector
│   │   ├── graph/            # Interactive DAG visualizer (React Flow)
│   │   ├── utxos/            # 3-stage UTXO lifecycle timeline
│   │   └── value-flow/       # Satoshi distribution & fee ribbon
│   └── lib/api.ts            # Typed client API layer
├── scripts/                  # Deterministic regtest scenario generators
│   ├── run_all_scenarios.sh  # Bash runner
│   └── run_all_scenarios.ps1 # PowerShell runner
├── docker-compose.yml        # Bitcoind Regtest & PostgreSQL 16 services
└── .env.example              # Configuration environment template
```

---

## Quickstart Guide

### 1. Prerequisites
- Docker & Docker Compose
- Rust 1.80+ (`cargo`)
- Node.js 18+ (`npm`)

### 2. Start Infrastructure
Launch Bitcoin Core in regtest mode and PostgreSQL:
```bash
docker compose up -d
```

### 3. Start Backend Server
```bash
cd backend
cargo run
```
The backend API initializes on `http://localhost:8080`.

### 4. Start Frontend
```bash
cd frontend
npm install
npm run dev
```
Open `http://localhost:3000` in your browser.

### 5. Generate Deterministic Regtest Scenarios
Run the scenario script to populate the regtest blockchain with real UTXO patterns:
```bash
# On Linux / macOS / WSL:
./scripts/run_all_scenarios.sh

# On Windows (PowerShell):
./scripts/run_all_scenarios.ps1
```
Scenarios generated:
1. **Coinbase Spend**: Mining 101 blocks to mature coinbase reward and spending it.
2. **Multi-Input Consolidation**: Consolidating 5 inputs into a single Taproot outpoint.
3. **Fan-Out Transaction**: Splitting 1 UTXO into 5 distinct outputs.
4. **Transaction Chain**: $TX_A \to TX_B \to TX_C$ confirmed across blocks.
5. **Unconfirmed Mempool Chain**: 0-conf child transactions in the mempool.
6. **Replace-By-Fee (RBF)**: Fee bumping an unconfirmed transaction.
7. **Child-Pays-For-Parent (CPFP)**: Mining incentive package with high-fee child.

---

## API Endpoints Reference

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/health` | Service health & database connectivity |
| `GET` | `/api/node` | Bitcoin Core node network, tip, and sync state |
| `GET` | `/api/blocks` | List indexed blocks with transaction summaries |
| `GET` | `/api/blocks/:height` | Block details with fee stats and transactions |
| `GET` | `/api/transactions/:txid` | Transaction details, inputs, outputs, vsize, and fee rate |
| `GET` | `/api/transactions/:txid/graph` | Bipartite DAG graph response (nodes and edges) |
| `GET` | `/api/transactions/graph/expand` | Lazy expansion query for ancestors/descendants |
| `GET` | `/api/utxos` | Paginated active UTXO set |
| `GET` | `/api/utxos/:txid/:vout` | Single UTXO outpoint details and 3-stage lifecycle |
| `GET` | `/api/utxos/:txid/:vout/spend` | Spending transaction details for spent output |
| `GET` | `/api/scripts/decode?hex=...` | Bitcoin script disassembler and classifier |
| `GET` | `/api/addresses/:address` | Balance aggregation and heuristic warning |
| `GET` | `/api/mempool` | Unconfirmed 0-conf transaction tracker |
| `GET` | `/api/events` | Server-Sent Events (SSE) stream for real-time blocks/txs |
| `GET` | `/api/analytics/utxos` | Global UTXO set count, satoshis, and coin days |
| `GET` | `/api/analytics/value-distribution` | Satoshi distribution buckets |
| `GET` | `/api/analytics/age-distribution` | UTXO lifespan and HODL bands |
| `GET` | `/api/analytics/scripts` | Script type percentage distribution |

---

## Testing & Verification

Run the full Rust test suite (29 tests):
```bash
cd backend
cargo test --all
```

Run Criterion performance benchmarks:
```bash
cd backend
cargo bench --no-run
```

Run Frontend validation:
```bash
cd frontend
npm run typecheck
npm run lint
npm run build
```

---

## Keyboard Shortcuts

- `/` : Focus main search bar
- `1` : Switch to UTXO Graph (DAG) View
- `2` : Switch to Transaction Explorer
- `3` : Switch to Value Flow & Fee Ribbon
- `4` : Switch to UTXO Set & Lifecycle Timeline
- `5` : Switch to UTXO Analytics & Distributions
- `Esc` : Clear selection / Close modals

---

## License

MIT License. Designed and engineered for serious Bitcoin systems engineering and education.
