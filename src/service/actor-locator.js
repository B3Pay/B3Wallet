import { canisterId, createActor } from "declarations/hello"

export function makeHelloActor() {
  return createActor(canisterId, {
    agentOptions: {
      host: process.env.NEXT_PUBLIC_IC_HOST
    }
  })
}
