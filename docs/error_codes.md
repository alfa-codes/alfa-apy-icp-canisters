# Error Codes Documentation

The project uses a structured error code format:

```
AA-DD-CC KK NN
```

- **AA-DD-CC** — Module Code (6 digits):
  - **AA**       – Area Code (2 digit)
  - **DD**       – Domain Code (2 digit)
  - **CC**       – Component Code (2 digit) (01..49 – main, 51..99 – mock)
- **KK**       — Error Kind (2 digits)
- **NN**       — Error Number (2 digits, unique within module+kind)

## Code Structure Description

### Module Code (AA-DD-CC)

#### 01. External Services

| Code       | Domain               | Module               |
|------------|----------------------|----------------------|
| `01-01-01` | 01 – KongSwap        | 01 – Core            |
| `01-02-01` | 02 – ICPSwap         | 01 – Core            |
| `01-03-01` | 03 – ICRC Ledger     | 01 – Core            |
| `01-03-51` | 03 – ICRC Ledger     | 51 – Mock Core       |
| `01-04-01` | 04 – Canister        | 01 – Core            |


#### 02. Libraries

| Code       | Domain               | Module               |
|------------|----------------------|----------------------|
| `02-01-01` | 01 – Swap            | 01 – Swap Service    |
| `02-01-02` | 01 – Swap            | 02 – KongSwap        |
| `02-01-03` | 01 – Swap            | 03 – ICPSwap         |
| `02-02-01` | 02 – Liquidity       | 01 – Core            |
| `02-02-02` | 02 – Liquidity       | 02 – KongSwap Client |
| `02-02-03` | 02 – Liquidity       | 03 – ICPSwap Client  |
| `02-03-01` | 03 – Validation      | 01 – Core            |
| `02-04-51` | 04 – Provider        | 51 – Mock KongSwap   |
| `02-04-52` | 04 – Provider        | 52 – Mock ICPSwap    |


#### 03. Canisters

| Code       | Domain               | Module               |
|------------|----------------------|----------------------|
| `03-01-01` | 01 – Vault           | 01 – Core            |
| `03-01-02` | 01 – Vault           | 02 – Strategies      |
| `03-02-01` | 02 – PoolStats       | 01 – Core            |
| `03-02-02` | 02 – PoolStats       | 02 – PoolMetrics     |
| `03-03-01` | 03 – StrategyHistory | 01 – Core            |


### Error Kind Code (KK)
| Code   | Kind            |
|--------|-----------------|
| `01`   | NotFound        |
| `02`   | Validation      |
| `03`   | BusinessLogic   |
| `04`   | ExternalService |
| `05`   | AccessDenied    |
| `06`   | Infrastructure  |
| `07`   | Timeout         |
| `08`   | Unknown         |

### Error Number (NN)
Unique error number within the module (01, 02, ...)

---

## Example Error Code

```
02-01-03 03 01
```
- **02-01-03** — Module: Libraries – Swap – ICPSwap
- **03**       — Error Kind: BusinessLogic
- **01**       — Error Number

---

## Usage Recommendations
- Store the error code as `u64` (or `nat64` in Candid).
- Document the meaning of all code blocks for each error code.
- Do not use a string for storing the code — only for displaying it.
- Use a builder function for error codes: `build_error_code(InternalErrorKind::..., number)`

---

## Extension
When adding new modules, error kinds, or error numbers — update this documentation.

---

# Error Сode list

## 01. External Services

### 01-01. KongSwap

#### 01-01-01. External Services – KongSwap – Core

