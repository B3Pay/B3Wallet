import { createReActorStore } from "@ic-reactor/store"
import {
  b3_system,
  canisterId,
  idlFactory
} from "../src/declarations/b3_system"

export type B3System = typeof b3_system

export const { callMethod, initialize } = createReActorStore<B3System>({
  canisterId,
  idlFactory,
  isLocal: process.env.NODE_ENV === "development"
})
