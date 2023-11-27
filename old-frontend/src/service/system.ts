import { Identity } from "@dfinity/agent"
import { getHttpAgent } from "service"
import {
  b3_system,
  canisterId,
  createActor
} from "../../declarations/b3_system"

export function createB3SystemActor(identity: Identity) {
  const agent = getHttpAgent(identity)

  return createActor(canisterId, {
    agent
  })
}

export type B3System = typeof b3_system