- `01-01-01 04 01` - IC error calling 'kongswap_canister_c2c_client::pools' from 'KongSwapProvider::pools' (External Service)  
- `01-01-01 03 02` - Error calling 'kongswap_canister_c2c_client::pools' from 'KongSwapProvider::pools' (Business Logic)  
- `01-01-01 04 03` - IC error calling 'kongswap_canister_c2c_client::swap_amounts' from 'KongSwapProvider::swap_amounts' (External Service)  
- `01-01-01 03 04` - Error calling 'kongswap_canister_c2c_client::swap_amounts' from 'KongSwapProvider::swap_amounts' (Business Logic)  
- `01-01-01 04 05` - Error calling 'kongswap_canister_c2c_client::swap' from 'KongSwapProvider::swap' (External Service)  
- `01-01-01 03 06` - Error calling 'kongswap_canister_c2c_client::swap' from 'KongSwapProvider::swap' (Business Logic)  
- `01-01-01 04 07` - IC error calling 'kongswap_canister_c2c_client::add_liquidity_amounts' from 'KongSwapProvider::add_liquidity_amounts' (External Service)  
- `01-01-01 03 08` - Error calling 'kongswap_canister_c2c_client::add_liquidity_amounts' from 'KongSwapProvider::add_liquidity_amounts' (Business Logic)  
- `01-01-01 04 09` - IC error calling 'kongswap_canister_c2c_client::add_liquidity' from 'KongSwapProvider::add_liquidity' (External Service)  
- `01-01-01 03 10` - Error calling 'kongswap_canister_c2c_client::add_liquidity' from 'KongSwapProvider::add_liquidity' (Business Logic)  
- `01-01-01 04 11` - IC error calling 'kongswap_canister_c2c_client::user_balances' from 'KongSwapProvider::user_balances' (External Service)  
- `01-01-01 03 12` - Error calling 'kongswap_canister_c2c_client::user_balances' from 'KongSwapProvider::user_balances' (Business Logic)  
- `01-01-01 04 13` - IC error calling 'kongswap_canister_c2c_client::remove_liquidity_amounts' from 'KongSwapProvider::remove_liquidity_amounts' (External Service)  
- `01-01-01 03 14` - Error calling 'kongswap_canister_c2c_client::remove_liquidity_amounts' from 'KongSwapProvider::remove_liquidity_amounts' (Business Logic) 
- `01-01-01 04 15` - IC error calling 'kongswap_canister_c2c_client::remove_liquidity' from 'KongSwapProvider::remove_liquidity' (External Service)  
- `01-01-01 03 16` - Error calling 'kongswap_canister_c2c_client::remove_liquidity' from 'KongSwapProvider::remove_liquidity' (Business Logic)  

### 01-02. ICPSwap

#### 01-02-01. External Services – ICPSwap – Core

