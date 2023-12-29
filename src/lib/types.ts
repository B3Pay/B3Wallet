import { CanisterStatusResponse } from "declarations/b3_system/b3_system.did"

export type ModuleHash = CanisterStatusResponse["module_hash"]

declare global {
  interface String {
    toTitleCase(): string
  }
}
