import { Actor, HttpAgent } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"
import { readFileSync } from "fs"
import { idlFactory as systemFactory } from "../frontend/declarations/b3_system"
import { idlFactory as userFactory } from "../frontend/declarations/b3_wallet"
import { B3System, B3Wallet } from "../frontend/src/service/actor"
import { initIdentity } from "./utils"

const systemPrincipalIC = () => {
  const buffer = readFileSync("./canister_ids.json")
  const { b3_system } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3_system.ic)
}

const systemPrincipalLocal = () => {
  const buffer = readFileSync("./.dfx/local/canister_ids.json")
  const { b3_system } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3_system.local)
}

const userPrincipalLocal = () => {
  const buffer = readFileSync("./.dfx/local/canister_ids.json")
  const { b3_wallet } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3_wallet.local)
}

export const icAgent = () => {
  const identity = initIdentity(true)

  // @ts-ignore
  return new HttpAgent({ identity, host: "https://icp0.io" })
}

export const localAgent = async () => {
  const identity = initIdentity(false)

  const agent = new HttpAgent({
    // @ts-ignore
    identity,
    host: "http://127.0.0.1:8080/"
  })

  await agent.fetchRootKey()

  return agent
}

export const systemLocalActor = async () => {
  const canisterId = systemPrincipalLocal()

  const agent = await localAgent()

  return Actor.createActor(systemFactory, {
    agent,
    canisterId
  }) as Promise<B3System>
}

export const systemActorIC = async () => {
  const canisterId = systemPrincipalIC()

  const agent = icAgent()

  return Actor.createActor(systemFactory, {
    agent,
    canisterId
  }) as Promise<B3System>
}

export const userLocalActor = async (canister_address?: string) => {
  const canisterId = canister_address
    ? Principal.fromText(canister_address)
    : userPrincipalLocal()

  const agent = await localAgent()

  return Actor.createActor(userFactory, {
    agent,
    canisterId
  }) as Promise<B3Wallet>
}