- `01-02-01 04 01` - IC error calling 'icpswap_swap_factory_canister_c2c_client::getPool' from 'ICPSwapProvider::get_pool' (External Service)  
- `01-02-01 03 02` - Error calling 'icpswap_swap_factory_canister_c2c_client::getPool' from 'ICPSwapProvider::get_pool' (Business Logic)  
- `01-02-01 04 03` - IC error calling 'icpswap_swap_pool_canister_c2c_client::quote' from 'ICPSwapProvider::quote' (External Service)  
- `01-02-01 03 04` - Error calling 'icpswap_swap_pool_canister_c2c_client::quote' from 'ICPSwapProvider::quote' (Business Logic)  
- `01-02-01 04 05` - IC error calling 'icpswap_swap_pool_canister_c2c_client::swap' from 'ICPSwapProvider::swap' (External Service)  
- `01-02-01 03 06` - Error calling 'icpswap_swap_pool_canister_c2c_client::swap' from 'ICPSwapProvider::swap' (Business Logic)  
- `01-02-01 04 07` - IC error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta' from 'ICPSwapProvider::get_token_meta' (External Service)  
- `01-02-01 03 08` - Error calling 'icpswap_swap_pool_canister_c2c_client::getTokenMeta' from 'ICPSwapProvider::get_token_meta' (Business Logic)  
- `01-02-01 04 09` - IC error calling 'icpswap_swap_pool_canister_c2c_client::depositFrom' from 'ICPSwapProvider::deposit_from' (External Service)  
- `01-02-01 03 10` - Error calling 'icpswap_swap_pool_canister_c2c_client::depositFrom' from 'ICPSwapProvider::deposit_from' (Business Logic)  
- `01-02-01 04 11` - IC error calling 'icpswap_swap_pool_canister_c2c_client::withdraw' from 'ICPSwapProvider::withdraw' (External Service)  
- `01-02-01 03 12` - Error calling 'icpswap_swap_pool_canister_c2c_client::withdraw' from 'ICPSwapProvider::withdraw' (Business Logic)  
- `01-02-01 04 13` - IC error calling 'icpswap_swap_pool_canister_c2c_client::metadata' from 'ICPSwapProvider::metadata' (External Service)  
- `01-02-01 03 14` - Error calling 'icpswap_swap_pool_canister_c2c_client::metadata' from 'ICPSwapProvider::metadata' (Business Logic)  
- `01-02-01 04 15` - IC error calling 'icpswap_swap_pool_canister_c2c_client::mint' from 'ICPSwapProvider::mint' (External Service)  
- `01-02-01 03 16` - Error calling 'icpswap_swap_pool_canister_c2c_client::mint' from 'ICPSwapProvider::mint' (Business Logic)  
- `01-02-01 04 17` - IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal' from 'ICPSwapProvider::get_user_position_ids_by_principal' (External Service)  
- `01-02-01 03 18` - Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionIdsByPrincipal' from 'ICPSwapProvider::get_user_position_ids_by_principal' (Business Logic)  
- `01-02-01 04 19` - IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal' from 'ICPSwapProvider::get_user_positions_by_principal' (External Service)  
- `01-02-01 03 20` - Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPositionsByPrincipal' from 'ICPSwapProvider::get_user_positions_by_principal' (Business Logic)  
- `01-02-01 04 21` - IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance' from 'ICPSwapProvider::get_user_unused_balance' (External Service)  
- `01-02-01 03 22` - Error calling 'icpswap_swap_pool_canister_c2c_client::getUserUnusedBalance' from 'ICPSwapProvider::get_user_unused_balance' (Business Logic)  
- `01-02-01 04 23` - IC error calling 'icpswap_swap_pool_canister_c2c_client::increaseLiquidity' from 'ICPSwapProvider::increase_liquidity' (External Service)  
- `01-02-01 03 24` - Error calling 'icpswap_swap_pool_canister_c2c_client::increaseLiquidity' from 'ICPSwapProvider::increase_liquidity' (Business Logic)  
- `01-02-01 04 25` - IC error calling 'icpswap_swap_pool_canister_c2c_client::decreaseLiquidity' from 'ICPSwapProvider::decrease_liquidity' (External Service)  
- `01-02-01 03 26` - Error calling 'icpswap_swap_pool_canister_c2c_client::decreaseLiquidity' from 'ICPSwapProvider::decrease_liquidity' (Business Logic)  
- `01-02-01 04 27` - IC error calling 'icpswap_swap_pool_canister_c2c_client::getUserPosition' from 'ICPSwapProvider::get_user_position' (External Service)  
- `01-02-01 03 28` - Error calling 'icpswap_swap_pool_canister_c2c_client::getUserPosition' from 'ICPSwapProvider::get_user_position' (Business Logic)  
- `01-02-01 04 29` - IC error calling 'icpswap_swap_pool_canister_c2c_client::claim' from 'ICPSwapProvider::claim' (External Service)  
- `01-02-01 03 30` - Error calling 'icpswap_swap_pool_canister_c2c_client::claim' from 'ICPSwapProvider::claim' (Business Logic)  
- `01-02-01 04 31` - IC error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice' from 'ICPSwapProvider::get_price' (External Service)  
- `01-02-01 03 32` - Error calling 'icpswap_swap_calculator_canister_c2c_client::getPrice' from 'ICPSwapProvider::get_price' (Business Logic)  
- `01-02-01 04 33` - IC error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity' from 'ICPSwapProvider::get_token_amount_by_liquidity' (External Service)  
- `01-02-01 03 34` - Error calling 'icpswap_swap_calculator_canister_c2c_client::getTokenAmountByLiquidity' from 'ICPSwapProvider::get_token_amount_by_liquidity' (Business Logic)  
- `01-02-01 04 35` - IC error calling 'icpswap_node_index_canister_c2c_client::getAllTokens' from 'ICPSwapProvider::get_all_tokens' (External Service)  
- `01-02-01 03 36` - Error calling 'icpswap_node_index_canister_c2c_client::getAllTokens' from 'ICPSwapProvider::get_all_tokens' (Business Logic)  
- `01-02-01 04 37` - IC error calling 'icpswap_global_index_canister_c2c_client::tvlStorageCanister' from 'ICPSwapProvider::get_tvl_storage_canister' (External Service)
- `01-02-01 03 38` - Error calling 'icpswap_global_index_canister_c2c_client::tvlStorageCanister' from 'ICPSwapProvider::get_tvl_storage_canister' (Business Logic)
- `01-02-01 04 39` - IC error calling 'icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl' from 'ICPSwapProvider::get_pool_chart_tvl' (External Service)  
- `01-02-01 03 40` - Error calling 'icpswap_tvl_storage_canister_c2c_client::getPoolChartTvl' from 'ICPSwapProvider::get_pool_chart_tvl' (Business Logic)  

