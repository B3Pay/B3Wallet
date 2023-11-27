import { Identity } from "@dfinity/agent"
import { b3_wallet, createActor } from "declarations/b3_wallet"
import { getHttpAgent } from "service"
import {
  b3_basic_wallet,
  createActor as createBasicWalletActor
} from "../../declarations/b3_basic_wallet"

export function createB3WalletActor(canisterId: string, identity: Identity) {
  const agent = getHttpAgent(identity)

  console.log("makeB3WalletActor", canisterId, agent)
  return createActor(canisterId, {
    agent
  })
}

export type B3Wallet = typeof b3_wallet

export function createB3BasicWalletActor(
  canisterId: string,
  identity: Identity
) {
  const agent = getHttpAgent(identity)

  console.log("makeB3WalletActor", canisterId, agent)
  return createBasicWalletActor(canisterId, {
    agent
  })
}

export type B3BasicWallet = typeof b3_basic_wallet
