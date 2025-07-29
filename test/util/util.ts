import {Actor, HttpAgent, Identity} from "@dfinity/agent";
import {IDL} from "@dfinity/candid";
import * as Agent from "@dfinity/agent";

const localhost: string = "http://127.0.0.1:8000";

interface ActorConfig {
  canisterId: string;
  identity: Identity;
  idl: IDL.InterfaceFactory;
  useLocalEnv?: boolean;
  host?: string;
}

export async function getTypedActor<T>(config: ActorConfig): Promise<Agent.ActorSubclass<T>> {
  const { canisterId, identity, idl, useLocalEnv = false, host } = config;

  const finalHost = host || (useLocalEnv ? localhost : "https://ic0.app");

  const agent: HttpAgent = await HttpAgent.create({
    host: finalHost,
    identity: identity,
    shouldFetchRootKey: useLocalEnv
  });

  return Actor.createActor(idl, {agent, canisterId});
}