### 01-03. ICRC Ledger

#### 01-03-01. External Services – ICRC Ledger – Core

- `01-03-01 04 01` - IC error calling 'icrc_ledger_canister_c2c_client::icrc1_decimals' from 'icrc_ledger_client::icrc1_decimals' (External Service)
- `01-03-01 04 02` - IC error calling 'icrc_ledger_canister_c2c_client::icrc2_approve' from 'icrc_ledger_client::icrc2_approve' (External Service)
- `01-03-01 03 03` - Error calling 'icrc_ledger_canister_c2c_client::icrc2_approve' from 'icrc_ledger_client::icrc2_approve' (Business Logic)
- `01-03-01 04 04` - IC error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from' from 'icrc_ledger_client::icrc2_transfer_from' (External Service)
- `01-03-01 03 05` - Error calling 'icrc_ledger_canister_c2c_client::icrc2_transfer_from' from 'icrc_ledger_client::icrc2_transfer_from' (Business Logic)
- `01-03-01 04 06` - IC error calling 'icrc_ledger_canister_c2c_client::icrc1_fee' from 'icrc_ledger_client::icrc1_fee' (External Service)

#### 01-03-51. External Services – ICRC Ledger – Mock Core

- `01-03-51 01 01` - Mock response not set for 'decimals' in 'MockICRCLedgerClient::icrc1_decimals' (NotFound)
- `01-03-51 01 02` - Mock response not set for 'approve' in 'MockICRCLedgerClient::icrc2_approve' (NotFound)
- `01-03-51 01 03` - Mock response not set for 'transfer_from' in 'MockICRCLedgerClient::icrc2_transfer_from' (NotFound)
- `01-03-51 01 04` - Mock response not set for 'fee' in 'MockICRCLedgerClient::icrc1_fee' (NotFound)

### 01-04. Canister

#### 01-04-01. External Services – Canister – Core

- `01-04-01 04 01` - IC error calling 'canister_client::make_c2c_call' from 'Utils::icrc1_transfer_to_user' (External Service)  
- `01-04-01 03 02` - Error calling 'canister_client::make_c2c_call' from 'Utils::icrc1_transfer_to_user' (Business Logic)  

## 02. Libraries

### 02-01. Swap

#### 02-01-01. Libraries – Swap – Swap Service

- `02-01-01 03 01` - Invalid provider in 'swap_service::swap_icrc2' (BusinessLogic)
- `02-01-01 03 02` - Invalid provider in 'swap_service::quote_swap_icrc2' (BusinessLogic)

