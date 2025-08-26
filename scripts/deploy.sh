#!/bin/bash

# Script for deploying canisters to the specified network
# Usage: ./scripts/deploy.sh --canister all|vault|pool_stats|strategy_history --env staging|dev|production

set -e  # Stop execution on error

# Function to show help
show_help() {
    echo "Usage: $0 --canister <canister> --env <environment>"
    echo ""
    echo "Available canisters:"
    echo "  all              - Deploy all canisters (vault, pool_stats, strategy_history)"
    echo "  vault            - Deploy only vault canister"
    echo "  pool_stats       - Deploy only pool_stats canister"
    echo "  strategy_history - Deploy only strategy_history canister"
    echo ""
    echo "Available environments:"
    echo "  dev        - Dev network (environment: Dev)"
    echo "  staging    - Staging network (environment: Staging)"
    echo "  production - Production network (environment: Production)"
    echo ""
    echo "Examples:"
    echo "  $0 --canister all --env staging"
    echo "  $0 --canister vault --env dev"
    echo "  $0 --canister pool_stats --env production"
}

# Parse arguments
ENV=""
CANISTER=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --env)
            ENV="$2"
            shift 2
            ;;
        --canister)
            CANISTER="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "ERROR: Unknown parameter: $1"
            show_help
            exit 1
            ;;
    esac
done

# Check if required parameters are provided
if [ -z "$CANISTER" ]; then
    echo "ERROR: --canister parameter is required"
    show_help
    exit 1
fi

if [ -z "$ENV" ]; then
    echo "ERROR: --env parameter is required"
    show_help
    exit 1
fi

# Set network and environment based on parameter
case $ENV in
    staging)
        NETWORK="staging"
        ENVIRONMENT="Staging"
        ;;
    dev)
        NETWORK="dev"
        ENVIRONMENT="Dev"
        ;;
    production)
        NETWORK="production"
        ENVIRONMENT="Production"
        ;;
    *)
        echo "ERROR: Unsupported environment: $ENV"
        show_help
        exit 1
        ;;
esac

# Validate canister parameter
case $CANISTER in
    all|vault|pool_stats|strategy_history)
        # Valid canister
        ;;
    *)
        echo "ERROR: Unsupported canister: $CANISTER"
        show_help
        exit 1
        ;;
esac

echo "Starting deployment to network: $NETWORK (environment: $ENVIRONMENT)..."

# Check if dfx is available
if ! command -v dfx &> /dev/null; then
    echo "ERROR: dfx not found. Install dfx and try again."
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "dfx.json" ]; then
    echo "ERROR: dfx.json not found. Run the script from the project root directory."
    exit 1
fi

# Function to deploy vault
deploy_vault() {
    echo "Deploying vault..."
    dfx deploy vault --network $NETWORK --argument "(
      opt record { controllers = null; },
      opt record { environment = variant { $ENVIRONMENT } }
    )"
    echo ""
}

# Function to deploy pool_stats
deploy_pool_stats() {
    echo "Deploying pool_stats..."
    dfx deploy pool_stats --network $NETWORK --argument "(
      opt record { environment = variant { $ENVIRONMENT } }
    )"
    echo ""
}

# Function to deploy strategy_history
deploy_strategy_history() {
    echo "Deploying strategy_history..."
    dfx deploy strategy_history --network $NETWORK --argument "(
      opt record { environment = variant { $ENVIRONMENT } }
    )"
    echo ""
}

# Deploy based on canister parameter
case $CANISTER in
    all)
        deploy_vault
        deploy_pool_stats
        deploy_strategy_history
        echo "All canisters successfully deployed to network $NETWORK!"
        ;;
    vault)
        deploy_vault
        echo "Vault canister successfully deployed to network $NETWORK!"
        ;;
    pool_stats)
        deploy_pool_stats
        echo "Pool_stats canister successfully deployed to network $NETWORK!"
        ;;
    strategy_history)
        deploy_strategy_history
        echo "Strategy_history canister successfully deployed to network $NETWORK!"
        ;;
esac

echo ""
echo "Information about deployed canister(s):"
echo ""

# Show status based on what was deployed
case $CANISTER in
    all)
        dfx canister --network $NETWORK status vault
        echo ""
        dfx canister --network $NETWORK status pool_stats
        echo ""
        dfx canister --network $NETWORK status strategy_history
        ;;
    vault)
        dfx canister --network $NETWORK status vault
        ;;
    pool_stats)
        dfx canister --network $NETWORK status pool_stats
        ;;
    strategy_history)
        dfx canister --network $NETWORK status strategy_history
        ;;
esac
