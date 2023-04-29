import { Actor, HttpAgent } from "@dfinity/agent"
import { Principal } from "@dfinity/principal"
import { readFileSync } from "fs"
import { B3System } from "service/actor"
import { idlFactory } from "../src/declarations/b3_system"
import { initIdentity } from "./utils"

const consolePrincipalIC = () => {
  const buffer = readFileSync("./canister_ids.json")
  const { console } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(console.ic)
}

const consolePrincipalLocal = () => {
  const buffer = readFileSync("./.dfx/local/canister_ids.json")
  const { b3_system } = JSON.parse(buffer.toString("utf-8"))
  return Principal.fromText(b3_system.local)
}

export const consoleActorIC = async () => {
  const canisterId = consolePrincipalIC()

  const agent = icAgent()

  return Actor.createActor(idlFactory, {
    agent,
    canisterId
  })
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

export const consoleSystemLocal = async () => {
  const canisterId = consolePrincipalLocal()

  const agent = await localAgent()

  return Actor.createActor(idlFactory, {
    agent,
    canisterId
  }) as Promise<B3System>
}