#### 02-01-02. Libraries – Swap – KongSwap

*No errors yet*

#### 02-01-03 – Libraries – Swap – ICPSwap

- `02-01-03 03 01` - Invalid token configuration for ICPSwap pool in 'ICPSwapSwapClient::is_zero_for_one_swap_direction' (Business Logic) 
- `02-01-03 03 02` - Invalid token configuration for ICPSwap pool in 'ICPSwapSwapClient::get_tokens_fee' (Business Logic)  

### 02-02. Liquidity

#### 02-02-01. Libraries – Liquidity – Core

*No errors yet*

#### 02-02-02. Libraries – Liquidity – KongSwap Client

- `02-02-02 03 01` - No user LP balance in 'KongSwapLiquidityClient::withdraw_liquidity_from_pool' (Business Logic)  
- `02-02-02 03 02` - No pool data in 'KongSwapLiquidityClient::get_position_by_id' (Business Logic)  
- `02-02-02 03 03` - No pool data in 'KongSwapLiquidityClient::get_pool_data' (Business Logic)  
- `02-02-02 03 04` - Insufficient amounts after swap/fees to add liquidity in 'KongSwapLiquidityClient::add_liquidity_to_pool' (Business Logic)  

#### 02-02-03. Libraries – Liquidity – ICPSwap Client

- `02-02-03 03 01` - Invalid token configuration for ICPSwap pool in 'ICPSwapLiquidityClient::get_tokens_fee' (Business Logic)  
- `02-02-03 03 02` - Invalid token configuration for ICPSwap pool in 'ICPSwapLiquidityClient::is_zero_for_one_swap_direction' (Business Logic)  
- `02-02-03 03 03` - Token order does not match pool metadata in 'ICPSwapLiquidityClient::add_liquidity_to_pool' (Business Logic)  
- `02-02-03 03 04` - No position ids found for user in 'ICPSwapLiquidityClient::withdraw_liquidity_from_pool' (Business Logic)  
- `02-02-03 03 05` - Token order does not match pool metadata in 'ICPSwapLiquidityClient::withdraw_liquidity_from_pool' (Business Logic)  

### 02-03. Validation

#### 02-03-01. Libraries – Validation – Core

- `02-03-01 02 01` - Field validation failed in 'FieldValidator::validate' (Validation)

### 02-04. Provider

#### 02-04-51. Libraries – Provider – Mock KongSwap

- `02-04-51 01 01` - Mock response not set for 'swap_amounts' in 'MockKongSwapProvider::swap_amounts' (NotFound) 
- `02-04-51 01 02` - Mock response not set for 'swap' in 'MockKongSwapProvider::swap' (NotFound)  
- `02-04-51 01 03` - Mock response not set for 'add_liquidity_amounts' in 'MockKongSwapProvider::add_liquidity_amounts' (NotFound)  
- `02-04-51 01 04` - Mock response not set for 'add_liquidity' in 'MockKongSwapProvider::add_liquidity' (NotFound)  
- `02-04-51 01 05` - Mock response not set for 'user_balances' in 'MockKongSwapProvider::user_balances' (NotFound)  
- `02-04-51 01 06` - Mock response not set for 'remove_liquidity_amounts' in 'MockKongSwapProvider::remove_liquidity_amounts' (NotFound)  
- `02-04-51 01 07` - Mock response not set for 'remove_liquidity' in 'MockKongSwapProvider::remove_liquidity' (NotFound)  
- `02-04-51 01 08` - Mock response not set for 'pools' in 'MockKongSwapProvider::pools' (NotFound)

#### 02-04-52. Libraries – Provider – Mock ICPSwap

