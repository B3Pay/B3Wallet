import { HttpAgent, Identity } from "@dfinity/agent"
import {
  b3_system,
  canisterId,
  createActor as createSystemActor
} from "declarations/b3_system"
import { b3_user, createActor as createUserActor } from "declarations/b3_user"

export function makeB3UserActor(canisterId: string, identity: Identity) {
  const agent = new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
    identity
  })
  console.log("makeB3UserActor", canisterId, agent)
  return createUserActor(canisterId, {
    agent
  })
}

export function makeB3SystemActor(identity: Identity) {
  const agent = new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
    identity
  })

  return createSystemActor(canisterId, {
    agent
  })
}

export type B3User = typeof b3_user

export type B3System = typeof b3_system
