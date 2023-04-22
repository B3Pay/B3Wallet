import { canisterId, createActor } from "declarations/b3_user"

export function makeHelloActor() {
  return createActor(canisterId, {
    agentOptions: {
      host: process.env.NEXT_PUBLIC_IC_HOST
    }
  })
}