- `02-04-52 01 01` - Mock response not set for 'get_all_tokens' in 'MockICPSwapProvider::get_all_tokens' (NotFound)  
- `02-04-52 01 02` - Mock response not set for 'get_tvl_storage_canister' in 'MockICPSwapProvider::get_tvl_storage_canister' (NotFound)  
- `02-04-52 01 03` - Mock response not set for 'get_pool' in 'MockICPSwapProvider::get_pool' (NotFound)  
- `02-04-52 01 04` - Mock response not set for 'quote' in 'MockICPSwapProvider::quote' (NotFound)  
- `02-04-52 01 05` - Mock response not set for 'swap' in 'MockICPSwapProvider::swap' (NotFound)  
- `02-04-52 01 06` - Mock response not set for 'get_token_meta' in 'MockICPSwapProvider::get_token_meta' (NotFound)  
- `02-04-52 01 07` - Mock response not set for 'deposit_from' in 'MockICPSwapProvider::deposit_from' (NotFound)  
- `02-04-52 01 08` - Mock response not set for 'withdraw' in 'MockICPSwapProvider::withdraw' (NotFound)  
- `02-04-52 01 09` - Mock response not set for 'metadata' in 'MockICPSwapProvider::metadata' (NotFound)  
- `02-04-52 01 10` - Mock response not set for 'mint' in 'MockICPSwapProvider::mint' (NotFound)  
- `02-04-52 01 11` - Mock response not set for 'get_user_position_ids_by_principal' in 'MockICPSwapProvider::get_user_position_ids_by_principal' (NotFound)  
- `02-04-52 01 12` - Mock response not set for 'get_user_positions_by_principal' in 'MockICPSwapProvider::get_user_positions_by_principal' (NotFound)  
- `02-04-52 01 13` - Mock response not set for 'get_user_unused_balance' in 'MockICPSwapProvider::get_user_unused_balance' (NotFound)  
- `02-04-52 01 14` - Mock response not set for 'increase_liquidity' in 'MockICPSwapProvider::increase_liquidity' (NotFound)  
- `02-04-52 01 15` - Mock response not set for 'decrease_liquidity' in 'MockICPSwapProvider::decrease_liquidity' (NotFound)  
- `02-04-52 01 16` - Mock response not set for 'get_user_position' in 'MockICPSwapProvider::get_user_position' (NotFound)  
- `02-04-52 01 17` - Mock response not set for 'claim' in 'MockICPSwapProvider::claim' (NotFound)  
- `02-04-52 01 18` - Mock response not set for 'get_price' in 'MockICPSwapProvider::get_price' (NotFound)  
- `02-04-52 01 19` - Mock response not set for 'get_token_amount_by_liquidity' in 'MockICPSwapProvider::get_token_amount_by_liquidity' (NotFound)  
- `02-04-52 01 20` - Mock response not set for 'get_pool_chart_tvl' in 'MockICPSwapProvider::get_pool_chart_tvl' (NotFound)

## 03. Canisters

### 03-01. Vault

#### 03-01-01. Canisters – Vault – Core

- `03-01-01 01 01` - Strategy not found in 'service::deposit' (NotFound) 
- `03-01-01 01 02` - Strategy not found in 'service::withdraw' (NotFound)  

#### 03-01-02. Canisters – Vault – Strategies

- `03-01-02 01 01` - No pool found to deposit in 'Strategy::deposit' (NotFound)  
- `03-01-02 01 02` - **DEPRECATED** No current pool found to deposit in 'Strategy::deposit' (NotFound)  
- `03-01-02 03 03` - No shares found for user in 'Strategy::withdraw' (BusinessLogic)  
- `03-01-02 03 04` - Not sufficient shares for user in 'Strategy::withdraw' (BusinessLogic)  
- `03-01-02 01 05` - No current pool found in strategy in 'Strategy::withdraw' (NotFound)  
- `03-01-02 01 06` - No current pool found in strategy in 'Strategy::rebalance' (NotFound)  
- `03-01-02 03 07` - Strategy has no current pool in 'strategy_stats_service::get_strategy_current_liquidity' (BusinessLogic)
- `03-01-02 03 08` - Strategy has no position id in 'strategy_stats_service::get_strategy_current_liquidity' (BusinessLogic)
- `03-01-02 03 09` - Strategy has no current pool in 'strategy_stats_service::get_strategy_current_liquidity_usd' (Business Logic)

