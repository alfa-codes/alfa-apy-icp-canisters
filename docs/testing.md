# Testing

## Overview

The AlfaAPY project uses a comprehensive testing approach that includes unit tests and integration tests to ensure system reliability and correctness.

## Test Types

### Unit Tests
- **Purpose**: Testing individual components and functions in isolation
- **Location**: Located in `src/**/*.rs` files alongside the code being tested
- **Execution**: Run separately for each canister

### Integration Tests
- **Purpose**: Testing interactions between different system components
- **Location**: `test/` folder with separate directories for each canister
- **Examples**: 
  - Vault canister testing (`test/vault/`)
  - Pool stats testing (`test/pool_stats/`)
  - Strategy testing (`test/integration/strategy.ts`)

## Mock Infrastructure

### Service Resolver
The system uses `service_resolver` to switch between real and mock implementations of external services depending on environment variables:

- **Real Services**: Used in production and staging environments
- **Mock Services**: Used in test environment for test isolation

### Supported Providers
- **ICP Swap**: Mocks for all external ICP Swap canisters
- **Kong Swap**: Mocks for Kong Swap API
- **ICRC Ledger**: Mocks for token operations

### Mock File Examples
```
src/libraries/providers/mock/
├── icpswap.rs
├── kongswap.rs
└── mod.rs
```

## Running Tests

Tests can be executed using different commands depending on the type:

### Integration Tests
```bash
npm run test
```

This command runs only the integration tests (TypeScript tests using dfx).

### Unit Tests
Unit tests for each canister can be run separately:

1. **VAULT canister tests**
   ```bash
   cargo test -p vault --lib
   ```

2. **POOL_STATS canister tests**
   ```bash
   cargo test -p pool_stats --lib
   ```

#### Running All Unit Tests
You can also run all unit tests at once using the bash script:

```bash
./scripts/run_tests.sh
```

This script runs both vault and pool_stats unit tests sequentially with quiet output and exits with failure if any test fails.

## Test Structure

### Canister Tests
- **vault**: Testing core vault logic, strategies, liquidity management
- **pool_stats**: Testing pool statistics, metrics, and calculations

### Integration Tests
- **TypeScript tests**: Use dfx to deploy canisters in local network
- **Constants**: `test/constants/dfx.const.ts` - test environment settings
- **Structure**: Each canister has its own test directory (`test/vault/`, `test/pool_stats/`)

## Environment Variables for Tests

Tests automatically use mock infrastructure through `service_resolver`, which determines the environment and connects appropriate service implementations.
