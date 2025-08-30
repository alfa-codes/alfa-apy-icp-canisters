import {Ed25519KeyIdentity} from "@dfinity/identity";
import {getTypedActor} from "../util/util";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as PoolStatsType} from "../idl/pool_stats";
import {idlFactory} from "../idl/pool_stats_idl";
import {Principal} from "@dfinity/principal";
import {ActorSubclass} from "@dfinity/agent";
import {AccountIdentifier} from '@dfinity/ledger-icp';
import {expect} from 'chai';
import canisterIds from '../../canister_ids.json';
import { POOLS, TOKENS, PROVIDERS } from '../data/pools';

// Helper function to convert string provider to ExchangeId
function getExchangeId(provider: string): any {
  switch (provider) {
    case PROVIDERS.KongSwap:
      return { KongSwap: null };
    case PROVIDERS.ICPSwap:
      return { ICPSwap: null };
    default:
      throw new Error(`Unknown provider: ${provider}`);
  }
}

// Helper function to get token name by address
function getTokenNameByAddress(address: string): string {
  for (const [name, tokenAddress] of Object.entries(TOKENS)) {
    if (tokenAddress === address) {
      return name;
    }
  }
  return "Unknown";
}

export const USE_LOCAL_ENV = false;

describe("Pool Stats DEV Integration Tests", () => {
  const canisterId = canisterIds.pool_stats.production;
  const identity = "87654321876543218765432187654399";
  const pandaCanisterId = "druyg-tyaaa-aaaaq-aactq-cai";
  const ledgerCanisterId = pandaCanisterId;

  let principalId: Principal;
  let memberIdentity: Ed25519KeyIdentity;
  let actorPoolStats: ActorSubclass<PoolStatsType>

  beforeEach(async () => {
      memberIdentity = getIdentity(identity);
      // 2ammq-nltzb-zsfkk-35abp-eprrz-eawlg-f36u7-arsde-gdhv5-flu25-iqe
      principalId = memberIdentity.getPrincipal();
      // 0d445feb87a73ff4dd16e744c70aede3ab806a4d6cf9a224d439d9d82489302a
      let userAddress = await principalToAddress(principalId);

      console.log("Member principal:", principalId.toText());
      console.log("Member address:", userAddress);

      actorPoolStats = await getTypedActor<PoolStatsType>({
          canisterId: canisterId,
          identity: memberIdentity,
          idl: idlFactory,
          useLocalEnv: USE_LOCAL_ENV
      });
  });

  it("Create pools", async () => {
    console.log("Create pools starting...");

    for (const pool of POOLS) {
      console.log("Pool:", pool);

      const token0Principal = Principal.fromText(pool.token0);
      const token1Principal = Principal.fromText(pool.token1);
      
      await actorPoolStats.add_pool(token0Principal, token1Principal, getExchangeId(pool.provider));

      console.log("Pool added:", pool.id);
    }
  });

  it("Deposit test liquidity to pools", async () => {
    console.log("Deposit test liquidity to pools starting...");

    let result = await actorPoolStats.get_pools();

    if ('Ok' in result) {
      const pools = result.Ok;

      for(let i = 0; i < pools.length; i++) {
        const pool = pools[i];
        const token0Name = getTokenNameByAddress(pool.token0.toText());
        const token1Name = getTokenNameByAddress(pool.token1.toText());

        if (
          pool.id != `${PROVIDERS.KongSwap}_${TOKENS.GLDT}_${TOKENS.CKUSDT}`
        ) {
          continue;
        }

        console.log("\n");
        console.log(`${i + 1}/${pools.length} - Pool id: ${pool.id}`);
        console.log("Pool token0:", token0Name, "(", pool.token0.toText(), ")");
        console.log("Pool token1:", token1Name, "(", pool.token1.toText(), ")");
        console.log("Pool provider:", pool.provider);

        let result = await actorPoolStats.deposit_test_liquidity_to_pool(pool.id);

        if ('Ok' in result) {
          const addLiquidityResp = result.Ok;
          console.log(
            "Add liquidity success:",
            addLiquidityResp.token_0_amount,
            addLiquidityResp.token_1_amount,
            addLiquidityResp.position_id,
            pool.id
          );

          expect(Number(addLiquidityResp.token_0_amount)).to.be.greaterThan(0);
          expect(Number(addLiquidityResp.token_1_amount)).to.be.greaterThan(0);
          expect(Number(addLiquidityResp.position_id)).to.be.greaterThan(0);
        } else {
          console.error("Add liquidity failed - Full error details:", result.Err);
          console.error("Error code:", result.Err.code);
          console.error("Error kind:", result.Err.kind);
          console.error("Error message:", result.Err.message);
          if (result.Err.details) {
            console.error("Error details:", JSON.stringify(result.Err.details, null, 2));
          }
          throw new Error(`Add liquidity failed: ${result.Err.message || 'Unknown error'}`);
        }
      }
    } else {
      console.error("Get pools failed - Full error details:", result.Err);
      console.error("Error code:", result.Err.code);
      console.error("Error kind:", result.Err.kind);
      console.error("Error message:", result.Err.message);
      if (result.Err.details) {
        console.error("Error details:", JSON.stringify(result.Err.details, null, 2));
      }
      throw new Error(`Get pools failed: ${result.Err.message || 'Unknown error'}`);
    }
  });
});

export const getIdentity = (seed: string): Ed25519KeyIdentity => {
  let seedEncoded = new TextEncoder().encode(seed);

  return Ed25519KeyIdentity.generate(seedEncoded);
};

export const checkAndApproveTokens = async (
  amount: bigint,
  canisterId: string,
  memberIdentity: Ed25519KeyIdentity,
  ledgerActor: ActorSubclass<ledgerService>
) => {
  let approveArgs: ApproveArgs = {
      amount: amount,
      spender: {
          owner: Principal.fromText(canisterId),
          subaccount: []
      },
      fee: [],
      memo: [],
      from_subaccount: [],
      created_at_time: [],
      expected_allowance: [],
      expires_at: []
  };

  console.log("Approve tokens starting...");

  // Approve tokens
  const approveResponse = await ledgerActor.icrc2_approve(approveArgs);
  console.log("IRC2 approve:", approveResponse);

  // Check allowance
  const allowanceResponse = await ledgerActor.icrc2_allowance({
      account: {
          owner: memberIdentity.getPrincipal(),
          subaccount: []
      },
      spender: {
          owner: Principal.fromText(canisterId),
          subaccount: []
      }
  });

  console.log("Allowance:", allowanceResponse);

  if (allowanceResponse.allowance < amount) {
      throw new Error("Insufficient allowance");
  }
}

export const principalToAddress = async (principalId: Principal): Promise<string> => {
  const accountIdentifier = AccountIdentifier.fromPrincipal({
      principal: principalId,
      subAccount: undefined
  });

  return accountIdentifier.toHex();
}