### 03-02. PoolStats

#### 03-02-01. Canisters – PoolStats – Core

- `03-02-01 01 01` - Pool not found in 'service::delete_pool' (NotFound)  
- `03-02-01 01 02` - Pool not found in 'service::get_pool_by_id' (NotFound)  
- `03-02-01 01 03` - Pool not found in 'service::add_liquidity_to_pool' (NotFound) 
- `03-02-01 03 04` - Pool already has liquidity in 'service::add_liquidity_to_pool' (BusinessLogic) 
- `03-02-01 01 05` - Pool not found in 'service::withdraw_liquidity_from_pool' (NotFound)  
- `03-02-01 03 06` - Pool has no liquidity in 'service::withdraw_liquidity_from_pool' (BusinessLogic) 

#### 03-02-02. Canisters – PoolStats – PoolMetrics

- `03-02-02 03 01` – Pool has no position_id in 'pool_snapshot_service::create_pool_snapshot' (BusinessLogic)

### 03-03. StrategyHistory

#### 03-03-01. Canisters – StrategyHistory – Core

- `03-03-01 03 01` - Failed to fetch strategies from vault from 'strategy_history_service::initialize_strategy_states_and_create_snapshots' (BusinessLogic)
- `03-03-01 03 02` - Failed to save snapshots from 'strategy_history_service::initialize_and_snapshot_strategies' (BusinessLogic)
- `03-03-01 03 03` - from_timestamp cannot be greater than to_timestamp from 'strategy_history_service::get_strategies_history' (BusinessLogic)
- `03-03-01 03 04` - Validation failed from 'strategy_snapshot_service::save_snapshot' (BusinessLogic)
- `03-03-01 03 05` - Failed to compute minimal deposit from 'strategy_history_service::deposit_test_liquidity_to_strategy' (BusinessLogic)
- `03-03-01 03 06` - Deposit call failed from 'strategy_history_service::deposit_test_liquidity_to_strategy' (BusinessLogic)
- `03-03-01 03 07` - Quote failed from 'strategy_history_service::swap_icp_to_target_for_amount' (BusinessLogic)
- `03-03-01 03 08` - Swap failed from 'strategy_history_service::swap_icp_to_target_for_amount' (BusinessLogic)
- `03-03-01 04 09` - IC error from 'vault_service::deposit' call (ExternalService)
- `03-03-01 03 10` - Vault returned error from 'vault_service::deposit' (BusinessLogic)
- `03-03-01 03 11` - Vault strategy not found for strategy ID in 'strategy_snapshot_service::create_strategies_snapshots' (BusinessLogic)

#### 03-03-02. Strategy History – Test Snapshots

- `03-03-02 01 01` - Strategy not found in 'test_snapshots_service::create_test_snapshots' (NotFound)
- `03-03-02 01 02` - Strategy state not found in 'test_snapshots_service::create_test_snapshots' (NotFound)
- `03-03-02 03 03` - min_apy must be less than max_apy in 'test_snapshots_service::create_test_snapshots' (BusinessLogic)
- `03-03-02 03 04` - snapshot_interval_secs must be greater than 0 in 'test_snapshots_service::create_test_snapshots' (BusinessLogic)
- `03-03-02 03 05` - from_timestamp must be in the past in 'test_snapshots_service::create_test_snapshots' (BusinessLogic)
- `03-03-02 03 06` - Failed to save test snapshot at 'test_snapshots_service::create_test_snapshots' (BusinessLogic)
- `03-03-02 03 07` - Strategy is not initialized with test liquidity data in 'test_snapshots_service::create_test_snapshots' (BusinessLogic)
- `03-03-02 04 08` - Failed to get strategies from vault in 'test_snapshots_service::create_test_snapshots' (ExternalService)
