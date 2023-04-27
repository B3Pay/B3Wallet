import { HttpAgent, Identity } from "@dfinity/agent"
import { b3_user, canisterId, createActor } from "declarations/b3_user"

export function makeB3UserActor(identity?: Identity) {
  const agent = new HttpAgent({
    host: process.env.NEXT_PUBLIC_IC_HOST,
    identity
  })

  return createActor(canisterId, {
    agent
  })
}

export type B3User = typeof b3_user
