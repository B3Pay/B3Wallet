import { HttpAgent, Identity, getManagementCanister } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"
import { IS_LOCAL } from "helpers/config"

export { createB3SystemActor } from "./system"
export type { B3System } from "./system"
export { createB3BasicWalletActor, createB3WalletActor } from "./wallet"
export type { B3BasicWallet, B3Wallet } from "./wallet"

export function getHttpAgent(identity: Identity) {
  console.log("getHttpAgent", process.env.NEXT_PUBLIC_IC_HOST)

  return new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
    identity
  })
}

export async function createManagmentActor(identity: Identity) {
  const agent = getHttpAgent(identity)

  if (IS_LOCAL) {
    await agent.fetchRootKey()
  }

  return getManagementCanister({
    agent
  })
}

export interface CanisterStatus {
  status:
    | {
        stopped: null
      }
    | {
        stopping: null
      }
    | {
        running: null
      }
  memory_size: bigint
  cycles: bigint
  settings: {
    controllers: Array<Principal>
    freezing_threshold: bigint
    memory_allocation: bigint
    compute_allocation: bigint
  }
  module_hash: [] | [Array<number>]
}
