import { HttpAgent, Identity } from "@dfinity/agent"
import {
  b3_basic_wallet,
  createActor as createBasicWalletActor
} from "declarations/b3_basic_wallet"
import {
  b3_wallet,
  createActor as createWalletActor
} from "declarations/b3_wallet"
import {
  b3_system,
  canisterId,
  createActor as createSystemActor
} from "../../declarations/b3_system"

export function getHttpAgent(identity: Identity) {
  console.log("getHttpAgent", process.env.NEXT_PUBLIC_IC_HOST)
  return new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
    identity
  })
}

export function makeB3BasicWalletActor(canisterId: string, identity: Identity) {
  const agent = getHttpAgent(identity)

  console.log("makeB3BasicWalletActor", canisterId, agent)
  return createBasicWalletActor(canisterId, {
    agent
  })
}

export function makeB3WalletActor(canisterId: string, identity: Identity) {
  const agent = getHttpAgent(identity)

  console.log("makeB3WalletActor", canisterId, agent)
  return createWalletActor(canisterId, {
    agent
  })
}

export function makeB3SystemActor(identity: Identity) {
  const agent = getHttpAgent(identity)

  return createSystemActor(canisterId, {
    agent
  })
}

export type B3Wallet = typeof b3_wallet

export type B3BasicWallet = typeof b3_basic_wallet

export type B3System = typeof b3_system
