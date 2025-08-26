# AlfaAPY – Main Documentation

## 1. Project Overview

### 1.1 What is AlfaAPY?
`AlfaAPY` is an on‑chain liquidity manager on the Internet Computer (ICP). Users invest into distinct strategy types (e.g., Conservative, Balanced, Aggressive, etc.); each type defines its own set of eligible pools and a rebalancing policy (weights, thresholds, cooldowns), so capital is routed differently depending on the chosen strategy. It allocates capital into DEX pools and periodically rebalances to sustain healthy, long‑term yield. Decisions are driven by measured on‑chain performance (real positions and snapshots), not by promotional or protocol‑reported numbers.

### 1.2 How it works
- A user deposits into a strategy in the `Vault` canister.
- The strategy opens/maintains an LP position in a selected DEX pool.
- The `PoolStats` canister captures periodic snapshots of pools and positions and derives APY/TVL.
- `SmartRebalance` (in `Vault`) computes a composite score per pool and applies safety gates (cooldown, score delta, expected gain vs cost). If gates pass, `Vault` executes the rebalance.
- `StrategyHistory` keeps longitudinal snapshots of each strategy for analysis and transparency.

<p align="center">
  <img src="./images/main_doc/1_project_diagram.jpg" width="80%" height="auto"/>
</p>

### 1.3 Why it’s different
- Real Data‑first: we continuously collect and store real pool and strategy data (snapshots of positions, TVL, derived APY). This gives an auditable historical dataset and realized APY (not fabricated or purely protocol‑reported).
- `SmartRebalance`: a proprietary scoring algorithm that leverages those real datasets and multiple factors (SMA APY in USD/tokens, log(TVL), capital efficiency, APY volatility, token price volatility) with explicit execution costs; the chosen strategy type adjusts weights and gates (cooldowns, score thresholds, gain‑vs‑cost), so rebalancing aligns with the user’s risk/return profile while avoiding churn and noise‑driven moves.
- Modular and auditable: clear separation of concerns across canisters; event logs and structured error codes make behavior inspectable.

### 1.4 Core components
- `Vault`: strategies, deposits/withdrawals, execution of rebalances, event logging.
- `PoolStats`: pool/position snapshots, pool metrics (APY, TVL) and aggregation.
- `StrategyHistory`: strategy‑level snapshots and history.
- DEX/ledger integrations: `KongSwap`, `ICPSwap`, `ICRC` ledgers.

### 1.5 Key features
- `SmartRebalance` with a composite score (details: `docs/smart_rebalance.md`)
- Strategy profiles (Conservative, Balanced, Aggressive, etc.) with per‑profile weights, cooldowns, thresholds.
- Accurate USD valuation via on‑chain quotes into a stable token at execution time.
- UI: strategy charts for APY and TVL.
- Event log and structured error codes (`docs/error_codes.md`).


## 2. Architecture

### 2.1 Canisters overview
#### 2.1.1 Vault
  - Consist of different liquidity pools from different DEXs.
  - Owns strategies and user flows (deposit/withdraw, rebalance execution).
  - Hosts `SmartRebalance` decision and applies safety gates before moving funds.
  - Computes and mints/burns strategy shares on deposit/withdraw with a transparent share formula; tracks total/user shares and current liquidity (see `docs/liquidity_pools_calculation_flow.md`).
  - Talks to DEX/ledger canisters for execution (via abstraction layers), logs events, persists strategy state.

#### 2.1.2 PoolStats
  - Periodically snapshots pools and positions, aggregates metrics (APY, TVL, Volume).
  - Provides read APIs used by `Vault` to fetch pool metrics and (optionally) historical series.

#### 2.1.3 StrategyHistory
  - Stores strategy‑level snapshots and long‑horizon stats (e.g., long‑term APY filters).
  - Useful for analytics/visuals and to inform future allocation heuristics.

#### 2.1.4 External canisters
  - DEX providers (`KongSwap`, `ICPSwap`) and `ICRC` ledgers are consumed via c2c clients.
  - Integration is encapsulated in libraries to keep `Vault` logic clean.


### 2.2 Libraries and shared crates
- `smart_rebalance`: scoring, metrics, decision engine, strategy profiles (weights/thresholds/cooldowns). See `docs/smart_rebalance.md`.
- `liquidity`, `providers`, `swap`: execution clients/routers for DEX and token operations.
- `types`, `utils`, `errors`, `event_records`, `service_resolver`: shared models, helpers, error codes, events, and resolution of external services.
- `yield_calculator`: APY utilities used by `PoolStats` when deriving pool yields.

### 2.3 Cross‑canister interactions
- `Vault` → `PoolStats`: fetch pool metrics (APY/TVL) and, where needed, historical series.
- `Vault` → DEX/ledgers: add/withdraw liquidity, swaps, transfers; all guarded by pre‑checks and correlated via request context IDs.
- `Vault` ↔ `StrategyHistory`: write/read strategy snapshots for longitudinal analysis and filters.


## 3. User Interface
Main doc: `docs/user_interface.md`



## 4. Smart Rebalance Algorithm

