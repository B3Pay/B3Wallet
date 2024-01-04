import { Actor, HttpAgent } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"
import { readFileSync } from "fs"
import { idlFactory as systemFactory } from "../src/declarations/b3system"
import { idlFactory as userFactory } from "../src/declarations/b3wallet"
import { initIdentity } from "./utils"
import { B3System } from "./system"
import { B3Wallet } from "./wallet"

const systemPrincipalIC = () => {
  const buffer = readFileSync("./canister_ids.json")
  const { b3system } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3system.ic)
}

const systemPrincipalLocal = () => {
  const buffer = readFileSync("./.dfx/local/canister_ids.json")
  const { b3system } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3system.local)
}

const walletPrincipalLocal = () => {
  const buffer = readFileSync("./.dfx/local/canister_ids.json")
  const { b3wallet } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3wallet.local)
}

const walletPrincipalIC = () => {
  const buffer = readFileSync("./canister_ids.json")
  const { b3wallet } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3wallet.ic)
}

export const icAgent = () => {
  const identity = initIdentity(true)

  return new HttpAgent({ identity, host: "https://icp0.io" })
}

export const localAgent = async () => {
  const identity = initIdentity(false)

  const agent = new HttpAgent({
    identity,
    host: "http://127.0.0.1:4943/"
  })

  await agent.fetchRootKey()

  return { agent, identity }
}

export const systemLocalActor = async () => {
  const canisterId = systemPrincipalLocal()

  const { agent } = await localAgent()

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

export const walletLocalActor = async () => {
  const canisterId = walletPrincipalLocal()

  const { agent } = await localAgent()

  return Actor.createActor(userFactory, {
    agent,
    canisterId
  }) as Promise<B3Wallet>
}

export const walletActorIC = async () => {
  const canisterId = walletPrincipalIC()

  const agent = icAgent()

  return Actor.createActor(userFactory, {
    agent,
    canisterId
  }) as Promise<B3Wallet>
}
