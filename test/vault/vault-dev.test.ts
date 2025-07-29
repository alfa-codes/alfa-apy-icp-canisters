import {Ed25519KeyIdentity} from "@dfinity/identity";
import {getTypedActor} from "../util/util";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as VaultType} from "../idl/vault";
import {idlFactory} from "../idl/vault_idl";
import {Principal} from "@dfinity/principal";
import {ActorSubclass} from "@dfinity/agent";
import {AccountIdentifier} from '@dfinity/ledger-icp';
import {expect} from 'chai';
import canisterIds from '../../canister_ids.json';
import {
    CKBTC_CANISTER_ID,
    PANDA_CANISTER_ID,
    ICS_CANISTER_ID,
    ICP_CANISTER_ID
} from "../util/constants";

export const USE_LOCAL_ENV = false;

describe("Vault DEV Integration Tests", () => {
    const canisterId = canisterIds.vault.dev;
    const identity = "87654321876543218765432187654399";

    console.log("canisterId", canisterId);

    // const ledgerCanisterId = ICS_CANISTER_ID;
    const ledgerCanisterId = PANDA_CANISTER_ID;

    let principalId: Principal;
    let memberIdentity: Ed25519KeyIdentity;
    let ledgerActor: ActorSubclass<ledgerService>
    let actorVault: ActorSubclass<VaultType>

    beforeEach(async () => {
        memberIdentity = getIdentity(identity);
        // 2ammq-nltzb-zsfkk-35abp-eprrz-eawlg-f36u7-arsde-gdhv5-flu25-iqe
        principalId = memberIdentity.getPrincipal();
        // 0d445feb87a73ff4dd16e744c70aede3ab806a4d6cf9a224d439d9d82489302a
        let userAddress = await principalToAddress(principalId);

        console.log("Member principal:", principalId.toText());
        console.log("Member address:", userAddress);

        ledgerActor = await getTypedActor<ledgerService>({
            canisterId: ledgerCanisterId,
            identity: memberIdentity,
            idl: ledger_idl,
            useLocalEnv: USE_LOCAL_ENV
        });

        actorVault = await getTypedActor<VaultType>({
            canisterId: canisterId,
            identity: memberIdentity,
            idl: idlFactory,
            useLocalEnv: USE_LOCAL_ENV
        });
    });

    describe(".deposit", () => {
        const strategyId = 4; // Panda-ICP Balanced Strategy
        // const strategyId = 5; // ICS-ICP Balanced Strategy

        const approveAmount = BigInt(10000000000);
        const depositAmount = BigInt(100_000_000);
        // const depositAmount = BigInt(40_000_000);
        // const depositAmount = BigInt(10_000);

        it("Deposits to strategy without any liquidity", async () => {
            console.log("== START \"Deposits to strategy without any liquidity\" TEST ==");

            // Approve tokens
            await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

            try {
                console.log("Deposit starting...");

                const result = await actorVault.deposit({
                    amount: depositAmount,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });

                if ('Ok' in result) {
                    const depositResp = result.Ok;
                    console.log(
                        "Deposit success:",
                        depositResp.amount,
                        depositResp.shares,
                        depositResp.tx_id,
                        depositResp.position_id,
                    );

                    expect(depositResp.amount).to.equal(depositAmount);
                    expect(depositResp.shares).to.equal(depositAmount);
                } else {
                    console.error("Deposit failed:", result.Err);
                    throw new Error(`Deposit failed: ${JSON.stringify(result.Err)}`);
                }
            } catch (e) {
                console.log("Deposit error:", e);
                throw new Error("Deposit failed with error: " + e);
            }
        });

        // TODO: Implement this test
        it.skip("Deposits to strategy with liquidity", async () => {
        });
    });

    describe(".withdraw", () => {
        const strategyId = 4; // Panda-ICP Balanced Strategy
        // const strategyId = 5; // ICS-ICP Balanced Strategy
        const approveAmount = BigInt(10000000000);
        const depositAmount = BigInt(100_000_000);
        // const depositAmount = BigInt(40_000_000);
        // const depositAmount = BigInt(50_000);

        let shares: bigint;
        let sharesToWithdraw: bigint;
        let remainingShares: bigint;

        beforeEach(async () => {
            // Approve tokens
            // await checkAndApproveTokens(approveAmount, canisterId, memberIdentity, ledgerActor);

            // try {
            //     console.log("Deposit starting...");

            //     // Deposit tokens
            //     const result = await actorVault.deposit({
            //         amount: depositAmount,
            //         strategy_id: strategyId,
            //         ledger: Principal.fromText(ledgerCanisterId)
            //     });

            //     if ('Ok' in result) {
            //         const depositResp = result.Ok;
            //         console.log("Deposit success:", depositResp.amount, depositResp.shares, depositResp.tx_id, depositResp.position_id);

            //         shares = BigInt(depositResp.shares);
            //     } else {
            //         console.error("Deposit failed:", result.Err);
            //         throw new Error(`Deposit failed: ${JSON.stringify(result.Err)}`);
            //     }
            // } catch (e) {
            //     console.log("Deposit error:", e);
            // }
        });

        it("Withdraws full balance", async () => {
            console.log("== START \"Withdraws full balance\" TEST ==");

            let percentage = 100n; // For testing without deposit
            remainingShares = 0n; // No shares left

            try {
                const result = await actorVault.withdraw({
                    percentage: percentage,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });

                if ('Ok' in result) {
                    const withdrawResp = result.Ok;
                    console.log("Withdraw success:", withdrawResp.amount, withdrawResp.current_shares);

                    expect(withdrawResp.current_shares).to.equal(0n);
                } else {
                    console.error("Withdraw failed:", result.Err);
                    throw new Error(`Withdraw failed: ${JSON.stringify(result.Err)}`);
                }
            } catch (e) {
                console.log("Withdraw error: ", e);
                throw new Error("Withdraw failed with error: " + e);
            }
        });

        it("Withdraws part of balance", async () => {
            console.log("== START \"Withdraws part of balance\" TEST ==");

            shares = depositAmount; // For testing without deposit
            let percentage = 50n; // 50% withdraw
            // let sharesToWithdraw = BigInt(100_000_000);
            let remainingShares = BigInt(shares) - sharesToWithdraw;

            try {
                console.log("Withdraw starting...");

                const result = await actorVault.withdraw({
                    percentage: percentage,
                    strategy_id: strategyId,
                    ledger: Principal.fromText(ledgerCanisterId)
                });

                if ('Ok' in result) {
                    const withdrawResp = result.Ok;
                    console.log("Withdraw success:", withdrawResp.amount, withdrawResp.current_shares);

                    expect(withdrawResp.current_shares).to.equal(remainingShares);
                } else {
                    console.error("Withdraw failed:", result.Err);
                    throw new Error(`Withdraw failed: ${JSON.stringify(result.Err)}`);
                }
            } catch (e) {
                console.log("Withdraw error: ", e);
                throw new Error("Withdraw failed with error: " + e);
            }
        });
    });

    describe(".user_strategies", () => {
        it("Returns user strategies", async () => {
            try {
                const userStrategies = await actorVault.user_strategies(memberIdentity.getPrincipal());
                console.log("User strategies count:", userStrategies.length);

                if (userStrategies.length > 0) {
                    userStrategies.forEach(strategy => {
                        console.log(
                            `Strategy ID: ${strategy.strategy_id}\n` +
                            `Name: ${strategy.strategy_name}\n` +
                            `Initial deposit: ${strategy.initial_deposit.toString()}\n` +
                            `User shares: ${strategy.user_shares.toString()}\n` +
                            `Total shares: ${strategy.total_shares.toString()}\n`
                        );
                    });
                } else {
                    console.log("No strategies found for this user");
                }
            } catch (e) {
                console.log("User strategies error: ", e);
                throw new Error("User strategies failed with error: " + e);
            }
        });
    });

    describe(".rebalance", () => {
        // it("Rebalance", async function () {
        //     console.log("== START REBALANCE TEST ==");
        //
        //     try {
        //         let rebalance = await actorVault.rebalance();
        //         console.log("Rebalance success" + rebalance)
        //     } catch (e) {
        //         console.log(e)
        //     }
        // });
    });

    describe(".get_strategies", () => {
        it("Returns strategies", async () => {
            const strategies = await actorVault.get_strategies();
            const pandaIcpStrategy = strategies.find(strategy => strategy.id === 4);
            const pandaIcpPools = pandaIcpStrategy.pools;

            strategies.forEach(strategy => {
                console.log(
                    `Strategy ID: ${strategy.id}\n` +
                    `Name: ${strategy.name}\n` +
                    `Current pool: ${JSON.stringify(strategy.current_pool)}\n` +
                    `Total balance: ${strategy.total_balance}\n` +
                    `Total shares: ${strategy.total_shares}\n` +
                    `User shares: ${JSON.stringify(strategy.user_shares.toString())}\n`
                );
            });
        });
    });

    // TODO: Remove this test later
    describe(".test_reset_strategy", () => {
        it("Resets strategy", async () => {
            const strategyId = 4; // Panda-ICP Balanced Strategy
            // const strategyId = 5; // ICS-ICP Balanced Strategy

            const resetResult = await actorVault.test_reset_strategy(strategyId);
            console.log("Reset result:", resetResult);
        });
    });

    // TODO: Remove this test later
    describe.skip(".test_icpswap_withdraw", () => {
        it("Withdraws", async () => {
            const token0 = Principal.fromText(PANDA_CANISTER_ID);
            const token1 = Principal.fromText(ICP_CANISTER_ID);
            const token0Fee = 10_000n;
            const token1Fee = 10_000n;

            let withdrawResult = await actorVault.test_icpswap_withdraw(token0, 5_577_528_681n, token1Fee);
            console.log("Withdraw result:", withdrawResult);

            withdrawResult = await actorVault.test_icpswap_withdraw(token1, 353_486n, token1Fee);
            console.log("Withdraw result:", withdrawResult);
        });
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
