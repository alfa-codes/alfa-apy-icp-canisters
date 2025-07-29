import {expect} from "chai";
import {getTypedActor} from "../util/util";
import {_SERVICE as VaultType} from "../idl/vault"
import {DFX} from "../constants/dfx.const";
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {idlFactory} from "../idl/vault_idl";
import {idlFactory as ledger_idl} from "../idl/ledger_idl";
import {_SERVICE as ledgerService, ApproveArgs} from "../idl/ledger";
import {Principal} from "@dfinity/principal";
import {execute} from "../util/call.util";

export const USE_LOCAL_ENV = true;

describe("Vault Local Integration Tests", () => {
  let canister_id;

  before(async () => {
    DFX.INIT();

    const deploy = await execute(
      `dfx deploy vault --argument '(
        opt record {
          controllers = opt vec { principal "${DFX.GET_PRINCIPAL()}" }
        },
        opt record {
          environment = variant { Test }
          }
        )'`
      );

    console.log(deploy);

    canister_id = DFX.GET_CANISTER_ID("vault");
    console.log("Deployed vault canister ID:", canister_id);

    await new Promise(r => setTimeout(r, 1000));
  });

  after(() => {
      // DFX.STOP();
  });

  it("Get config", async function () {
    let actor = await getTypedActor<VaultType>({
      canisterId: canister_id,
      identity: Ed25519KeyIdentity.generate(),
      idl: idlFactory,
      useLocalEnv: USE_LOCAL_ENV
    });

    let config = await actor.get_config();

    expect(config.controllers).not.null;
  });

  it("Get runtime config", async function () {
    console.log("Getting runtime config");
    console.log("Canister ID:", canister_id);

    let actor = await getTypedActor<VaultType>({
      canisterId: canister_id,
      identity: Ed25519KeyIdentity.generate(),
      idl: idlFactory,
      useLocalEnv: USE_LOCAL_ENV
    });

    let runtimeConfig = await actor.get_runtime_config();

    expect("Test" in runtimeConfig.environment).to.be.true;
  });

  // TODO: Fix or remove this test
  it.skip("User balance", async function () {
    const icpCanisterId = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    const ownerCanisterId = "bd3sg-teaaa-aaaaa-qaaba-cai";

    // Fill balance
    let member_identity = getIdentity("87654321876543218765432187654322");
    console.log('Member identity:', member_identity.getPrincipal().toText());

    const ledgerCanisterId = await DFX.LEDGER_FILL_BALANCE(member_identity.getPrincipal().toText())
    console.log('Ledger canister ID:', ledgerCanisterId);

    let actor = await getTypedActor<ledgerService>({
      canisterId: icpCanisterId,
      identity: member_identity,
      idl: ledger_idl,
      useLocalEnv: USE_LOCAL_ENV
    });

    let balance = await actor.icrc1_balance_of({
      subaccount: [],
      owner: member_identity.getPrincipal()
    });

    console.log('Balance:', balance);

    let approveargs: ApproveArgs = {
      amount: BigInt(200000000),
      spender: {
          owner: Principal.fromText(ownerCanisterId),
          subaccount: []
      },
      fee: [],
      memo: [],
      from_subaccount: [],
      created_at_time: [],
      expected_allowance: [],
      expires_at: []
    }

    let icrc2approve = await actor.icrc2_approve(approveargs);
    console.log('ICRC2 approve:', icrc2approve);

    let allowance = await actor.icrc2_allowance({
      account: {
        owner: member_identity.getPrincipal(),
        subaccount: []
      },
      spender: {
        owner: Principal.fromText(ownerCanisterId),
        subaccount: []
      }
    });

    let actorVault = await getTypedActor<VaultType>({
      canisterId: canister_id,
      identity: member_identity,
      idl: idlFactory,
      useLocalEnv: USE_LOCAL_ENV
    });

    let accept = await actorVault.deposit({
      ledger: Principal.fromText(icpCanisterId), amount: BigInt(100000000),
      strategy_id: 1
    });

    let balance2 = await actor.icrc1_balance_of({
      subaccount: [],
      owner: Principal.fromText(ownerCanisterId)
    });

    console.log('Balance2:', balance2);

    let withdraw = await actorVault.withdraw({
      ledger: Principal.fromText(icpCanisterId), percentage: BigInt(100), strategy_id: 1
    });

    console.log('Withdraw:', withdraw);

    let balance3 = await actor.icrc1_balance_of({
      subaccount: [],
      owner: Principal.fromText(ownerCanisterId)
    });

    console.log('Balance3:', balance3);

    expect(balance3 < balance2).is.true;
  });
})

export const getIdentity = (seed: string): Ed25519KeyIdentity => {
  let seedEncoded = new TextEncoder().encode(seed);

  return Ed25519KeyIdentity.generate(seedEncoded);
};
