<div align="center" style="margin: 40px 0;">
  <img src="./static/wolf.png" alt="ALFA Wolf" width="100%" height="auto" style="border-radius: 16px; box-shadow: 0 8px 32px rgba(168, 85, 247, 0.3); max-width: 800px;"/>
</div>

# ALFA APY

## 🚀 Advanced Yield Optimization DeFi Protocol on ICP

### Automatically maximize returns through intelligent liquidity management, real-time APY tracking, and dynamic asset rebalancing across multiple DEX providers.

## 🌟 [Live Application](https://47r3x-paaaa-aaaao-qj6ha-cai.icp0.io/) | 📚 [Documentation](https://alfa-codes.github.io/alfa-apy-icp-canisters/)


## What is ALFA APY?

ALFA APY is an advanced on-chain liquidity manager built on the Internet Computer Protocol (ICP). Users invest into distinct strategy types (e.g. Conservative, Balanced, Aggressive) with automated rebalancing policies that dynamically allocate capital to the highest-yielding liquidity pools across multiple DEX providers.

### User Interface

<p align="center">
    <img src="./static/1_strategies.png" width="80%" height="auto"/>
</p>


### Key Features

- **🚀 Smart Rebalance Algorithm** - Proprietary scoring system with composite metrics (APY, TVL, volatility, execution costs)
- **💎 Multi-Strategy Support** - Conservative, Balanced, Aggressive, etc, profiles with customizable risk parameters
- **🌊 Multi-Provider Integration** - KongSwap, ICPSwap with unified abstraction layer
- **⚡ Real-Time Performance** - On-chain snapshots and realized APY calculations
- **📊 Comprehensive Analytics** - Strategy history, pool metrics, and performance tracking
- **🔒 Non-custodial & Auditable** - Full event logging and structured error handling

### Project Diagram

<p align="center">
  <img src="./static/1_project_diagram.jpg" width="80%" height="auto"/>
</p>


## Technology Stack

- **Backend**: Rust + Internet Computer Protocol (ICP)
- **Frontend**: TypeScript + React + Vite
- **Blockchain**: ICP with ICRC-1/ICRC-2 token standards
- **DEX Integration**: KongSwap, ICPSwap via provider abstraction
- **Architecture**: Modular canister design (Vault, PoolStats, StrategyHistory)

## 🏗️ Architecture Overview

### Core Canisters
- **Vault**: Strategy management, deposits/withdrawals, SmartRebalance execution
- **PoolStats**: Real-time pool metrics, APY calculations, position snapshots
- **StrategyHistory**: Longitudinal strategy analysis and performance tracking

### Smart Rebalance Engine
Advanced algorithm that evaluates pools using:
- Short-term APY smoothing (SMA)
- Pool size and capital efficiency metrics
- APY and token price volatility analysis
- Explicit execution cost calculations
- Strategy-specific safety gates and cooldowns

## 🚀 Roadmap & Open Items

### 🎯 Planned Features
- [ ] **AI factor for Smart Rebalance** - Machine learning integration for enhanced decision making
- [ ] **Index-based strategies** - Portfolio strategies based on market indices
- [ ] **Customizable strategies** - User-configured strategy parameters and risk profiles
- [ ] **Retry mechanism** - Automatic retry for failed deposit/withdrawal operations

### ✅ Completed Features
- [x] **SmartRebalance Algorithm** - Advanced scoring and rebalancing engine
- [x] **Integration tests and mocks** - Comprehensive testing infrastructure
- [x] **UI events grouping** - Enhanced user interface event management
- [x] **Multi-wallet support** - Support for all ICP wallets
- [x] **Strategy charts** - Visual performance analytics
- [x] **Strategy history** - Complete transaction and performance tracking

## Source Code

- **Backend**: [https://github.com/alfa-codes/alfa-apy-icp-canisters](https://github.com/alfa-codes/alfa-apy-icp-canisters)
- **Frontend**: [https://github.com/alfa-codes/alfa-apy-frontend](https://github.com/alfa-codes/alfa-apy-frontend)

## Community & Support

- **Website**: [https://47r3x-paaaa-aaaao-qj6ha-cai.icp0.io/](https://47r3x-paaaa-aaaao-qj6ha-cai.icp0.io/)
- **Internet Computer**: [https://internetcomputer.org](https://internetcomputer.org)
- **DFINITY Foundation**: [https://dfinity.org](https://dfinity.org)
