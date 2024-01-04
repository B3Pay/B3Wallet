import { CanisterStatusResponse } from "declarations/b3system/b3system.did"

export type ModuleHash = CanisterStatusResponse["module_hash"]

declare global {
  interface String {
    toTitleCase(): string
  }
}