`SmartRebalance` evaluates every eligible pool with a composite score and moves liquidity only when it is economically justified. The score blends short‑term smoothed APY in USD and tokens (SMA), pool size (log(TVL)), capital efficiency (volume/TVL), APY volatility, token‑price volatility, and explicit execution costs (DEX fees + gas). Decisions are gated by cooldowns, minimal score delta, and an expected‑gain‑versus‑cost check over the intended holding period. Strategy profiles (Conservative, Balanced, Aggressive, etc.) tune the weights and gates so the behavior matches a user’s risk/return preference. For the full specification and formulas, see: `docs/smart_rebalance.md`.





## 5. Strategies in Vault

Strategies are the user‑facing abstraction in `Vault`: a user chooses a strategy type and deposits funds; in return they receive shares that represent a proportional claim on the strategy’s assets. The strategy keeps exactly one active LP position at a time in one of its eligible pools and may migrate liquidity via `SmartRebalance` when gates are satisfied.

### 5.1 Lifecycle
- Create: strategies are defined with a set of eligible pools and a profile (weights, thresholds, cooldowns).
- Deposit: `Vault` mints shares using a transparent share formula based on current strategy NAV; funds are allocated into the current pool.
- Rebalance: decision is evaluated periodically/as scheduled; if cooldown, score delta, and expected gain vs cost check pass, liquidity is withdrawn, normalized to base token, and re‑added to the target pool.
- Withdraw: `Vault` burns user shares and returns proceeds; balances are unwound from the LP and settled back to the user.

### 5.2 State and tracking
- Strategy state includes current pool, total shares, per‑user shares, current liquidity (base token), and timestamps for last updates.
- Liquidity and position data are refreshed in the background; `PoolStats` and `StrategyHistory` provide snapshots for realized APY and long‑term analysis.

### 5.3 Events and audit trail
Every critical action emits an event (started/completed/failed) with a correlation ID: deposit, withdraw, rebalance. This stream powers the UI (activity timelines) and enables operational forensics and audits.




## 6. Liquidity Management

Vault enacts deposits and withdrawals through a lightweight router and provider clients (`KongSwap`, `ICPSwap`). We never embed DEX specifics into strategies: execution details live behind a stable interface, so strategy logic stays simple and auditable.

On deposit, the router prepares funds (normalizes to the base token if needed), derives LP amounts with our share formula, and adds liquidity to the current pool. On withdraw, the router decreases the LP position proportionally, realizes fees, converts proceeds back to the base token when appropriate, and settles them to the user. Along the way we apply basic safeguards (slippage/min‑amount checks, provider availability) and tag every operation with a correlation ID for traceability.

See also:
- Liquidity pools calculation flow: `docs/liquidity_pools_calculation_flow.md`
- `KongSwap` provider flow: `docs/kong_swap_provider_flow.md`
- `ICPSwap` doc: `docs/icp_swap.md`




## 7. Error Handling

We use structured, non‑panicking errors across all canisters: each failure returns a typed error with a stable code, concise message, source, and metadata, and is mirrored to the event log with the same correlation ID; both error and event are persisted in stable state and can be joined for end‑to‑end traceability and analytics. Full format, categories, and the live catalog are in docs/error_codes.md; implementation details are covered in section 11 (Observability & Operations) and the `event_records` library.

Full format, categories, and the live catalog: `docs/error_codes.md`

## 8. Events

AlfaAPY uses a comprehensive event system to track all critical operations. Each event follows a "Started → Completed/Failed" pattern and includes metadata for auditing and debugging.

### 8.1 Event Structure

Every event contains:
- **`id`**: Unique event identifier
- **`timestamp`**: Event timestamp in seconds
- **`event`**: Specific event data
- **`correlation_id`**: Links related events in an operation
- **`user`**: User principal (optional)
- **`strategy_id`**: Associated strategy ID (optional)

### 8.2 Event Types

**Strategy Events:**
- `StrategyDepositStarted/Completed/Failed`: Strategy deposits
- `StrategyWithdrawStarted/Completed/Failed`: Strategy withdrawals  
- `StrategyRebalanceStarted/Completed/Failed`: Strategy rebalancing

**Pool Events:**
- `AddLiquidityToPoolStarted/Completed/Failed`: Adding liquidity to pools
- `WithdrawLiquidityFromPoolStarted/Completed/Failed`: Removing liquidity from pools

**Swap Events:**
- `SwapTokenStarted/Completed/Failed`: Token swaps between pools

**Event Flow Pattern:**
All events follow the pattern: `Started` → `Completed` or `Failed`, enabling operation tracking and error handling.

### 8.3 Use Cases

Events enable:
- **Monitoring**: Track operation progress and performance
- **Debugging**: Trace user issues and errors
- **Audit**: Complete operation history for compliance
- **Analytics**: User activity and strategy performance metrics

## 9. Testing & QA

### 9.1 Unit tests
Pure functions (APY/yield calc, metrics, scoring, decision gates) have deterministic tests with edge‑case coverage.

### 9.2 Integration and canister‑level tests
Local replica tests cover strategy flows: deposit → LP add → rebalance decision → migrate → withdraw; providers are mocked with controllable slippage/latency.

### 9.3 Environment‑specific mocking
In `Environment::Test`, all provider/DEX and ledger calls are routed to mocks via the `service_resolver`: `providers::mock::{kongswap, icpswap}` and `icrc_ledger_client::mock::MockICRCLedgerClient`. In `Environment::Production`, default implementations are used (`DefaultKongSwapProvider`, `DefaultICPSwapProvider`, `DefaultICRCLedgerClient`). See `service_resolver::ServiceResolver`. 
