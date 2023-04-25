import { b3_user, canisterId, createActor } from "declarations/b3_user"

export function makeB3UserActor() {
  return createActor(canisterId, {
    agentOptions: {
      host: process.env.NEXT_PUBLIC_IC_HOST
    }
  })
}

export type B3User = typeof b3_user
